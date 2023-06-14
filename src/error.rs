use core::fmt;

#[derive(Debug)]
pub enum CorporeumError {
    FailedToLoadFile(std::io::Error),
    FailedToParseIO(std::io::Error),
    FailedToParseRecursionLimitExceeded,
    FailedToParseSemantic(Option<usize>, String),
    FailedToParseSyntax(usize),
    FailedToSaveFileIO(std::io::Error),
    FailedToSaveFileValue(String),
    UnsupportedFileExtension,
}

impl fmt::Display for CorporeumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CorporeumError::FailedToLoadFile(err) => write!(f, "Failed to open file: {}", err),
            CorporeumError::FailedToParseIO(err) => {
                write!(
                    f,
                    "Failed to parse corporeum file while reading bytes: {}",
                    err
                )
            }
            CorporeumError::FailedToParseSyntax(offset) => {
                write!(
                    f,
                    "Failed to parse corporeum file while parsing bytes at offset: {}",
                    offset
                )
            }
            CorporeumError::FailedToParseSemantic(opt, s) => {
                write!(
                    f,
                    "Failed to parse corporeum file while processing parsed value: {}, {}",
                    opt.unwrap_or_default(),
                    s
                )
            }
            CorporeumError::FailedToParseRecursionLimitExceeded => write!(
                f,
                "Failed to parse corporeum file due to exceeding the recursion limit"
            ),
            CorporeumError::FailedToSaveFileIO(err) => write!(
                f,
                "Failed to save file. An error occured while writing bytes: {}",
                err
            ),
            CorporeumError::FailedToSaveFileValue(desc) => write!(
                f,
                "Failed to save file. Value cannot be serialzed: {}",
                desc
            ),
            CorporeumError::UnsupportedFileExtension => write!(f, "Unsupported file extension"),
        }
    }
}

impl std::error::Error for CorporeumError {}

impl From<ciborium::de::Error<std::io::Error>> for CorporeumError {
    fn from(err: ciborium::de::Error<std::io::Error>) -> Self {
        match err {
            ciborium::de::Error::Io(err) => CorporeumError::FailedToParseIO(err),
            ciborium::de::Error::Syntax(offset) => CorporeumError::FailedToParseSyntax(offset),
            ciborium::de::Error::Semantic(offset, desc) => {
                CorporeumError::FailedToParseSemantic(offset, desc)
            }
            ciborium::de::Error::RecursionLimitExceeded => {
                CorporeumError::FailedToParseRecursionLimitExceeded
            }
        }
    }
}

impl From<ciborium::ser::Error<std::io::Error>> for CorporeumError {
    fn from(err: ciborium::ser::Error<std::io::Error>) -> Self {
        match err {
            ciborium::ser::Error::Io(err) => CorporeumError::FailedToSaveFileIO(err),
            ciborium::ser::Error::Value(desc) => CorporeumError::FailedToSaveFileValue(desc),
        }
    }
}

impl From<std::io::Error> for CorporeumError {
    fn from(err: std::io::Error) -> Self {
        CorporeumError::FailedToLoadFile(err)
    }
}
