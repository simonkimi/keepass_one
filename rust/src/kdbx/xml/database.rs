use crate::kdbx::{
    db::kdbx4::inner_header::Kdbx4InnerHeader,
    xml::{
        entities::{KeePassDocument, Value},
        errors::KdbxDatabaseError,
        protected_value,
    },
};
use std::collections::HashMap;

pub struct KeePassDatabase {
    pub document: KeePassDocument,
    pub inner_header: Kdbx4InnerHeader,
}

impl KeePassDatabase {
    pub fn try_from(xml: &[u8], inner_header: Kdbx4InnerHeader) -> Result<Self, KdbxDatabaseError> {
        let mut document: KeePassDocument = quick_xml::de::from_reader(xml)?;
        protected_value::collect_protected_values_document(&mut document);
        Ok(Self {
            document,
            inner_header,
        })
    }

    pub fn get_value_string(&self, entry: &Value) -> Result<String, KdbxDatabaseError> {
        match entry {
            Value::Unprotected(ref value) => Ok(value.to_string()),
            Value::WaitProtect(ref value) => Ok(value.to_string()),
            Value::Protected { value, offset } => {
                if let Some(offset) = offset {
                    let mut cipher = self.inner_header.encryption.get_stream_cipher();
                    let data = cipher
                        .decrypt_at_offset(*offset, value)
                        .map_err(KdbxDatabaseError::ProtectedValueDecryptError)?;
                    Ok(String::from_utf8_lossy(&data).to_string())
                } else {
                    Ok(String::from_utf8_lossy(&value).to_string())
                }
            }
        }
    }
}
