use crate::schema::Corpus;

use ciborium::from_reader;
use ciborium::into_writer;

use std::{ffi::OsStr, fs, path::Path};

const EXTENSION: &str = "ucf"; // Unified Corpora Format

pub struct Corporeum<'a> {
    original_file_path: &'a Path,
    corpus: Corpus,
}

impl Corporeum<'_> {
    // return corporeum with an empty corpora from a given path
    pub fn new(buffer: &Path) -> Corporeum {
        let corpus = Corpus::default();

        Corporeum {
            original_file_path: buffer,
            corpus,
        }
    }

    // function to load an already existing corpus
    pub fn load<P: AsRef<Path> + std::io::Read>(source: &P) -> Corporeum {
        let mut _data: Vec<u8> = Vec::new();
        if source.as_ref().is_file() {
            _data = fs::read(source).expect("Unable to read file");
        } else {
            panic!("Not a file");
        }

        let corpus: Corpus = match source.as_ref().extension().and_then(OsStr::to_str).unwrap() {
            EXTENSION => from_reader(_data.as_slice()).unwrap(),
            _ => panic!("Unsupported file format"),
        };
        Corporeum {
            original_file_path: source.as_ref(),
            corpus,
        }
    }

    pub fn save<P: AsRef<Path>>(&self) {
        let dest = Path::with_extension(self.original_file_path, EXTENSION);
        let dest = dest.as_path();
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest)
            .unwrap();

        into_writer(&self.corpus, file).unwrap();
    }

    pub fn save_as<P: AsRef<Path>>(&self, destination: &P) {
        let destination = destination.as_ref();

        if destination.is_file() {
            let dest = Path::with_extension(destination, EXTENSION);
            let dest = dest.as_path();
            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(dest)
                .unwrap();

            into_writer(&self.corpus, file).unwrap();
        }
    }

    pub fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}
