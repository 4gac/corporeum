use crate::schema::Sentence;
use crate::schema::Source;
use crate::schema::Target;
use crate::schema::Token;
use std::marker::PhantomData;

impl Sentence<Source> {
    pub(crate) fn new(id: u32, lang: &str) -> Self {
        Self {
            t: PhantomData,
            id,
            lang: lang.to_string(),
            tokens: Vec::new(),
            translations: Vec::new(),
        }
    }

    /// Creates a new translation.
    ///
    /// To add this translation to a [`Sentence<Source>`](Self), use
    /// [`add_translation()`](Self::add_translation).
    pub fn create_translation(&self, lang: &str) -> Sentence<Target> {
        Sentence::<Target>::new(self.translations.last().map_or(0, |t| t.id + 1), lang)
    }

    /// Add a translation.
    ///
    /// To create a new translation, use [`create_translation()`](Self::create_translation).
    pub fn add_translation(&mut self, translation: Sentence<Target>) {
        if translation.tokens.is_empty() {
            return;
        }
        self.translations.push(translation);
    }

    pub fn remove_translation(&mut self, id: usize) {
        self.translations.remove(id);
    }

    pub fn translation(&self, id: u32) -> Option<&Sentence<Target>> {
        self.translations.iter().find(|trans| trans.id == id)
    }

    pub fn get_translation_mut(&mut self, id: u32) -> Option<&mut Sentence<Target>> {
        self.translations.iter_mut().find(|trans| trans.id == id)
    }

    pub const fn translations(&self) -> &Vec<Sentence<Target>> {
        &self.translations
    }

    pub fn translations_mut(&mut self) -> &mut Vec<Sentence<Target>> {
        &mut self.translations
    }
}

impl Sentence<Target> {
    pub(crate) fn new(id: u32, lang: &str) -> Self {
        Self {
            t: PhantomData,
            id,
            lang: lang.to_string(),
            tokens: Vec::new(),
            translations: Vec::new(),
        }
    }
}

impl<T> Sentence<T> {
    /// Creates a new [`Token`](crate::Token) with a proper unique ID.
    ///
    /// To add this token to this Sentence, use [`add_token`](Self::add_token).
    pub fn create_token(&self, form: &str) -> Token {
        Token::new(self.tokens.last().map_or(0, |t| t.id + 1), form)
    }

    /// Add a token to this sentence.
    ///
    /// To create a [`Token`](crate::Token) with a proper unique ID, use
    /// [`create_token()`](Self::create_token).
    pub fn add_token(&mut self, token: Token) {
        if token.form.is_empty() {
            return;
        }
        self.tokens.push(token);
    }

    pub const fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub const fn sentence_id(&self) -> u32 {
        self.id
    }

    pub fn to_text(&self) -> String {
        return self
            .tokens()
            .iter()
            .map(|t| t.form())
            .collect::<Vec<&str>>()
            .join(" ");
    }
}
