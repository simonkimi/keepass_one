use serde::{Deserialize, Serialize};

use crate::kdbx::xml::entities::{TBool, TDateTime};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Times {
    #[serde(rename = "CreationTime", default)]
    pub creation_time: TDateTime,
    #[serde(rename = "LastModificationTime", default)]
    pub last_modification_time: TDateTime,
    /// In general, last access times are not reliable, because an access is not considered to be a database change.
    ///
    /// 通常，上次访问时间不可靠，因为访问不被视为数据库更改。
    /// See the UIFlags value 0x20000:
    ///
    /// 请参阅UIFlags值0x20000：
    /// <https://keepass.info/help/v2_dev/customize.html#uiflags>
    #[serde(rename = "LastAccessTime")]
    pub last_access_time: TDateTime,
    #[serde(rename = "ExpiryTime")]
    pub expiry_time: TDateTime,
    #[serde(rename = "Expires")]
    pub expires: TBool,
    /// Cf. LastAccessTime.
    ///
    /// 参阅LastAccessTime。
    #[serde(rename = "UsageCount")]
    pub usage_count: u32,
    /// Last date/time when the object has been moved (within its parent group or to a different group). This is used by the synchronization algorithm to determine the latest location of the object.
    ///
    /// 对象最后移动（在其父组内或移动到不同组）的日期/时间。同步算法使用此信息来确定对象的最新位置。
    #[serde(rename = "LocationChanged")]
    pub location_changed: TDateTime,
}
