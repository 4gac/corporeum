#![allow(clippy::must_use_candidate, clippy::module_name_repetitions)]
//
//! A library for working with text corpora.
//
use ciborium::from_reader;
pub use corporeum::Corporeum;
pub use error::CorporeumError;
use flate2::bufread::ZlibDecoder;
pub use schema::{Author, Corpus, Document, Metadata, Sentence, Token};
use std::{ffi::OsStr, fs, io::Read, path::Path};

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

/// Creates a new `Corporeum` with an empty corpora from a given path.
///
/// # Example
/// ```
/// # use corporum::new;
/// let corp = new("some_file.ucf");
/// ```
///
/// # Errors
/// If the given file does not exist or is inaccessible, an error is returned.
///
/// To load a `Corpus` from a `.ucf` file, you may want to
/// use [`load()`](load) instead.
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

/// Load an already existing corpus from a `.ucf` file.
///
/// # Example
/// ```no_run
/// # use corporum::load;
/// let corp = match load("some_file.ucf") {
///     Ok(corp) => corp,
///     Err(e) => panic!("Error loading corpus: {e}"),
/// };
///
/// // ...
/// ```
///
/// # Errors
/// This will return an error if:
/// - The given file does not exist or is inaccessible.
/// - The contents could not be decompressed.
/// - The file extension is incorrect (only `.ucf` is supported)
/// - The contents could not be deserialized.
///
/// # Panics
/// This will panic if the file extension could not be determined.
///
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
