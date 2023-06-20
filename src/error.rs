use core::fmt;

/// Represents all possible errors.
#[derive(Debug)]
pub enum CorporeumError {
    /// I/O operation failed.
    IOFailed(std::io::Error),
    /// Failed to deserialize an object.
    FailedToParseIO(std::io::Error),
    /// The input caused serde to recurse too much.
    FailedToParseRecursionLimitExceeded,
    /// An error occurred while processing a parsed value.
    FailedToParseSemantic(Option<usize>, String),
    /// An error occurred while parsing bytes.
    /// Contains the offset into the stream where the syntax error occurred.
    FailedToParseSyntax(usize),
    /// Failed to write serialized data to a file.
    FailedToSaveFileIO(std::io::Error),
    /// Failed to serialize a value.
    FailedToSaveFileValue(String),
    /// The file extension is unsupported or is missing.
    UnsupportedFileExtension,
    /// Zlib decompression failed.
    DecompressionError(std::io::Error),
    /// Zlib compression failed.
    CompressionError(std::io::Error),
    /// The specified file does not exist or the given path was not a file.
    IncorrectPath(String),
}

impl fmt::Display for CorporeumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOFailed(err) => write!(f, "Failed to open file: {err}"),
            Self::FailedToParseIO(err) => {
                write!(
                    f,
                    "Failed to parse corporeum file while reading bytes: {err}"
                )
            }
            Self::FailedToParseSyntax(offset) => {
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
            Self::FailedToSaveFileIO(err) => write!(
                f,
                "Failed to save file. An error occurred while writing bytes: {err}"
            ),
            Self::FailedToSaveFileValue(desc) => {
                write!(f, "Failed to save file. Value cannot be serialized: {desc}")
            }
            Self::UnsupportedFileExtension => write!(f, "Unsupported file extension"),
            Self::DecompressionError(err) => write!(f, "Decompression failed: {err}"),
            Self::CompressionError(err) => write!(f, "Compression failed: {err}"),
            Self::IncorrectPath(path) => write!(
                f,
                "Cannot create file {path}. Input should be a file, not a directory"
            ),
        }
    }
}

impl std::error::Error for CorporeumError {}

impl From<ciborium::de::Error<std::io::Error>> for CorporeumError {
    fn from(err: ciborium::de::Error<std::io::Error>) -> Self {
        match err {
            ciborium::de::Error::Io(err) => Self::FailedToParseIO(err),
            ciborium::de::Error::Syntax(offset) => Self::FailedToParseSyntax(offset),
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
            ciborium::ser::Error::Io(err) => Self::FailedToSaveFileIO(err),
            ciborium::ser::Error::Value(desc) => Self::FailedToSaveFileValue(desc),
        }
    }
}

impl From<std::io::Error> for CorporeumError {
    fn from(err: std::io::Error) -> Self {
        Self::IOFailed(err)
    }
}
