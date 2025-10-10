use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct TUuid {
    pub uuid: Uuid,
}

impl Serialize for TUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            self.uuid.as_bytes(),
        );
        serializer.serialize_str(&encoded)
    }
}
impl<'de> Deserialize<'de> for TUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &value)
            .map_err(|e| D::Error::custom(format!("Failed to decode base64: {} {}", value, e)))?;
        let uuid = Uuid::from_slice(&decoded)
            .map_err(|e| D::Error::custom(format!("Failed to parse UUID: {} {}", value, e)))?;
        Ok(Self { uuid })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TOptionUuid {
    pub uuid: Option<Uuid>,
}

impl Default for TOptionUuid {
    fn default() -> Self {
        Self { uuid: None }
    }
}

impl Serialize for TOptionUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(uuid) = self.uuid {
            let encoded =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, uuid.as_bytes());
            serializer.serialize_str(&encoded)
        } else {
            serializer.serialize_str("")
        }
    }
}

impl<'de> Deserialize<'de> for TOptionUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if value.is_empty() {
            return Ok(Self { uuid: None });
        }

        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &value)
            .map_err(|e| D::Error::custom(format!("Failed to decode base64: {} {}", value, e)))?;
        let uuid = Uuid::from_slice(&decoded)
            .map_err(|e| D::Error::custom(format!("Failed to parse UUID: {} {}", value, e)))?;
        Ok(Self { uuid: Some(uuid) })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TBool {
    pub value: bool,
}

impl From<bool> for TBool {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl Serialize for TBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(if self.value { "True" } else { "False" })
    }
}

impl<'de> Deserialize<'de> for TBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "" => Ok(Self { value: false }),
            "True" | "true" => Ok(Self { value: true }),
            "False" | "false" => Ok(Self { value: false }),
            _ => Err(D::Error::custom(format!(
                "Invalid TBool value: {}. Expected 'True' or 'False'",
                value
            ))),
        }
    }
}

/// 可空布尔值扩展，支持 Null/null/False/false/True/true 值
#[derive(Debug, PartialEq, Clone)]
pub enum TNullableBoolEx {
    Null,
    False,
    True,
}

impl From<Option<bool>> for TNullableBoolEx {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::Null,
        }
    }
}

impl From<TNullableBoolEx> for Option<bool> {
    fn from(value: TNullableBoolEx) -> Self {
        match value {
            TNullableBoolEx::True => Some(true),
            TNullableBoolEx::False => Some(false),
            TNullableBoolEx::Null => None,
        }
    }
}

impl Serialize for TNullableBoolEx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Null => "Null",
            Self::False => "False",
            Self::True => "True",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TNullableBoolEx {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Null" | "null" => Ok(Self::Null),
            "False" | "false" => Ok(Self::False),
            "True" | "true" => Ok(Self::True),
            _ => Err(D::Error::custom(format!(
                "Invalid TNullableBoolEx value: {}",
                value
            ))),
        }
    }
}

/// 颜色值，支持十六进制CSS颜色格式（#RRGGBB）或空字符串
#[derive(Debug, PartialEq, Clone)]
pub enum TColor {
    /// 十六进制颜色值，格式为 #RRGGBB
    Hex(String),
    /// 空字符串，表示使用默认值
    Default,
}

impl Default for TColor {
    fn default() -> Self {
        Self::Default
    }
}

impl From<String> for TColor {
    fn from(value: String) -> Self {
        if value.is_empty() {
            Self::Default
        } else {
            Self::Hex(value)
        }
    }
}

impl From<Option<String>> for TColor {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(color) if color.is_empty() => Self::Default,
            Some(color) => Self::Hex(color),
            None => Self::Default,
        }
    }
}

impl From<TColor> for Option<String> {
    fn from(value: TColor) -> Self {
        match value {
            TColor::Hex(color) => Some(color),
            TColor::Default => None,
        }
    }
}

impl Serialize for TColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Hex(color) => color.as_str(),
            Self::Default => "",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if value.is_empty() {
            return Ok(Self::Default);
        }

        // 验证十六进制颜色格式 #RRGGBB
        if value.len() == 7
            && value.starts_with('#')
            && value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        {
            Ok(Self::Hex(value))
        } else {
            Err(D::Error::custom(format!(
                "Invalid TColor format: {}. Expected empty string or #RRGGBB format",
                value
            )))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TDateTime {
    Base64Encoded(i64),
    IsoDateTime(DateTime<Utc>),
}

impl From<DateTime<Utc>> for TDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self::IsoDateTime(value)
    }
}

impl From<TDateTime> for Option<DateTime<Utc>> {
    fn from(value: TDateTime) -> Self {
        match value {
            TDateTime::IsoDateTime(dt) => Some(dt),
            TDateTime::Base64Encoded(_) => None, // 需要特殊处理转换为DateTime
        }
    }
}

impl From<TDateTime> for DateTime<Utc> {
    fn from(value: TDateTime) -> Self {
        match value {
            TDateTime::IsoDateTime(dt) => dt,
            TDateTime::Base64Encoded(seconds) => {
                let base_date = chrono::NaiveDate::from_ymd_opt(1, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();

                let naive_dt = base_date + chrono::Duration::seconds(seconds);
                DateTime::from_naive_utc_and_offset(naive_dt, Utc)
            }
        }
    }
}

impl Serialize for TDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Base64Encoded(seconds) => {
                // 将i64转换为8字节的字节数组，然后进行Base64编码
                let bytes = seconds.to_le_bytes();
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes)
            }
            Self::IsoDateTime(dt) => dt.to_rfc3339(),
        };
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for TDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        // 首先尝试解析为ISO 8601日期时间格式
        if let Ok(dt) = DateTime::parse_from_rfc3339(&value) {
            return Ok(Self::IsoDateTime(dt.with_timezone(&Utc)));
        }

        // 如果失败，尝试解析为Base64编码的Int64
        if let Ok(decoded) =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &value)
        {
            if decoded.len() == 8 {
                let bytes: [u8; 8] = decoded
                    .try_into()
                    .map_err(|_| D::Error::custom("Invalid Base64 length for TDateTime"))?;
                let seconds = i64::from_le_bytes(bytes);
                return Ok(Self::Base64Encoded(seconds));
            }
        }

        Err(D::Error::custom(format!(
            "Invalid TDateTime format: {}. Expected ISO 8601 dateTime or Base64 encoded Int64",
            value
        )))
    }
}

/// 非负整数，值必须大于等于0
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TNonNegativeInt {
    pub value: i32,
}

impl From<i32> for TNonNegativeInt {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

impl From<TNonNegativeInt> for i32 {
    fn from(value: TNonNegativeInt) -> Self {
        value.value
    }
}

impl TNonNegativeInt {
    /// 创建一个新的非负整数，如果值小于0则返回错误
    pub fn new(value: i32) -> Result<Self, String> {
        if value >= 0 {
            Ok(Self { value })
        } else {
            Err(format!("TNonNegativeInt value must be >= 0, got {}", value))
        }
    }
}

impl Serialize for TNonNegativeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value.to_string())
    }
}

impl<'de> Deserialize<'de> for TNonNegativeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let int_value: i32 = value
            .parse()
            .map_err(|e| D::Error::custom(format!("Failed to parse TNonNegativeInt: {}", e)))?;

        if int_value >= 0 {
            Ok(Self { value: int_value })
        } else {
            Err(D::Error::custom(format!(
                "TNonNegativeInt value must be >= 0, got {}",
                int_value
            )))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TBase64Binary {
    pub data: Vec<u8>,
}

impl Serialize for TBase64Binary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let encoded =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.data);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for TBase64Binary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &value)
            .map_err(|e| D::Error::custom(format!("Failed to decode base64: {}", e)))?;
        Ok(Self { data })
    }
}
