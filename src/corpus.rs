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
    pub fn get_documents(&self) -> Option<&Vec<Document>> {
        Some(self.documents.as_ref())
    }

    pub fn get_document_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.iter().find(|&doc| doc.get_doc_id() == id)
    }

    pub fn get_document_by_id_mut(&mut self, id: u32) -> Option<&mut Document> {
        self.documents.iter_mut().find(|doc| doc.get_doc_id() == id)
    }
    pub fn add_document(&mut self) -> u32 {
        if self.documents.is_empty() {
            let doc = Document::new(0);
            self.documents.push(doc);
            return 0;
        }

        let mut max_id = 0;
        for doc in self.documents.iter() {
            if doc.get_doc_id() > max_id {
                max_id = doc.get_doc_id()
            }
        }

        let doc: Document = Document::new(max_id + 1);
        self.documents.push(doc);
        max_id + 1
    }
}
