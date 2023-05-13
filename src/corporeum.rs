use crate::schema::Corpus;
use flate2::{read::GzDecoder, write::GzEncoder, Compression as FlateCompression};
use lzma_rs::{xz_compress, xz_decompress};
use std::{
    fs,
    io::{BufReader, Cursor, Read, Result as IoResult, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Compression {
    None,
    Lzma,
    Deflate,
}

pub struct Corporeum<'a> {
    original_file_path: &'a Path,
    corpus: Corpus,
}

impl Corporeum<'_> {
    pub fn new(buffer: &Path) -> Corporeum {
        let corpus = Corpus::default();

        Corporeum {
            original_file_path: buffer,
            corpus,
        }
    }

    // function to load an already existing corpus
    pub fn load<P: AsRef<Path>>(source: &P) -> IoResult<Corporeum> {
        let path = source.as_ref();
        let file = fs::OpenOptions::new().read(true).open(path)?;
        let compression = path.try_into().expect("Unknown format");
        let mut data = Vec::new();

        match compression {
            Compression::None => (),
            Compression::Deflate => {
                let mut decompressor = GzDecoder::new(file);
                decompressor.read_to_end(&mut data)?;
            }
            Compression::Lzma => {
                let mut reader = BufReader::new(file);
                xz_decompress(&mut reader, &mut data).unwrap();
            }
        }

        let corpus: Corpus = serde_cbor::from_slice(&data).unwrap();
        Ok(Corporeum {
            original_file_path: source.as_ref(),
            corpus,
        })
    }

    pub fn save(&self, packed: bool) -> IoResult<()> {
        self.save_as(
            &self.original_file_path,
            packed,
            self.original_file_path.try_into().unwrap(),
        )
    }

    pub fn save_as<P: AsRef<Path>>(
        &self,
        destination: &P,
        packed: bool,
        compression: Compression,
    ) -> IoResult<()> {
        let buffer = if packed {
            serde_cbor::ser::to_vec_packed(&self.corpus).unwrap()
        } else {
            serde_cbor::ser::to_vec(&self.corpus).unwrap()
        };

        let destination = Self::generate_save_path(destination.as_ref(), compression);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(destination)?;

        match compression {
            Compression::None => file.write_all(&buffer),
            Compression::Deflate => {
                let mut compressor = GzEncoder::new(file, FlateCompression::best());
                compressor.write_all(&buffer)
            }
            Compression::Lzma => {
                let mut cursor = Cursor::new(buffer);
                xz_compress(&mut cursor, &mut file).unwrap();
                Ok(())
            }
        }
    }

    fn generate_save_path(path: &Path, compression: Compression) -> PathBuf {
        let raw_name = path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap();
        let mut path = path.with_file_name(raw_name).display().to_string();

        path.push_str(".corp");
        if compression != Compression::None {
            path.push_str(&format!(".{}", compression.as_str()));
        }
        path.into()
    }

    pub const fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}

impl TryFrom<&Path> for Compression {
    type Error = ();

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        match value
            .extension()
            .unwrap()
            .to_ascii_lowercase()
            .to_str()
            .unwrap()
        {
            "gz" | "zip" => Ok(Self::Deflate),
            "lz" | "xz" | "lzma" => Ok(Self::Lzma),
            "corp" | "cbor" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

impl Compression {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Deflate => "gz",
            Self::Lzma => "xz",
        }
    }
}
