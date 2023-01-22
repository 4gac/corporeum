use crate::Metadata;

impl Metadata {
    pub fn get_corpus_name(&self) -> String {
        self.name.as_ref().unwrap().text.to_string()
    }

    pub fn get_authors(&self) -> Vec<String> {
        let mut to_ret: Vec<String> = Vec::new();

        let authors = self.authors.as_ref().unwrap();
        authors.iter().for_each(|author| {
            to_ret.push(author.text.to_string());
        });

        to_ret
    }

    pub fn get_description(&self) -> String {
        self.description.as_ref().unwrap().text.to_string()
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
