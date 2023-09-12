use serde::{Deserialize, Serialize};

use super::location::Location;

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    pub namespace_key: Option<String>,
    pub key: Option<String>,
    pub context: Option<Vec<String>>,
    pub location: Location,
}
