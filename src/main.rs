use anyhow::Result;
use clap::Parser;
use ffs::scanner::Scanner;
use ffs::types;
use ffs::types::flag::Flag;
use human_panic::setup_panic;
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
    #[arg(short, long, help = "Display lines of context around flag")]
    pub context: bool,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Format {
    Json,
    Text,
}

fn main() -> Result<ExitCode> {
    setup_panic!();

    let args = Args::parse();

    let mut out_writer: Box<dyn std::io::Write> = match args.output {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };

    let mut ffs = Scanner::new(args.language, args.dir);

    let found_flags = ffs.scan()?;
    let filtered_flags = match args.namespace {
        Some(s) => found_flags
            .into_iter()
            .filter(|f| match &f.namespace_key {
                Some(n) => n == &s,
                None => false,
            })
            .collect(),
        None => found_flags,
    };

    if !filtered_flags.is_empty() {
        match args.format {
            Some(Format::Json) => {
                let writer = JSONWriter::new(filtered_flags);
                write!(out_writer, "{}", writer)?;
            }
            Some(Format::Text) | None => {
                writeln!(out_writer, "Found {} results:", filtered_flags.len())?;
                let writer = TextWriter::new(filtered_flags, args.context);
                write!(out_writer, "{}", writer)?;
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

struct JSONWriter {
    results: Vec<Flag>,
}

impl JSONWriter {
    fn new(results: Vec<Flag>) -> Self {
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
    results: Vec<Flag>,
    context: bool,
}

impl TextWriter {
    fn new(results: Vec<Flag>, context: bool) -> Self {
        TextWriter { results, context }
    }
}

impl fmt::Display for TextWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for flag in &self.results {
            if flag.namespace_key.is_some() && flag.key.is_some() {
                write!(
                    f,
                    "\n- Flag: [ Key: {}, Namespace: {} ]\n  File: {}\n",
                    flag.key.as_ref().unwrap(),
                    flag.namespace_key.as_ref().unwrap(),
                    flag.location.file
                )?;
            } else {
                write!(f, "\n- File: {}\n", flag.location.file)?;
            }

            writeln!(
                f,
                "  Line: [ Start: {}, End: {} ]\n  Column: [ Start: {}, End: {} ]",
                flag.location.start_line,
                flag.location.end_line,
                flag.location.start_column,
                flag.location.end_column
            )?;

            if self.context {
                write!(
                    f,
                    "\n```\n{}\n```\n",
                    flag.context.as_ref().unwrap().join("\n")
                )?;
            }
        }

        Ok(())
    }
}
