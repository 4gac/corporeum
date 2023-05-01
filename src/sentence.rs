use crate::schema::Sentence;
use crate::schema::SentenceType;
use crate::schema::Token;
use crate::schema::Translation;

impl Sentence {
    pub fn new(id: u32, lang: &str) -> Sentence {
        // let mut words: Vec<Token> = Vec::with_capacity(tokens.len());

        // for (i, token) in tokens.iter().enumerate() {
        //     let t = Token::new(i as u32, token);
        //     words.push(t);
        // }

        Sentence {
            id,
            lang: lang.to_string(),
            tokens: Vec::new(),
            translations: Vec::new(),
        }
    }

    pub fn get_sentence_id(&self) -> u32 {
        self.id
    }

    pub fn create_translation(&self, lang: &str) -> Sentence {
        if self.translations.is_empty() {
            return Sentence::new(0, lang);
        }
        Sentence::new(self.translations.last().unwrap().id + 1, lang)
    }

    pub fn add_translation(&mut self, translation: Sentence) {
        if translation.tokens.is_empty() {
            return;
        }
        self.translations.push(translation);
    }

    pub fn create_token(&self) -> Token {
        if self.tokens.is_empty() {
            return Token::new(0, "");
        }
        Token::new(self.tokens.last().unwrap().id + 1, "")
    }

    pub fn add_token(&mut self, token: Token) {
        if token.form.is_empty() {
            return;
        }
        self.tokens.push(token);
    }

    // cache next_id once when loading file, so we do not have iterate over the structure every time
    // pub(crate) fn setup_last_translation_id(&self) -> u32 {
    //     self.sentences.last().unwrap().id + 1
    // }

    // fn generate_new_translation_id(&mut self) -> u32 {
    //     self.last_translation_id += 1;
    //     self.last_translation_id
    // }
}
