use crate::schema::Author;
use crate::Metadata;

impl Metadata {
    pub(crate) fn new(name: &str) -> Metadata {
        Metadata {
            authors: None,
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
        let author = Author::new(first_name, last_name, mail);
        if let Some(authors) = &mut self.authors {
            authors.push(author);
        } else {
            self.authors = Some(vec![author]);
        }
    }

    pub fn get_authors(&self) -> Vec<&Author> {
        let mut to_ret: Vec<&Author> = Vec::new();

        let authors = self.authors.as_ref().unwrap();
        authors.iter().for_each(|author| {
            to_ret.push(author);
        });

        to_ret
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
