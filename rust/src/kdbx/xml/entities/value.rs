use serde::de::Error;
use serde::{Deserialize, Serialize};

use super::t_types::TBool;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Protected {
        value: Vec<u8>,
        offset: Option<usize>,
    },
    Unprotected(String),
    WaitProtect(String),
}

#[derive(Serialize, Deserialize)]
struct ValueXml {
    #[serde(rename = "@Protected", skip_serializing_if = "Option::is_none")]
    protected: Option<TBool>,
    #[serde(rename = "$text", default)]
    value: String,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Protected { ref value, .. } => ValueXml {
                protected: Some(TBool { value: true }),
                value: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, value),
            },
            Self::Unprotected(ref value) => ValueXml {
                protected: None,
                value: value.clone(),
            },
            Self::WaitProtect(s) => {
                return Err(serde::ser::Error::custom(format!("WaitProtect: {}", s)))
            }
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = ValueXml::deserialize(deserializer)?;
        if let Some(protected) = value.protected {
            if protected.value {
                let decoded =
                    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &value.value)
                        .map_err(|e| D::Error::custom(format!("Failed to decode base64: {}", e)))?;
                Ok(Self::Protected {
                    value: decoded,
                    offset: None,
                })
            } else {
                Ok(Self::Unprotected(value.value))
            }
        } else {
            Ok(Self::Unprotected(value.value))
        }
    }
}
