use thiserror::Error;

use crate::crypto::errors::CryptoError;
use crate::kdbx::db::kdbx4::header_entity::kdf_config::KdfConfigError;
use crate::kdbx::db::kdbx4::header_entity::variant_dictionary::VariantDictionaryError;
use crate::crypto::kdf::KdfError;
use crate::kdbx::xml::errors::KdbxDatabaseError;

#[derive(Debug, Error)]
pub enum Kdbx4HeaderError {
    #[error("Invalid KDBX header")]
    InvalidHeader,

    #[error("Invalid variant dictionary")]
    InvalidVariantDictionary(#[from] VariantDictionaryError),

    #[error("Invalid master seed")]
    InvalidMasterSeed,

    #[error("Invalid encryption algorithm")]
    InvalidEncryptionAlgorithm(String),

    #[error("Invalid compression algorithm")]
    InvalidCompressionAlgorithm(u32),

    #[error("Invalid KDF parameters")]
    InvalidKdfParameters(#[from] KdfConfigError),
}

#[derive(Debug, Error)]
pub enum Kdbx4InnerHeaderError {
    #[error("Unknown inner header type: {0}")]
    UnknownInnerHeaderType(u8),

    #[error("Unknown inner encryption algorithm: {0}")]
    UnknownInnerEncryptionAlgorithm(u32),

    #[error("Missing inner encryption algorithm")]
    MissingInnerEncryptionAlgorithm,

    #[error("Missing inner encryption key")]
    MissingInnerEncryptionKey,
}

#[derive(Debug, Error)]
pub enum Kdbx4Error {
    #[error("Invalid KDBX header")]
    InvalidHeader(#[from] Kdbx4HeaderError),

    #[error("Header SHA-256 checksum mismatch")]
    HeaderSha256ChecksumMismatch,

    #[error("Header HMAC checksum mismatch")]
    HeaderHmacChecksumMismatch,
    
    #[error("KDF transform key error")]
    KdfTransformKeyError(#[from] KdfError),

    #[error("Decrypt payload error")]
    DecryptPayloadError(#[from] CryptoError),

    #[error("Decompress payload error")]
    DecompressPayloadError(#[from] std::io::Error),

    #[error("Inner header error")]
    InnerHeaderError(#[from] Kdbx4InnerHeaderError),

    #[error("XML parse error")]
    DatabaseError(#[from] KdbxDatabaseError),
}
