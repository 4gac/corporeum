use crate::error::CorporeumError;
use crate::schema::Corpus;

use ciborium::from_reader;
use ciborium::into_writer;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, path::Path};

const EXTENSION: &str = "ucf"; // Unified Corpora Format

pub struct Corporeum {
    original_file_path: PathBuf,
    corpus: Corpus,
}

impl Corporeum {
    // return corporeum with an empty corpora from a given path
    pub fn new<P: AsRef<Path>>(buffer: P) -> Corporeum {
        let corpus = Corpus::default();

        Corporeum {
            original_file_path: buffer.as_ref().to_path_buf(),
            corpus,
        }
    }

    // function to load an already existing corpus
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
