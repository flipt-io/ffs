use anyhow::{Ok, Result};
use clap::Parser;
use human_panic::setup_panic;
use std::collections::HashMap;
use std::{fmt, fs};
use tree_sitter::{Query, QueryCursor};
use walkdir::{DirEntry, WalkDir};

mod types;
use types::Location;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    language: Language,
    #[arg(short, long, help = "Path to output file (default STDOUT)")]
    output: Option<String>,
    #[arg(short, long, help = "Path to directory to scan (default .)")]
    directory: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Language {
    Go,
    Rust,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Language::Go => write!(f, "go"),
            Language::Rust => write!(f, "rust"),
        }
    }
}

fn main() -> Result<()> {
    setup_panic!();

    let args = Args::parse();

    let mut out_writer: Box<dyn std::io::Write> = match args.output {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };

    let dir = match args.directory {
        Some(s) => s,
        None => ".".to_string(),
    };

    let rules =
        fs::read_to_string(format!("./rules/{}.scm", args.language)).expect("Unable to read file");

    let ll = match args.language {
        Language::Go => tree_sitter_go::language(),
        Language::Rust => tree_sitter_rust::language(),
    };

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(ll).expect("Error loading grammar");
    let query = Query::new(ll, &rules).expect("Error loading query");

    // create hashmap of keys to Locations
    let mut locations = HashMap::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(is_go_file)
    {
        let path = entry.path().to_str().unwrap();
        parse_file(path, &mut parser, &query, &mut locations)?;
    }

    for (k, v) in locations {
        for loc in v {
            writeln!(out_writer, "{}: {}", k, loc)?;
        }
    }

    Ok(())
}

// TODO: make this generic
fn is_go_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".go"))
        .unwrap_or(false)
}

fn parse_file(
    input: &str,
    parser: &mut tree_sitter::Parser,
    query: &tree_sitter::Query,
    col: &mut HashMap<String, Vec<Location>>,
) -> Result<()> {
    let code = fs::read_to_string(input).expect("Unable to read file");
    let parsed = parser.parse(&code, None).expect("Error parsing code");

    let mut query_cursor = QueryCursor::new();
    let all_matches = query_cursor.matches(&query, parsed.root_node(), code.as_bytes());
    let flag_key_idx = query.capture_index_for_name("v").unwrap();

    for each_match in all_matches {
        for capture in each_match
            .captures
            .iter()
            .filter(|c| c.index == flag_key_idx)
        {
            let range = capture.node.range();
            let text = &code[range.start_byte..range.end_byte];
            let line = range.start_point.row;
            let column = range.start_point.column;

            let loc = Location {
                file: input.to_string(),
                line,
                column,
            };

            col.entry(text.to_string()).or_default().push(loc);
        }
    }

    Ok(())
}
