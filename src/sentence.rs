use crate::schema::Sentence;
use crate::schema::Source;
use crate::schema::Target;
use crate::schema::Token;
use std::marker::PhantomData;

impl Sentence<Source> {
    pub fn new(id: u32, lang: &str) -> Self {
        Self {
            t: PhantomData::default(),
            id,
            lang: lang.to_string(),
            tokens: Vec::new(),
            translations: Vec::new(),
        }
    }

    pub fn create_translation(&self, lang: &str) -> Sentence<Target> {
        if self.translations.is_empty() {
            return Sentence::<Target>::new(0, lang);
        }
        Sentence::<Target>::new(self.translations.last().unwrap().id + 1, lang)
    }

    pub fn add_translation(&mut self, translation: Sentence<Target>) {
        if translation.tokens.is_empty() {
            return;
        }
        self.translations.push(translation);
    }
}

impl Sentence<Target> {
    pub fn new(id: u32, lang: &str) -> Self {
        Self {
            t: PhantomData::default(),
            id,
            lang: lang.to_string(),
            tokens: Vec::new(),
            translations: Vec::new(),
        }
    }
}

impl<T> Sentence<T> {
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

    pub const fn sentence_id(&self) -> u32 {
        self.id
    }
}
