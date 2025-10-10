use crate::crypto::ciphers::{ChaCha20Cipher, Salsa20Cipher, StreamCipherExt};
use crate::crypto::hash::{calculate_sha256, calculate_sha512};
use crate::kdbx::db::kdbx4::errors::Kdbx4InnerHeaderError;
use crate::kdbx::db::kdbx4::header_entity::binary_content::BinaryContent;
use crate::kdbx::db::kdbx4::header_entity::inner_encryption_algorithm::{INNER_ENCRYPTION_ALGORITHM_KEY_SIZE_CHACHA20, InnerEncryptionAlgorithm};
use crate::utils::writer::{FixedSizeExt, Writable, WSExt};
use byteorder::LittleEndian;
use byteorder::{ByteOrder, WriteBytesExt};
use hex_literal::hex;

pub const INNER_HEADER_END_OF_HEADER: u8 = 0x00;
pub const INNER_HEADER_INNER_ENCRYPTION_ALGORITHM: u8 = 0x01;
pub const INNER_HEADER_INNER_ENCRYPTION_KEY: u8 = 0x02;
pub const INNER_HEADER_BINARY_CONTENT: u8 = 0x03;
pub const SALSA20_IV: [u8; 8] = hex!("E830094B97205D2A");

pub struct Kdbx4InnerHeader {
    pub encryption: Kdbx4InnerEncryption,
    pub binary_content: Vec<BinaryContent>,
}

pub struct Kdbx4InnerEncryption {
    pub inner_encryption_algorithm: InnerEncryptionAlgorithm,
    pub inner_encryption_key: Vec<u8>,
}

impl Kdbx4InnerHeader {
    pub fn try_from(value: &[u8]) -> Result<(Self, usize), Kdbx4InnerHeaderError> {
        let mut pos = 0;

        let mut inner_encryption_algorithm: Option<InnerEncryptionAlgorithm> = None;
        let mut inner_encryption_key: Option<Vec<u8>> = None;
        let mut binary_content_vec: Vec<BinaryContent> = Vec::new();

        loop {
            let header_type = value[pos];
            pos += 1;
            let header_size = LittleEndian::read_u32(&value[pos..pos + 4]);
            pos += 4;
            let header_data = &value[pos..pos + header_size as usize];
            pos += header_size as usize;

            match header_type {
                INNER_HEADER_END_OF_HEADER => break,
                INNER_HEADER_INNER_ENCRYPTION_ALGORITHM => {
                    let alg_value = LittleEndian::read_u32(header_data);
                    inner_encryption_algorithm =
                        Some(InnerEncryptionAlgorithm::try_from(alg_value)?);
                }
                INNER_HEADER_INNER_ENCRYPTION_KEY => {
                    inner_encryption_key = Some(header_data.to_vec());
                }
                INNER_HEADER_BINARY_CONTENT => {
                    let binary_content = BinaryContent::from(header_data);
                    binary_content_vec.push(binary_content);
                }
                _ => {
                    return Err(Kdbx4InnerHeaderError::UnknownInnerHeaderType(header_type));
                }
            }
        }

        if let None = inner_encryption_algorithm {
            return Err(Kdbx4InnerHeaderError::MissingInnerEncryptionAlgorithm);
        }

        if let None = inner_encryption_key {
            return Err(Kdbx4InnerHeaderError::MissingInnerEncryptionKey);
        }

        Ok((
            Kdbx4InnerHeader {
                encryption: Kdbx4InnerEncryption {
                    inner_encryption_algorithm: inner_encryption_algorithm.unwrap(),
                    inner_encryption_key: inner_encryption_key.unwrap(),
                },
                binary_content: binary_content_vec,
            },
            pos,
        ))
    }

    pub fn copy_with(&self, encryption: Kdbx4InnerEncryption) -> Self {
        Self {
            encryption,
            binary_content: self.binary_content.clone(),
        }
    }
}

impl Writable for Kdbx4InnerHeader {
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        writer.write_u8(INNER_HEADER_INNER_ENCRYPTION_ALGORITHM)?;
        writer.write_fixed_size_data(&self.encryption.inner_encryption_algorithm)?;

        writer.write_u8(INNER_HEADER_INNER_ENCRYPTION_KEY)?;
        writer.write_bytes_with_length(&self.encryption.inner_encryption_key)?;
        for binary_content in &self.binary_content {
            writer.write_u8(INNER_HEADER_BINARY_CONTENT)?;
            writer.write_fixed_size_data(binary_content)?;
        }

        writer.write_u8(INNER_HEADER_END_OF_HEADER)?;
        writer.write_u32::<LittleEndian>(0)?;
        Ok(())
    }
}

impl Kdbx4InnerEncryption {
    pub fn get_stream_cipher(&self) -> Box<dyn StreamCipherExt> {
        match self.inner_encryption_algorithm {
            InnerEncryptionAlgorithm::ChaCha20 => {
                let h = calculate_sha512(&self.inner_encryption_key);
                Box::new(ChaCha20Cipher::new(&h[0..32], &h[32..44]))
            }
            InnerEncryptionAlgorithm::Salsa20 => {
                let key = calculate_sha256(&self.inner_encryption_key);
                Box::new(Salsa20Cipher::new(&key, &SALSA20_IV))
            }
        }
    }

    // 生成一个新的内层加密
    pub fn new() -> Result<Self, std::io::Error> {
        let mut inner_encryption_key = vec![0; INNER_ENCRYPTION_ALGORITHM_KEY_SIZE_CHACHA20];
        getrandom::fill(&mut inner_encryption_key)?;
        Ok(Self {
            inner_encryption_algorithm: InnerEncryptionAlgorithm::ChaCha20,
            inner_encryption_key,
        })
    }
}
