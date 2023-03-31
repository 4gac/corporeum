use crate::Metadata;

use super::schema::Corpus;
use serde_json;
use std::{fs, io::Write, path::Path};

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
    pub fn load(source: &Path) -> Corporeum {
        let mut data: String = String::new();
        if source.is_file() {
            // FIXME error handling
            data = fs::read_to_string(source).expect("Unable to read file");
        }

        // parse json file
        let mut corpus: Corpus = serde_json::from_str(&data).unwrap();
        // iterate over docs and setup last sentence id,
        // so we do not have search for last available id every time we add new sentence
        corpus
            .documents
            .iter_mut()
            .for_each(|doc| doc.last_sentence_id = doc.setup_last_sentence_id());

        Corporeum {
            original_file_path: source,
            corpus: corpus,
        }
    }

    // TODO return Result<OK,FailedToWrite>
    pub fn save(&self) {
        let buffer = serde_json::to_vec(&self.corpus).unwrap();

        let dest: &Path = self.original_file_path;

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dest);

        file.unwrap().write_all(&buffer).unwrap();
    }

    // TODO return Result<OK,FailedToWrite>
    pub fn save_as(&self, destination: &mut Path) {
        // let mut buffer = Vec::new();

        // serde_xml_rs::to_writer(&mut buffer, &self.corpus).unwrap();
        let buffer = serde_json::to_vec(&self.corpus).unwrap();
        if destination.is_file() {
            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(destination);

            file.unwrap().write_all(&buffer).unwrap();
        }
    }

    pub fn get_corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn get_corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }
}
