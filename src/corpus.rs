use crate::{
    schema::{Corpus, Document, Metadata},
    CorporeumError,
};

impl Corpus {
    /// Return a reference to Metadata.
    pub const fn get_metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Return a mutable reference to Metadata.
    pub fn get_metadata_mut(&mut self) -> Option<&mut Metadata> {
        self.metadata.as_mut()
    }

    /// Adds metadata to corpus.
    /// - `name` - corpus name
    pub fn add_metadata(&mut self, name: &str) {
        self.metadata = Some(Metadata::new(name));
    }

    /// Return a list of documents in the corpus.
    pub const fn get_docs(&self) -> &Vec<Document> {
        &self.documents
    }

    /// Return a mutable reference to documents in the corpus.
    pub fn get_docs_mut(&mut self) -> &mut Vec<Document> {
        &mut self.documents
    }

    /// Fetch a document in the `Corpus` by id and return a reference to it.
    /// Returns `None` if the document does not exist in the corpus.
    pub fn get_doc(&self, id: u32) -> Option<&Document> {
        self.documents.iter().find(|&doc| doc.get_doc_id() == id)
    }

    /// Fetch a document in the `Corpus` by id and return a mutable reference to it.
    /// Returns `None` if the document does not exist in the corpus.
    pub fn get_doc_mut(&mut self, id: u32) -> Option<&mut Document> {
        self.documents.iter_mut().find(|doc| doc.get_doc_id() == id)
    }

    /// Creates a new empty document with a unique ID.
    ///
    /// This new document can then be added to this corpus using [`add_doc`](Self::add_doc).
    pub fn create_doc(&mut self) -> Document {
        Document::new(self.documents.last().map_or(0, |doc| doc.id + 1))
    }

    /// Adds the specified document to the corpus.
    ///
    /// # Errors
    /// This will return an error if the specified document is empty (contains no sentences).
    ///
    /// To create a [`Document`](crate::Document) with a proper unique ID, use
    /// [`create_doc()`](Self::create_doc).
    pub fn add_doc(&mut self, doc: Document) -> Result<(), CorporeumError> {
        if doc.sentences.is_empty() {
            return Err(CorporeumError::EmptyObject(
                "Document has no sentences in it".to_owned(),
            ));
        }
        self.documents.push(doc);
        Ok(())
    }

    /// Removes a document from the corpus by id.
    ///
    /// # Errors
    /// This will return an error if the corpus does not contain a document
    /// with the specified ID.
    pub fn remove_document(&mut self, id: u32) -> Result<(), CorporeumError> {
        if id as usize >= self.documents.len() {
            return Err(CorporeumError::ElementNotFound(format!(
                "Document with ID {id} does not exist"
            )));
        }

        self.documents.remove(id as usize);
        Ok(())
    }
}
