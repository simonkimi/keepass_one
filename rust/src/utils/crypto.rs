use sha2::{Digest, Sha256};

pub fn calculate_sha256(elements: &[u8]) -> Vec<u8> {
    let mut digest = Sha256::new();
    digest.update(elements);
    digest.finalize().to_vec()
}
