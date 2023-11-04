use core::fmt;
use std::{io, string::FromUtf8Error};

macro_rules! impl_error_conversion {
    ($feature: literal, $external_type: ty => $variant: ident) => {
        #[cfg(feature = $feature)]
        impl From<$external_type> for CorporeumError {
            fn from(value: $external_type) -> Self {
                Self::$variant(value.to_string())
            }
        }
    };
}

/// Represents all possible errors.
#[derive(Debug)]
pub enum CorporeumError {
    /// Generic I/O operation failure.
    IOFailed(std::io::Error),
    /// Decompression failed.
    DecompressionError(io::Error),
    /// Compression failed.
    CompressionError(io::Error),
    /// Serialization error
    SerializeError(String),
    /// Deserialization error
    DeserializeError(String),
    /// Specified object is empty.
    EmptyObject(String),
    /// An element was not found
    ElementNotFound(String),
}

impl fmt::Display for CorporeumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOFailed(why) => write!(f, "I/O operation failed: {why}"),
            Self::CompressionError(why) => write!(f, "Compression error: {why}"),
            Self::DecompressionError(why) => write!(f, "Decompression error: {why}"),
            Self::SerializeError(why) => write!(f, "Failed to serialize: {why}"),
            Self::DeserializeError(why) => write!(f, "Failed to deserialize: {why}"),
            Self::EmptyObject(desc) => write!(f, "Empty object: {desc}"),
            Self::ElementNotFound(desc) => write!(f, "{desc}"),
        }
    }
}

impl_error_conversion!(
    "cbor-format",
    ciborium::de::Error<std::io::Error> => DeserializeError
);
impl_error_conversion!(
    "cbor-format",
    ciborium::ser::Error<std::io::Error> => SerializeError
);

impl_error_conversion!("rmp-format", rmp_serde::encode::Error => SerializeError);
impl_error_conversion!("rmp-format", rmp_serde::decode::Error => DeserializeError);

/*
serde_json, serde_xml_rs and bincode don't have separate error types for de/serialization
*/

impl From<FromUtf8Error> for CorporeumError {
    fn from(value: FromUtf8Error) -> Self {
        Self::IOFailed(io::Error::new(
            io::ErrorKind::Other,
            format!("Unable to parse bytes as UTF-8 string: {value}"),
        ))
    }
}

impl From<std::io::Error> for CorporeumError {
    fn from(err: std::io::Error) -> Self {
        Self::IOFailed(err)
    }
}

impl std::error::Error for CorporeumError {}
