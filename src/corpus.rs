use crate::schema::{Corpus, Document, Metadata};

impl Corpus {
    /// Return a reference to Metadata.
    pub const fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Return a mutable reference to Metadata.
    pub fn metadata_mut(&mut self) -> Option<&mut Metadata> {
        self.metadata.as_mut()
    }

    /// Adds metadata to corpus.
    /// - `name` - corpus name
    pub fn add_metadata(&mut self, name: &str) {
        self.metadata = Some(Metadata::new(name));
    }

    /// Return a list of documents in the corpus.
    pub const fn docs(&self) -> &Vec<Document> {
        &self.documents
    }

    /// Return a mutable reference to documents in the corpus.
    pub fn docs_mut(&mut self) -> &mut Vec<Document> {
        &mut self.documents
    }

    /// Fetch a document in the `Corpus` by id and retur a reference to it.
    /// Returns `None` if the document does not exist in the corpus.
    pub fn doc_by_id(&self, id: u32) -> Option<&Document> {
        self.documents.iter().find(|&doc| doc.doc_id() == id)
    }

    /// Fetch a document in the `Corpus` by id and return a mutable reference to it.
    /// Returns `None` if the document does not exist in the corpus.
    pub fn doc_by_id_mut(&mut self, id: u32) -> Option<&mut Document> {
        self.documents.iter_mut().find(|doc| doc.doc_id() == id)
    }

    /// Creates a new empty document with a unique ID.
    ///
    /// This new document can then be added to this corpus using [`add_doc`](Self::add_doc).
    pub fn create_doc(&mut self) -> Document {
        if self.documents.is_empty() {
            return Document::new(0);
        }
        Document::new(self.documents.last().unwrap().id + 1)
    }

    /// Adds the specified document to the corpus.
    ///
    /// To create a [`Document`](crate::Document) with a proper unique ID, use
    /// [`create_doc()`](Self::create_doc).
    // TODO: return Result<>, refuse to add document with no sentences
    pub fn add_doc(&mut self, doc: Document) {
        if doc.sentences.is_empty() {
            return;
        }
        self.documents.push(doc);
    }

    /// Removes a document from the corpus by id.
    // TODO: return Result<>, failed to remove doc, if id was not found
    pub fn remove_document(&mut self, id: u32) {
        self.documents.retain(|d| d.id == id);
    }
}
