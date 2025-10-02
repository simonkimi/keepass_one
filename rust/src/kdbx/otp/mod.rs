pub mod rfc6238;
pub mod steam;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TotpGenerateError {
    #[error("Invalid secret key length")]
    InvalidSecretLength,
    
    #[error("HMAC key length invalid: {0}")]
    HmacKeyError(#[from] hmac::digest::InvalidLength),
    
    #[error("Invalid timestamp")]
    InvalidTimestamp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TotpCode {
    pub code: String,
    pub period_start: u64,
    pub period_end: u64,
}

pub trait Totp {
    fn generate(&self, timestamp: u64) -> Result<TotpCode, TotpGenerateError>;
    fn get_issuer(&self) -> Option<&str>;
    fn get_account(&self) -> Option<&str>;
}
