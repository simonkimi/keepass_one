use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::kdbx::xml::entities::{TBase64Binary, TDateTime, TUuid};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Icon {
    #[serde(rename = "UUID")]
    pub uuid: TUuid,
    #[serde(rename = "LastModificationTime", default)]
    pub last_modification_time: TDateTime,
    #[serde(rename = "Data")]
    pub data: TBase64Binary,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct CustomIcon {
    #[serde(rename = "Icon", default)]
    pub icon: Vec<Icon>,
}