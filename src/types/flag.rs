use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    pub namespace_key: String,
    pub key: String,
    pub loc: Location,
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Namespace: {} Key: {} [{}]",
            self.namespace_key, self.key, self.loc
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
            "File: {} [Line: {}, Col: {}]",
            self.file, self.line, self.column
        )
    }
}
