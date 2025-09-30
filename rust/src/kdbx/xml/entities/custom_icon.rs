use serde::{Deserialize, Serialize};

/// A custom icon.
///
/// 自定义图标
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CustomIcon {
    /// The UUID of the custom icon.
    ///
    /// 自定义图标的UUID
    #[serde(rename = "UUID")]
    pub uuid: String,
    /// The data of the custom icon (PNG, Base64-encoded).
    ///
    /// 自定义图标的数据 (PNG, Base64编码)
    #[serde(rename = "Data")]
    pub data: String,
    /// The name of the custom icon.
    ///
    /// 自定义图标的名称
    #[serde(rename = "Name")]
    pub name: String,
}
