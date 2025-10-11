use thiserror::Error;

use crate::crypto::errors::CryptoError;
use crate::crypto::kdf::KdfError;
use crate::crypto::memory_crypt::SecureDataError;

#[derive(Debug, Error)]

pub enum KdbxDatabaseError {
    #[error("XML parse error")]
    XmlParseError(#[from] quick_xml::DeError),

    #[error("Protected value offset not found")]
    ProtectedValueOffsetNotFound,

    #[error("Protected value decrypt error")]
    ProtectedValueDecryptError(#[from] CryptoError),

    #[error("Secure data error")]
    SecureDataError(#[from] SecureDataError),
}

#[derive(Debug, Error)]
pub enum KdbxSaveError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Cipher error")]
    CipherError(#[from] CryptoError),

    #[error("Secure data error")]
    SecureDataError(#[from] SecureDataError),

    #[error("KDF error")]
    KdfError(#[from] KdfError),
}
