use serde::{Deserialize, Serialize};

fn is_option_string_empty(s: &Option<String>) -> bool {
    s.as_ref().map_or(true, |s| s.is_empty())
}

/// If the attribute is true, the content of the element has been encrypted (and Base64-encoded). See "inner encryption" on
///
/// 如果属性为true，则元素的内容已加密（并使用Base64编码）。请参阅"内部加密"：
/// <https://keepass.info/help/kb/kdbx.html>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Value {
    #[serde(rename = "@Protected", skip_serializing_if = "is_option_string_empty")]
    pub protected: Option<String>,
    #[serde(rename = "$text", default)]
    pub value: String,
}

impl Value {
    pub fn is_protected(&self) -> bool {
        self.protected.as_ref().map_or(false, |s| s == "True")
    }
}
