use crate::kdbx::xml::entities::auto_type::AutoType;
use crate::kdbx::xml::entities::history::History;
use crate::kdbx::xml::entities::protected_binary::ProtectedBinary;
use crate::kdbx::xml::entities::string_field::ProtectedString;
use crate::kdbx::xml::entities::times::Times;
use crate::kdbx::xml::entities::{TBool, TColor, TOptionUuid, TUuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entry {
    #[serde(rename = "UUID")]
    pub uuid: TUuid,
    /// See TGroup/IconID.
    ///
    /// 参阅TGroup/IconID。
    #[serde(rename = "IconID")]
    pub icon_id: u32,
    /// See TGroup/CustomIconUUID.
    ///
    /// 参阅TGroup/CustomIconUUID。
    #[serde(rename = "CustomIconUUID", default)]
    pub custom_icon_uuid: TOptionUuid,
    #[serde(rename = "ForegroundColor")]
    #[serde(default)]
    pub foreground_color: Option<TColor>,
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    pub background_color: Option<TColor>,
    /// <https://keepass.info/help/base/autourl.html#override>
    #[serde(rename = "OverrideURL")]
    #[serde(default)]
    pub override_url: Option<String>,
    /// <https://keepass.info/help/v2/entry.html#gen>
    /// <https://keepass.info/help/kb/pw_quality_est.html>
    #[serde(rename = "QualityCheck")]
    pub quality_check: Option<TBool>,
    /// See TGroup/Tags.
    ///
    /// 参阅TGroup/Tags。
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: String,
    /// See TGroup/PreviousParentGroup.
    ///
    /// 参阅TGroup/PreviousParentGroup。
    #[serde(rename = "PreviousParentGroup", default)]
    pub previous_parent_group: TOptionUuid,
    #[serde(rename = "Times")]
    pub times: Times,
    #[serde(rename = "String", default)]
    pub string: Vec<ProtectedString>,
    #[serde(rename = "Binary", default)]
    pub binary: Vec<ProtectedBinary>,
    /// <https://keepass.info/help/base/autotype.html>
    #[serde(rename = "AutoType")]
    #[serde(default)]
    pub auto_type: Option<AutoType>,
    /// <https://keepass.info/help/v2/entry.html#hst>
    #[serde(
        rename = "History",
        default,
        skip_serializing_if = "should_skip_history"
    )]
    pub history: Option<History>,
}

fn should_skip_history(history: &Option<History>) -> bool {
    match history {
        None => true,
        Some(h) => h.entry.is_empty(),
    }
}
