use thiserror::Error;

/// Represents all possible errors.
#[derive(Debug, Error)]
pub enum CorporeumError {
    /// Generic I/O operation failure.
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    /// Zlib decompression failed.
    #[error("Zlib: Decompression failed: {0}")]
    DecompressionError(std::io::Error),

    /// Zlib compression failed.
    #[error("Zlib: Compression failed: {0}")]
    CompressionError(std::io::Error),

    /// De/Serialization error.
    #[error("Serde: {0}")]
    Serde(#[from] serde_json::Error),

    /// Specified object is empty.
    #[error("Empty Object: {0}")]
    EmptyObject(String),

    /// An element was not found.
    #[error("Element not found: {0}")]
    ElementNotFound(String),

    /// The input caused serde to recurse too much.
    #[error("Recursion limit exceeded")]
    FailedToParseRecursionLimitExceeded,

    /// An error occurred while processing a parsed value.
    #[error("Semantic: {0:?}, {1}")]
    FailedToParseSemantic(Option<usize>, String),

    /// An error occurred while parsing bytes.
    /// Contains the offset into the stream where the syntax error occurred.
    #[error("Syntax Error at offset {0}")]
    SyntaxError(usize),

    /// Failed to serialize a value.
    #[error("Bad Value: {0}")]
    BadValue(String),
}
