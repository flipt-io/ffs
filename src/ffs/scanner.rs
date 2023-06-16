use crate::types::{
    flag::Flag,
    language::{Language, SupportedLanguage},
    location::Location,
};
use anyhow::{Ok, Result};
use rust_embed::RustEmbed;
use snailquote::unescape;
use std::{collections::HashMap, fs};
use tree_sitter::{Query, QueryCapture, QueryCursor};
use walkdir::{DirEntry, WalkDir};

pub struct Scanner {
    language: SupportedLanguage,
    dir: Option<String>,
}

#[derive(RustEmbed)]
#[folder = "rules/"]
struct Rules;

impl Scanner {
    pub fn new(language: SupportedLanguage, dir: Option<String>) -> Self {
        Scanner { language, dir }
    }

    /// Scan the directory for files for the given language and find all flag keys along with their locations.
    pub fn scan(&mut self) -> Result<Vec<Flag>> {
        let mut flags = Vec::new();

        let dir = match self.dir.to_owned() {
            Some(s) => s,
            None => ".".to_string(),
        };

        let f = match Rules::get(&format!("{}.scm", self.language)) {
            Some(s) => s,
            None => panic!("Unable to find rules for language {}", self.language),
        };

        let rules = std::str::from_utf8(f.data.as_ref()).expect("Unable to load rules");

        let ll = Language::from(self.language.to_string());

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(ll.tree_sitter)
            .expect("Error loading grammar");
        let query = Query::new(ll.tree_sitter, rules).expect("Error loading query");

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| is_file_ext(e, &ll.file_extension))
        {
            let path = entry.path().to_str().unwrap();

            let code = fs::read_to_string(path).expect("Unable to read file");
            let parsed = parser.parse(&code, None).expect("Error parsing code");

            let mut cursor = QueryCursor::new();
            for m in cursor.matches(&query, parsed.root_node(), code.as_bytes()) {
                // captures is a hashmap that stores the value of each capture in the query
                let captures: HashMap<_, _> = m
                    .captures
                    .iter()
                    .map(|c: &QueryCapture| (query.capture_names()[c.index as usize].clone(), c))
                    .collect();

                // root node of the query match
                let root = captures["call"];

                let namespace_key = match captures.get("namespace") {
                    Some(n) => n.node.utf8_text(code.as_bytes()).unwrap(),
                    None => "default",
                };

                let flag_key = match captures.get("flag") {
                    Some(n) => n.node.utf8_text(code.as_bytes()).unwrap(),
                    None => "",
                };

                let range = root.node.range();

                flags.push(Flag {
                    namespace_key: unescape(namespace_key).unwrap(),
                    key: unescape(flag_key).unwrap(),
                    location: Location {
                        file: path.to_string(),
                        start_line: range.start_point.row + 1,
                        start_column: range.start_point.column + 1,
                        end_line: range.end_point.row + 1,
                        end_column: range.end_point.column + 1,
                    },
                });
            }
        }

        Ok(flags)
    }
}

fn is_file_ext(entry: &DirEntry, ext: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(ext))
        .unwrap_or(false)
}
