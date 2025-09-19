use crate::crypto;
use crate::crypto::hash::calculate_sha256;
use base64::Engine;
use generic_array::{typenum::U32, GenericArray};
use std::io::Read;
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdbxKeyError {
    #[error("No key parts available. Please provide a master key or key file.")]
    NoKeyParts,
    #[error("Failed to parse key file. Ensure it is a valid key file format.")]
    FailedToParseKeyFile,
}

pub struct KdbxKey {
    pub master_key: Option<String>,
    pub key_file: Option<Vec<u8>>,
}

pub type KeyElement = Vec<u8>;

impl KdbxKey {
    pub fn new() -> Self {
        KdbxKey {
            master_key: None,
            key_file: None,
        }
    }

    pub fn add_master_key(&mut self, key: &str) {
        self.master_key = Some(key.to_string());
    }

    pub fn add_key_file(&mut self, key_file: &mut dyn Read) -> Result<(), std::io::Error> {
        let mut buf = Vec::new();
        key_file.read_to_end(&mut buf)?;
        self.key_file = Some(buf);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.master_key.is_none() && self.key_file.is_none()
    }

    pub fn calc_key_hash(&self) -> Result<GenericArray<u8, U32>, KdbxKeyError> {
        let mut key_parts: Vec<Vec<u8>> = Vec::new();

        if let Some(ref master_key) = self.master_key {
            let master_key_hash = calculate_sha256(master_key.as_bytes());
            key_parts.push(master_key_hash.to_vec());
        }

        if let Some(ref key_file_buf) = self.key_file {
            key_parts.push(
                try_parse_keyfile(key_file_buf)
                    .ok_or_else(|| KdbxKeyError::FailedToParseKeyFile)?,
            );
        }

        if key_parts.is_empty() {
            return Err(KdbxKeyError::NoKeyParts);
        }

        Ok(crypto::hash::calculate_sha256_multiple(
            &key_parts.iter().map(Vec::as_slice).collect::<Vec<&[u8]>>(),
        ))
    }
}

fn try_parse_keyfile(key_buf: &[u8]) -> Option<Vec<u8>> {
    if let Some(v) = try_parse_xml_keyfile(key_buf) {
        Some(v)
    } else if key_buf.len() == 32 {
        Some(key_buf.to_vec())
    } else {
        Some(calculate_sha256(key_buf).to_vec())
    }
}

fn try_parse_xml_keyfile(key_buf: &[u8]) -> Option<Vec<u8>> {
    let xml = std::str::from_utf8(key_buf).ok()?;
    let package = parser::parse(xml).ok()?;
    let document = package.as_document();

    let version: Option<String> = evaluate_xpath(&document, "//Meta/Version/text()")
        .ok()
        .map(|v| v.string());

    let data = evaluate_xpath(&document, "//Key/Data/text()")
        .ok()?
        .string()
        .split_whitespace()
        .collect::<String>();

    if version == Some("2.0".to_string()) {
        return if let Ok(key_buf) = hex::decode(&data) {
            Some(key_buf)
        } else {
            Some(data.as_bytes().to_vec())
        };
    }

    let key_bytes = data.as_bytes().to_vec();

    if let Ok(key) = base64::engine::general_purpose::STANDARD.decode(&key_bytes) {
        Some(key)
    } else {
        Some(key_bytes)
    }
}

#[cfg(test)]
mod key_tests {
    use hex_literal::hex;

    const XML_KEYFILE1: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<KeyFile>
    <Meta>
        <Version>2.0</Version>
    </Meta>
    <Key>
        <Data Hash="7DEDDE1D">
            5D008FBC 4E6BE14A 89CAC795 DDB9A180
            D7662141 E6662ECC 8D33E168 0882516D
        </Data>
    </Key>
</KeyFile>"#;

    const XML_KEYFILE2: &str = r#"<?xml version="1.0" encoding="UTF-8"?><KeyFile><Meta><Version>1.00</Version></Meta><Key><Data>NXyYiJMHg3ls+eBmjbAjWec9lcOToJiofbhNiFMTJMw=</Data></Key></KeyFile>"#;

    #[test]
    fn test_try_parse_xml_keyfile() {
        let result = super::try_parse_xml_keyfile(XML_KEYFILE1.as_bytes());
        assert!(result.is_some());
        let key = result.unwrap();
        assert_eq!(key.len(), 32);
        assert_eq!(
            key,
            hex!("5D008FBC4E6BE14A89CAC795DDB9A180D7662141E6662ECC8D33E1680882516D")
        );
    }

    #[test]
    fn test_try_parse_xml_keyfile2() {
        let result = super::try_parse_xml_keyfile(XML_KEYFILE2.as_bytes());
        assert!(result.is_some());
        let key = result.unwrap();
        assert_eq!(key.len(), 32);
        assert_eq!(
            key,
            hex!("357C9888930783796CF9E0668DB02359E73D95C393A098A87DB84D88531324CC")
        );
    }
}
