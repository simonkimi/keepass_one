use crate::kdbx::xml::entities::root::Root;
use crate::kdbx::xml::{entities::meta::Meta, errors::KdbxDatabaseError};
use crate::utils::writer::Writable;
use serde::{Deserialize, Serialize};

/// KDBX 4.1 XML Schema.
///
/// Copyright (C) 2007-2025 Dominik Reichl.
/// <https://keepass.info/help/kb/kdbx.html>
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct KeePassDocument {
    #[serde(rename = "Meta")]
    pub meta: Meta,
    #[serde(rename = "Root")]
    pub root: Root,
}

impl TryFrom<&[u8]> for KeePassDocument {
    type Error = KdbxDatabaseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let document: KeePassDocument = quick_xml::de::from_reader(value)?;
        Ok(document)
    }
}

impl Writable for KeePassDocument {
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        let mut serialized = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut serialized);
        ser.expand_empty_elements(true);
        self.serialize(ser).unwrap();
        writer.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
