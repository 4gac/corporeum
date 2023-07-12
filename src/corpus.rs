use ciborium::{from_reader, into_writer};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Cursor, Read, Write};

use crate::{
    schema::{Corpus, Document, Metadata},
    CorporeumError,
};

impl Corpus {
    /// Creates a new empty `Corpus`.
    ///
    /// # Warnings
    /// - The function only creates an in-memory representation.
    ///
    /// # Example
    /// ```
    /// # use corporum::Corpus;
    /// let corp = Corpus::new();
    /// ```
    ///
    /// # Errors
    /// This function will never return an error.
    pub const fn new() -> Self {
        Self {
            metadata: None,
            documents: Vec::new(),
        }
    }

    /// Load an already existing corpus from a stream.
    ///
    /// # Example
    /// ```no_run
    /// # use corporum::Corpus;
    /// # use std::fs::File;
    /// let file = File::open("some_file.ucf").unwrap();
    /// let corp = match Corpus::load(file) {
    ///     Ok(corp) => corp,
    ///     Err(e) => panic!("Error loading corpus: {e}"),
    /// };
    ///
    /// // ...
    /// ```
    ///
    /// # Errors
    /// This will return an error if:
    /// - The contents could not be decompressed.
    /// - The contents could not be deserialized.
    pub fn load<R: Read>(source: R) -> Result<Self, CorporeumError> {
        let mut decompressed = Vec::new();
        let mut decompressor = ZlibDecoder::new(source);

        decompressor
            .read_to_end(&mut decompressed)
            .map_err(CorporeumError::DecompressionError)?;

        Ok(from_reader(decompressed.as_slice())?)
    }

    /// Save the corpus into a readable stream of bytes.
    ///
    /// # Example
    /// ```no_run
    /// # use corporum::Corpus;
    /// # use std::fs::OpenOptions;
    /// # use std::process::exit;
    /// #
    /// let corp = Corpus::new();
    /// // ... do some work ...
    ///
    /// let mut stream = match corp.save_stream() {
    ///     Ok(stream) => stream,
    ///     Err(e) => {
    ///         eprintln!("Failed to save: {e}");
    ///         exit(0);
    ///     },
    /// };
    /// // ...
    /// ```
    ///
    /// # Errors
    /// This will return an error if:
    /// - The serialization fails
    /// - Compression fails
    pub fn save_stream(&self) -> Result<Box<dyn Read>, CorporeumError> {
        let result = Vec::new();
        let mut result_cursor = Cursor::new(result);

        let mut serialized = Vec::new();
        into_writer(self, Cursor::new(&mut serialized))?;

        {
            let mut compressor = ZlibEncoder::new(&mut result_cursor, Compression::best());

            compressor
                .write_all(&serialized)
                .map_err(CorporeumError::CompressionError)?;
        }

        Ok(Box::new(result_cursor))
    }

    /// Save the corpus into a writable stream.
    ///
    /// # Example
    /// ```no_run
    /// # use corporum::Corpus;
    /// # use std::fs::OpenOptions;
    /// #
    /// let corp = Corpus::new();
    /// // ... do some work ...
    ///
    /// // This will save the corpus into ./some_file.ucf
    /// if let Err(e) = corp.save_stream() {
    ///     eprintln!("Failed to save: {e}");
    /// }
    ///
    /// let mut file = OpenOptions::new().write(true).open("some_file.ucf").unwrap();
    /// match corp.save_into(file) {
    ///     Ok(()) => println!("OK"),
    ///     Err(e) => eprintln!("Failed to save: {e}"),
    /// }
    /// ```
    ///
    /// # Errors
    /// This will return an error if:
    /// - The serialization fails
    /// - Compression fails
    pub fn save_into<W: Write>(&self, dest: W) -> Result<(), CorporeumError> {
        let mut serialized = Vec::new();
        into_writer(self, Cursor::new(&mut serialized))?;

        let mut compressor = ZlibEncoder::new(dest, Compression::best());
        compressor
            .write_all(&serialized)
            .map_err(CorporeumError::CompressionError)?;

        Ok(())
    }

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
