use serde::{Deserialize, Serialize};

/// A protected binary.
///
/// 受保护的二进制文件
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TProtectedBinaryDef {
    /// The ID of the protected binary.
    ///
    /// 受保护的二进制文件的ID
    #[serde(rename = "@ID")]
    pub id: String,
    /// Whether the protected binary is compressed.
    ///
    /// 受保护的二进制文件是否被压缩
    #[serde(rename = "@Compressed")]
    pub compressed: String,
    /// Whether the protected binary is protected.
    ///
    /// 受保护的二进制文件是否受保护
    #[serde(rename = "@Protected")]
    pub protected: String,
    /// The value of the protected binary.
    ///
    /// 受保护的二进制文件的值
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtectedBinary {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: ProtectedBinaryValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtectedBinaryValue {
    /// Reference to a binary content stored in the inner header (KDBX file) or in the Meta/Binaries element (unencrypted XML file).
    ///
    /// 对存储在内部标头（KDBX文件）或Meta/Binaries元素（未加密的XML文件）中的二进制内容的引用。
    #[serde(rename = "@Ref")]
    pub reference: u32,
    #[serde(rename = "$text", default)]
    pub value: String,
}
