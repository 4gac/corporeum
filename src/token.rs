use crate::schema::Token;

impl Token {
    pub fn new(id: u32, token: &str) -> Token {
        Token {
            id,
            text: token.to_string(),
        }
    }
}
