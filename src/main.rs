use std::fmt;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use colored::*;
use ffs::{ffs::scanner::Scanner, types};
use human_panic::setup_panic;
use serde::Serialize;

use futures::future::join_all;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "1", help = "Override exit code on issues found")]
    pub issue_exit_code: u8,
    #[arg(short, long, value_enum)]
    pub language: types::language::SupportedLanguage,
    #[arg(short, long, help = "Path to output file [default: STDOUT]")]
    pub output: Option<String>,
    #[arg(short, long, value_enum, default_value = "text")]
    pub format: Option<Format>,
    #[arg(short, long, help = "Path to directory to scan [default: .]")]
    pub dir: Option<String>,
    #[arg(short, long, help = "Namespace to scan [default: 'default']")]
    pub namespace: Option<String>,
}

const NOT_FOUND_ERROR_CODE: i32 = 5;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Format {
    Json,
    Text,
}

#[tokio::main]
async fn main() -> Result<ExitCode> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);

    let found_flags = ffs.scan()?;

    let flipt_config = flipt::Config::new_from_env().unwrap_or_default();

    // checks if the flipt server is up and running
    flipt::meta::MetaClient::new(flipt_config.clone())?
        .info()
        .get()
        .await?;

    let flipt_client = &flipt::api::ApiClient::new(flipt_config)?;

    let flag_results: Vec<types::flag::Flag> =
        join_all(found_flags.into_iter().map(|f| async move {
            let resp = flipt_client
                .flags()
                .get(&flipt::api::flag::FlagGetRequest {
                    namespace_key: Some(f.namespace_key.clone()),
                    key: f.key.clone(),
                })
                .await;

            match resp {
                Ok(_) => None,
                Err(error) => match error.downcast_ref::<flipt::error::Error>() {
                    Some(flipt::error::Error::Upstream(e)) => {
                        if e.code == NOT_FOUND_ERROR_CODE {
                            Some(f)
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
            }
        }))
        .await
        .into_iter()
        .flatten()
        .collect();

    let mut out_writer: Box<dyn std::io::Write> = match args.output {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };

    // ensure all found flags exist in flipt, write to output if not
    if !flag_results.is_empty() {
        let results = Results {
            errors: flag_results
                .into_iter()
                .map(|f| Error {
                    message: format!(
                        "Flag: [key: {}, namespace: {}] not found in your Flipt instance",
                        f.key, f.namespace_key
                    ),
                    flag: f,
                })
                .collect(),
        };

        match args.format {
            Some(Format::Json) => {
                let writer = JSONWriter::new(results);
                write!(out_writer, "{}", writer)?;
            }
            Some(Format::Text) => {
                write!(out_writer, "{}", "Error: ".bright_red())?;
                writeln!(out_writer, "Found {} issues\n", results.errors.len())?;
                let writer = TextWriter::new(results);
                write!(out_writer, "{}", writer)?;
            }
            None => {
                write!(out_writer, "{}", "Error: ".bright_red())?;
                writeln!(out_writer, "Found {} issues\n", results.errors.len())?;
                let writer = TextWriter::new(results);
                write!(out_writer, "{}", writer)?;
            }
        }

        return Ok(ExitCode::from(args.issue_exit_code));
    }

    Ok(ExitCode::SUCCESS)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Results {
    errors: Vec<Error>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Error {
    message: String,
    flag: crate::types::flag::Flag,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- Message: {}\n  File: {}\n  Line: {{ Start: {}, End: {} }}\n  Column: {{ Start: {}, End: {} }}", self.message, self.flag.location.file, self.flag.location.start_line, self.flag.location.end_line, self.flag.location.start_column, self.flag.location.end_column)
    }
}

struct JSONWriter {
    results: Results,
}

impl JSONWriter {
    fn new(results: Results) -> Self {
        JSONWriter { results }
    }
}

impl fmt::Display for JSONWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(&self.results).unwrap();
        writeln!(f, "{}", json)
    }
}

struct TextWriter {
    results: Results,
}

impl TextWriter {
    fn new(results: Results) -> Self {
        TextWriter { results }
    }
}

impl fmt::Display for TextWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in &self.results.errors {
            writeln!(f, "{}\n", error)?;
        }
        Ok(())
    }
}
