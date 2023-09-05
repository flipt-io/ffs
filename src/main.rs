use anyhow::Result;
use clap::Parser;
use ffs::scanner::Scanner;
use ffs::types;
use human_panic::setup_panic;
use serde::Serialize;
use std::fmt;
use std::process::ExitCode;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub language: types::language::SupportedLanguage,
    #[arg(short, long, help = "Path to output file [default: STDOUT]")]
    pub output: Option<String>,
    #[arg(short, long, value_enum, default_value = "text")]
    pub format: Option<Format>,
    #[arg(short, long, help = "Path to directory to scan [default: .]")]
    pub dir: Option<String>,
    #[arg(short, long, help = "Namespace to filter [default: '']")]
    pub namespace: Option<String>,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Format {
    Json,
    Text,
}

fn main() -> Result<ExitCode> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);

    let found_flags = ffs.scan()?;

    let mut out_writer: Box<dyn std::io::Write> = match args.output {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };

    if !found_flags.is_empty() {
        let results = found_flags
            .into_iter()
            .filter(|f| {
                if let Some(ns) = &args.namespace {
                    f.namespace_key == *ns
                } else {
                    true
                }
            })
            .map(|f| Res {
                message: format!(
                    "Found flag: [key: {}, namespace: {}]",
                    f.key, f.namespace_key
                ),
                flag: f,
            })
            .collect();

        match args.format {
            Some(Format::Json) => {
                let writer = JSONWriter::new(results);
                write!(out_writer, "{}", writer)?;
            }
            Some(Format::Text) | None => {
                writeln!(out_writer, "Found {} results\n", results.len())?;
                let writer = TextWriter::new(results);
                write!(out_writer, "{}", writer)?;
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Res {
    message: String,
    flag: crate::types::flag::Flag,
}

impl fmt::Display for Res {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- {}\n  File: {}\n  Line: {{ Start: {}, End: {} }}\n  Column: {{ Start: {}, End: {} }}", self.message, self.flag.location.file, self.flag.location.start_line, self.flag.location.end_line, self.flag.location.start_column, self.flag.location.end_column)
    }
}

struct JSONWriter {
    results: Vec<Res>,
}

impl JSONWriter {
    fn new(results: Vec<Res>) -> Self {
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
    results: Vec<Res>,
}

impl TextWriter {
    fn new(results: Vec<Res>) -> Self {
        TextWriter { results }
    }
}

impl fmt::Display for TextWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.results {
            writeln!(f, "{}\n", r)?;
        }
        Ok(())
    }
}
