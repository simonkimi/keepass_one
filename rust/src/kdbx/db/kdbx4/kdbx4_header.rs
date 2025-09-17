use crate::kdbx::db::variant_dictionary::{VariantDictionary, VariantDictionaryValue};
use crate::kdbx::error::KdbxFileError;
use crate::utils::cursor_utils::CursorExt;
use anyhow::anyhow;
use byteorder::ByteOrder;
use byteorder::{LittleEndian, ReadBytesExt};
use hex_literal::hex;
use std::io::Cursor;

const HEADER_END: u8 = 0;
const HEADER_ENCRYPTION_ALGORITHM: u8 = 2;
const HEADER_COMPRESSION_ALGORITHM: u8 = 3;
const HEADER_MASTER_SEED: u8 = 4;
const HEADER_ENCRYPTION_IV: u8 = 7;
const HEADER_KDF_PARAMETERS: u8 = 11;
const HEADER_PUBLIC_CUSTOM_DATA: u8 = 12;

const CIPHERSUITE_AES256: [u8; 16] = hex!("31C1F2E6BF714350BE5805216AFC5AFF");
const CIPHERSUITE_CHACHA20: [u8; 16] = hex!("D6038A2B8B6F4CB5A524339A31DBB59A");
const CIPHERSUITE_TWOFISH: [u8; 16] = hex!("AD68F29F576F4BB9A36AD47AF965346C");

const KDF_AES: [u8; 16] = hex!("C9D9F39A628A4460BF740D08C18A4FEA");
const KDF_ARGON2D: [u8; 16] = hex!("EF636DDF8C29444B91F7A9A403E30A0C");
const KDF_ARGON2ID: [u8; 16] = hex!("9E298B1956DB4773B23DFC3EC6F0A1E6");

pub enum CompressionConfig {
    None,
    GZip,
}

impl TryFrom<u32> for CompressionConfig {
    type Error = KdbxFileError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CompressionConfig::None),
            1 => Ok(CompressionConfig::GZip),
            _ => Err(KdbxFileError::InvalidHeader),
        }
    }
}

pub enum EncryptionAlgorithm {
    Aes256,
    ChaCha20,
    Twofish,
}

impl TryFrom<&[u8]> for EncryptionAlgorithm {
    type Error = KdbxFileError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value == CIPHERSUITE_AES256 {
            Ok(EncryptionAlgorithm::Aes256)
        } else if value == CIPHERSUITE_CHACHA20 {
            Ok(EncryptionAlgorithm::ChaCha20)
        } else if value == CIPHERSUITE_TWOFISH {
            Ok(EncryptionAlgorithm::Twofish)
        } else {
            Err(KdbxFileError::InvalidHeader)
        }
    }
}

pub enum KdfConfig {
    Aes {
        salt: [u8; 32],
        rounds: u64,
    },
    Argon2 {
        version: u32,
        salt: Vec<u8>,
        iterations: u64,
        memory: u64,
        parallelism: u32,
    },
}

struct Kdbx4Header {
    encryption_algorithm: Option<EncryptionAlgorithm>,
    compression_config: Option<CompressionConfig>,
    master_salt_seed: Option<[u8; 32]>,
    encryption_iv: Option<Vec<u8>>,
    kdf_parameters: Option<KdfConfig>,
    public_custom_data: Option<Vec<u8>>,
}

impl Kdbx4Header {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, KdbxFileError> {
        let mut entity = Kdbx4Header {
            encryption_algorithm: None,
            compression_config: None,
            master_salt_seed: None,
            encryption_iv: None,
            kdf_parameters: None,
            public_custom_data: None,
        };

        loop {
            let hf_type = cursor.read_u8().map_err(|_| KdbxFileError::InvalidHeader)?;
            let hf_size = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| KdbxFileError::InvalidHeader)? as usize;
            let hf_buffer = cursor
                .read_slice(hf_size)
                .map_err(|_| KdbxFileError::InvalidHeader)?;

            match hf_type {
                HEADER_END => {
                    break;
                }
                HEADER_ENCRYPTION_ALGORITHM => {}
                HEADER_COMPRESSION_ALGORITHM => {
                    entity.compression_config = Some(CompressionConfig::try_from(
                        LittleEndian::read_u32(&hf_buffer),
                    )?)
                }
                HEADER_MASTER_SEED => {
                    if hf_buffer.len() != 32 {
                        return Err(KdbxFileError::InvalidHeader);
                    }
                    entity.master_salt_seed = Some(hf_buffer[..32].try_into().unwrap());
                }
                HEADER_ENCRYPTION_IV => {
                    entity.encryption_iv = Some(hf_buffer.to_vec());
                }
                HEADER_KDF_PARAMETERS => {
                    let value = VariantDictionary::parse(&hf_buffer)
                        .map_err(|_| KdbxFileError::InvalidVariantDictionary)?;
                    let kdf = parse_kdf_keys(&value)
                        .map_err(|_| KdbxFileError::InvalidVariantDictionary)?;
                    entity.kdf_parameters = Some(kdf);
                }
                _ => {
                    return Err(KdbxFileError::InvalidHeader);
                }
            }
        }

        Ok(entity)
    }
}

fn parse_kdf_keys(vd: &VariantDictionary) -> anyhow::Result<KdfConfig> {
    let uuid = vd.get::<Vec<u8>>("$UUID")?;
    if uuid == &KDF_AES {
        let salt = vd.get::<Vec<u8>>("S")?;
        if salt.len() != 32 {
            return Err(anyhow!("Aes密钥长度不匹配"));
        }

        let rounds = vd.get::<u64>("R")?;

        Ok(KdfConfig::Aes {
            salt: salt[..].try_into().unwrap(),
            rounds: *rounds,
        })
    } else if uuid == &KDF_ARGON2D || uuid == &KDF_ARGON2ID {
        let version = vd.get::<u32>("V")?;
        let salt = vd.get::<Vec<u8>>("S")?;
        let iterations = vd.get::<u64>("I")?;
        let memory = vd.get::<u64>("M")?;
        let parallelism = vd.get::<u32>("P")?;

        Ok(KdfConfig::Argon2 {
            version: *version,
            salt: salt[..].into(),
            iterations: *iterations,
            memory: *memory,
            parallelism: *parallelism,
        })
    } else {
        Err(anyhow!(""))
    }
}
