use core::fmt;

/// Represents all possible errors.
#[derive(Debug)]
pub enum CorporeumError {
    /// Generic I/O operation failure.
    IOFailed(std::io::Error),
    /// Zlib decompression failed.
    DecompressionError(std::io::Error),
    /// Zlib compression failed.
    CompressionError(std::io::Error),
    /// Specified object is empty.
    EmptyObject(String),
    /// An element was not found
    ElementNotFound(String),
    /// Failed to deserialize an object.
    FailedToParseIO(std::io::Error),
    /// The input caused serde to recurse too much.
    FailedToParseRecursionLimitExceeded,
    /// An error occurred while processing a parsed value.
    FailedToParseSemantic(Option<usize>, String),
    /// An error occurred while parsing bytes.
    /// Contains the offset into the stream where the syntax error occurred.
    SyntaxError(usize),
    /// Failed to write serialized data to a stream.
    FailedSerializedWrite(std::io::Error),
    /// Failed to serialize a value.
    BadValue(String),
}

impl fmt::Display for CorporeumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOFailed(err) => write!(f, "{err}"),
            Self::FailedToParseIO(err) => {
                write!(
                    f,
                    "Failed to parse corporeum file while reading bytes: {err}"
                )
            }
            Self::SyntaxError(offset) => {
                write!(
                    f,
                    "Failed to parse corporeum file while parsing bytes at offset: {offset}"
                )
            }
            Self::FailedToParseSemantic(opt, s) => {
                write!(
                    f,
                    "Failed to parse corporeum file while processing parsed value: {}, {s}",
                    opt.unwrap_or_default(),
                )
            }
            Self::FailedToParseRecursionLimitExceeded => write!(
                f,
                "Failed to parse corporeum file due to exceeding the recursion limit"
            ),
            Self::FailedSerializedWrite(err) => {
                write!(f, "An error occurred while writing bytes: {err}")
            }
            Self::BadValue(desc) => {
                write!(f, "Value cannot be serialized: {desc}")
            }
            Self::DecompressionError(err) => write!(f, "Decompression failed: {err}"),
            Self::CompressionError(err) => write!(f, "Compression failed: {err}"),
            Self::EmptyObject(desc) => write!(f, "Empty object: {desc}"),
            Self::ElementNotFound(desc) => write!(f, "{desc}"),
        }
    }
}

impl From<ciborium::de::Error<std::io::Error>> for CorporeumError {
    fn from(err: ciborium::de::Error<std::io::Error>) -> Self {
        match err {
            ciborium::de::Error::Io(err) => Self::FailedToParseIO(err),
            ciborium::de::Error::Syntax(offset) => Self::SyntaxError(offset),
            ciborium::de::Error::Semantic(offset, desc) => {
                Self::FailedToParseSemantic(offset, desc)
            }
            ciborium::de::Error::RecursionLimitExceeded => {
                Self::FailedToParseRecursionLimitExceeded
            }
        }
    }
}

impl From<ciborium::ser::Error<std::io::Error>> for CorporeumError {
    fn from(err: ciborium::ser::Error<std::io::Error>) -> Self {
        match err {
            ciborium::ser::Error::Io(err) => Self::FailedSerializedWrite(err),
            ciborium::ser::Error::Value(desc) => Self::BadValue(desc),
        }
    }
}

impl From<std::io::Error> for CorporeumError {
    fn from(err: std::io::Error) -> Self {
        Self::IOFailed(err)
    }
}

impl std::error::Error for CorporeumError {}
