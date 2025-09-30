use generic_array::typenum::{U32, U64};
use generic_array::GenericArray;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::crypto::errors::CryptoError;

pub fn calculate_sha256(elements: &[u8]) -> GenericArray<u8, U32> {
    let mut digest = Sha256::new();
    digest.update(elements);
    digest.finalize()
}

pub fn calculate_sha256_multiple(elements: &[&[u8]]) -> GenericArray<u8, U32> {
    let mut digest = Sha256::new();
    for element in elements {
        digest.update(element);
    }
    digest.finalize()
}

pub fn calculate_sha512(elements: &[u8]) -> GenericArray<u8, U64> {
    let mut hasher = sha2::Sha512::new();
    hasher.update(elements);
    hasher.finalize()
}

pub fn calculate_sha512_multiple(elements: &[&[u8]]) -> GenericArray<u8, U64> {
    let mut hasher = sha2::Sha512::new();
    for element in elements {
        hasher.update(element);
    }
    hasher.finalize()
}

pub(crate) fn calculate_hmac_multiple(
    elements: &[&[u8]],
    key: &[u8],
) -> Result<GenericArray<u8, U32>, CryptoError> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).map_err(CryptoError::InvalidLength)?;

    for element in elements {
        mac.update(element);
    }

    let result = mac.finalize();
    Ok(result.into_bytes())
}
