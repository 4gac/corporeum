use crate::schema::{Corpus, Document, Metadata};

impl Corpus {
    pub fn add_metadata(&mut self, name: &str) {
        self.metadata = Some(Metadata::new(name));
    }

    pub fn get_metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    pub fn get_metadata_mut(&mut self) -> Option<&mut Metadata> {
        self.metadata.as_mut()
    }

    pub fn get_documents(&self) -> &Vec<Document> {
        &self.documents
    }

    pub fn get_document_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.iter().find(|&doc| doc.get_doc_id() == id)
    }

    pub fn get_document_by_id_mut(&mut self, id: u32) -> Option<&mut Document> {
        self.documents.iter_mut().find(|doc| doc.get_doc_id() == id)
    }

    pub fn add_document(&mut self) -> &mut Document {
        if self.documents.is_empty() {
            let doc = Document::new(0);
            self.documents.push(doc);
            return self.documents.last_mut().unwrap();
        }

        let doc: Document = Document::new(self.documents.last().unwrap().id + 1);
        self.documents.push(doc);
        self.documents.last_mut().unwrap()
    }

    pub fn remove_document(&mut self, id: u32) {
        self.documents.retain(|d| d.id == id);
    }
}
