use byteorder::ByteOrder;
use byteorder::LittleEndian;
use thiserror::Error;

const KDBX_IDENTIFIER: [u8; 4] = [0x03, 0xd9, 0xa2, 0x9a];
const KDBX_HEADER_SIZE: usize = 12;

const KEEPASS_1_ID: u32 = 0xb54bfb65;
const KEEPASS_2_ID: u32 = 0xb54bfb66;
const KEEPASS_LATEST_ID: u32 = 0xb54bfb67;

const KDBX3_MAJOR_VERSION: u16 = 3;
const KDBX4_MAJOR_VERSION: u16 = 4;

pub enum KdbxVersion {
    KDB(u16),
    KDB2(u16),
    KDB3(u16),
    KDB4(u16),
}

#[derive(Debug, Error)]
pub enum KdbxHeaderError {
    #[error("Invalid KDBX magic number")]
    InvalidMagicNumber,
    #[error(
        "Invalid KDBX version: {}.{}.{}",
        version,
        file_major_version,
        file_minor_version
    )]
    InvalidKDBXVersion {
        version: u32,
        file_major_version: u16,
        file_minor_version: u16,
    },
}

impl KdbxVersion {
    pub fn parse(data: &[u8]) -> Result<Self, KdbxHeaderError> {
        if data.len() < KDBX_HEADER_SIZE {
            return Err(KdbxHeaderError::InvalidMagicNumber);
        }

        if data[..4] != KDBX_IDENTIFIER {
            return Err(KdbxHeaderError::InvalidMagicNumber);
        }

        let version = LittleEndian::read_u32(&data[4..8]);
        let file_minor_version = LittleEndian::read_u16(&data[8..10]);
        let file_major_version = LittleEndian::read_u16(&data[10..12]);

        let kdbx_version = match version {
            KEEPASS_1_ID => KdbxVersion::KDB(file_major_version),
            KEEPASS_2_ID => KdbxVersion::KDB2(file_major_version),
            KEEPASS_LATEST_ID if file_major_version == KDBX3_MAJOR_VERSION => {
                KdbxVersion::KDB3(file_major_version)
            }
            KEEPASS_LATEST_ID if file_major_version == KDBX4_MAJOR_VERSION => {
                KdbxVersion::KDB4(file_major_version)
            }
            _ => {
                return Err(KdbxHeaderError::InvalidKDBXVersion {
                    version,
                    file_major_version,
                    file_minor_version,
                })
            }
        };

        Ok(kdbx_version)
    }
}
