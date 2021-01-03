use std::io::Error as StdError;

/// Custom `Result`
pub type Result<T> = std::result::Result<T, StdError>;
