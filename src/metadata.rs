use crate::schema::Author;
use crate::Metadata;

impl Metadata {
    pub(crate) fn new(name: &str) -> Metadata {
        Metadata {
            authors: Vec::new(),
            created: None,
            description: None,
            modified: None,
            name: name.to_owned(),
            version: 1,
        }
    }

    pub fn get_corpus_name(&self) -> &str {
        &self.name
    }

    pub fn add_author(&mut self, first_name: &str, last_name: &str, mail: Option<&str>) {
        self.authors.push(Author::new(first_name, last_name, mail));
    }

    // removes author(s) based on first and last name
    pub fn remove_author(&mut self, first_name: &str, last_name: &str) {
        self.authors
            .retain(|a| a.first_name == first_name && a.last_name == last_name);
    }

    pub fn get_authors(&self) -> &Vec<Author> {
        &self.authors
    }

    pub fn set_description(&mut self, desc: &str) {
        self.description = Some(desc.to_owned())
    }

    pub fn get_description(&self) -> &str {
        self.description.as_ref().unwrap()
    }

    pub fn get_version(&self) -> u16 {
        self.version
    }

    pub fn get_creation_date(&self) -> String {
        self.created.as_ref().unwrap().to_string()
    }

    pub fn get_modified_date(&self) -> String {
        self.modified.as_ref().unwrap().to_string()
    }
}
