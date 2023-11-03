use std::io::{Cursor, Read, Write};

use crate::{
    compression::Compression,
    format::Format,
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

    pub fn load<R: Read>(
        source: R,
        format: Format,
        compression: Option<Compression>,
    ) -> Result<Self, CorporeumError> {
        let mut raw = Vec::new();

        if let Some(format) = compression {
            match format {
                #[cfg(feature = "flate2-compression")]
                Compression::Flate2 => {
                    use flate2::read::ZlibDecoder;
                    let mut decompressor = ZlibDecoder::new(source);

                    decompressor
                        .read_to_end(&mut raw)
                        .map_err(CorporeumError::DecompressionError)?;
                }
                #[cfg(feature = "zstd-compression")]
                Compression::Zstd => {
                    zstd::stream::copy_decode(source, &mut raw)
                        .map_err(CorporeumError::DecompressionError)?;
                }
                #[cfg(feature = "lzma-compression")]
                Compression::Lzma2 => {
                    use std::io::{BufReader, Error, ErrorKind};
                    let mut buffered_source = BufReader::new(source);

                    lzma_rs::xz_decompress(&mut buffered_source, &mut raw).map_err(|e| {
                        CorporeumError::DecompressionError(Error::new(
                            ErrorKind::Other,
                            e.to_string(),
                        ))
                    })?;
                }
            }
        }

        match format {
            #[cfg(feature = "cbor-format")]
            Format::Cbor => ciborium::from_reader(raw.as_slice())?,
            #[cfg(feature = "json-format")]
            Format::Json => serde_json::de::from_slice(&raw)?,
            #[cfg(feature = "xml-format")]
            Format::Xml => serde_xml_rs::de::from_reader(raw.as_slice())?,
            #[cfg(feature = "rmp-format")]
            Format::Rmp => rmp_serde::decode::from_slice(&raw)?,
            #[cfg(feature = "bincode-format")]
            Format::Bincode => bincode::deserialize(&raw)?,
        }

        todo!()
    }

    #[allow(unused_assignments)] // Wrong?
    pub fn save_stream(
        &self,
        format: Format,
        compression: Option<Compression>,
    ) -> Result<Box<dyn Read>, CorporeumError> {
        let mut result = Vec::new();

        self.save_into(&mut result, format, compression)?;

        Ok(Box::new(Cursor::new(result)))
    }

    #[cfg_attr(not(feature = "lzma-compression"), allow(unused_mut))]
    pub fn save_into<W: Write>(
        &self,
        mut dest: W,
        format: Format,
        compression: Option<Compression>,
    ) -> Result<(), CorporeumError> {
        let raw = match format {
            #[cfg(feature = "cbor-format")]
            Format::Cbor => {
                let mut serialized = Vec::new();
                ciborium::into_writer(self, &mut serialized)?;
                serialized
            }
            #[cfg(feature = "json-format")]
            Format::Json => serde_json::to_string(self)?.bytes().collect(),
            #[cfg(feature = "xml-format")]
            Format::Xml => serde_xml_rs::to_string(self)?.bytes().collect(),
            #[cfg(feature = "rmp-format")]
            Format::Rmp => rmp_serde::to_vec(self)?,
            #[cfg(feature = "bincode-format")]
            Format::Bincode => bincode::serialize(self)?,
        };

        if let Some(format) = compression {
            match format {
                #[cfg(feature = "flate2-compression")]
                Compression::Flate2 => {
                    use flate2::{write::ZlibEncoder, Compression};

                    let mut compressor = ZlibEncoder::new(dest, Compression::best());
                    compressor.write_all(&raw)?;
                }
                #[cfg(feature = "zstd-compression")]
                Compression::Zstd => {
                    zstd::stream::copy_encode(raw.as_slice(), dest, 22)?;
                }
                #[cfg(feature = "lzma-compression")]
                Compression::Lzma2 => {
                    use lzma_rs::xz_compress;
                    use std::io::BufReader;

                    let mut buffered_source = BufReader::new(raw.as_slice());
                    xz_compress(&mut buffered_source, &mut dest)?;
                }
            }
        }

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
