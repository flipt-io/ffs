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
        // keep track of all flags found, using the file path, start line, and end line as the key for deduplication
        let mut flags = HashMap::new();

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

                let range = root.node.range();

                let file = path;
                let start_line = range.start_point.row + 1;
                let end_line = range.end_point.row + 1;

                let start_byte = range.start_byte;
                let end_byte = range.end_byte;

                let before_line = code[..start_byte].lines().count().saturating_sub(10);
                let after_line = code[..end_byte].lines().count().saturating_add(10);

                // Print 10 lines before and after the node
                let surrounding_lines: Vec<_> = code
                    .lines()
                    .skip(before_line - 1)
                    .take(after_line - before_line + 1)
                    .map(|l| l.to_string())
                    .collect();

                let implicit = match captures.get("args") {
                    Some(_n) => false,
                    None => true,
                };

                let location = Location {
                    file: file.to_string(),
                    start_line,
                    start_column: range.start_point.column + 1,
                    end_line,
                    end_column: range.end_point.column + 1,
                };

                if !implicit {
                    let namespace_key = match captures.get("namespace_value") {
                        Some(n) => n.node.utf8_text(code.as_bytes()).unwrap(),
                        None => "default",
                    };

                    let flag_key = match captures.get("flag_value") {
                        Some(n) => n.node.utf8_text(code.as_bytes()).unwrap(),
                        None => "unknown",
                    };

                    flags
                        .entry(format!("{file}/{start_line}/{end_line}"))
                        .or_insert(Flag {
                            namespace_key: Some(unescape(namespace_key).unwrap()),
                            key: Some(unescape(flag_key).unwrap()),
                            location,
                            context: Some(surrounding_lines),
                        });
                } else {
                    flags
                        .entry(format!("{file}/{start_line}/{end_line}"))
                        .or_insert(Flag {
                            namespace_key: None,
                            key: None,
                            location,
                            context: Some(surrounding_lines),
                        });
                }
            }
        }
        Ok(flags.values().cloned().collect())
    }
}

fn is_file_ext(entry: &DirEntry, ext: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(ext))
        .unwrap_or(false)
}
