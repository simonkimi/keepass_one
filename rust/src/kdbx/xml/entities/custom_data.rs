use serde::{Deserialize, Serialize};

use crate::kdbx::xml::entities::TDateTime;

use zeroize::{Zeroize, ZeroizeOnDrop};
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone, Zeroize, ZeroizeOnDrop)]
pub struct CustomData {
    #[serde(rename = "Item", default)]
    pub item: Vec<Item>,
}

/// Custom data item (key/value pair) for plugins/ports.
///
/// 插件/端口的自定义数据项（键/值对）。
/// The key should be unique, e.g. "PluginName_ItemName".
///
/// 密钥应该是唯一的，例如"PluginName_ItemName"。
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Item {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "LastModificationTime", default)]
    pub last_modification_time: TDateTime,
}
