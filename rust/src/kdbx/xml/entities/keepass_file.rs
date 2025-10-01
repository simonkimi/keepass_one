use std::io::Cursor;

use crate::kdbx::xml::entities::root::Root;
use crate::kdbx::xml::{entities::meta::Meta, errors::KdbxDatabaseError};
use crate::utils::writer::Writable;
use serde::{Deserialize, Serialize};

/// KDBX 4.1 XML Schema.
///
/// Copyright (C) 2007-2025 Dominik Reichl.
/// <https://keepass.info/help/kb/kdbx.html>
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct KeePassFile {
    #[serde(rename = "Meta")]
    pub meta: Meta,
    #[serde(rename = "Root")]
    pub root: Root,
}

impl TryFrom<&[u8]> for KeePassFile {
    type Error = KdbxDatabaseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let document: KeePassFile = quick_xml::de::from_reader(value)?;
        Ok(document)
    }
}

impl Writable for KeePassFile {
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        let xml_declaration = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;
        writer.write_all(xml_declaration.as_bytes())?;
        writer.write_all(b"\n")?;
        let mut serialized = String::new();
        let mut serializer = quick_xml::se::Serializer::new(&mut serialized);
        serializer.indent(' ', 4); // 设置缩进为4个空格
        self.serialize(serializer).unwrap();
        writer.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

impl KeePassFile {
    pub fn dump(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        self.write(&mut writer)?;
        Ok(buffer)
    }
}
