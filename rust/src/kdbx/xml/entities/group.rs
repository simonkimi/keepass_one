use serde::{Deserialize, Serialize};
use crate::kdbx::xml::entities::times::Times;
use crate::kdbx::xml::entities::entry::Entry;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    /// 请参阅KeePass源代码包中的"Ext/Images_Client_HighRes"文件夹。
    #[serde(rename = "IconID")]
    pub icon_id: u32,
    /// Reference to a custom icon stored in the KeePassFile/Meta/CustomIcons element. If non-zero, it overrides IconID.
    ///
    /// 对存储在KeePassFile/Meta/CustomIcons元素中的自定义图标的引用。如果非零，则会覆盖IconID。
    #[serde(rename = "CustomIconUUID")]
    pub custom_icon_uuid: Option<String>,
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
    pub previous_parent_group: Option<String>,
    #[serde(rename = "Entry", default)]
    pub entry: Vec<Entry>,
    #[serde(rename = "Group", default)]
    pub group: Vec<Group>,
}
