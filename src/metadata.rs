use crate::schema::Author;
use crate::schema::Metadata;

impl Metadata {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            authors: Vec::new(),
            created: None,
            description: None,
            modified: None,
            name: name.to_owned(),
            version: 1,
        }
    }

    /// Returns the corpus name.
    pub fn corpus_name(&self) -> &str {
        &self.name
    }

    /// Sets the corpus name to the specified value.
    pub fn set_corpus_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns a reference to the list of authors of this metadata.
    pub const fn authors(&self) -> &Vec<Author> {
        &self.authors
    }

    /// Adds a new author to this metadata.
    pub fn add_author(&mut self, first_name: &str, last_name: &str, mail: Option<&str>) {
        self.authors.push(Author::new(first_name, last_name, mail));
    }

    // Removes author(s) based on their first and last name.
    pub fn remove_author(&mut self, first_name: &str, last_name: &str) {
        self.authors
            .retain(|a| a.first_name == first_name && a.last_name == last_name);
    }

    /// Returns the description of this metadata.
    pub fn description(&self) -> &str {
        self.description.as_ref().unwrap()
    }

    /// Sets the description of this metadata.
    pub fn set_description(&mut self, desc: &str) {
        self.description = Some(desc.to_owned());
    }

    /// Returns the version of this metadata.
    pub const fn version(&self) -> u16 {
        self.version
    }

    // TODO
    // pub fn creation_date(&self) -> &str {
    //     let x = self.created.as_ref().unwrap();
    // }

    // pub fn modified_date(&self) -> String {
    //     self.modified.as_ref().unwrap().to_string()
    // }
}
