use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// https://keepass.info/help/base/autotype.html
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone, Zeroize, ZeroizeOnDrop)]
pub struct AutoType {
    #[serde(rename = "Enabled")]
    pub enabled: String,
    #[serde(rename = "DataTransferObfuscation")]
    pub data_transfer_obfuscation: u32,
    #[serde(rename = "DefaultSequence")]
    pub default_sequence: Option<String>,
}
