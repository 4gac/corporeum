use core::fmt;

#[derive(Debug)]
pub enum CorporeumError {
    IOFailed(std::io::Error),
    FailedToParseIO(std::io::Error),
    FailedToParseRecursionLimitExceeded,
    FailedToParseSemantic(Option<usize>, String),
    FailedToParseSyntax(usize),
    FailedToSaveFileIO(std::io::Error),
    FailedToSaveFileValue(String),
    UnsupportedFileExtension,
    DecompressionError(std::io::Error),
    CompressionError(std::io::Error),
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
                "Failed to save file. An error occured while writing bytes: {err}"
            ),
            Self::FailedToSaveFileValue(desc) => {
                write!(f, "Failed to save file. Value cannot be serialzed: {desc}")
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
