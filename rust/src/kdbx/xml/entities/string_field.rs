use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::kdbx::xml::entities::value::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct ProtectedString {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Value,
}
