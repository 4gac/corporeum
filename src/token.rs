use crate::schema::Token;

impl Token {
    pub fn new(id: u32, token: &str) -> Token {
        Token {
            id,
            form: token.to_string(),
        }
    }

    pub fn set_form(&mut self, form: &str) {
        self.form = form.to_string();
    }

    pub fn get_form(&self) -> &str {
        &self.form
    }
}
