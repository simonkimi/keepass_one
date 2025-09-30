use byteorder::{WriteBytesExt, LE};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::kdbx::db::kdbx4::errors::Kdbx4InnerHeaderError;
use crate::utils::writer::{FixedSize, Writable};

const INNER_ENCRYPTION_ALGORITHM_SALSA20: u32 = 2;
const INNER_ENCRYPTION_ALGORITHM_CHACHA20: u32 = 3;

#[derive(Zeroize, ZeroizeOnDrop)]
pub enum InnerEncryptionAlgorithm {
    Salsa20,
    ChaCha20,
}

impl TryFrom<u32> for InnerEncryptionAlgorithm {
    type Error = Kdbx4InnerHeaderError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            INNER_ENCRYPTION_ALGORITHM_SALSA20 => Ok(InnerEncryptionAlgorithm::Salsa20),
            INNER_ENCRYPTION_ALGORITHM_CHACHA20 => Ok(InnerEncryptionAlgorithm::ChaCha20),
            _ => Err(Kdbx4InnerHeaderError::UnknownInnerEncryptionAlgorithm(value)),
        }
    }
}

impl Writable for InnerEncryptionAlgorithm {
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        writer.write_u32::<LE>(match self {
            InnerEncryptionAlgorithm::Salsa20 => INNER_ENCRYPTION_ALGORITHM_SALSA20,
            InnerEncryptionAlgorithm::ChaCha20 => INNER_ENCRYPTION_ALGORITHM_CHACHA20,
        })?;
        Ok(())
    }
}

impl FixedSize for InnerEncryptionAlgorithm {
    fn fix_size(&self) -> usize {
        4
    }
}
