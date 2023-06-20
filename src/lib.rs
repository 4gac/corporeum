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

/// Creates a new `Corporeum` with an empty corpora from a given path. The path is used when ['save()'](Corporeum::save) function is called.
///
/// # Warnings
/// - The function only creates an in-memory representation.
///  No new file will be created, until ['save()'](Corporeum::save) or ['save_as()'](Corporeum::save_as) is called!
///
/// # Example
/// ```
/// # use corporum::new;
/// let corp = new("some_file.ucf");
/// ```
///
/// To load a `Corpus` from a `.ucf` file, you may want to
/// use [`load()`](load) instead.
#[inline]
pub fn new<P: AsRef<Path>>(buffer: P) -> Result<Corporeum, CorporeumError> {
    let corpus = Corpus::default();

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
#[inline]
pub fn load<P: AsRef<Path>>(source: P) -> Result<Corporeum, CorporeumError> {
    let input_data = fs::read(&source)?;
    let mut decompresed = Vec::new();
    let mut decompressor = ZlibDecoder::new(&input_data[..]);
    let extension = source
        .as_ref()
        .extension()
        .and_then(OsStr::to_str)
        .ok_or(CorporeumError::UnsupportedFileExtension)?;

    decompressor
        .read_to_end(&mut decompresed)
        .map_err(CorporeumError::DecompressionError)?;

    let corpus: Corpus = match extension {
        EXTENSION => from_reader(decompresed.as_slice())?,
        _ => return Err(CorporeumError::UnsupportedFileExtension),
    };
    Ok(Corporeum {
        original_file_path: source.as_ref().to_path_buf(),
        corpus,
    })
}
