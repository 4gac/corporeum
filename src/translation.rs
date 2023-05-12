use crate::schema::{SentenceType, Token, Translation};

impl Translation {
    #[allow(unused)]
    pub const fn get_translation_id(&self) -> u32 {
        self.id
    }

    #[allow(unused)]
    pub fn new(id: u32, tokens: &Vec<&str>, lang: &str, sent_type: SentenceType) -> Self {
        let mut words: Vec<Token> = Vec::with_capacity(tokens.len());

        for (i, token) in tokens.iter().enumerate() {
            let t = Token::new(i as u32, token);
            words.push(t);
        }

        Self {
            lang: lang.to_string(),
            id,
            sentence_type: sent_type,
            tokens: words,
        }
    }
}
