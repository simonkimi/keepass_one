use crate::kdbx::{self, db::kdbx4::errors::Kdbx4HeaderError};
use hex_literal::hex;

const COMPRESSION_CONFIG_NONE: [u8; 4] = hex!("00000000");
const COMPRESSION_CONFIG_GZIP: [u8; 4] = hex!("01000000");

pub enum CompressionConfig {
    None,
    GZip,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        CompressionConfig::GZip
    }
}

impl TryFrom<&[u8]> for CompressionConfig {
    type Error = Kdbx4HeaderError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value == COMPRESSION_CONFIG_NONE {
            Ok(CompressionConfig::None)
        } else if value == COMPRESSION_CONFIG_GZIP {
            Ok(CompressionConfig::GZip)
        } else {
            Err(Kdbx4HeaderError::InvalidHeader)
        }
    }
}

impl CompressionConfig {
    pub fn get_compression(&self) -> Box<dyn kdbx::compression::Compression> {
        match self {
            CompressionConfig::None => Box::new(kdbx::compression::NoCompression {}),
            CompressionConfig::GZip => Box::new(kdbx::compression::GZipCompression {}),
        }
    }

    pub fn write(&self) -> &[u8] {
        match self {
            CompressionConfig::None => &COMPRESSION_CONFIG_NONE,
            CompressionConfig::GZip => &COMPRESSION_CONFIG_GZIP,
        }
    }
}
