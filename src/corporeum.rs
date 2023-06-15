use crate::error::CorporeumError;
use crate::schema::Corpus;
use crate::EXTENSION;

use ciborium::into_writer;

use flate2::write::ZlibEncoder;
use flate2::Compression;

use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, path::Path};

// const EXTENSION: &str = "ucf"; // Unified Corpora Format

/// Holds a [`Corpus`](crate::Corpus) created from a file.
pub struct Corporeum {
    pub(crate) original_file_path: PathBuf,
    pub(crate) corpus: Corpus,
}

impl Corporeum {
    #[inline]
    pub fn save(&self) -> Result<(), CorporeumError> {
        let dest = Path::with_extension(self.original_file_path.as_path(), EXTENSION);
        let dest = dest.as_path();
        let mut data = Vec::new();

        into_writer(&self.corpus, Cursor::new(&mut data))?;

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest)?;
        let mut compressor = ZlibEncoder::new(file, Compression::best());

        compressor
            .write_all(&data)
            .map_err(CorporeumError::CompressionError)?;

        Ok(())
    }

    #[inline]
    pub fn save_as<P: AsRef<Path>>(&self, destination: P) -> Result<(), CorporeumError> {
        let dest = Path::with_extension(destination.as_ref(), EXTENSION);
        let dest = dest.as_path();

        let mut data = Vec::new();

        into_writer(&self.corpus, Cursor::new(&mut data))?;

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest)?;
        let mut compressor = ZlibEncoder::new(file, Compression::best());

        compressor
            .write_all(&data)
            .map_err(CorporeumError::CompressionError)?;

        Ok(())
    }

    pub const fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}
