use serde::{Deserialize, Serialize};

use super::location::Location;

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    pub namespace_key: String,
    pub key: String,
    pub location: Location,
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "namespace_key: {} key: {} [{}]",
            self.namespace_key, self.key, self.location
        )
    }
}
