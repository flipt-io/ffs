use clap::Parser;
use human_panic::setup_panic;
use std::{fmt, fs};
use tree_sitter::{Query, QueryCursor};

mod types;
use types::{Location, Token};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    language: Language,
    #[arg(short, long)]
    input: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Language {
    Go,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Language::Go => write!(f, "go"),
        }
    }
}

fn main() {
    setup_panic!();

    let args = Args::parse();

    let mut parser = tree_sitter::Parser::new();

    let lang = match args.language {
        Language::Go => tree_sitter_go::language(),
    };

    parser.set_language(lang).expect("Error loading grammar");

    let code = fs::read_to_string(&args.input).expect("Unable to read file");
    let parsed = parser.parse(&code, None).expect("Error parsing code");

    let rules =
        fs::read_to_string(format!("./rules/{}.scm", args.language)).expect("Unable to read file");

    let query = Query::new(lang, &rules).expect("Error loading query");
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

            let t = Token {
                key: text.to_string(),
                location: Location {
                    file: args.input.to_string(),
                    line,
                    column,
                },
            };

            println!("{}", t);
        }
    }
}
