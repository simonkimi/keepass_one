use serde::{Deserialize, Serialize};
use crate::kdbx::xml::entities::memory_protection::MemoryProtection;
use crate::kdbx::xml::entities::custom_data::CustomData;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Meta {
    /// Name of the application that has generated the XML document.
    ///
    /// 生成XML文档的应用程序的名称。
    #[serde(rename = "Generator")]
    pub generator: String,
    /// Last date/time when a database setting (stored in the Meta element) has been changed.
    ///
    /// 数据库设置（存储在Meta元素中）的最后更改日期/时间。
    #[serde(rename = "SettingsChanged")]
    pub settings_changed: String,
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "DatabaseNameChanged")]
    pub database_name_changed: String,
    #[serde(rename = "DatabaseDescription")]
    #[serde(default)]
    pub database_description: String,
    #[serde(rename = "DatabaseDescriptionChanged")]
    pub database_description_changed: String,
    /// User name that is used by default for new entries.
    ///
    /// 默认用于新条目的用户名。
    #[serde(rename = "DefaultUserName")]
    #[serde(default)]
    pub default_user_name: String,
    #[serde(rename = "DefaultUserNameChanged")]
    pub default_user_name_changed: String,
    /// Number of days until history entries are deleted in a database maintenance operation.
    ///
    /// 在数据库维护操作中删除历史记录条目之前的天数。
    #[serde(rename = "MaintenanceHistoryDays")]
    pub maintenance_history_days: u32,
    /// Database color. The user interface can colorize elements with this color in order to allow the user to quickly identify the database.
    ///
    /// 数据库颜色。用户界面可以使用此颜色对元素进行着色，以便用户快速识别数据库。
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: String,
    /// Last date/time when the master key has been changed.
    ///
    /// 主密钥最后更改的日期/时间。
    #[serde(rename = "MasterKeyChanged")]
    pub master_key_changed: String,
    /// Number of days until a change of the master key is recommended. -1 means never.
    ///
    /// 建议更改主密钥之前的天数。-1表示永不。
    #[serde(rename = "MasterKeyChangeRec")]
    pub master_key_change_rec: i32,
    /// Number of days until a change of the master key is enforced. -1 means never.
    ///
    /// 强制更改主密钥之前的天数。-1表示永不。
    #[serde(rename = "MasterKeyChangeForce")]
    pub master_key_change_force: i32,
    #[serde(rename = "MemoryProtection")]
    pub memory_protection: MemoryProtection,
    #[serde(rename = "RecycleBinEnabled")]
    pub recycle_bin_enabled: String,
    /// UUID of the group that is used as recycle bin. Zero UUID = create new group when necessary.
    ///
    /// 用作回收站的组的UUID。零UUID = 必要时创建新组。
    #[serde(rename = "RecycleBinUUID")]
    pub recycle_bin_uuid: String,
    #[serde(rename = "RecycleBinChanged")]
    pub recycle_bin_changed: String,
    #[serde(rename = "EntryTemplatesGroup")]
    pub entry_templates_group: String,
    #[serde(rename = "EntryTemplatesGroupChanged")]
    pub entry_templates_group_changed: String,
    /// Maximum number of history entries that each entry may have. -1 means unlimited.
    ///
    /// 每个条目可能拥有的最大历史记录条目数。-1表示无限制。
    #[serde(rename = "HistoryMaxItems")]
    pub history_max_items: i32,
    /// Maximum estimated size in bytes (in the process memory) of the history of each entry. -1 means unlimited.
    ///
    /// 每个条目历史记录的最大估计大小（在进程内存中），以字节为单位。-1表示无限制。
    #[serde(rename = "HistoryMaxSize")]
    pub history_max_size: i64,
    #[serde(rename = "LastSelectedGroup")]
    pub last_selected_group: String,
    #[serde(rename = "LastTopVisibleGroup")]
    pub last_top_visible_group: String,
    #[serde(rename = "CustomData")]
    #[serde(default)]
    pub custom_data: Option<CustomData>,
}
