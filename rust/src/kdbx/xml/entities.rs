use serde::{Deserialize, Serialize};

/// KDBX 4.1 XML Schema.
///
/// Copyright (C) 2007-2025 Dominik Reichl.
/// <https://keepass.info/help/kb/kdbx.html>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeePassFile {
    #[serde(rename = "Meta")]
    pub meta: Meta,
    #[serde(rename = "Root")]
    pub root: Root,
}

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

/// Process memory protection settings, describing which standard fields should be protected. KeePass resets these settings to their default values after opening a database.
///
/// 进程内存保护设置，描述应保护哪些标准字段。KeePass在打开数据库后将这些设置重置为其默认值。
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MemoryProtection {
    #[serde(rename = "ProtectTitle")]
    pub protect_title: String,
    #[serde(rename = "ProtectUserName")]
    pub protect_user_name: String,
    #[serde(rename = "ProtectPassword")]
    pub protect_password: String,
    #[serde(rename = "ProtectURL")]
    pub protect_url: String,
    #[serde(rename = "ProtectNotes")]
    pub protect_notes: String,
}

/// A custom icon.
///
/// 自定义图标
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CustomData {
    #[serde(rename = "Item", default)]
    pub item: Vec<Item>,
}

/// Custom data item (key/value pair) for plugins/ports.
///
/// 插件/端口的自定义数据项（键/值对）。
/// The key should be unique, e.g. "PluginName_ItemName".
///
/// 密钥应该是唯一的，例如“PluginName_ItemName”。
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Root {
    #[serde(rename = "Group")]
    pub group: Group,
    /// When the user deletes an object (group, entry, ...), an item is created in this list. When synchronizing/merging database files, this information can be used to decide whether an object has been deleted.
    ///
    /// 当用户删除一个对象（组、条目…）时，会在此列表中创建一个项目。在同步/合并数据库文件时，此信息可用于确定对象是否已被删除。
    #[serde(rename = "DeletedObjects")]
    #[serde(default)]
    pub deleted_objects: DeletedObjects,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct DeletedObjects {
    #[serde(rename = "DeletedObject", default)]
    pub deleted_object: Vec<DeletedObject>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeletedObject {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "DeletionTime")]
    pub deletion_time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Group {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Notes")]
    #[serde(default)]
    pub notes: String,
    /// See the folder "Ext/Images_Client_HighRes" in the KeePass source code package.
    ///
    /// 请参阅KeePass源代码包中的“Ext/Images_Client_HighRes”文件夹。
    #[serde(rename = "IconID")]
    pub icon_id: u32,
    /// Reference to a custom icon stored in the KeePassFile/Meta/CustomIcons element. If non-zero, it overrides IconID.
    ///
    /// 对存储在KeePassFile/Meta/CustomIcons元素中的自定义图标的引用。如果非零，则会覆盖IconID。
    #[serde(rename = "CustomIconUUID")]
    pub custom_icon_uuid: String,
    #[serde(rename = "Times")]
    pub times: Times,
    /// Specifies whether the group is displayed as expanded in the user interface.
    ///
    /// 指定组在用户界面中是否显示为展开。
    #[serde(rename = "IsExpanded")]
    pub is_expanded: String,
    #[serde(rename = "DefaultAutoTypeSequence")]
    #[serde(default)]
    pub default_auto_type_sequence: String,
    #[serde(rename = "EnableAutoType")]
    pub enable_auto_type: String,
    #[serde(rename = "EnableSearching")]
    pub enable_searching: String,
    #[serde(rename = "LastTopVisibleEntry")]
    pub last_top_visible_entry: String,
    /// UUID of the group in which the current group was stored previously. This information can for instance be used by a recycle bin restoration command.
    ///
    /// 当前组先前存储在其中的组的UUID。此信息可用于回收站还原命令。
    #[serde(rename = "PreviousParentGroup")]
    pub previous_parent_group: String,
    #[serde(rename = "Entry", default)]
    pub entry: Vec<Entry>,
    #[serde(rename = "Group", default)]
    pub group: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Times {
    #[serde(rename = "CreationTime")]
    pub creation_time: String,
    #[serde(rename = "LastModificationTime")]
    pub last_modification_time: String,
    /// In general, last access times are not reliable, because an access is not considered to be a database change.
    ///
    /// 通常，上次访问时间不可靠，因为访问不被视为数据库更改。
    /// See the UIFlags value 0x20000:
    ///
    /// 请参阅UIFlags值0x20000：
    /// <https://keepass.info/help/v2_dev/customize.html#uiflags>
    #[serde(rename = "LastAccessTime")]
    pub last_access_time: String,
    #[serde(rename = "ExpiryTime")]
    pub expiry_time: String,
    #[serde(rename = "Expires")]
    pub expires: String,
    /// Cf. LastAccessTime.
    ///
    /// 参阅LastAccessTime。
    #[serde(rename = "UsageCount")]
    pub usage_count: u32,
    /// Last date/time when the object has been moved (within its parent group or to a different group). This is used by the synchronization algorithm to determine the latest location of the object.
    ///
    /// 对象最后移动（在其父组内或移动到不同组）的日期/时间。同步算法使用此信息来确定对象的最新位置。
    #[serde(rename = "LocationChanged")]
    pub location_changed: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    #[serde(rename = "UUID")]
    pub uuid: String,
    /// See TGroup/IconID.
    ///
    /// 参阅TGroup/IconID。
    #[serde(rename = "IconID")]
    pub icon_id: u32,
    /// See TGroup/CustomIconUUID.
    ///
    /// 参阅TGroup/CustomIconUUID。
    #[serde(rename = "CustomIconUUID")]
    pub custom_icon_uuid: String,
    #[serde(rename = "ForegroundColor")]
    #[serde(default)]
    pub foreground_color: String,
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    pub background_color: String,
    /// <https://keepass.info/help/base/autourl.html#override>
    #[serde(rename = "OverrideURL")]
    #[serde(default)]
    pub override_url: String,
    /// <https://keepass.info/help/v2/entry.html#gen>
    /// <https://keepass.info/help/kb/pw_quality_est.html>
    #[serde(rename = "QualityCheck")]
    pub quality_check: String,
    /// See TGroup/Tags.
    ///
    /// 参阅TGroup/Tags。
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: String,
    /// See TGroup/PreviousParentGroup.
    ///
    /// 参阅TGroup/PreviousParentGroup。
    #[serde(rename = "PreviousParentGroup")]
    pub previous_parent_group: String,
    #[serde(rename = "Times")]
    pub times: Times,
    #[serde(rename = "String", default)]
    pub string: Vec<StringField>,
    #[serde(rename = "Binary", default)]
    pub binary: Vec<ProtectedBinary>,
    /// <https://keepass.info/help/base/autotype.html>
    #[serde(rename = "AutoType")]
    #[serde(default)]
    pub auto_type: Option<AutoType>,
    /// <https://keepass.info/help/v2/entry.html#hst>
    #[serde(rename = "History")]
    #[serde(default)]
    pub history: Option<History>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StringField {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Value,
}

fn is_option_string_empty(s: &Option<String>) -> bool {
    s.as_ref().map_or(true, |s| s.is_empty())
}

/// If the attribute is true, the content of the element has been encrypted (and Base64-encoded). See "inner encryption" on
///
/// 如果属性为true，则元素的内容已加密（并使用Base64编码）。请参阅“内部加密”：
/// <https://keepass.info/help/kb/kdbx.html>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Value {
    #[serde(rename = "@Protected", skip_serializing_if = "is_option_string_empty")]
    pub protected: Option<String>,
    #[serde(rename = "$text", default)]
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

/// https://keepass.info/help/base/autotype.html
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct AutoType {
    #[serde(rename = "Enabled")]
    pub enabled: String,
    #[serde(rename = "DataTransferObfuscation")]
    pub data_transfer_obfuscation: u32,
    #[serde(rename = "DefaultSequence")]
    pub default_sequence: String,
}

/// https://keepass.info/help/v2/entry.html#hst
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct History {
    #[serde(rename = "Entry", default)]
    pub entry: Vec<Entry>,
}
