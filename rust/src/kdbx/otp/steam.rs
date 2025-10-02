use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::kdbx::otp::{Totp, TotpCode, TotpGenerateError};

const STEAM_ALPHABET: &[u8] = b"23456789BCDFGHJKMNPQRTVWXY";
const STEAM_CODE_LENGTH: usize = 5;
const STEAM_PERIOD: u64 = 30;

pub struct SteamTotp {
    pub secret: Vec<u8>,
}

impl Totp for SteamTotp {
    fn get_issuer(&self) -> Option<&str> {
        None
    }

    fn get_account(&self) -> Option<&str> {
        None
    }

    fn generate(&self, timestamp: u64) -> Result<TotpCode, TotpGenerateError> {
        if self.secret.is_empty() {
            return Err(TotpGenerateError::InvalidSecretLength);
        }
        

        let counter = timestamp / STEAM_PERIOD;
        
        let period_start = counter * STEAM_PERIOD;
        let period_end = period_start + STEAM_PERIOD;
        
        let counter_bytes = counter.to_be_bytes();
        
        let mut mac = Hmac::<Sha1>::new_from_slice(&self.secret)?;
        mac.update(&counter_bytes);
        let hmac_result = mac.finalize().into_bytes();
        
        let offset = (hmac_result[hmac_result.len() - 1] & 0x0f) as usize;
        
        let mut full_code = u32::from_be_bytes([
            hmac_result[offset],
            hmac_result[offset + 1],
            hmac_result[offset + 2],
            hmac_result[offset + 3],
        ]);
        
        full_code &= 0x7fffffff;
        
        let alphabet_len = STEAM_ALPHABET.len() as u32;
        let mut code = String::with_capacity(STEAM_CODE_LENGTH);
        for _ in 0..STEAM_CODE_LENGTH {
            let index = (full_code % alphabet_len) as usize;
            code.push(STEAM_ALPHABET[index] as char);
            full_code /= alphabet_len;
        }
        
        Ok(TotpCode {
            code,
            period_start,
            period_end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steam_generate() {
        let secret = b"12345678901234567890";
        let totp = SteamTotp {
            secret: secret.to_vec(),
        };

        let result = totp.generate(1234567890).unwrap();
        
        assert_eq!(result.code.len(), STEAM_CODE_LENGTH);
        
        assert!(result.code.chars().all(|c| STEAM_ALPHABET.contains(&(c as u8))));
    }

    #[test]
    fn test_steam_alphabet() {
        assert_eq!(STEAM_ALPHABET.len(), 26);
        
        assert_eq!(STEAM_ALPHABET, b"23456789BCDFGHJKMNPQRTVWXY");
    }

    #[test]
    fn test_steam_different_times_produce_different_codes() {
        let secret = b"test_secret_key";
        let totp = SteamTotp {
            secret: secret.to_vec(),
        };

        let result1 = totp.generate(1000000000).unwrap();
        let result2 = totp.generate(2000000000).unwrap();
        
        assert_ne!(result1.code, result2.code);
    }

    #[test]
    fn test_steam_same_period_produces_same_code() {
        let secret = b"test_secret_key";
        let totp = SteamTotp {
            secret: secret.to_vec(),
        };

        let result1 = totp.generate(60).unwrap();
        let result2 = totp.generate(70).unwrap();
        let result3 = totp.generate(89).unwrap();
        
        assert_eq!(result1.code, result2.code);
        assert_eq!(result2.code, result3.code);
        
        let result4 = totp.generate(90).unwrap();
        assert_ne!(result1.code, result4.code);
    }

    #[test]
    fn test_steam_period_times() {
        let secret = b"test_secret_key";
        let totp = SteamTotp {
            secret: secret.to_vec(),
        };

        let result = totp.generate(59).unwrap();
        assert_eq!(result.period_start, 30);
        assert_eq!(result.period_end, 60);

        let result = totp.generate(0).unwrap();
        assert_eq!(result.period_start, 0);
        assert_eq!(result.period_end, 30);
    }

    #[test]
    fn test_steam_empty_secret_error() {
        let totp = SteamTotp {
            secret: vec![],
        };

        let result = totp.generate(1234567890);
        assert!(result.is_err());
        assert!(matches!(result, Err(TotpGenerateError::InvalidSecretLength)));
    }
}
