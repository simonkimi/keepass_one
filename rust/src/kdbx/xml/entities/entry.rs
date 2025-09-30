use serde::{Deserialize, Serialize};
use crate::kdbx::xml::entities::times::Times;
use crate::kdbx::xml::entities::string_field::StringField;
use crate::kdbx::xml::entities::protected_binary::ProtectedBinary;
use crate::kdbx::xml::entities::auto_type::AutoType;
use crate::kdbx::xml::entities::history::History;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    pub custom_icon_uuid: Option<String>,
    #[serde(rename = "ForegroundColor")]
    #[serde(default)]
    pub foreground_color: Option<String>,
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    pub background_color: Option<String>,
    /// <https://keepass.info/help/base/autourl.html#override>
    #[serde(rename = "OverrideURL")]
    #[serde(default)]
    pub override_url: Option<String>,
    /// <https://keepass.info/help/v2/entry.html#gen>
    /// <https://keepass.info/help/kb/pw_quality_est.html>
    #[serde(rename = "QualityCheck")]
    pub quality_check: Option<String>,
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
    pub previous_parent_group: Option<String>,
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
