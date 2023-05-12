use crate::schema::Author;

impl Author {
    pub fn new(first_name: &str, last_name: &str, mail: Option<&str>) -> Self {
        Self {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            mail: mail.map(str::to_string),
        }
    }
}
