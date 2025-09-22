use byteorder::ByteOrder;
use byteorder::LittleEndian;

pub const INNER_HEADER_END_OF_HEADER: u8 = 0x00;
pub const INNER_HEADER_INNER_ENCRYPTION_ALGORITHM: u8 = 0x01;
pub const INNER_HEADER_INNER_ENCRYPTION_KEY: u8 = 0x02;
pub const INNER_HEADER_BINARY_CONTENT: u8 = 0x03;

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

pub struct Kdbx4InnerHeader {
    pub inner_encryption_algorithm: InnerEncryptionAlgorithm,
    pub inner_encryption_key: Vec<u8>,
    pub binary_content: Vec<Kdbx4BinaryContent>,
    pub header_size: usize,
}

pub struct Kdbx4BinaryContent {
    pub flag: u8,
    pub content: Vec<u8>,
}

impl From<&[u8]> for Kdbx4BinaryContent {
    fn from(data: &[u8]) -> Self {
        let flag = data[0];
        let content = data[1..].to_vec();
        Self { flag, content }
    }
}

impl Kdbx4InnerHeader {
    pub fn parse(data: &[u8]) -> anyhow::Result<Self> {
        let mut pos = 0;

        let mut inner_encryption_algorithm: Option<InnerEncryptionAlgorithm> = None;
        let mut inner_encryption_key: Option<Vec<u8>> = None;
        let mut binary_content_vec: Vec<Kdbx4BinaryContent> = Vec::new();

        loop {
            let header_type = data[pos];
            pos += 1;
            let header_size = LittleEndian::read_u32(&data[pos..pos + 4]);
            pos += 4;
            let header_data = &data[pos..pos + header_size as usize];
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
                    let binary_content = Kdbx4BinaryContent::from(header_data);
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
