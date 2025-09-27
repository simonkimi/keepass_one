use byteorder::{ByteOrder, WriteBytesExt};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{collections::HashMap, io::Cursor, io::Write};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::utils::cursor_utils::CursorExt;

const VARIANT_DICTIONARY_VERSION: u16 = 0x100;

const U32_TYPE_ID: u8 = 0x04;
const U64_TYPE_ID: u8 = 0x05;
const BOOL_TYPE_ID: u8 = 0x08;
const I32_TYPE_ID: u8 = 0x0c;
const I64_TYPE_ID: u8 = 0x0d;
const STR_TYPE_ID: u8 = 0x18;
const BYTES_TYPE_ID: u8 = 0x42;

pub struct VariantDictionary {
    items: HashMap<String, VariantDictionaryValue>,
}

impl TryFrom<&[u8]> for VariantDictionary {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = Cursor::new(value);
        let version = reader.read_u16::<LittleEndian>()?;
        if version != VARIANT_DICTIONARY_VERSION {
            return Err(anyhow::anyhow!("Invalid variant dictionary version"));
        }

        let mut data = HashMap::new();

        while reader.remaining() > 0 {
            let value_type = reader.read_u8()?;
            if value_type == 0 {
                break;
            }
            let name_size = reader.read_u32::<LittleEndian>()? as usize;
            let name_buffer = reader.read_slice(name_size)?;
            let name = String::from_utf8_lossy(name_buffer).to_string();
            let value_size = reader.read_u32::<LittleEndian>()? as usize;
            let value_buf = reader.read_slice(value_size)?;

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
                    return Err(anyhow::anyhow!(
                        "Invalid variant dictionary value type: {}",
                        value_type
                    ));
                }
            };
            data.insert(name.to_string(), value);
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

    pub fn get<'a, T: 'a>(&'a self, key: &str) -> anyhow::Result<&'a T>
    where
        &'a VariantDictionaryValue: Into<Option<&'a T>>,
    {
        let entity = self
            .items
            .get(key)
            .ok_or(anyhow::anyhow!("未找到对应的key"))?;

        entity.into().ok_or_else(|| anyhow::anyhow!("类型不匹配"))
    }

    pub fn write(&self) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        cursor
            .write_u16::<LittleEndian>(VARIANT_DICTIONARY_VERSION)
            .unwrap();

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
            cursor.write_u8(type_id).unwrap();

            cursor.write_u32::<LittleEndian>(key.len() as u32).unwrap();
            cursor.write_all(key.as_bytes()).unwrap();

            match value {
                VariantDictionaryValue::UInt32(v) => {
                    cursor.write_u32::<LittleEndian>(4).unwrap();
                    cursor.write_u32::<LittleEndian>(*v).unwrap();
                }
                VariantDictionaryValue::UInt64(v) => {
                    cursor.write_u32::<LittleEndian>(8).unwrap();
                    cursor.write_u64::<LittleEndian>(*v).unwrap();
                }
                VariantDictionaryValue::Bool(v) => {
                    cursor.write_u32::<LittleEndian>(1).unwrap();
                    cursor.write_u8(if *v { 1 } else { 0 }).unwrap();
                }
                VariantDictionaryValue::Int32(v) => {
                    cursor.write_u32::<LittleEndian>(4).unwrap();
                    cursor.write_i32::<LittleEndian>(*v).unwrap();
                }
                VariantDictionaryValue::Int64(v) => {
                    cursor.write_u32::<LittleEndian>(8).unwrap();
                    cursor.write_i64::<LittleEndian>(*v).unwrap();
                }
                VariantDictionaryValue::String(v) => {
                    let bytes = v.as_bytes();
                    cursor
                        .write_u32::<LittleEndian>(bytes.len() as u32)
                        .unwrap();
                    cursor.write_all(bytes).unwrap();
                }
                VariantDictionaryValue::ByteArray(v) => {
                    cursor.write_u32::<LittleEndian>(v.len() as u32).unwrap();
                    cursor.write_all(v).unwrap();
                }
            }
        }

        cursor.write_u8(0).unwrap();

        cursor.into_inner()
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

        let written_data = vd.write();
        let parsed_vd = VariantDictionary::try_from(&written_data[..]).unwrap();

        assert_eq!(vd.items, parsed_vd.items);
    }
}
