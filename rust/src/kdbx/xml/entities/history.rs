use crate::kdbx::xml::entities::entry::Entry;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// https://keepass.info/help/v2/entry.html#hst
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone, Zeroize, ZeroizeOnDrop)]
pub struct History {
    #[serde(rename = "Entry", default)]
    pub entry: Vec<Entry>,
}
