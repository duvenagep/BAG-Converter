#[derive(Debug, thiserror::Error)]
pub enum BagError {
    /// IO Error fo reading & writing files
    #[error("IO Error {0}")]
    IoError(std::io::Error),
    /// UTF-8 Error converting from raw bytes to UTF-8 Chars
    #[error("UTF-8 Error {0}")]
    Utf8Error(std::str::Utf8Error),
}

/// Result to be used in LVBAG CLI
pub type BagResult<T> = Result<T, BagError>;

impl From<std::io::Error> for BagError {
    fn from(err: std::io::Error) -> Self {
        BagError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for BagError {
    fn from(err: std::str::Utf8Error) -> Self {
        BagError::Utf8Error(err)
    }
}
