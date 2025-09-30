use thiserror::Error;

use crate::crypto::errors::CryptoError;

#[derive(Debug, Error)]

pub enum KdbxDatabaseError {
    #[error("XML parse error")]
    XmlParseError(#[from] quick_xml::DeError),

    #[error("Protected value offset not found")]
    ProtectedValueOffsetNotFound,

    #[error("Protected value decrypt error")]
    ProtectedValueDecryptError(#[from] CryptoError),
}
