use anyhow::anyhow;
use byteorder::ByteOrder;
use byteorder::{LittleEndian, ReadBytesExt};
use std::{collections::HashMap, io::Cursor};

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
    data: HashMap<String, VariantDictionaryValue>,
}

impl VariantDictionary {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn parse(data: &[u8]) -> anyhow::Result<Self> {
        let mut cursor = Cursor::new(data);
        let version = cursor.read_u16::<LittleEndian>()?;
        if version != VARIANT_DICTIONARY_VERSION {
            return Err(anyhow::anyhow!("Invalid variant dictionary version"));
        }

        let mut data = HashMap::new();

        while cursor.remaining() > 0 {
            let value_type = cursor.read_u8()?;
            if value_type == 0 {
                break;
            }
            let name_size = cursor.read_i32::<LittleEndian>()? as usize;
            let name_buffer = cursor.read_slice(name_size)?;
            let name = String::from_utf8_lossy(name_buffer).to_string();
            let value_size = cursor.read_i32::<LittleEndian>()? as usize;
            let value_buf = cursor.read_slice(value_size)?;

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

        Ok(Self { data })
    }

    pub fn get<'a, T: 'a>(&'a self, key: &str) -> anyhow::Result<&'a T>
    where
        &'a VariantDictionaryValue: Into<Option<&'a T>>,
    {
        let entity = self
            .data
            .get(key)
            .ok_or(anyhow::anyhow!("未找到对应的key"))?;

        entity.into().ok_or_else(|| anyhow::anyhow!("类型不匹配"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
