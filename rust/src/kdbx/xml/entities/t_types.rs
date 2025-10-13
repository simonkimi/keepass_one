use chrono::{DateTime, TimeZone, Utc};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Clone)]
pub struct TUuid(Uuid);

impl Zeroize for TUuid {
    fn zeroize(&mut self) {}
}

impl Serialize for TUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            self.0.as_bytes(),
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
        Ok(Self(uuid))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TOptionUuid(Option<Uuid>);

impl Zeroize for TOptionUuid {
    fn zeroize(&mut self) {
        self.0 = None;
    }
}

impl Default for TOptionUuid {
    fn default() -> Self {
        Self(None)
    }
}

impl Serialize for TOptionUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(uuid) = self.0 {
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
            return Ok(Self(None));
        }

        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &value)
            .map_err(|e| D::Error::custom(format!("Failed to decode base64: {} {}", value, e)))?;
        let uuid = Uuid::from_slice(&decoded)
            .map_err(|e| D::Error::custom(format!("Failed to parse UUID: {} {}", value, e)))?;
        Ok(Self(Some(uuid)))
    }
}

#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct TBool(bool);

impl From<bool> for TBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl TBool {
    pub fn value(&self) -> bool {
        self.0
    }
}

impl Serialize for TBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(if self.0 { "True" } else { "False" })
    }
}

impl<'de> Deserialize<'de> for TBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "" => Ok(Self(false)),
            "True" | "true" => Ok(Self(true)),
            "False" | "false" => Ok(Self(false)),
            _ => Err(D::Error::custom(format!(
                "Invalid TBool value: {}. Expected 'True' or 'False'",
                value
            ))),
        }
    }
}

/// 可空布尔值扩展，支持 Null/null/False/false/True/true 值
#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
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
#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
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
pub struct TDateTime(Option<DateTime<Utc>>);

impl Zeroize for TDateTime {
    fn zeroize(&mut self) {
        self.0 = None;
    }
}

impl ZeroizeOnDrop for TDateTime {}

impl Default for TDateTime {
    fn default() -> Self {
        Self(None)
    }
}

impl Serialize for TDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(value) = self.0 {
            let base_date = Utc.with_ymd_and_hms(1, 1, 1, 0, 0, 0).unwrap();
            let seconds = value.signed_duration_since(base_date).num_seconds();
            let bytes = seconds.to_le_bytes();
            let encoded =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
            serializer.serialize_str(&encoded)
        } else {
            serializer.serialize_str("")
        }
    }
}

impl<'de> Deserialize<'de> for TDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.is_empty() {
            return Ok(Self(None));
        }
        if let Ok(decoded) =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &value)
        {
            if decoded.len() == 8 {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&decoded);
                let seconds = i64::from_le_bytes(bytes);

                // 基准日期：0001-01-01 00:00:00 UTC
                let base_date = Utc.with_ymd_and_hms(1, 1, 1, 0, 0, 0).unwrap();

                // 添加秒数得到目标日期
                if let Some(target_date) =
                    base_date.checked_add_signed(chrono::Duration::seconds(seconds))
                {
                    return Ok(Self(Some(target_date)));
                } else {
                    return Err(D::Error::custom(format!(
                        "Invalid timestamp: {} seconds from base date would overflow",
                        seconds
                    )));
                }
            }
        }

        match DateTime::parse_from_rfc3339(&value) {
            Ok(parsed_date) => Ok(Self(Some(parsed_date.with_timezone(&Utc)))),
            Err(_) => {
                // 如果RFC3339解析失败，尝试其他常见的ISO 8601格式
                if let Ok(parsed_date) = DateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S%.fZ") {
                    Ok(Self(Some(parsed_date.with_timezone(&Utc))))
                } else if let Ok(parsed_date) =
                    DateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%SZ")
                {
                    Ok(Self(Some(parsed_date.with_timezone(&Utc))))
                } else if let Ok(_parsed_date) =
                    DateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S")
                {
                    // 假设没有时区信息的日期是UTC
                    let naive_dt =
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S")
                            .map_err(|e| {
                                D::Error::custom(format!("Failed to parse datetime: {}", e))
                            })?;
                    Ok(Self(Some(Utc.from_utc_datetime(&naive_dt))))
                } else {
                    Err(D::Error::custom(format!(
                        "Invalid TDateTime format: {}. Expected Base64-encoded Int64 or xs:dateTime format",
                        value
                    )))
                }
            }
        }
    }
}

/// 非负整数，值必须大于等于0
#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct TNonNegativeInt(i32);

impl Serialize for TNonNegativeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
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
            Ok(Self(int_value))
        } else {
            Err(D::Error::custom(format!(
                "TNonNegativeInt value must be >= 0, got {}",
                int_value
            )))
        }
    }
}

#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct TBase64Binary(Vec<u8>);

impl Serialize for TBase64Binary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.0);
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
        Ok(Self(data))
    }
}
