use serde::de::Error;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::crypto::secure_data::SecureData;

use super::t_types::TBool;

#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub enum Value {
    Protected {
        value: SecureData,
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
            Self::Protected { ref value, .. } => {
                let data = value
                    .unsecure()
                    .map_err(|e| serde::ser::Error::custom(format!("Failed to unsecure: {}", e)))?;

                ValueXml {
                    protected: Some(true.into()),
                    value: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data),
                }
            }
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
            if protected.into() {
                let decoded = Zeroizing::new(
                    base64::Engine::decode(
                        &base64::engine::general_purpose::STANDARD,
                        &value.value,
                    )
                    .map_err(|e| D::Error::custom(format!("Failed to decode base64: {}", e)))?,
                );
                Ok(Self::Protected {
                    value: SecureData::new(&decoded),
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
