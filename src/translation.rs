use crate::schema::{SentenceType, Token, Tokens, Translation};

impl Translation {
    pub fn new(id: u32, tokens: &Vec<&str>, lang: &str, sent_type: SentenceType) -> Translation {
        let mut words: Vec<Token> = Vec::with_capacity(tokens.len());

        for (i, token) in tokens.iter().enumerate() {
            let t = Token::new(i as u32, token);
            words.push(t);
        }

        let tokens = Tokens { token: words };

        Translation {
            id: id,
            sentence_type: sent_type,
            lang: lang.to_string(),
            tokens: tokens,
        }
    }

    pub fn get_translation_id(&self) -> u32 {
        self.id
    }
}
