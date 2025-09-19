use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdbxOpenError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

