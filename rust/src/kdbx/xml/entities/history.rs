use serde::{Deserialize, Serialize};
use crate::kdbx::xml::entities::entry::Entry;

/// https://keepass.info/help/v2/entry.html#hst
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct History {
    #[serde(rename = "Entry", default)]
    pub entry: Vec<Entry>,
}
