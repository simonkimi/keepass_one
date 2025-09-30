use crate::kdbx::db::kdbx4::config::Kdbx4Config;
use crate::kdbx::db::kdbx4::errors::Kdbx4HeaderError;
use crate::kdbx::db::kdbx4::header_entity::compression::CompressionConfig;
use crate::kdbx::db::kdbx4::header_entity::encryption_algorithm::EncryptionAlgorithm;
use crate::kdbx::db::kdbx4::header_entity::kdf_config::KdfConfig;
use crate::kdbx::db::kdbx4::header_entity::variant_dictionary::VariantDictionary;
use crate::kdbx::db::version::{KDBX4_MAJOR_VERSION, KDBX_IDENTIFIER, KEEPASS_LATEST_ID};
use crate::utils::writer::{FixedSizeExt, WritableExt};
use byteorder::{ByteOrder, WriteBytesExt, LE};
use std::collections::HashMap;
use std::io::{Cursor, Write};

const HEADER_END: u8 = 0;
const HEADER_ENCRYPTION_ALGORITHM: u8 = 2;
const HEADER_COMPRESSION_ALGORITHM: u8 = 3;
const HEADER_MASTER_SEED: u8 = 4;
const HEADER_ENCRYPTION_IV: u8 = 7;
const HEADER_KDF_PARAMETERS: u8 = 11;
const HEADER_PUBLIC_CUSTOM_DATA: u8 = 12;

pub struct Kdbx4Header {
    pub config: Kdbx4Config,
    public_custom_data: Option<VariantDictionary>,
    unknown_header: HashMap<u8, Vec<u8>>,
}

impl Kdbx4Header {
    pub fn copy_from(&self, config: Kdbx4Config) -> Self {
        Self {
            config,
            public_custom_data: self.public_custom_data.clone(),
            unknown_header: self.unknown_header.clone(),
        }
    }

    pub fn try_from(value: &[u8]) -> Result<(Self, usize), Kdbx4HeaderError> {
        let mut encryption_algorithm: Option<EncryptionAlgorithm> = None;
        let mut compression_config: Option<CompressionConfig> = None;
        let mut master_salt_seed: Option<[u8; 32]> = None;
        let mut encryption_iv: Option<Vec<u8>> = None;
        let mut kdf_parameters: Option<KdfConfig> = None;
        let mut public_custom_data: Option<VariantDictionary> = None;
        let mut unknown_header: HashMap<u8, Vec<u8>> = HashMap::new();

        let mut pos: usize = 12;

        loop {
            let hf_type = value[pos];
            pos += 1;
            let hf_size = LE::read_u32(&value[pos..]) as usize;
            pos += 4;
            let hf_buffer = &value[pos..pos + hf_size];
            pos += hf_size;
            match hf_type {
                HEADER_END => {
                    break;
                }
                HEADER_ENCRYPTION_ALGORITHM => {
                    encryption_algorithm = Some(EncryptionAlgorithm::try_from(hf_buffer)?);
                }
                HEADER_COMPRESSION_ALGORITHM => {
                    compression_config = Some(CompressionConfig::try_from(hf_buffer)?)
                }
                HEADER_MASTER_SEED => {
                    if hf_buffer.len() != 32 {
                        return Err(Kdbx4HeaderError::InvalidMasterSeed);
                    }
                    master_salt_seed = Some(hf_buffer[..32].try_into().unwrap());
                }
                HEADER_ENCRYPTION_IV => {
                    encryption_iv = Some(hf_buffer.to_vec());
                }
                HEADER_KDF_PARAMETERS => {
                    kdf_parameters = Some(
                        KdfConfig::try_from(hf_buffer)
                            .map_err(Kdbx4HeaderError::InvalidKdfParameters)?,
                    );
                }

                HEADER_PUBLIC_CUSTOM_DATA => {
                    let vd = VariantDictionary::try_from(hf_buffer)
                        .map_err(Kdbx4HeaderError::InvalidVariantDictionary)?;
                    public_custom_data = Some(vd);
                }
                _ => {
                    unknown_header.insert(hf_type, hf_buffer.to_vec());
                }
            }
        }

        Ok((
            Kdbx4Header {
                config: Kdbx4Config {
                    encryption_algorithm: encryption_algorithm.ok_or(
                        Kdbx4HeaderError::MissingRequiredHeaderFields("encryption_algorithm"),
                    )?,
                    compression_config: compression_config.ok_or(
                        Kdbx4HeaderError::MissingRequiredHeaderFields("compression_config"),
                    )?,
                    master_salt_seed: master_salt_seed.ok_or(
                        Kdbx4HeaderError::MissingRequiredHeaderFields("master_salt_seed"),
                    )?,
                    encryption_iv: encryption_iv.ok_or(
                        Kdbx4HeaderError::MissingRequiredHeaderFields("encryption_iv"),
                    )?,
                    kdf_parameters: kdf_parameters.ok_or(
                        Kdbx4HeaderError::MissingRequiredHeaderFields("kdf_parameters"),
                    )?,
                },
                public_custom_data,
                unknown_header,
            },
            pos,
        ))
    }

    pub fn rekey(&self) -> Self {
        Self {
            config: self.config.rekey(),
            public_custom_data: self.public_custom_data.clone(),
            unknown_header: self.unknown_header.clone(),
        }
    }

    pub fn dump(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        // kdbx固定12字节头
        writer.write_all(&KDBX_IDENTIFIER)?;
        writer.write_u32::<LE>(KEEPASS_LATEST_ID)?;
        writer.write_u16::<LE>(1)?;
        writer.write_u16::<LE>(KDBX4_MAJOR_VERSION)?;

        // 写入其他头信息
        writer.write_u8(HEADER_ENCRYPTION_ALGORITHM)?;
        writer.write_fixed_size_data(&self.config.encryption_algorithm)?;

        writer.write_u8(HEADER_COMPRESSION_ALGORITHM)?;
        writer.write_fixed_size_data(&self.config.compression_config)?;

        writer.write_u8(HEADER_MASTER_SEED)?;
        writer.write_bytes_with_length(&self.config.master_salt_seed)?;

        writer.write_u8(HEADER_ENCRYPTION_IV)?;
        writer.write_bytes_with_length(&self.config.encryption_iv)?;

        writer.write_u8(HEADER_KDF_PARAMETERS)?;
        writer.write_with_calculated_length(&self.config.kdf_parameters)?;

        if let Some(public_custom_data) = &self.public_custom_data {
            writer.write_u8(HEADER_PUBLIC_CUSTOM_DATA)?;
            writer.write_with_calculated_length(public_custom_data)?;
        }

        for (key, value) in &self.unknown_header {
            writer.write_u8(*key)?;
            writer.write_bytes_with_length(value)?;
        }

        Ok(buffer)
    }
}
