pub struct Token {
    pub key: String,
    pub location: Location,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File: {} [Line: {}, Col: {}] Key: `{}`",
            self.location.file, self.location.line, self.location.column, self.key
        )
    }
}

pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
}
