use base64;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// KeePass文件的根元素
/// Root element of KeePass database file
#[derive(Debug, Serialize, Deserialize)]
pub struct KeePassFile {
    /// 元数据
    /// Metadata of the database
    pub meta: Meta,
    /// 根组
    /// Root group
    pub root: Root,
}

/// 元数据部分
/// Meta section containing database metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// 生成XML文档的应用程序名称
    /// Name of the application that generated the XML document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    
    /// KDBX文件头的哈希（仅用于KDBX 4之前的版本）
    /// Hash of the KDBX file header (only for versions before KDBX 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_hash: Option<String>,
    
    /// 数据库设置最后修改时间
    /// Time when database settings were last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_changed: Option<TimeData>,
    
    /// 数据库名称
    /// Name of the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    
    /// 数据库名称修改时间
    /// Time when database name was last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name_changed: Option<TimeData>,
    
    /// 数据库描述
    /// Description of the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_description: Option<String>,
    
    /// 数据库描述修改时间
    /// Time when database description was last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_description_changed: Option<TimeData>,
    
    /// 默认用户名
    /// Default username for new entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_user_name: Option<String>,
    
    /// 默认用户名修改时间
    /// Time when default username was last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_user_name_changed: Option<TimeData>,
    
    /// 维护历史天数
    /// Number of days to maintain history
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_history_days: Option<u32>,
    
    /// 数据库颜色
    /// Color of the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    
    /// 主密钥修改时间
    /// Time when master key was last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_key_changed: Option<TimeData>,
    
    /// 主密钥推荐更改天数，-1表示永不
    /// Number of days after which master key change is recommended, -1 means never
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_key_change_rec: Option<i64>,
    
    /// 主密钥强制更改天数，-1表示永不
    /// Number of days after which master key change is forced, -1 means never
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_key_change_force: Option<i64>,
    
    /// 是否在用户打开数据库后强制更改主密钥一次
    /// Whether to force master key change once after user opens the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_key_change_force_once: Option<bool>,
    
    /// 内存保护配置
    /// Memory protection configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_protection: Option<MemoryProtectionConfig>,
    
    /// 自定义图标
    /// Custom icons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_icons: Option<CustomIcons>,
    
    /// 回收站是否启用
    /// Whether recycle bin is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_bin_enabled: Option<bool>,
    
    /// 回收站组的UUID
    /// UUID of the recycle bin group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_bin_uuid: Option<Uuid>,
    
    /// 回收站修改时间
    /// Time when recycle bin settings were last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_bin_changed: Option<TimeData>,
    
    /// 条目模板组UUID
    /// UUID of the entry templates group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_templates_group: Option<Uuid>,
    
    /// 条目模板组修改时间
    /// Time when entry templates group was last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_templates_group_changed: Option<TimeData>,
    
    /// 每个条目的最大历史条目数，-1表示无限制
    /// Maximum number of history items per entry, -1 means unlimited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_max_items: Option<i32>,
    
    /// 每个条目历史的最大估计大小（字节），-1表示无限制
    /// Maximum estimated size of history per entry in bytes, -1 means unlimited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_max_size: Option<i64>,
    
    /// 最后选择的组UUID
    /// UUID of the last selected group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_selected_group: Option<Uuid>,
    
    /// 最后可见的顶部组UUID
    /// UUID of the last top visible group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_top_visible_group: Option<Uuid>,
    
    /// 二进制数据（仅用于未加密的XML文件和KDBX 4之前的版本）
    /// Binary data (only for unencrypted XML files and versions before KDBX 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binaries: Option<Binaries>,
    
    /// 自定义数据
    /// Custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataWithTimes>,
}

/// 内存保护配置
/// Memory protection configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryProtectionConfig {
    /// 是否保护标题
    /// Whether to protect title field in memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_title: Option<bool>,
    
    /// 是否保护用户名
    /// Whether to protect username field in memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_user_name: Option<bool>,
    
    /// 是否保护密码
    /// Whether to protect password field in memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_password: Option<bool>,
    
    /// 是否保护URL
    /// Whether to protect URL field in memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_url: Option<bool>,
    
    /// 是否保护备注
    /// Whether to protect notes field in memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_notes: Option<bool>,
}

/// 自定义图标集合
/// Collection of custom icons
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomIcons {
    /// 图标列表
    /// List of custom icons
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub icon: Vec<CustomIcon>,
}

/// 自定义图标
/// Custom icon definition
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomIcon {
    /// 图标UUID
    /// UUID of the icon
    pub uuid: Uuid,
    
    /// 图标数据（Base64编码）
    /// Icon data (Base64 encoded)
    pub data: String,
    
    /// 图标名称
    /// Name of the icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// 最后修改时间
    /// Last modification time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<TimeData>,
}

/// 二进制数据集合
/// Collection of binary data
#[derive(Debug, Serialize, Deserialize)]
pub struct Binaries {
    /// 二进制数据列表
    /// List of binary data definitions
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub binary: Vec<ProtectedBinaryDef>,
}

/// 受保护的二进制数据定义
/// Protected binary data definition
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtectedBinaryDef {
    /// 数据ID
    /// ID of the binary data
    pub id: u32,
    
    /// 是否压缩
    /// Whether the data is compressed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    
    /// 是否受保护
    /// Whether the data is protected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    
    /// 数据内容（Base64编码）
    /// Data content (Base64 encoded)
    pub value: String,
}

/// 受保护的二进制数据
/// Protected binary data
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtectedBinary {
    /// 键名
    /// Key name
    pub key: String,
    
    /// 值
    /// Value
    pub value: BinaryValue,
}

/// 二进制值
/// Binary value
#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryValue {
    /// 引用ID
    /// Reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<u32>,
    
    /// 数据内容（Base64编码）
    /// Data content (Base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// 受保护的字符串
/// Protected string
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtectedString {
    /// 键名
    /// Key name
    pub key: String,
    
    /// 值
    /// Value
    pub value: StringValue,
}

/// 字符串值
/// String value
#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    /// 是否受保护（用于KDBX文件）
    /// Whether the string is protected (for KDBX files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    
    /// 是否在内存中受保护（用于未加密的XML文件）
    /// Whether the string is protected in memory (for unencrypted XML files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_in_memory: Option<bool>,
    
    /// 字符串内容
    /// String content
    #[serde(rename = "$value")]
    pub content: String,
}

/// 自定义数据项
/// Custom data item
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDataItem {
    /// 键名
    /// Key name
    pub key: String,
    
    /// 值
    /// Value
    pub value: String,
}

/// 自定义数据
/// Custom data
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomData {
    /// 数据项列表
    /// List of custom data items
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<CustomDataItem>,
}

/// 带时间戳的自定义数据项
/// Custom data item with timestamp
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDataItemWithTime {
    /// 键名
    /// Key name
    pub key: String,
    
    /// 值
    /// Value
    pub value: String,
    
    /// 最后修改时间
    /// Last modification time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<TimeData>,
}

/// 带时间戳的自定义数据
/// Custom data with timestamps
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDataWithTimes {
    /// 数据项列表
    /// List of custom data items with timestamps
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<CustomDataItemWithTime>,
}

/// 根节点
/// Root node
#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    /// 根组
    /// Root group
    pub group: Group,
    
    /// 已删除对象
    /// Deleted objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_objects: Option<DeletedObjects>,
}

/// 已删除对象列表
/// List of deleted objects
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedObjects {
    /// 已删除对象列表
    /// List of deleted objects
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deleted_object: Vec<DeletedObject>,
}

/// 已删除对象
/// Deleted object
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedObject {
    /// 对象UUID
    /// UUID of the deleted object
    pub uuid: Uuid,
    
    /// 删除时间
    /// Time of deletion
    pub deletion_time: TimeData,
}

/// 时间相关数据
/// Time-related data
#[derive(Debug, Serialize, Deserialize)]
pub struct Times {
    /// 创建时间
    /// Creation time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<TimeData>,
    
    /// 最后修改时间
    /// Last modification time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<TimeData>,
    
    /// 最后访问时间
    /// Last access time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<TimeData>,
    
    /// 过期时间
    /// Expiry time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<TimeData>,
    
    /// 是否过期
    /// Whether the item expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<bool>,
    
    /// 使用次数
    /// Usage count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<u64>,
    
    /// 位置变更时间
    /// Location changed time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_changed: Option<TimeData>,
}

/// 组
/// Group
#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    /// 组UUID
    /// UUID of the group
    pub uuid: Uuid,
    
    /// 组名称
    /// Name of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// 备注
    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    
    /// 图标ID
    /// Icon ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<u32>,
    
    /// 自定义图标UUID
    /// UUID of custom icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_icon_uuid: Option<Uuid>,
    
    /// 时间相关数据
    /// Time-related data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<Times>,
    
    /// 是否展开显示
    /// Whether the group is expanded in UI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expanded: Option<bool>,
    
    /// 默认自动输入序列
    /// Default auto-type sequence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_type_sequence: Option<String>,
    
    /// 是否启用自动输入
    /// Whether auto-type is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_type: Option<NullableBool>,
    
    /// 是否启用搜索
    /// Whether searching is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_searching: Option<NullableBool>,
    
    /// 最后可见的顶部条目UUID
    /// UUID of the last top visible entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_top_visible_entry: Option<Uuid>,
    
    /// 上一个父组UUID
    /// UUID of the previous parent group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_parent_group: Option<Uuid>,
    
    /// 标签
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    
    /// 自定义数据
    /// Custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    
    /// 条目列表
    /// List of entries
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<Entry>,
    
    /// 子组列表
    /// List of child groups
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<Group>,
}

/// 条目
/// Entry
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    /// 条目UUID
    /// UUID of the entry
    pub uuid: Uuid,
    
    /// 图标ID
    /// Icon ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<u32>,
    
    /// 自定义图标UUID
    /// UUID of custom icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_icon_uuid: Option<Uuid>,
    
    /// 前景色
    /// Foreground color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
    
    /// 背景色
    /// Background color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    
    /// 覆盖URL
    /// Override URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_url: Option<String>,
    
    /// 质量检查
    /// Quality check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_check: Option<bool>,
    
    /// 标签
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    
    /// 上一个父组UUID
    /// UUID of the previous parent group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_parent_group: Option<Uuid>,
    
    /// 时间相关数据
    /// Time-related data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<Times>,
    
    /// 字符串列表
    /// List of strings
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub string: Vec<ProtectedString>,
    
    /// 二进制数据列表
    /// List of binary data
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub binary: Vec<ProtectedBinary>,
    
    /// 自动输入配置
    /// Auto-type configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_type: Option<AutoType>,
    
    /// 自定义数据
    /// Custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    
    /// 历史记录
    /// History
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<History>,
}

/// 历史记录
/// History
#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    /// 历史条目列表
    /// List of historical entries
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<Entry>,
}

/// 自动输入配置
/// Auto-type configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoType {
    /// 是否启用
    /// Whether auto-type is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    
    /// 数据传输混淆
    /// Data transfer obfuscation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_obfuscation: Option<i32>,
    
    /// 默认序列
    /// Default sequence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sequence: Option<String>,
    
    /// 关联列表
    /// List of associations
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub association: Vec<Association>,
}

/// 自动输入关联
/// Auto-type association
#[derive(Debug, Serialize, Deserialize)]
pub struct Association {
    /// 窗口标题
    /// Window title
    pub window: String,
    
    /// 按键序列
    /// Keystroke sequence
    pub keystroke_sequence: String,
}

/// 可为空的布尔值
/// Nullable boolean value
#[derive(Debug, Serialize, Deserialize)]
pub enum NullableBool {
    #[serde(rename = "Null")]
    Null,
    #[serde(rename = "null")]
    NullLower,
    #[serde(rename = "False")]
    False,
    #[serde(rename = "false")]
    FalseLower,
    #[serde(rename = "True")]
    True,
    #[serde(rename = "true")]
    TrueLower,
}

/// UUID类型（Base64编码的128位UUID）
/// UUID type (Base64 encoded 128-bit UUID)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uuid(pub String);

/// 时间数据类型
/// 在KDBX 4及以上版本中，日期存储为自0001-01-01 00:00:00 UTC以来经过的秒数（Int64）
/// 在未加密的XML文件中，日期存储为xs:dateTime
/// Time data type
/// In KDBX 4 and above, dates are stored as seconds since 0001-01-01 00:00:00 UTC (Int64)
/// In unencrypted XML files, dates are stored as xs:dateTime
#[derive(Debug, Clone)]
pub enum TimeData {
    DateTime(DateTime<Utc>),
    Base64(String),
}

impl Serialize for TimeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TimeData::DateTime(dt) => dt.serialize(serializer),
            TimeData::Base64(data) => data.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for TimeData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 使用serde的枚举标签方式处理
        // Use serde's untagged enum approach
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum TimeDataHelper {
            DateTime(DateTime<Utc>),
            Base64(String),
        }
        
        let helper = TimeDataHelper::deserialize(deserializer)?;
        
        match helper {
            TimeDataHelper::DateTime(dt) => Ok(TimeData::DateTime(dt)),
            TimeDataHelper::Base64(data) => Ok(TimeData::Base64(data)),
        }
    }
}

impl TimeData {
    /// 从Base64编码的秒数创建TimeData
    /// Create TimeData from Base64 encoded seconds
    pub fn from_base64(data: &str) -> Result<Self, base64::DecodeError> {
        Ok(TimeData::Base64(data.to_string()))
    }
    
    /// 从DateTime创建TimeData
    /// Create TimeData from DateTime
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        TimeData::DateTime(dt)
    }
}