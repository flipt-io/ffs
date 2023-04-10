use std::fmt;

pub struct Language {
    pub name: SupportedLanguage,
    pub tree_sitter: tree_sitter::Language,
    pub file_extension: String,
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        match s.as_str() {
            "go" => Language {
                name: SupportedLanguage::Go,
                tree_sitter: tree_sitter_go::language(),
                file_extension: ".go".to_string(),
            },
            &_ => todo!("Language not supported"),
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SupportedLanguage {
    Go,
    //  Rust,
}

impl fmt::Display for SupportedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SupportedLanguage::Go => write!(f, "go"),
        }
    }
}
