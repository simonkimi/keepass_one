use serde::{Deserialize, Serialize};

/// https://keepass.info/help/base/autotype.html
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct AutoType {
    #[serde(rename = "Enabled")]
    pub enabled: String,
    #[serde(rename = "DataTransferObfuscation")]
    pub data_transfer_obfuscation: u32,
    #[serde(rename = "DefaultSequence")]
    pub default_sequence: Option<String>,
}
