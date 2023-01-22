use crate::schema::{Document, Sentence, SentenceType, Sentences, Translation, Translations};

impl Document {
    pub fn new(id: u32) -> Document {
        Document {
            id: id,
            source: None,
            description: None,
            sentences: None,
        }
    }

    pub fn get_doc_id(&self) -> u32 {
        self.id
    }

    pub fn add_sentence(&mut self, tokens: Vec<&str>, lang: &str) -> u32 {
        if let Some(sentences) = self.sentences.as_mut() {
            let mut sent_id: u32 = 0;
            if let Some(sents) = sentences.sents.as_mut() {
                for sent in sents.iter() {
                    if sent.get_sentence_id() > sent_id {
                        sent_id = sent.get_sentence_id()
                    }
                }
            }
            let sent = Sentence::new(sent_id + 1, &tokens, lang, SentenceType::Source);
            self.sentences
                .as_mut()
                .unwrap()
                .sents
                .as_mut()
                .unwrap()
                .push(sent);
            sent_id + 1
        } else {
            let mut vec: Vec<Sentence> = Vec::new();
            let sent = Sentence::new(0, &tokens, lang, SentenceType::Source);
            vec.push(sent);

            let sents = Sentences {
                sents: Some(Vec::new()),
            };

            self.sentences = Some(sents);
            self.sentences.as_mut().unwrap().sents = Some(vec);
            0
        }
    }

    pub fn get_sentence_by_id(&self, id: u32) -> Option<&Sentence> {
        self.sentences
            .as_ref()
            .unwrap()
            .sents
            .as_ref()
            .unwrap()
            .iter()
            .find(|&sent| sent.id == id)
    }

    pub fn get_sentence_by_id_mut(&mut self, id: u32) -> Option<&mut Sentence> {
        self.sentences
            .as_mut()
            .unwrap()
            .sents
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|sent| sent.id == id)
    }

    pub fn add_translation(&mut self, sentence_id: u32, tokens: Vec<&str>, lang: &str) {
        let Some(sentence) = self.get_sentence_by_id_mut(sentence_id) else{
          panic!("Failed to find sentence with {sentence_id}");  
        };

        match &mut sentence.translations {
            Some(translations) => {
                let mut trans_id: u32 = 0;

                for tran in translations.translations.iter() {
                    if tran.id > sentence_id {
                        trans_id = tran.id
                    }
                }

                let t = Translation::new(trans_id, &tokens, lang, SentenceType::Translation);

                translations.translations.push(t);
            }
            None => {
                let mut translations = Vec::new();
                let t = Translation::new(0, &tokens, lang, SentenceType::Translation);
                translations.push(t);

                sentence.translations = Some(Translations {
                    translations: translations,
                });
            }
        }
    }
}
