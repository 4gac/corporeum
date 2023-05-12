use crate::schema::Corpus;
use std::{fs, io::Write, path::Path};

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
    pub fn load<P: AsRef<Path>>(source: &P) -> Corporeum {
        let data = if source.as_ref().is_file() {
            // FIXME error handling
            fs::read(source).expect("Unable to read file")
        } else {
            panic!("Not a file");
        };

        // parse json file
        let corpus: Corpus = serde_cbor::from_slice(&data).unwrap();
        // let mut corpus: Corpus = serde_json::from_str(&data).unwrap();
        // let mut corp: Corpus =
        // serde_pickle::from_slice(&data.as_bytes(), Default::default()).unwrap();
        // iterate over docs and setup last sentence id,
        // so we do not have search for last available id every time we add new sentence
        // corpus
        //     .documents
        //     .iter_mut()
        //     .for_each(|doc| doc.last_sentence_id = doc.setup_last_sentence_id());

        Corporeum {
            original_file_path: source.as_ref(),
            corpus,
        }
    }

    pub fn save_cbor(&self, packed: bool) {
        let buffer = if packed {
            serde_cbor::ser::to_vec_packed(&self.corpus).unwrap()
        } else {
            serde_cbor::ser::to_vec(&self.corpus).unwrap()
        };

        let dest = Path::with_extension(self.original_file_path, "cbor");
        let dest = dest.as_path();

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest);

        file.unwrap().write_all(&buffer).unwrap();
    }

    pub fn save_as_cbor<P: AsRef<Path>>(&self, destination: &P, packed: bool) {
        let buffer = if packed {
            serde_cbor::ser::to_vec_packed(&self.corpus).unwrap()
        } else {
            serde_cbor::ser::to_vec(&self.corpus).unwrap()
        };

        let destination = destination.as_ref();

        if destination.is_file() {
            let dest = Path::with_extension(destination, "cbor");
            let dest = dest.as_path();

            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(dest);

            file.unwrap().write_all(&buffer).unwrap();
        }
    }

    pub const fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}
