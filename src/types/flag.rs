use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    pub namespace_key: String,
    pub flag_key: String,
    pub location: Location,
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "namespace_key: {} flag_key: {} [{}]",
            self.namespace_key, self.flag_key, self.location
        )
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "file: {} [line: {}, col: {}]",
            self.file, self.line, self.column
        )
    }
}
