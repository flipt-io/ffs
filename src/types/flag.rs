use serde::{Deserialize, Serialize};

use super::location::Location;

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    pub namespace_key: Option<String>,
    pub key: Option<String>,
    pub location: Location,
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.namespace_key.is_none() && self.key.is_none() {
            write!(f, "[{}]", self.location)?;
            return Ok(());
        }

        let namespace_key = match &self.namespace_key {
            Some(s) => s,
            None => "default",
        };

        let key = match &self.key {
            Some(s) => s,
            None => "unknown",
        };

        write!(
            f,
            "namespace_key: {} key: {} [{}]",
            namespace_key, key, self.location
        )
    }
}
