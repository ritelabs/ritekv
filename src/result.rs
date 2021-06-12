use thiserror::Error;

/// Custom `Error`
#[derive(Error, Debug)]
pub enum KvsError {
    #[error("Invalid Data -> Empty Key")]
    EmptyKey,
    #[error("Invalid Data: {0}")]
    InvalidData(String),
    #[error("Internal Error -> IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Internal Error: {0}")]
    Internal(String),
    #[error("serde_json error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Sled Error: {0}")]
    Sled(#[from] sled::Error),
}

/// Custom `Result`
pub type Result<T> = std::result::Result<T, KvsError>;
