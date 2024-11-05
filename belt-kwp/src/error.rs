use core::fmt;

/// Result type with the `belt-kwp` crate's [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// Errors emitted from the wrap and unwrap operations.
#[derive(Debug)]
pub enum Error {
    /// Input data length invalid.
    InvalidDataSize,

    /// Invalid KEK size.
    InvalidKekSize {
        /// KEK size provided in bytes (expected 32).
        size: usize,
    },

    /// Output buffer size invalid.
    InvalidOutputSize {
        /// Expected size in bytes.
        expected: usize,
    },

    /// Integrity check did not pass.
    IntegrityCheckFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidDataSize => write!(
                f,
                "data must be a multiple of 64 bits and more than 16 bytes"
            ),
            Error::InvalidOutputSize { expected } => {
                write!(f, "invalid output buffer size: expected {}", expected)
            }
            Error::IntegrityCheckFailed => {
                write!(f, "integrity check failed")
            }
            Error::InvalidKekSize { size } => {
                write!(f, "invalid KEK size: {}", size)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}