use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Invalid length")]
    InvalidLength(#[from] cipher::InvalidLength),
    #[error("Stream cipher error")]
    StreamCipherError(#[from] cipher::StreamCipherError),
    #[error("Unpad error")]
    UnpadError(#[from] block_padding::UnpadError),
    #[error("HMAC mismatch")]
    HmacMismatch,
}
