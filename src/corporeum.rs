use rkyv::Deserialize;

use crate::schema::Corpus;
use std::{ffi::OsStr, fs, io::Write, path::Path};

const FILE_EXT: &str = "corp";

pub struct Corporeum<'a> {
    original_file_path: &'a Path,
    corpus: Corpus,
}

impl Corporeum<'_> {
    pub fn new<P: AsRef<Path>>(buffer: &P) -> Corporeum {
        let corpus = Corpus::default();

        Corporeum {
            original_file_path: buffer.as_ref(),
            corpus,
        }
    }

    // function to load an already existing corpus
    pub fn load<P: AsRef<Path>>(source: &P) -> Corporeum {
        let mut data: Vec<u8> = Vec::new();
        if source.as_ref().is_file() {
            // FIXME error handling
            data = fs::read(source).expect("Unable to read file");
        } else {
            panic!("Not a file");
        }

        // parse file
        let corpus = match source.as_ref().extension().and_then(OsStr::to_str).unwrap() {
            FILE_EXT => {
                let archived = unsafe { rkyv::archived_root::<Corpus>(&data) };
                archived.deserialize(&mut rkyv::Infallible).unwrap()
            }
            _ => panic!("Unsupported file format"),
        };
        Corporeum {
            original_file_path: source.as_ref(),
            corpus,
        }
    }

    pub fn save(&self) {
        let buffer = rkyv::to_bytes::<_, 256>(&self.corpus).unwrap();

        let dest = Path::with_extension(self.original_file_path, FILE_EXT);
        let dest = dest.as_path();

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest);

        file.unwrap().write_all(&buffer).unwrap();
    }

    pub fn save_as<P: AsRef<Path>>(&self, destination: &P) {
        let buffer = rkyv::to_bytes::<_, 256>(&self.corpus).unwrap();

        let destination = destination.as_ref();

        if destination.is_file() {
            let dest = Path::with_extension(&destination, FILE_EXT);
            let dest = dest.as_path();
            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(dest);

            file.unwrap().write_all(&buffer).unwrap();
        }
    }

    pub fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}
