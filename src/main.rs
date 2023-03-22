use std::fs;
use human_panic::setup_panic;
use tree_sitter::{Parser, Query, QueryCursor};


mod types;
use types::{Token, Location};

fn main() {
    setup_panic!();

    let code = fs::read_to_string("./examples/go/basic.go").expect("Unable to read file");

    let mut parser = Parser::new();

    let go_lang = tree_sitter_go::language();
    parser
        .set_language(go_lang)
        .expect("Error loading Go grammar");

    let parsed = parser.parse(&code, None).expect("Error parsing code");

    let rules = fs::read_to_string("./rules/go.scm").expect("Unable to read file");

    let query = Query::new(go_lang, &rules).expect("Error loading query");
    let mut query_cursor = QueryCursor::new();
    let all_matches = query_cursor.matches(&query, parsed.root_node(), code.as_bytes());
    let flag_key_idx = query.capture_index_for_name("v").unwrap();

    for each_match in all_matches {
        // iterate over all captures called "raise"
        for capture in each_match
            .captures
            .iter()
            .filter(|c| c.index == flag_key_idx)
        {
            let range = capture.node.range();
            let text = &code[range.start_byte..range.end_byte];
            let line = range.start_point.row;
            let column = range.start_point.column;

            let _t = Token {
                key: text.to_string(),
                location: Location {
                    file: "basic.go".to_string(),
                    line,
                    column,
                },
            };

            println!("[Line: {}, Col: {}] Found flagKey: `{}`", line, column, text);
        }
    }
}
