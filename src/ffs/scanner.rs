use crate::types::language::{Language, SupportedLanguage};
use crate::types::token::{Location, TokenSet};
use anyhow::{Ok, Result};
use std::collections::HashMap;
use std::fs;
use tree_sitter::{Query, QueryCursor};
use walkdir::{DirEntry, WalkDir};

pub struct Scanner {
    language: SupportedLanguage,
    dir: Option<String>,
}

impl Scanner {
    pub fn new(language: SupportedLanguage, dir: Option<String>) -> Self {
        Scanner { language, dir }
    }

    /// Scan the directory for files for the given language and find all flag keys along with their locations.
    pub fn scan(&mut self) -> Result<TokenSet> {
        let mut tokens: TokenSet = HashMap::new();

        let dir = match self.dir.to_owned() {
            Some(s) => s,
            None => ".".to_string(),
        };

        let rules = fs::read_to_string(format!("./rules/{}.scm", self.language))
            .expect("Unable to read file");

        let ll = Language::from(self.language.to_string());

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
            parse_file(path, &mut parser, &query, &mut tokens)?;
        }

        Ok(tokens)
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
