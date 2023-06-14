use crate::schema::{Document, Sentence, Source};

impl Document {
    pub fn new(id: u32) -> Document {
        Document {
            id,
            source: None,
            description: None,
            sentences: Vec::new(),
        }
    }

    pub fn doc_id(&self) -> u32 {
        self.id
    }

    // add already tokenized sentence to the document
    pub fn create_sentence(&mut self, lang: &str) -> Sentence<Source> {
        if self.sentences.is_empty() {
            return Sentence::<Source>::new(0, lang);
        }
        Sentence::<Source>::new(self.sentences.last().unwrap().id + 1, lang)
    }

    pub fn add_sentence(&mut self, sent: Sentence<Source>) {
        if sent.tokens.is_empty() {
            return;
        }
        self.sentences.push(sent);
    }

    pub fn sentence_by_id(&self, id: u32) -> Option<&Sentence<Source>> {
        self.sentences.iter().find(|&sent| sent.id == id)
    }

    pub fn sentence_by_id_mut(&mut self, id: u32) -> Option<&mut Sentence<Source>> {
        self.sentences.iter_mut().find(|sent| sent.id == id)
    }
}
