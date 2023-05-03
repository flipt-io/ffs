use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use ffs::{ffs::scanner::Scanner, types};
use human_panic::setup_panic;
use serde::Serialize;

use futures::future::join_all;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub language: types::language::SupportedLanguage,
    #[arg(short, long, help = "Path to output file (default: STDOUT)")]
    pub output: Option<String>,
    #[arg(short, long, help = "Path to directory to scan (default: .)")]
    pub dir: Option<String>,
    #[arg(short, long, help = "Namespace to scan (default: 'default')")]
    pub namespace: Option<String>,
}

const NOT_FOUND_ERROR_CODE: i32 = 5;

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
                    key: f.flag_key.clone(),
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
                        f.flag_key, f.namespace_key
                    ),
                    location: f.location,
                })
                .collect(),
        };
        let json = serde_json::to_string(&results)?;
        writeln!(out_writer, "{json}")?;
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
    location: crate::types::location::Location,
}
