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
}

/// Custom `Result`
pub type Result<T> = std::result::Result<T, KvsError>;
