use crate::schema::Token;

impl Token {
    pub fn new(id: u32, token: &str) -> Token {
        Token {
            id,
            form: token.to_string(),
        }
    }

    pub fn form(&self) -> &str {
        &self.form
    }

    pub fn form_mut(&mut self) -> &mut String {
        &mut self.form
    }
}
