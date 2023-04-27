use crate::types::{
    flag::{Flag, Location},
    language::{Language, SupportedLanguage},
};
use anyhow::{Ok, Result};
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
    pub fn scan(&mut self) -> Result<Vec<Flag>> {
        let mut flags = Vec::new();

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

            let code = fs::read_to_string(path).expect("Unable to read file");
            let parsed = parser.parse(&code, None).expect("Error parsing code");

            let mut query_cursor = QueryCursor::new();
            let fn_arg_index = query.capture_index_for_name("arg").unwrap();

            for each_match in query_cursor.matches(&query, parsed.root_node(), code.as_bytes()) {
                for capture in each_match
                    .captures
                    .iter()
                    .filter(|c| c.index == fn_arg_index)
                {
                    // get namespace and flag key from the function call
                    let namespace_index = query.capture_index_for_name("namespaceValue").unwrap();
                    let flag_index = query.capture_index_for_name("flagValue").unwrap();

                    let namespace_key = each_match
                        .captures
                        .iter()
                        .find(|c| c.index == namespace_index)
                        .unwrap()
                        .node
                        .utf8_text(code.as_bytes())
                        .unwrap_or("default");

                    let flag_key = each_match
                        .captures
                        .iter()
                        .find(|c| c.index == flag_index)
                        .unwrap()
                        .node
                        .utf8_text(code.as_bytes())
                        .unwrap_or_default();

                    let range = capture.node.range();

                    let flag = Flag {
                        namespace_key: namespace_key.to_string(),
                        key: flag_key.to_string(),
                        loc: Location {
                            file: path.to_string(),
                            line: range.start_point.row,
                            column: range.start_point.column,
                        },
                    };

                    flags.push(flag.clone());
                }
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
