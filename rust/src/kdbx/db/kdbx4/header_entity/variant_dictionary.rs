use byteorder::LittleEndian;
use byteorder::{ByteOrder, WriteBytesExt, LE};
use std::{collections::HashMap, io::Write};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::utils::writer::Writable;

const VARIANT_DICTIONARY_VERSION: u16 = 0x100;

const U32_TYPE_ID: u8 = 0x04;
const U64_TYPE_ID: u8 = 0x05;
const BOOL_TYPE_ID: u8 = 0x08;
const I32_TYPE_ID: u8 = 0x0c;
const I64_TYPE_ID: u8 = 0x0d;
const STR_TYPE_ID: u8 = 0x18;
const BYTES_TYPE_ID: u8 = 0x42;

#[derive(Debug, Error)]
pub enum VariantDictionaryError {
    #[error("Invalid variant dictionary version: {0}")]
    InvalidVersion(u16),

    #[error("Invalid variant dictionary value type: {0}")]
    InvalidValueType(u8),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Type mismatch: {0}")]
    TypeMismatch(String),
}

#[derive(Debug, Clone)]
pub struct VariantDictionary {
    items: HashMap<String, VariantDictionaryValue>,
}

impl TryFrom<&[u8]> for VariantDictionary {
    type Error = VariantDictionaryError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut pos: usize = 0;
        let version = LE::read_u16(&value[pos..]);
        pos += 2;
        if version != VARIANT_DICTIONARY_VERSION {
            return Err(VariantDictionaryError::InvalidVersion(version));
        }

        let mut data = HashMap::new();

        while pos < value.len() {
            let value_type = value[pos];
            pos += 1;
            if value_type == 0 {
                break;
            }
            let name_size = LE::read_u32(&value[pos..]) as usize;
            pos += 4;
            let name_buffer = &value[pos..pos + name_size];
            pos += name_size;
            let value_size = LE::read_u32(&value[pos..]) as usize;
            pos += 4;
            let value_buf = &value[pos..pos + value_size];
            pos += value_size;

            let value = match value_type {
                U32_TYPE_ID => VariantDictionaryValue::UInt32(LittleEndian::read_u32(value_buf)),
                U64_TYPE_ID => VariantDictionaryValue::UInt64(LittleEndian::read_u64(value_buf)),
                BOOL_TYPE_ID => VariantDictionaryValue::Bool(value_buf != [0]),
                I32_TYPE_ID => VariantDictionaryValue::Int32(LittleEndian::read_i32(value_buf)),
                I64_TYPE_ID => VariantDictionaryValue::Int64(LittleEndian::read_i64(value_buf)),
                STR_TYPE_ID => {
                    VariantDictionaryValue::String(String::from_utf8_lossy(value_buf).to_string())
                }
                BYTES_TYPE_ID => VariantDictionaryValue::ByteArray(value_buf.to_vec()),
                _ => {
                    return Err(VariantDictionaryError::InvalidValueType(value_type));
                }
            };
            data.insert(String::from_utf8_lossy(name_buffer).to_string(), value);
        }

        Ok(Self { items: data })
    }
}

impl VariantDictionary {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn from(items: HashMap<String, VariantDictionaryValue>) -> Self {
        Self { items }
    }

    pub fn get<'a, T: 'a>(&'a self, key: &str) -> Result<&'a T, VariantDictionaryError>
    where
        &'a VariantDictionaryValue: Into<Option<&'a T>>,
    {
        let entity = self
            .items
            .get(key)
            .ok_or(VariantDictionaryError::KeyNotFound(key.to_string()))?;

        entity
            .into()
            .ok_or_else(|| VariantDictionaryError::TypeMismatch(key.to_string()))
    }
}

impl Writable for VariantDictionary {
    fn write<W: Write + std::io::Seek>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_u16::<LE>(VARIANT_DICTIONARY_VERSION)?;
        for (key, value) in &self.items {
            let type_id = match value {
                VariantDictionaryValue::UInt32(_) => U32_TYPE_ID,
                VariantDictionaryValue::UInt64(_) => U64_TYPE_ID,
                VariantDictionaryValue::Bool(_) => BOOL_TYPE_ID,
                VariantDictionaryValue::Int32(_) => I32_TYPE_ID,
                VariantDictionaryValue::Int64(_) => I64_TYPE_ID,
                VariantDictionaryValue::String(_) => STR_TYPE_ID,
                VariantDictionaryValue::ByteArray(_) => BYTES_TYPE_ID,
            };
            writer.write_u8(type_id)?;
            writer.write_u32::<LE>(key.len() as u32)?;
            writer.write_all(key.as_bytes())?;
            match value {
                VariantDictionaryValue::UInt32(v) => {
                    writer.write_u32::<LE>(4)?;
                    writer.write_u32::<LE>(*v)?;
                }
                VariantDictionaryValue::UInt64(v) => {
                    writer.write_u32::<LE>(8)?;
                    writer.write_u64::<LE>(*v)?;
                }
                VariantDictionaryValue::Bool(v) => {
                    writer.write_u32::<LE>(1)?;
                    writer.write_u8(if *v { 1 } else { 0 })?;
                }
                VariantDictionaryValue::Int32(v) => {
                    writer.write_u32::<LE>(4)?;
                    writer.write_i32::<LE>(*v)?;
                }
                VariantDictionaryValue::Int64(v) => {
                    writer.write_u32::<LE>(8)?;
                    writer.write_i64::<LE>(*v)?;
                }
                VariantDictionaryValue::String(v) => {
                    let bytes = v.as_bytes();
                    writer.write_u32::<LE>(bytes.len() as u32)?;
                    writer.write_all(bytes)?;
                }
                VariantDictionaryValue::ByteArray(v) => {
                    writer.write_u32::<LE>(v.len() as u32)?;
                    writer.write_all(v)?;
                }
            }
        }
        writer.write_u8(0)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Zeroize, ZeroizeOnDrop)]
pub enum VariantDictionaryValue {
    UInt32(u32),
    UInt64(u64),
    Bool(bool),
    Int32(i32),
    Int64(i64),
    String(String),
    ByteArray(Vec<u8>),
}

impl From<u32> for VariantDictionaryValue {
    fn from(v: u32) -> Self {
        VariantDictionaryValue::UInt32(v)
    }
}

impl From<u64> for VariantDictionaryValue {
    fn from(v: u64) -> Self {
        VariantDictionaryValue::UInt64(v)
    }
}

impl From<i32> for VariantDictionaryValue {
    fn from(v: i32) -> Self {
        VariantDictionaryValue::Int32(v)
    }
}

impl From<i64> for VariantDictionaryValue {
    fn from(v: i64) -> Self {
        VariantDictionaryValue::Int64(v)
    }
}

impl From<bool> for VariantDictionaryValue {
    fn from(v: bool) -> Self {
        VariantDictionaryValue::Bool(v)
    }
}

impl From<String> for VariantDictionaryValue {
    fn from(v: String) -> Self {
        VariantDictionaryValue::String(v)
    }
}

impl From<Vec<u8>> for VariantDictionaryValue {
    fn from(v: Vec<u8>) -> Self {
        VariantDictionaryValue::ByteArray(v)
    }
}

impl<'a> From<&'a VariantDictionaryValue> for Option<&'a u32> {
    fn from(value: &'a VariantDictionaryValue) -> Self {
        match value {
            VariantDictionaryValue::UInt32(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a VariantDictionaryValue> for Option<&'a u64> {
    fn from(value: &'a VariantDictionaryValue) -> Self {
        match value {
            VariantDictionaryValue::UInt64(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a VariantDictionaryValue> for Option<&'a i32> {
    fn from(value: &'a VariantDictionaryValue) -> Self {
        match value {
            VariantDictionaryValue::Int32(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a VariantDictionaryValue> for Option<&'a i64> {
    fn from(value: &'a VariantDictionaryValue) -> Self {
        match value {
            VariantDictionaryValue::Int64(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a VariantDictionaryValue> for Option<&'a bool> {
    fn from(value: &'a VariantDictionaryValue) -> Self {
        match value {
            VariantDictionaryValue::Bool(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a VariantDictionaryValue> for Option<&'a String> {
    fn from(value: &'a VariantDictionaryValue) -> Self {
        match value {
            VariantDictionaryValue::String(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a VariantDictionaryValue> for Option<&'a Vec<u8>> {
    fn from(value: &'a VariantDictionaryValue) -> Self {
        match value {
            VariantDictionaryValue::ByteArray(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_and_parse() {
        let mut items = HashMap::new();
        items.insert("u32".to_string(), VariantDictionaryValue::UInt32(123));
        items.insert("u64".to_string(), VariantDictionaryValue::UInt64(456));
        items.insert("bool_true".to_string(), VariantDictionaryValue::Bool(true));
        items.insert(
            "bool_false".to_string(),
            VariantDictionaryValue::Bool(false),
        );
        items.insert("i32".to_string(), VariantDictionaryValue::Int32(-123));
        items.insert("i64".to_string(), VariantDictionaryValue::Int64(-456));
        items.insert(
            "string".to_string(),
            VariantDictionaryValue::String("hello".to_string()),
        );
        items.insert(
            "bytes".to_string(),
            VariantDictionaryValue::ByteArray(vec![1, 2, 3]),
        );

        let vd = VariantDictionary::from(items);
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);

        vd.write(&mut writer).unwrap();
        let parsed_vd = VariantDictionary::try_from(buffer.as_slice()).unwrap();

        assert_eq!(vd.items, parsed_vd.items);
    }
}
