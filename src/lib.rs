#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

use std::{ffi::OsStr, fs, io::Read, path::Path};

use ciborium::from_reader;
pub use corporeum::Corporeum;
pub use error::CorporeumError;
use flate2::bufread::ZlibDecoder;
pub use schema::{Author, Corpus, Document, Metadata, Sentence, Source};

mod author;
mod corporeum;
mod corpus;
mod document;
mod error;
mod metadata;
mod schema;
mod sentence;
mod token;

/// Unified Corpora Format
const EXTENSION: &str = "ucf";

/// return corporeum with an empty corpora from a given path
#[inline]
pub fn new<P: AsRef<Path>>(buffer: P) -> Result<Corporeum, CorporeumError> {
    let corpus = Corpus::default();

    let md = std::fs::metadata(buffer.as_ref())?;
    if md.is_dir() {
        return Err(CorporeumError::IncorrectPath(
            buffer.as_ref().to_str().unwrap_or_default().to_string(),
        ));
    }

    Ok(Corporeum {
        original_file_path: buffer.as_ref().to_path_buf(),
        corpus,
    })
}

/// function to load an already existing corpus
#[inline]
pub fn load<P: AsRef<Path>>(source: P) -> Result<Corporeum, CorporeumError> {
    let input_data = fs::read(&source)?;
    let mut decompresed = Vec::new();
    let mut decompressor = ZlibDecoder::new(&input_data[..]);

    decompressor
        .read_to_end(&mut decompresed)
        .map_err(CorporeumError::DecompressionError)?;

    let corpus: Corpus = match source.as_ref().extension().and_then(OsStr::to_str).unwrap() {
        EXTENSION => from_reader(decompresed.as_slice())?,
        _ => return Err(CorporeumError::UnsupportedFileExtension),
    };
    Ok(Corporeum {
        original_file_path: source.as_ref().to_path_buf(),
        corpus,
    })
}
