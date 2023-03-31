use crate::schema::Sentence;
use crate::schema::SentenceType;
use crate::schema::Token;
use crate::schema::Translation;

impl Sentence {
    pub fn new(id: u32, tokens: &Vec<&str>, lang: &str, sent_type: SentenceType) -> Sentence {
        let mut words: Vec<Token> = Vec::with_capacity(tokens.len());

        for (i, token) in tokens.iter().enumerate() {
            let t = Token::new(i as u32, token);
            words.push(t);
        }

        Sentence {
            id,
            sentence_type: sent_type,
            lang: lang.to_string(),
            tokens: words,
            translations: Vec::new(),
        }
    }

    pub fn get_sentence_id(&self) -> u32 {
        self.id
    }

    pub fn add_translation(&mut self, tokens: Vec<&str>, lang: &str) {
        self.translations.push(Translation::new(
            0,
            &tokens,
            lang,
            SentenceType::Translation,
        ));
    }
}
