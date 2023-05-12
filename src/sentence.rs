use crate::schema::Sentence;
use crate::schema::Token;

impl Sentence {
    pub fn new(id: u32, lang: &str) -> Self {
        Self {
            id,
            lang: lang.to_string(),
            tokens: Vec::new(),
            translations: Vec::new(),
        }
    }

    pub const fn sentence_id(&self) -> u32 {
        self.id
    }

    pub fn create_translation(&self, lang: &str) -> Self {
        if self.translations.is_empty() {
            return Self::new(0, lang);
        }
        Self::new(self.translations.last().unwrap().id + 1, lang)
    }

    pub fn add_translation(&mut self, translation: Self) {
        if translation.tokens.is_empty() {
            return;
        }
        self.translations.push(translation);
    }

    pub fn create_token(&self, form: &str) -> Token {
        if self.tokens.is_empty() {
            return Token::new(0, form);
        }
        Token::new(self.tokens.last().unwrap().id + 1, form)
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
