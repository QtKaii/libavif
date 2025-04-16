//! Error types for the Rust implementation of libavif.

use thiserror::Error;

/// Result type for libavif operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for libavif operations.
#[derive(Error, Debug)]
pub enum Error {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// BMFF parsing error.
    #[error("BMFF parsing error: {0}")]
    BmffParse(String),

    /// Decoder error.
    #[error("Decoder error: {0}")]
    Decode(String),

    /// Memory allocation error.
    #[error("Memory allocation error")]
    OutOfMemory,

    /// Invalid parameter.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Unsupported feature.
    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    /// Unknown error.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// FFI-compatible result type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvifResult {
    /// Operation completed successfully.
    Ok = 0,
    /// Unknown error.
    Unknown = 1,
    /// Invalid parameter.
    InvalidParameter = 2,
    /// Out of memory.
    OutOfMemory = 3,
    /// BMFF parsing failed.
    BmffParseFailed = 4,
    /// Decoder failed.
    DecodeFailed = 5,
    /// Encoder failed.
    EncodeFailed = 6,
    /// I/O error.
    IoError = 7,
    /// Unsupported feature.
    Unsupported = 8,
}

impl From<Error> for AvifResult {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(_) => AvifResult::IoError,
            Error::BmffParse(_) => AvifResult::BmffParseFailed,
            Error::Decode(_) => AvifResult::DecodeFailed,
            Error::OutOfMemory => AvifResult::OutOfMemory,
            Error::InvalidParameter(_) => AvifResult::InvalidParameter,
            Error::Unsupported(_) => AvifResult::Unsupported,
            Error::Unknown(_) => AvifResult::Unknown,
        }
    }
}
