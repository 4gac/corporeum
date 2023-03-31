use rust_tokenizers::{
    adapters::Example,
    tokenizer::{BertTokenizer, Tokenizer, TruncationStrategy},
    vocab::{BertVocab, Vocab},
};

use crate::schema::{Document, Sentence, SentenceType, Translation};

impl Document {
    pub fn new(id: u32) -> Document {
        Document {
            id,
            source: None,
            description: None,
            sentences: Vec::new(),
            last_sentence_id: 0,
        }
    }

    pub fn get_doc_id(&self) -> u32 {
        self.id
    }

    // add string and use internal tokenizer
    pub fn add_sentence(&mut self, sentence: &str, lang: &str) -> &mut Sentence {
        let vocab_path = "path/to/vocab";
        let vocab = BertVocab::from_file(vocab_path).unwrap();

        // let test_sentence = Example::new_from_string("This is a sample sentence to be tokenized");
        let bert_tokenizer: BertTokenizer = BertTokenizer::from_existing_vocab(vocab, true, true);

        let binding = bert_tokenizer.tokenize(sentence);
        let tokens: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

        // let mut sents: Vec<Sentence> = Vec::new();
        let sent = Sentence::new(
            self.generate_new_sentence_id(),
            &tokens,
            lang,
            SentenceType::Source,
        );
        self.sentences.push(sent);
        self.sentences.last_mut().unwrap()
    }

    // add already tokenized sentence to the document
    pub fn add_sentence_tokenized(&mut self, tokens: Vec<&str>, lang: &str) -> &mut Sentence {
        let sent = Sentence::new(
            self.generate_new_sentence_id(),
            &tokens,
            lang,
            SentenceType::Source,
        );
        self.sentences.push(sent);
        self.sentences.last_mut().unwrap()
    }

    pub fn get_sentence_by_id(&self, id: u32) -> Option<&Sentence> {
        self.sentences.iter().find(|&sent| sent.id == id)
    }

    pub fn get_sentence_by_id_mut(&mut self, id: u32) -> Option<&mut Sentence> {
        self.sentences.iter_mut().find(|sent| sent.id == id)
    }

    // cache next_id once when loading file, so we do not have iterate over the structure every time
    pub(crate) fn setup_last_sentence_id(&self) -> u32 {
        self.sentences.last().unwrap().id + 1
    }

    fn generate_new_sentence_id(&mut self) -> u32 {
        self.last_sentence_id += 1;
        self.last_sentence_id
    }
}
