use crate::schema::{Corpus, Document, Metadata};

impl Corpus {
    // returns imutable reference to Metadata
    pub const fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    // returns mutable reference to Metadata
    pub fn metadata_mut(&mut self) -> Option<&mut Metadata> {
        self.metadata.as_mut()
    }

    // adds metadata to corpus
    // name = corpus name
    pub fn add_metadata(&mut self, name: &str) {
        self.metadata = Some(Metadata::new(name));
    }

    // returns a list of documents in the corpus
    pub const fn docs(&self) -> &Vec<Document> {
        &self.documents
    }

    // return a mutable reference to documents in the corpus
    pub fn docs_mut(&mut self) -> &mut Vec<Document> {
        &mut self.documents
    }

    // returns specific document by id
    pub fn doc_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.iter().find(|&doc| doc.doc_id() == id)
    }

    // return specific document by id as mutable
    pub fn doc_by_id_mut(&mut self, id: u32) -> Option<&mut Document> {
        self.documents.iter_mut().find(|doc| doc.doc_id() == id)
    }

    // returns an empty document with unique ID
    pub fn create_doc(&mut self) -> Document {
        if self.documents.is_empty() {
            return Document::new(0);
        }
        Document::new(self.documents.last().unwrap().id + 1)
    }

    // adds document to the corpus
    // TODO return Result<>, refuse to add document with no sentences
    pub fn add_doc(&mut self, doc: Document) {
        if doc.sentences.is_empty() {
            return;
        }
        self.documents.push(doc);
    }

    // removes document from corpus by id
    // TODO return Result<>, failed to remove doc, if id was not found
    pub fn remove_document(&mut self, id: u32) {
        self.documents.retain(|d| d.id == id);
    }
}
