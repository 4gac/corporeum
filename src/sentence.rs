use crate::schema::Sentence;
use crate::schema::SentenceType;
use crate::schema::Token;
use crate::schema::Tokens;

impl Sentence {
    pub fn new(id: u32, tokens: &Vec<&str>, lang: &str, sent_type: SentenceType) -> Sentence {
        let mut words: Vec<Token> = Vec::with_capacity(tokens.len());

        for (i, token) in tokens.iter().enumerate() {
            let t = Token::new(i as u32, token);
            words.push(t);
        }

        let tokens = Tokens { token: words };

        Sentence {
            id: id,
            sentence_type: sent_type,
            lang: lang.to_string(),
            tokens: tokens,
            translations: None,
        }
    }

    pub fn get_sentence_id(&self) -> u32 {
        self.id
    }
}
