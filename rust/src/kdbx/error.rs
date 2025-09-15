use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdbxOpenError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum KdbxFileError {
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
