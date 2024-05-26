#[derive(Debug, thiserror::Error)]
pub enum BagError {
    /// IO Error fo reading & writing files
    #[error("IO Error {0}")]
    IoError(std::io::Error),
    /// UTF-8 Error converting from raw bytes to UTF-8 Chars
    #[error("UTF-8 Error {0}")]
    Utf8Error(std::str::Utf8Error),
    /// Zip Error reading/writing ZipArchive
    #[error("Zip Error {0}")]
    ZipError(zip::result::ZipError),
    /// Decompression Error
    #[error("Decompression Error {0}")]
    DecompressionError(libdeflater::DecompressionError),
    /// QuickXml Deserialisation Error
    #[error("Deserialization Error {0}")]
    DeserializationError(quick_xml::de::DeError),
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

impl From<zip::result::ZipError> for BagError {
    fn from(err: zip::result::ZipError) -> Self {
        BagError::ZipError(err)
    }
}

impl From<libdeflater::DecompressionError> for BagError {
    fn from(err: libdeflater::DecompressionError) -> Self {
        BagError::DecompressionError(err)
    }
}

impl From<quick_xml::de::DeError> for BagError {
    fn from(err: quick_xml::de::DeError) -> Self {
        BagError::DeserializationError(err)
    }
}
