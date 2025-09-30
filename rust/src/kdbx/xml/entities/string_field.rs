use serde::{Deserialize, Serialize};
use crate::kdbx::xml::entities::value::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct StringField {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Value,
}
