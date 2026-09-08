use crate::crypto;
use crate::crypto::hash::calculate_sha256;
use base64::Engine;
use generic_array::{typenum::U32, GenericArray};
use secrecy::{ExposeSecret, SecretBox, SecretSlice};
use std::io::Read;
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;
use thiserror::Error;
use zeroize::Zeroize;

#[derive(Debug, Error)]
pub enum KdbxKeyError {
    #[error("No key parts available. Please provide a master key or key file.")]
    NoKeyParts,
    #[error("Failed to parse key file. Ensure it is a valid key file format.")]
    FailedToParseKeyFile,
    #[error("Key file hash mismatch")]
    InvalidKeyFileHash,
}

pub struct KdbxKey {
    pub master_key: Option<SecretBox<String>>,
    pub key_file: Option<SecretSlice<u8>>,
}

impl KdbxKey {
    pub fn new() -> Self {
        KdbxKey {
            master_key: None,
            key_file: None,
        }
    }

    pub fn add_master_key(&mut self, key: &str) {
        self.master_key = Some(SecretBox::new(Box::new(key.to_string())));
    }

    pub fn add_key_file(&mut self, key_file: &mut dyn Read) -> Result<(), std::io::Error> {
        let mut buf = Vec::new();
        key_file.read_to_end(&mut buf)?;
        self.key_file = Some(SecretSlice::from(buf));
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.master_key.is_none() && self.key_file.is_none()
    }

    pub fn calc_key_hash(&self) -> Result<GenericArray<u8, U32>, KdbxKeyError> {
        let mut key_parts: Vec<Vec<u8>> = Vec::new();

        if let Some(ref master_key) = self.master_key {
            let master_key_hash = calculate_sha256(master_key.expose_secret().as_bytes());
            key_parts.push(master_key_hash.to_vec());
        }

        if let Some(ref key_file_buf) = self.key_file {
            key_parts.push(parse_keyfile(key_file_buf.expose_secret())?);
        }

        if key_parts.is_empty() {
            return Err(KdbxKeyError::NoKeyParts);
        }

        let result = Ok(crypto::hash::calculate_sha256_multiple(
            &key_parts.iter().map(Vec::as_slice).collect::<Vec<&[u8]>>(),
        ));
        key_parts.zeroize();
        result
    }
}

pub fn parse_keyfile(key_buf: &[u8]) -> Result<Vec<u8>, KdbxKeyError> {
    if let Some(v) = try_parse_xml_keyfile(key_buf)? {
        return Ok(v);
    }
    if key_buf.len() == 32 {
        return Ok(key_buf.to_vec());
    }
    if let Some(v) = try_parse_hex_keyfile(key_buf) {
        return Ok(v);
    }
    Ok(calculate_sha256(key_buf).to_vec())
}

fn try_parse_hex_keyfile(key_buf: &[u8]) -> Option<Vec<u8>> {
    if key_buf.len() != 64 {
        return None;
    }
    let s = std::str::from_utf8(key_buf).ok()?;
    if !s.bytes().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    hex::decode(s).ok().filter(|v| v.len() == 32)
}

fn try_parse_xml_keyfile(key_buf: &[u8]) -> Result<Option<Vec<u8>>, KdbxKeyError> {
    let xml = match std::str::from_utf8(key_buf) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let package = match parser::parse(xml) {
        Ok(p) => p,
        Err(_) => return Ok(None),
    };
    let document = package.as_document();

    let version: Option<String> = evaluate_xpath(&document, "//Meta/Version/text()")
        .ok()
        .map(|v| v.string());

    let data = match evaluate_xpath(&document, "//Key/Data/text()") {
        Ok(v) => {
            let s = v.string().split_whitespace().collect::<String>();
            if s.is_empty() {
                return Ok(None);
            }
            s
        }
        Err(_) => return Ok(None),
    };

    if version.as_deref().is_some_and(|v| v.starts_with('2')) {
        let key = hex::decode(&data).map_err(|_| KdbxKeyError::FailedToParseKeyFile)?;
        let hash_str = evaluate_xpath(&document, "//Key/Data/@Hash")
            .ok()
            .map(|v| v.string().split_whitespace().collect::<String>())
            .filter(|s| !s.is_empty())
            .ok_or(KdbxKeyError::InvalidKeyFileHash)?;
        let expected = hex::decode(&hash_str).map_err(|_| KdbxKeyError::FailedToParseKeyFile)?;
        let computed = calculate_sha256(&key);
        let prefix = computed
            .get(..expected.len())
            .ok_or(KdbxKeyError::InvalidKeyFileHash)?;
        if prefix != expected.as_slice() {
            return Err(KdbxKeyError::InvalidKeyFileHash);
        }
        return Ok(Some(key));
    }

    let key_bytes = data.as_bytes().to_vec();

    if let Ok(key) = base64::engine::general_purpose::STANDARD.decode(&key_bytes) {
        Ok(Some(key))
    } else {
        Ok(Some(key_bytes))
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
        let result = super::try_parse_xml_keyfile(XML_KEYFILE1.as_bytes())
            .unwrap()
            .unwrap();
        assert_eq!(result.len(), 32);
        assert_eq!(
            result,
            hex!("5D008FBC4E6BE14A89CAC795DDB9A180D7662141E6662ECC8D33E1680882516D")
        );
    }

    #[test]
    fn test_try_parse_xml_keyfile2() {
        let result = super::try_parse_xml_keyfile(XML_KEYFILE2.as_bytes())
            .unwrap()
            .unwrap();
        assert_eq!(result.len(), 32);
        assert_eq!(
            result,
            hex!("357C9888930783796CF9E0668DB02359E73D95C393A098A87DB84D88531324CC")
        );
    }

    #[test]
    fn test_parse_hex_keyfile() {
        let hex_key = b"5D008FBC4E6BE14A89CAC795DDB9A180D7662141E6662ECC8D33E1680882516D";
        let key = super::parse_keyfile(hex_key).unwrap();
        assert_eq!(
            key,
            hex!("5D008FBC4E6BE14A89CAC795DDB9A180D7662141E6662ECC8D33E1680882516D")
        );
    }

    #[test]
    fn test_parse_raw_32_keyfile() {
        let raw = [0xABu8; 32];
        let key = super::parse_keyfile(&raw).unwrap();
        assert_eq!(key, raw);
    }

    #[test]
    fn test_xml_v2_hash_mismatch() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<KeyFile>
    <Meta>
        <Version>2.0</Version>
    </Meta>
    <Key>
        <Data Hash="00000000">
            5D008FBC 4E6BE14A 89CAC795 DDB9A180
            D7662141 E6662ECC 8D33E168 0882516D
        </Data>
    </Key>
</KeyFile>"#;
        let err = super::parse_keyfile(xml.as_bytes()).unwrap_err();
        assert!(matches!(err, super::KdbxKeyError::InvalidKeyFileHash));
    }

    #[test]
    fn test_xml_v2_missing_hash() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<KeyFile>
    <Meta>
        <Version>2.0</Version>
    </Meta>
    <Key>
        <Data>
            5D008FBC 4E6BE14A 89CAC795 DDB9A180
            D7662141 E6662ECC 8D33E168 0882516D
        </Data>
    </Key>
</KeyFile>"#;
        let err = super::parse_keyfile(xml.as_bytes()).unwrap_err();
        assert!(matches!(err, super::KdbxKeyError::InvalidKeyFileHash));
    }
}
