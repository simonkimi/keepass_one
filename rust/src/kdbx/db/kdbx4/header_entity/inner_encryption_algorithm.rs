use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub enum InnerEncryptionAlgorithm {
    Salsa20,
    ChaCha20,
}

impl TryFrom<u32> for InnerEncryptionAlgorithm {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(InnerEncryptionAlgorithm::Salsa20),
            3 => Ok(InnerEncryptionAlgorithm::ChaCha20),
            _ => Err(anyhow::anyhow!(
                "Unknown inner encryption algorithm: {}",
                value
            )),
        }
    }
}
