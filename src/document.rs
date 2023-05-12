use crate::schema::{Document, Sentence};

impl Document {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            source: None,
            description: None,
            sentences: Vec::new(),
            // last_sentence_id: 0,
        }
    }

    pub const fn doc_id(&self) -> u32 {
        self.id
    }

    // add already tokenized sentence to the document
    pub fn create_sentence(&mut self, lang: &str) -> Sentence {
        if self.sentences.is_empty() {
            return Sentence::new(0, lang);
        }
        Sentence::new(self.sentences.last().unwrap().id + 1, lang)
    }

    pub fn add_sentence(&mut self, sent: Sentence) {
        if sent.tokens.is_empty() {
            return;
        }
        self.sentences.push(sent);
    }

    pub fn sentence_by_id(&self, id: u32) -> Option<&Sentence> {
        self.sentences.iter().find(|&sent| sent.id == id)
    }

    pub fn sentence_by_id_mut(&mut self, id: u32) -> Option<&mut Sentence> {
        self.sentences.iter_mut().find(|sent| sent.id == id)
    }

    // cache next_id once when loading file, so we do not have iterate over the structure every time
    // pub(crate) fn setup_last_sentence_id(&self) -> u32 {
    //     self.sentences.last().unwrap().id + 1
    // }

    // fn generate_new_sentence_id(&mut self) -> u32 {
    //     self.last_sentence_id += 1;
    //     self.last_sentence_id
    // }
}
