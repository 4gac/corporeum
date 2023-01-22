use crate::schema::{Corpus, Document, Metadata};

impl Corpus {
    pub fn get_metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
    pub fn get_documents(&self) -> Option<&Vec<Document>> {
        self.documents.docs.as_ref()
    }

    pub fn get_document_by_id(&self, id: u32) -> Option<&Document> {
        let docs = self.documents.docs.as_ref().unwrap();
        docs.iter().find(|&doc| doc.get_doc_id() == id)
    }

    pub fn get_document_by_id_mut(&mut self, id: u32) -> Option<&mut Document> {
        let docs = self.documents.docs.as_mut().unwrap();

        docs.iter_mut().find(|doc| doc.get_doc_id() == id)
    }
    pub fn add_document(&mut self) -> u32 {
        if let Some(docs) = &mut self.documents.docs.as_mut() {
            let mut max_id = 0;
            for doc in docs.iter() {
                if doc.get_doc_id() > max_id {
                    max_id = doc.get_doc_id()
                }
            }

            let doc: Document = Document::new(max_id + 1);
            docs.push(doc);
            max_id + 1
        } else {
            let mut vec: Vec<Document> = Vec::new();
            let doc = Document::new(0);
            vec.push(doc);
            self.documents.docs = Some(vec);
            0
        }
    }
}
