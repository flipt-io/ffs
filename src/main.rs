use crate::types::language::{Language, SupportedLanguage};
use crate::types::token::{Location, Token};
use anyhow::{Ok, Result};
use clap::Parser;
use human_panic::setup_panic;
use std::collections::HashMap;
use std::fs;
use tree_sitter::{Query, QueryCursor};
use walkdir::{DirEntry, WalkDir};
mod types;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    language: SupportedLanguage,
    #[arg(short, long, help = "Path to output file (default STDOUT)")]
    output: Option<String>,
    #[arg(short, long, help = "Path to directory to scan (default .)")]
    directory: Option<String>,
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

    let ll = Language::from(args.language.to_string());

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(ll.tree_sitter)
        .expect("Error loading grammar");
    let query = Query::new(ll.tree_sitter, &rules).expect("Error loading query");

    // create hashmap of keys to Locations
    let mut locations = HashMap::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_file_ext(e, &ll.file_extension))
    {
        let path = entry.path().to_str().unwrap();
        parse_file(path, &mut parser, &query, &mut locations)?;
    }

    for (k, v) in locations {
        for loc in v {
            let t = Token {
                key: k.to_string(),
                loc,
            };

            let json = serde_json::to_string(&t)?;
            writeln!(out_writer, "{json}")?;
        }
    }

    Ok(())
}

fn is_file_ext(entry: &DirEntry, ext: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(ext))
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
    let all_matches = query_cursor.matches(query, parsed.root_node(), code.as_bytes());
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
