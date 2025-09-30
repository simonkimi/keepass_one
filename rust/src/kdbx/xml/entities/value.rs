use serde::de::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
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
    protected: Option<String>,
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
                protected: Some("True".to_string()),
                value: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, value),
            },
            Self::Unprotected(ref value) => ValueXml {
                protected: None,
                value: value.clone(),
            },
            Self::WaitProtect(s) => {
                return Err(serde::ser::Error::custom(format!("WaitProcess: {}", s)))
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
        if Some("True".to_string()) == value.protected {
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
    }
}
