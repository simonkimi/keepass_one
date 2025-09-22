use crate::crypto::ciphers::{AES256Cipher, ChaCha20Cipher, Cipher, TwofishCipher};
use crate::kdbx::db::variant_dictionary::VariantDictionary;
use crate::utils::cursor_utils::CursorExt;
use crate::{crypto, kdbx};
use anyhow::anyhow;
use byteorder::ByteOrder;
use byteorder::{LittleEndian, ReadBytesExt};
use hex_literal::hex;
use std::io::{Cursor, Seek, SeekFrom};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum Kdbx4HeaderError {
    #[error("Invalid KDBX header")]
    InvalidHeader,

    #[error("Invalid variant dictionary")]
    InvalidVariantDictionary,
}

pub enum CompressionConfig {
    None,
    GZip,
}

impl TryFrom<u32> for CompressionConfig {
    type Error = Kdbx4HeaderError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CompressionConfig::None),
            1 => Ok(CompressionConfig::GZip),
            _ => Err(Kdbx4HeaderError::InvalidHeader),
        }
    }
}

impl CompressionConfig {
    pub fn get_compression(&self) -> Box<dyn kdbx::compression::Compression> {
        match self {
            CompressionConfig::None => Box::new(kdbx::compression::NoCompression {}),
            CompressionConfig::GZip => Box::new(kdbx::compression::GZipCompression {}),
        }
    }
}

pub enum EncryptionAlgorithm {
    Aes256,
    ChaCha20,
    Twofish,
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
            Err(Kdbx4HeaderError::InvalidHeader)
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
        variant: argon2::Variant,
    },
}

impl KdfConfig {
    pub fn get_kdf(&self) -> Box<dyn crypto::kdf::Kdf> {
        match self {
            KdfConfig::Aes { salt, rounds } => Box::new(crypto::kdf::AesKdf {
                seed: salt.to_vec(),
                rounds: *rounds,
            }),
            KdfConfig::Argon2 {
                version,
                salt,
                iterations,
                memory,
                parallelism,
                variant,
            } => {
                Box::new(crypto::kdf::Argon2Kdf {
                    version: match version {
                        0x10 => argon2::Version::Version10,
                        0x13 => argon2::Version::Version13,
                        _ => argon2::Version::Version13, // 默认使用最新版本
                    },
                    salt: salt.to_vec(),
                    iterations: *iterations,
                    memory: *memory,
                    parallelism: *parallelism,
                    variant: *variant,
                })
            }
        }
    }
}

pub struct Kdbx4Header {
    pub encryption_algorithm: EncryptionAlgorithm,
    pub compression_config: CompressionConfig,
    pub master_salt_seed: [u8; 32],
    pub encryption_iv: Vec<u8>,
    pub kdf_parameters: KdfConfig,
    pub public_custom_data: Option<VariantDictionary>,
}

impl Kdbx4Header {
    pub fn parse(reader: &[u8]) -> Result<(Self, usize), Kdbx4HeaderError> {
        let mut encryption_algorithm: Option<EncryptionAlgorithm> = None;
        let mut compression_config: Option<CompressionConfig> = None;
        let mut master_salt_seed: Option<[u8; 32]> = None;
        let mut encryption_iv: Option<Vec<u8>> = None;
        let mut kdf_parameters: Option<KdfConfig> = None;
        let mut public_custom_data: Option<VariantDictionary> = None;

        let reader = &mut Cursor::new(reader);
        reader
            .seek(SeekFrom::Start(12))
            .map_err(|_| Kdbx4HeaderError::InvalidHeader)?;

        loop {
            let hf_type = reader
                .read_u8()
                .map_err(|_| Kdbx4HeaderError::InvalidHeader)?;
            let hf_size = reader
                .read_u32::<LittleEndian>()
                .map_err(|_| Kdbx4HeaderError::InvalidHeader)? as usize;
            let hf_buffer = reader
                .read_slice(hf_size)
                .map_err(|_| Kdbx4HeaderError::InvalidHeader)?;

            match hf_type {
                HEADER_END => {
                    break;
                }
                HEADER_ENCRYPTION_ALGORITHM => {
                    encryption_algorithm = Some(EncryptionAlgorithm::try_from(hf_buffer)?);
                }
                HEADER_COMPRESSION_ALGORITHM => {
                    compression_config = Some(CompressionConfig::try_from(LittleEndian::read_u32(
                        &hf_buffer,
                    ))?)
                }
                HEADER_MASTER_SEED => {
                    if hf_buffer.len() != 32 {
                        return Err(Kdbx4HeaderError::InvalidHeader);
                    }
                    master_salt_seed = Some(hf_buffer[..32].try_into().unwrap());
                }
                HEADER_ENCRYPTION_IV => {
                    encryption_iv = Some(hf_buffer.to_vec());
                }
                HEADER_KDF_PARAMETERS => {
                    let vd = VariantDictionary::parse(&hf_buffer)
                        .map_err(|_| Kdbx4HeaderError::InvalidVariantDictionary)?;
                    let kdf = parse_kdf_keys(&vd)
                        .map_err(|_| Kdbx4HeaderError::InvalidVariantDictionary)?;
                    kdf_parameters = Some(kdf);
                }

                HEADER_PUBLIC_CUSTOM_DATA => {
                    let vd = VariantDictionary::parse(&hf_buffer)
                        .map_err(|_| Kdbx4HeaderError::InvalidVariantDictionary)?;
                    public_custom_data = Some(vd);
                }
                _ => {
                    return Err(Kdbx4HeaderError::InvalidHeader);
                }
            }
        }

        Ok((
            Kdbx4Header {
                encryption_algorithm: encryption_algorithm
                    .ok_or(Kdbx4HeaderError::InvalidHeader)?,
                compression_config: compression_config.ok_or(Kdbx4HeaderError::InvalidHeader)?,
                master_salt_seed: master_salt_seed.ok_or(Kdbx4HeaderError::InvalidHeader)?,
                encryption_iv: encryption_iv.ok_or(Kdbx4HeaderError::InvalidHeader)?,
                kdf_parameters: kdf_parameters.ok_or(Kdbx4HeaderError::InvalidHeader)?,
                public_custom_data,
            },
            reader.position() as usize,
        ))
    }
}

fn parse_kdf_keys(vd: &VariantDictionary) -> anyhow::Result<KdfConfig> {
    let uuid: &Vec<u8> = vd.get("$UUID")?;
    if uuid == &KDF_AES {
        let salt: &Vec<u8> = vd.get("S")?;
        if salt.len() != 32 {
            return Err(anyhow!("Aes密钥长度不匹配"));
        }

        let rounds: &u64 = vd.get("R")?;

        Ok(KdfConfig::Aes {
            salt: salt[..].try_into()?,
            rounds: *rounds,
        })
    } else if uuid == &KDF_ARGON2D || uuid == &KDF_ARGON2ID {
        let version: &u32 = vd.get("V")?;
        let salt: &Vec<u8> = vd.get("S")?;
        let iterations: &u64 = vd.get("I")?;
        let memory: &u64 = vd.get("M")?;
        let parallelism: &u32 = vd.get("P")?;

        if version != &0x10 && version != &0x13 {
            return Err(anyhow!("不支持的Argon2版本"));
        }

        Ok(KdfConfig::Argon2 {
            version: *version,
            salt: salt[..].into(),
            iterations: *iterations,
            memory: *memory,
            parallelism: *parallelism,
            variant: if uuid == &KDF_ARGON2D {
                argon2::Variant::Argon2d
            } else {
                argon2::Variant::Argon2id
            },
        })
    } else {
        Err(anyhow!(""))
    }
}
