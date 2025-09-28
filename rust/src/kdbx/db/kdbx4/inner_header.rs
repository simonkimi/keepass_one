use crate::kdbx::db::kdbx4::header_entity::binary_content::BinaryContent;
use crate::kdbx::db::kdbx4::header_entity::inner_encryption_algorithm::InnerEncryptionAlgorithm;
use crate::utils::writer::{FixedSizeExt, Writable, WritableExt};
use byteorder::LittleEndian;
use byteorder::{ByteOrder, WriteBytesExt};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const INNER_HEADER_END_OF_HEADER: u8 = 0x00;
pub const INNER_HEADER_INNER_ENCRYPTION_ALGORITHM: u8 = 0x01;
pub const INNER_HEADER_INNER_ENCRYPTION_KEY: u8 = 0x02;
pub const INNER_HEADER_BINARY_CONTENT: u8 = 0x03;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Kdbx4InnerHeader {
    pub inner_encryption_algorithm: InnerEncryptionAlgorithm,
    pub inner_encryption_key: Vec<u8>,
    pub binary_content: Vec<BinaryContent>,
    pub header_size: usize,
}

impl TryFrom<&[u8]> for Kdbx4InnerHeader {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
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
                    return Err(anyhow::anyhow!(
                        "Unknown inner header type: {}",
                        header_type
                    ));
                }
            }
        }

        if let None = inner_encryption_algorithm {
            return Err(anyhow::anyhow!("Inner encryption algorithm not found"));
        }

        if let None = inner_encryption_key {
            return Err(anyhow::anyhow!("Inner encryption key not found"));
        }

        Ok(Kdbx4InnerHeader {
            inner_encryption_algorithm: inner_encryption_algorithm.unwrap(),
            inner_encryption_key: inner_encryption_key.unwrap(),
            binary_content: binary_content_vec,
            header_size: pos,
        })
    }
}

impl Writable for Kdbx4InnerHeader {
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        writer.write_u8(INNER_HEADER_INNER_ENCRYPTION_ALGORITHM)?;
        writer.write_fixed_size_data(&self.inner_encryption_algorithm)?;

        writer.write_u8(INNER_HEADER_INNER_ENCRYPTION_KEY)?;
        writer.write_bytes_with_length(&self.inner_encryption_key)?;
        for binary_content in &self.binary_content {
            writer.write_u8(INNER_HEADER_BINARY_CONTENT)?;
            writer.write_fixed_size_data(binary_content)?;
        }

        writer.write_u8(INNER_HEADER_END_OF_HEADER)?;
        Ok(())
    }
}
