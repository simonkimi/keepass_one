use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kdbx4HeaderError {
    #[error("Invalid KDBX header")]
    InvalidHeader,

    #[error("Invalid variant dictionary")]
    InvalidVariantDictionary,
}
