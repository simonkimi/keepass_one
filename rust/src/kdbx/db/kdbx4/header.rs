use crate::kdbx::db::kdbx4::errors::Kdbx4HeaderError;
use crate::kdbx::db::kdbx4::header_entity::compression::CompressionConfig;
use crate::kdbx::db::kdbx4::header_entity::encryption_algorithm::EncryptionAlgorithm;
use crate::kdbx::db::kdbx4::header_entity::kdf_config::KdfConfig;
use crate::kdbx::db::kdbx4::header_entity::variant_dictionary::VariantDictionary;
use crate::kdbx::db::version::{KDBX4_MAJOR_VERSION, KDBX_IDENTIFIER, KEEPASS_LATEST_ID};
use crate::utils::cursor_utils::CursorExt;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt, LE};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom, Write};

const HEADER_END: u8 = 0;
const HEADER_ENCRYPTION_ALGORITHM: u8 = 2;
const HEADER_COMPRESSION_ALGORITHM: u8 = 3;
const HEADER_MASTER_SEED: u8 = 4;
const HEADER_ENCRYPTION_IV: u8 = 7;
const HEADER_KDF_PARAMETERS: u8 = 11;
const HEADER_PUBLIC_CUSTOM_DATA: u8 = 12;

pub struct Kdbx4Header {
    pub encryption_algorithm: EncryptionAlgorithm,
    pub compression_config: CompressionConfig,
    pub master_salt_seed: [u8; 32],
    pub encryption_iv: Vec<u8>,
    pub kdf_parameters: KdfConfig,
    public_custom_data: Option<VariantDictionary>,
    unknown_header: HashMap<u8, Vec<u8>>,
    pub size: usize,
}

impl TryFrom<&[u8]> for Kdbx4Header {
    type Error = Kdbx4HeaderError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut encryption_algorithm: Option<EncryptionAlgorithm> = None;
        let mut compression_config: Option<CompressionConfig> = None;
        let mut master_salt_seed: Option<[u8; 32]> = None;
        let mut encryption_iv: Option<Vec<u8>> = None;
        let mut kdf_parameters: Option<KdfConfig> = None;
        let mut public_custom_data: Option<VariantDictionary> = None;
        let mut unknown_header: HashMap<u8, Vec<u8>> = HashMap::new();

        let reader = &mut Cursor::new(value);
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
                    compression_config = Some(CompressionConfig::try_from(hf_buffer)?)
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
                    kdf_parameters = Some(
                        KdfConfig::try_from(hf_buffer)
                            .map_err(|_| Kdbx4HeaderError::InvalidHeader)?,
                    );
                }

                HEADER_PUBLIC_CUSTOM_DATA => {
                    let vd = VariantDictionary::try_from(hf_buffer)
                        .map_err(|_| Kdbx4HeaderError::InvalidVariantDictionary)?;
                    public_custom_data = Some(vd);
                }
                _ => {
                    unknown_header.insert(hf_type, hf_buffer.to_vec());
                }
            }
        }

        Ok(Kdbx4Header {
            encryption_algorithm: encryption_algorithm.ok_or(Kdbx4HeaderError::InvalidHeader)?,
            compression_config: compression_config.ok_or(Kdbx4HeaderError::InvalidHeader)?,
            master_salt_seed: master_salt_seed.ok_or(Kdbx4HeaderError::InvalidHeader)?,
            encryption_iv: encryption_iv.ok_or(Kdbx4HeaderError::InvalidHeader)?,
            kdf_parameters: kdf_parameters.ok_or(Kdbx4HeaderError::InvalidHeader)?,
            public_custom_data,
            unknown_header,
            size: reader.position() as usize,
        })
    }
}

impl Kdbx4Header {
    pub fn write(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);

        // kdbx固定12字节头
        writer.write_all(&KDBX_IDENTIFIER)?;
        writer.write_u32::<LE>(KEEPASS_LATEST_ID)?;
        writer.write_u16::<LE>(1)?;
        writer.write_u16::<LE>(KDBX4_MAJOR_VERSION)?;

        // 其他部分
        write_header_field(
            &mut writer,
            HEADER_ENCRYPTION_ALGORITHM,
            &self.encryption_algorithm.write(),
        )?;
        write_header_field(
            &mut writer,
            HEADER_COMPRESSION_ALGORITHM,
            &self.compression_config.write(),
        )?;
        write_header_field(&mut writer, HEADER_MASTER_SEED, &self.master_salt_seed)?;
        write_header_field(&mut writer, HEADER_ENCRYPTION_IV, &self.encryption_iv)?;
        write_header_field(
            &mut writer,
            HEADER_KDF_PARAMETERS,
            &self.kdf_parameters.write(),
        )?;
        if let Some(vd) = &self.public_custom_data {
            write_header_field(&mut writer, HEADER_PUBLIC_CUSTOM_DATA, &vd.write())?;
        }
        for (hf_type, hf_buffer) in &self.unknown_header {
            write_header_field(&mut writer, *hf_type, hf_buffer)?;
        }

        write_header_field(&mut writer, HEADER_END, &[])?;

        Ok(buffer)
    }
}

fn write_header_field<T: Write>(
    writer: &mut T,
    hf_type: u8,
    hf_buffer: &[u8],
) -> anyhow::Result<()> {
    writer.write_u8(hf_type)?;
    writer.write_u32::<LE>(hf_buffer.len() as u32)?;
    writer.write_all(hf_buffer)?;
    Ok(())
}
