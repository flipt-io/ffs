use crate::types::args::Args;
use crate::types::language::Language;
use crate::types::token::{Location, Token};
use anyhow::{Ok, Result};
use std::collections::HashMap;
use std::fs;
use tree_sitter::{Query, QueryCursor};
use walkdir::{DirEntry, WalkDir};

pub mod types;

pub struct FFS {
    args: Args,
    tokens: HashMap<String, Vec<Location>>,
}

impl FFS {
    pub fn new(args: Args) -> Self {
        FFS {
            args,
            tokens: HashMap::new(),
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        let mut out_writer: Box<dyn std::io::Write> = match &self.args.output {
            Some(s) => Box::new(std::fs::File::create(s)?),
            None => Box::new(std::io::stdout()),
        };

        let dir = match self.args.directory.to_owned() {
            Some(s) => s,
            None => ".".to_string(),
        };

        let rules = fs::read_to_string(format!("./rules/{}.scm", self.args.language))
            .expect("Unable to read file");

        let ll = Language::from(self.args.language.to_string());

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(ll.tree_sitter)
            .expect("Error loading grammar");
        let query = Query::new(ll.tree_sitter, &rules).expect("Error loading query");

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| is_file_ext(e, &ll.file_extension))
        {
            let path = entry.path().to_str().unwrap();
            let tokens = &mut self.tokens;
            parse_file(path, &mut parser, &query, tokens)?;
        }

        for (k, v) in &self.tokens {
            for loc in v {
                let t = Token {
                    key: k.to_string(),
                    loc: loc.clone(),
                };

                let json = serde_json::to_string(&t)?;
                writeln!(out_writer, "{json}")?;
            }
        }
        Ok(())
    }
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
