use crate::schema::Metadata;

use crate::schema::Corpus;
use rmp_serde;
use serde_cbor::de::from_slice;
use serde_cbor::ser::{to_vec, to_vec_packed};
use serde_json;
use std::{ffi::OsStr, fs, io::Write, path::Path};

const JSON: &str = "json";
const PICKLE: &str = "pickle";
const MSGPACK: &str = "msgpack";
const CBOR: &str = "cbor";

pub struct Corporeum<'a> {
    original_file_path: &'a Path,
    corpus: Corpus,
}

impl Corporeum<'_> {
    pub fn new(buffer: &Path) -> Corporeum {
        let corpus = Corpus::default();

        Corporeum {
            original_file_path: buffer,
            corpus: corpus,
        }
    }

    // function to load an already existing corpus
    pub fn load<P: AsRef<Path>>(source: &P) -> Corporeum {
        let mut data: String = String::new();
        if source.as_ref().is_file() {
            // FIXME error handling
            data = fs::read_to_string(source).expect("Unable to read file");
        } else {
            panic!("Not a file");
        }

        // parse json file
        let mut corpus: Corpus = match source.as_ref().extension().and_then(OsStr::to_str).unwrap()
        {
            JSON => serde_json::from_str(&data).unwrap(),
            PICKLE => serde_pickle::from_slice(&data.as_bytes(), Default::default()).unwrap(),
            MSGPACK => rmp_serde::from_slice(&data.as_bytes()).unwrap(),
            CBOR => serde_cbor::from_slice(&data.as_bytes()).unwrap(),
            _ => panic!("Unsupported file format"),
        };
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
            corpus: corpus,
        }
    }

    // TODO return Result<OK,FailedToWrite>
    pub fn save_json(&self, pretty: bool) {
        let buffer = if pretty {
            serde_json::to_vec_pretty(&self.corpus).unwrap()
        } else {
            serde_json::to_vec(&self.corpus).unwrap()
        };

        let dest = Path::with_extension(self.original_file_path, JSON);
        let dest = dest.as_path();

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest);

        file.unwrap().write_all(&buffer).unwrap();
    }

    // TODO return Result<OK,FailedToWrite>
    pub fn save_as_json<P: AsRef<Path>>(&self, destination: &P, pretty: bool) {
        // let mut buffer = Vec::new();

        // serde_xml_rs::to_writer(&mut buffer, &self.corpus).unwrap();
        let buffer = if pretty {
            serde_json::to_vec(&self.corpus).unwrap()
        } else {
            serde_json::to_vec_pretty(&self.corpus).unwrap()
        };

        let destination = destination.as_ref();

        if destination.is_file() {
            let dest = Path::with_extension(destination, JSON);
            let dest = dest.as_path();
            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(dest);

            file.unwrap().write_all(&buffer).unwrap();
        }
    }

    pub fn save_pickle(&self) {
        let buffer = serde_pickle::to_vec(&self.corpus, Default::default()).unwrap();

        let dest = Path::with_extension(self.original_file_path, PICKLE);
        let dest = dest.as_path();

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest);

        file.unwrap().write_all(&buffer).unwrap();
    }

    pub fn save_as_pickle<P: AsRef<Path>>(&self, destination: &P) {
        let buffer = serde_pickle::to_vec(&self.corpus, Default::default()).unwrap();

        let destination = destination.as_ref();

        if destination.is_file() {
            let dest = Path::with_extension(&destination, PICKLE);
            let dest = dest.as_path();
            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(dest);

            file.unwrap().write_all(&buffer).unwrap();
        }
    }

    pub fn save_msgpack(&self) {
        let buffer = rmp_serde::to_vec(&self.corpus).unwrap();

        let dest = Path::with_extension(self.original_file_path, MSGPACK);
        let dest = dest.as_path();

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest);

        file.unwrap().write_all(&buffer).unwrap();
    }

    pub fn save_as_msgpack<P: AsRef<Path>>(&self, destination: &mut P) {
        let buffer = rmp_serde::to_vec(&self.corpus).unwrap();

        let destination = destination.as_ref();

        if destination.is_file() {
            let dest = Path::with_extension(&destination, MSGPACK);
            let dest = dest.as_path();

            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(dest);

            file.unwrap().write_all(&buffer).unwrap();
        }
    }

    pub fn save_cbor(&self, packed: bool) {
        let buffer = if packed {
            serde_cbor::ser::to_vec_packed(&self.corpus).unwrap()
        } else {
            serde_cbor::ser::to_vec(&self.corpus).unwrap()
        };

        let dest = Path::with_extension(self.original_file_path, CBOR);
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
            let dest = Path::with_extension(&destination, CBOR);
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
