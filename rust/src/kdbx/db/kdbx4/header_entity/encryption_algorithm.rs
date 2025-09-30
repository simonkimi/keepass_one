use crate::{
    crypto::ciphers::{AES256Cipher, ChaCha20Cipher, Cipher, TwofishCipher},
    kdbx::db::kdbx4::errors::Kdbx4HeaderError,
    utils::writer::{FixedSize, Writable},
};
use hex_literal::hex;

const CIPHERSUITE_AES256: [u8; 16] = hex!("31C1F2E6BF714350BE5805216AFC5AFF");
const CIPHERSUITE_CHACHA20: [u8; 16] = hex!("D6038A2B8B6F4CB5A524339A31DBB59A");
const CIPHERSUITE_TWOFISH: [u8; 16] = hex!("AD68F29F576F4BB9A36AD47AF965346C");

#[derive(Debug, Clone)]
pub enum EncryptionAlgorithm {
    Aes256,
    ChaCha20,
    Twofish,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        EncryptionAlgorithm::Aes256
    }
}

impl TryFrom<&[u8]> for EncryptionAlgorithm {
    type Error = Kdbx4HeaderError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value == CIPHERSUITE_AES256 {
            Ok(EncryptionAlgorithm::Aes256)
        } else if value == CIPHERSUITE_CHACHA20 {
            Ok(EncryptionAlgorithm::ChaCha20)
        } else if value == CIPHERSUITE_TWOFISH {
            Ok(EncryptionAlgorithm::Twofish)
        } else {
            Err(Kdbx4HeaderError::InvalidEncryptionAlgorithm(hex::encode(
                value,
            )))
        }
    }
}

impl EncryptionAlgorithm {
    pub fn get_cipher(&self, key: &[u8], iv: &[u8]) -> Box<dyn Cipher> {
        match self {
            EncryptionAlgorithm::Aes256 => Box::new(AES256Cipher::new(key, iv)),
            EncryptionAlgorithm::ChaCha20 => Box::new(ChaCha20Cipher::new(key, iv)),
            EncryptionAlgorithm::Twofish => Box::new(TwofishCipher::new(key, iv)),
        }
    }

    pub fn get_random_iv(&self) -> Vec<u8> {
        let mut iv = match self {
            EncryptionAlgorithm::Aes256 => vec![0; 16],
            EncryptionAlgorithm::ChaCha20 => vec![0; 12],
            EncryptionAlgorithm::Twofish => vec![0; 16],
        };
        getrandom::fill(&mut iv);
        iv
    }
}

impl Writable for EncryptionAlgorithm {
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        writer.write_all(match self {
            EncryptionAlgorithm::Aes256 => &CIPHERSUITE_AES256,
            EncryptionAlgorithm::ChaCha20 => &CIPHERSUITE_CHACHA20,
            EncryptionAlgorithm::Twofish => &CIPHERSUITE_TWOFISH,
        })?;
        Ok(())
    }
}

impl FixedSize for EncryptionAlgorithm {
    fn fix_size(&self) -> usize {
        16
    }
}
