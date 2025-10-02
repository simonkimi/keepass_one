use std::str::FromStr;

use data_encoding::BASE32_NOPAD;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use thiserror::Error;
use url::Url;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::kdbx::otp::{Totp, TotpCode, TotpGenerateError};

const DEFAULT_DIGITS: u32 = 6;
const DEFAULT_PERIOD: u32 = 30;

#[derive(Debug, Error)]
pub enum Rfc6238ParseError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Invalid otpauth URL: must use totp type")]
    InvalidOtpAuthType,

    #[error("Missing required parameter: {0}")]
    MissingParameter(&'static str),

    #[error("Invalid Base32 encoding: {0}")]
    InvalidBase32(#[from] data_encoding::DecodeError),

    #[error("Invalid algorithm: {0}")]
    InvalidAlgorithm(String),

    #[error("Invalid digits format: {0}")]
    InvalidDigits(#[from] std::num::ParseIntError),
}

#[derive(Zeroize, ZeroizeOnDrop, PartialEq, Debug)]
pub enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

impl Default for TotpAlgorithm {
    fn default() -> Self {
        TotpAlgorithm::Sha1
    }
}

#[derive(Debug)]
pub struct Rfc6238Totp {
    pub issuer: Option<String>,
    pub account: Option<String>,
    pub algorithm: TotpAlgorithm,
    pub digits: u32,
    pub period: u32,
    pub secret: Vec<u8>,
}

impl Rfc6238Totp {
    fn from_url(url: &Url) -> Result<Self, Rfc6238ParseError> {
        if url.scheme() != "otpauth" {
            return Err(Rfc6238ParseError::InvalidUrl(
                url::ParseError::InvalidDomainCharacter,
            ));
        }

        if url.host_str() != Some("totp") {
            return Err(Rfc6238ParseError::InvalidOtpAuthType);
        }

        let path = url.path().trim_start_matches('/');
        let (issuer_from_path, account) = if let Some(colon_pos) = path.find(':') {
            let issuer = path[..colon_pos].to_string();
            let account_str = path[colon_pos + 1..].to_string();
            let account = if account_str.is_empty() || account_str == "none" {
                None
            } else {
                Some(account_str)
            };
            (Some(issuer), account)
        } else if !path.is_empty() && path != "none" {
            (None, Some(path.to_string()))
        } else {
            (None, None)
        };

        let query_params: std::collections::HashMap<_, _> = url.query_pairs().collect();

        let secret_str = query_params
            .get("secret")
            .ok_or(Rfc6238ParseError::MissingParameter("secret"))?;
        let secret = BASE32_NOPAD.decode(secret_str.to_uppercase().as_bytes())?;

        let issuer = query_params
            .get("issuer")
            .map(|s| s.to_string())
            .or(issuer_from_path);

        let algorithm = if let Some(algo_str) = query_params.get("algorithm") {
            match algo_str.to_uppercase().as_str() {
                "SHA1" => TotpAlgorithm::Sha1,
                "SHA256" => TotpAlgorithm::Sha256,
                "SHA512" => TotpAlgorithm::Sha512,
                _ => return Err(Rfc6238ParseError::InvalidAlgorithm(algo_str.to_string())),
            }
        } else {
            TotpAlgorithm::default()
        };

        let digits = if let Some(digits_str) = query_params.get("digits") {
            digits_str.parse()?
        } else {
            DEFAULT_DIGITS
        };

        let period = if let Some(period_str) = query_params.get("period") {
            period_str.parse()?
        } else {
            DEFAULT_PERIOD
        };

        Ok(Rfc6238Totp {
            issuer,
            account,
            algorithm,
            digits,
            period,
            secret,
        })
    }

    fn from_base32(base32_str: &str) -> Result<Self, Rfc6238ParseError> {
        let secret = BASE32_NOPAD.decode(base32_str.to_uppercase().as_bytes())?;

        Ok(Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::default(),
            digits: DEFAULT_DIGITS,
            period: DEFAULT_PERIOD,
            secret,
        })
    }
}

impl Totp for Rfc6238Totp {
    fn get_issuer(&self) -> Option<&str> {
        self.issuer.as_deref()
    }

    fn get_account(&self) -> Option<&str> {
        self.account.as_deref()
    }

    fn generate(&self, timestamp: u64) -> Result<TotpCode, TotpGenerateError> {
        if self.secret.is_empty() {
            return Err(TotpGenerateError::InvalidSecretLength);
        }
        
        let counter = timestamp / (self.period as u64);
        
        let period_start = counter * (self.period as u64);
        let period_end = period_start + (self.period as u64);
        
        let counter_bytes = counter.to_be_bytes();
        let hmac_result = match self.algorithm {
            TotpAlgorithm::Sha1 => {
                let mut mac = Hmac::<Sha1>::new_from_slice(&self.secret)?;
                mac.update(&counter_bytes);
                mac.finalize().into_bytes().to_vec()
            }
            TotpAlgorithm::Sha256 => {
                let mut mac = Hmac::<Sha256>::new_from_slice(&self.secret)?;
                mac.update(&counter_bytes);
                mac.finalize().into_bytes().to_vec()
            }
            TotpAlgorithm::Sha512 => {
                let mut mac = Hmac::<Sha512>::new_from_slice(&self.secret)?;
                mac.update(&counter_bytes);
                mac.finalize().into_bytes().to_vec()
            }
        };
        
        let offset = (hmac_result[hmac_result.len() - 1] & 0x0f) as usize;
        
        let code = u32::from_be_bytes([
            hmac_result[offset],
            hmac_result[offset + 1],
            hmac_result[offset + 2],
            hmac_result[offset + 3],
        ]);
        
        let code = code & 0x7fffffff;
        
        let modulus = 10u32.pow(self.digits);
        let otp = code % modulus;
        
        let code_str = format!("{:0width$}", otp, width = self.digits as usize);
        
        Ok(TotpCode {
            code: code_str,
            period_start,
            period_end,
        })
    }
}

impl FromStr for Rfc6238Totp {
    type Err = Rfc6238ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("otpauth://") {
            let url = Url::parse(s)?;
            Self::from_url(&url)
        } else {
            Self::from_base32(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_otpauth_url_full() {
        let url_str = "otpauth://totp/ACME:alice@example.com?secret=JBSWY3DPEHPK3PXP&issuer=ACME&algorithm=SHA256&digits=8&period=60";
        let totp = Rfc6238Totp::from_str(url_str).unwrap();

        assert_eq!(totp.issuer.as_deref(), Some("ACME"));
        assert_eq!(totp.account.as_deref(), Some("alice@example.com"));
        assert_eq!(totp.digits, 8);
        assert_eq!(totp.period, 60);
        assert_eq!(totp.secret, b"Hello!\xde\xad\xbe\xef");
    }

    #[test]
    fn test_parse_otpauth_url_minimal() {
        let url_str = "otpauth://totp/ACME:alice@example.com?secret=JBSWY3DPEHPK3PXP&issuer=ACME&algorithm=SHA256";
        let totp = Rfc6238Totp::from_str(url_str).unwrap();

        assert_eq!(totp.issuer.as_deref(), Some("ACME"));
        assert_eq!(totp.account.as_deref(), Some("alice@example.com"));
        assert_eq!(totp.digits, DEFAULT_DIGITS);
        assert_eq!(totp.period, DEFAULT_PERIOD);
    }

    #[test]
    fn test_parse_otpauth_url_default_algorithm() {
        let url_str = "otpauth://totp/alice@example.com?secret=JBSWY3DPEHPK3PXP";
        let totp = Rfc6238Totp::from_str(url_str).unwrap();

        assert_eq!(totp.issuer, None);
        assert_eq!(totp.account.as_deref(), Some("alice@example.com"));
        assert_eq!(totp.digits, DEFAULT_DIGITS);
        assert_eq!(totp.period, DEFAULT_PERIOD);
    }

    #[test]
    fn test_parse_otpauth_url_with_none_account() {
        let url_str = "otpauth://totp/KeePassXC:none?secret=JBSWY3DPEHPK3PXP&period=30&digits=6&issuer=KeePassXC";
        let totp = Rfc6238Totp::from_str(url_str).unwrap();

        assert_eq!(totp.issuer.as_deref(), Some("KeePassXC"));
        assert_eq!(totp.account, None);
        assert_eq!(totp.digits, 6);
        assert_eq!(totp.period, 30);
    }

    #[test]
    fn test_parse_otpauth_url_with_empty_account() {
        let url_str = "otpauth://totp/KeePassXC:?secret=JBSWY3DPEHPK3PXP&period=30&digits=6&issuer=KeePassXC";
        let totp = Rfc6238Totp::from_str(url_str).unwrap();

        assert_eq!(totp.issuer.as_deref(), Some("KeePassXC"));
        assert_eq!(totp.account, None);
        assert_eq!(totp.digits, 6);
        assert_eq!(totp.period, 30);
    }

    #[test]
    fn test_parse_otpauth_url_with_none_as_path() {
        let url_str = "otpauth://totp/none?secret=JBSWY3DPEHPK3PXP";
        let totp = Rfc6238Totp::from_str(url_str).unwrap();

        assert_eq!(totp.issuer, None);
        assert_eq!(totp.account, None);
        assert_eq!(totp.digits, DEFAULT_DIGITS);
        assert_eq!(totp.period, DEFAULT_PERIOD);
    }

    #[test]
    fn test_parse_base32_secret() {
        let secret = "JBSWY3DPEHPK3PXP";
        let totp = Rfc6238Totp::from_str(secret).unwrap();

        assert_eq!(totp.issuer, None);
        assert_eq!(totp.account, None);
        assert_eq!(totp.algorithm, TotpAlgorithm::Sha1);
        assert_eq!(totp.digits, DEFAULT_DIGITS);
        assert_eq!(totp.period, DEFAULT_PERIOD);
        assert_eq!(totp.secret, b"Hello!\xde\xad\xbe\xef");
    }

    #[test]
    fn test_parse_base32_secret_lowercase() {
        let secret = "jbswy3dpehpk3pxp";
        let totp = Rfc6238Totp::from_str(secret).unwrap();

        assert_eq!(totp.secret, b"Hello!\xde\xad\xbe\xef");
    }

    #[test]
    fn test_parse_invalid_base32() {
        let secret = "INVALID!@#$";
        let result = Rfc6238Totp::from_str(secret);

        assert!(result.is_err());
        assert!(matches!(result, Err(Rfc6238ParseError::InvalidBase32(_))));
    }

    #[test]
    fn test_parse_url_missing_secret() {
        let url_str = "otpauth://totp/ACME:alice@example.com?issuer=ACME";
        let result = Rfc6238Totp::from_str(url_str);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Rfc6238ParseError::MissingParameter("secret"))
        ));
    }

    #[test]
    fn test_parse_url_invalid_type() {
        let url_str = "otpauth://hotp/ACME:alice@example.com?secret=JBSWY3DPEHPK3PXP";
        let result = Rfc6238Totp::from_str(url_str);

        assert!(result.is_err());
        assert!(matches!(result, Err(Rfc6238ParseError::InvalidOtpAuthType)));
    }

    #[test]
    fn test_parse_url_invalid_algorithm() {
        let url_str = "otpauth://totp/alice@example.com?secret=JBSWY3DPEHPK3PXP&algorithm=MD5";
        let result = Rfc6238Totp::from_str(url_str);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Rfc6238ParseError::InvalidAlgorithm(_))
        ));
    }

    #[test]
    fn test_parse_url_invalid_digits() {
        let url_str = "otpauth://totp/alice@example.com?secret=JBSWY3DPEHPK3PXP&digits=abc";
        let result = Rfc6238Totp::from_str(url_str);

        assert!(result.is_err());
        assert!(matches!(result, Err(Rfc6238ParseError::InvalidDigits(_))));
    }

    const TEST_SECRET_SHA1: &[u8] = b"12345678901234567890";
    const TEST_SECRET_SHA256: &[u8] = b"12345678901234567890123456789012";
    const TEST_SECRET_SHA512: &[u8] =
        b"1234567890123456789012345678901234567890123456789012345678901234";

    #[test]
    fn test_generate_sha1() {
        let totp = Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::Sha1,
            digits: 8,
            period: 30,
            secret: TEST_SECRET_SHA1.to_vec(),
        };

        assert_eq!(totp.generate(59).unwrap().code, "94287082");
        assert_eq!(totp.generate(1111111109).unwrap().code, "07081804");
        assert_eq!(totp.generate(1111111111).unwrap().code, "14050471");
        assert_eq!(totp.generate(1234567890).unwrap().code, "89005924");
        assert_eq!(totp.generate(2000000000).unwrap().code, "69279037");
        assert_eq!(totp.generate(20000000000).unwrap().code, "65353130");
    }

    #[test]
    fn test_generate_sha256() {
        let totp = Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::Sha256,
            digits: 8,
            period: 30,
            secret: TEST_SECRET_SHA256.to_vec(),
        };

        assert_eq!(totp.generate(59).unwrap().code, "46119246");
        assert_eq!(totp.generate(1111111109).unwrap().code, "68084774");
        assert_eq!(totp.generate(1111111111).unwrap().code, "67062674");
        assert_eq!(totp.generate(1234567890).unwrap().code, "91819424");
        assert_eq!(totp.generate(2000000000).unwrap().code, "90698825");
        assert_eq!(totp.generate(20000000000).unwrap().code, "77737706");
    }

    #[test]
    fn test_generate_sha512() {
        let totp = Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::Sha512,
            digits: 8,
            period: 30,
            secret: TEST_SECRET_SHA512.to_vec(),
        };

        assert_eq!(totp.generate(59).unwrap().code, "90693936");
        assert_eq!(totp.generate(1111111109).unwrap().code, "25091201");
        assert_eq!(totp.generate(1111111111).unwrap().code, "99943326");
        assert_eq!(totp.generate(1234567890).unwrap().code, "93441116");
        assert_eq!(totp.generate(2000000000).unwrap().code, "38618901");
        assert_eq!(totp.generate(20000000000).unwrap().code, "47863826");
    }

    #[test]
    fn test_generate_6_digits() {
        let totp = Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period: 30,
            secret: TEST_SECRET_SHA1.to_vec(),
        };

        let result = totp.generate(59).unwrap();
        assert_eq!(result.code.len(), 6);
        assert_eq!(result.code, "287082");
    }

    #[test]
    fn test_generate_different_period() {
        let totp = Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::Sha1,
            digits: 8,
            period: 60,
            secret: TEST_SECRET_SHA1.to_vec(),
        };

        let result1 = totp.generate(59).unwrap();
        let result2 = totp.generate(119).unwrap();

        assert_ne!(result1.code, result2.code);
    }

    #[test]
    fn test_generate_with_real_secret() {
        let totp = Rfc6238Totp::from_str("JBSWY3DPEHPK3PXP").unwrap();

        let result = totp.generate(1234567890).unwrap();
        assert_eq!(result.code.len(), 6);
        assert!(result.code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_with_period_times() {
        let totp = Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period: 30,
            secret: TEST_SECRET_SHA1.to_vec(),
        };

        let result = totp.generate(59).unwrap();
        assert_eq!(result.period_start, 30);
        assert_eq!(result.period_end, 60);

        let result = totp.generate(0).unwrap();
        assert_eq!(result.period_start, 0);
        assert_eq!(result.period_end, 30);

        let result = totp.generate(90).unwrap();
        assert_eq!(result.period_start, 90);
        assert_eq!(result.period_end, 120);
    }

    #[test]
    fn test_generate_empty_secret_error() {
        let totp = Rfc6238Totp {
            issuer: None,
            account: None,
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period: 30,
            secret: vec![],
        };

        let result = totp.generate(1234567890);
        assert!(result.is_err());
        assert!(matches!(result, Err(TotpGenerateError::InvalidSecretLength)));
    }
}
