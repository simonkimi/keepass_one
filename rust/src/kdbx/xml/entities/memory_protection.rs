use serde::{Deserialize, Serialize};

use crate::kdbx::xml::entities::TBool;

/// Process memory protection settings, describing which standard fields should be protected. KeePass resets these settings to their default values after opening a database.
///
/// 进程内存保护设置，描述应保护哪些标准字段。KeePass在打开数据库后将这些设置重置为其默认值。
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MemoryProtection {
    #[serde(rename = "ProtectTitle")]
    pub protect_title: TBool,
    #[serde(rename = "ProtectUserName")]
    pub protect_user_name: TBool,
    #[serde(rename = "ProtectPassword")]
    pub protect_password: TBool,
    #[serde(rename = "ProtectURL")]
    pub protect_url: TBool,
    #[serde(rename = "ProtectNotes")]
    pub protect_notes: TBool,
}
