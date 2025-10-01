use crate::{
    kdbx::{
        db::kdbx4::inner_header::{Kdbx4InnerEncryption, Kdbx4InnerHeader},
        xml::{
            entities::{KeePassFile, Value},
            errors::KdbxDatabaseError,
            protected_value,
        },
    },
    utils::writer::Writable,
};

pub struct KeePassDatabase {
    pub document: KeePassFile,
    pub inner_header: Kdbx4InnerHeader,
}

impl KeePassDatabase {
    pub fn new(document: KeePassFile, inner_header: Kdbx4InnerHeader) -> Self {
        Self {
            document,
            inner_header,
        }
    }

    pub fn try_from(xml: &[u8], inner_header: Kdbx4InnerHeader) -> Result<Self, KdbxDatabaseError> {
        let mut document: KeePassFile = quick_xml::de::from_reader(xml)?;
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

    pub fn encrypt_database(&self) -> Result<KeePassDatabase, KdbxDatabaseError> {
        let new_inner_header = self.inner_header.copy_with(Kdbx4InnerEncryption::new());
        let mut old_cipher = self.inner_header.encryption.get_stream_cipher();
        let mut new_cipher = new_inner_header.encryption.get_stream_cipher();

        let mut new_document = self.document.clone();
        protected_value::encrypt_protected_value(
            &mut new_document,
            &mut old_cipher,
            &mut new_cipher,
        );
        Ok(Self {
            document: new_document,
            inner_header: new_inner_header,
        })
    }
}

impl Writable for KeePassDatabase {
    fn write<W: std::io::Write + std::io::Seek + Sized>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        self.inner_header.write(writer)?;
        self.document.write(writer)?;
        Ok(())
    }
}
