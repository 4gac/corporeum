use crate::{
    schema::{Document, Sentence, Source},
    CorporeumError,
};

impl Document {
    pub(crate) const fn new(id: u32) -> Self {
        Self {
            id,
            source: None,
            description: None,
            sentences: Vec::new(),
        }
    }

    /// Return the ID of this document.
    pub const fn get_doc_id(&self) -> u32 {
        self.id
    }

    /// Creates a new `Sentence` with its language set to the specified language.
    ///
    /// To add this sentence to this document, use [`add_sentence()`](Self::add_sentence).
    ///
    /// # Example
    /// ```no_run
    /// use corporum::Corpus;
    ///
    /// let mut corp = Corpus::new();
    /// let mut doc = corp.create_doc();
    ///
    /// doc.create_sentence("en");
    /// ```
    pub fn create_sentence(&mut self, lang: &str) -> Sentence<Source> {
        Sentence::<Source>::new(self.sentences.last().map_or(0, |s| s.id + 1), lang)
    }

    /// Add a [`Sentence`](crate::Sentence) to the document.
    ///
    /// To create a new [`Sentence`](crate::Sentence), use [`create_sentence()`](Self::create_sentence).
    ///
    /// # Example
    /// ```no_run
    /// use corporum::Corpus;
    ///
    /// let mut corp = Corpus::new();
    /// let mut doc = corp.create_doc();
    /// let sentence = doc.create_sentence("en");
    /// let sentence_id = sentence.get_sentence_id();
    /// doc.add_sentence(sentence).unwrap_err();
    ///
    /// // This sentence is empty, so it will *not* be added
    /// assert!(doc.get_sentence(sentence_id).is_none());
    /// ```
    ///
    /// # Errors
    /// This will return an error if the specified sentence has no tokens in it.
    pub fn add_sentence(&mut self, sent: Sentence<Source>) -> Result<(), CorporeumError> {
        if sent.tokens.is_empty() {
            return Err(CorporeumError::EmptyObject(
                "Sentence has no tokens in it".to_owned(),
            ));
        }
        self.sentences.push(sent);
        Ok(())
    }

    /// Removes a sentence from this `Document` by its ID.
    ///
    /// # Example
    /// ```no_run
    /// use corporum::Corpus;
    /// use std::fs::File;
    ///
    /// let src = File::open("some_file.ucf").unwrap();
    /// let mut corp = Corpus::load(src).unwrap();
    /// let mut doc = corp.get_doc_mut(0).unwrap();
    /// doc.remove_sentence(0).unwrap();
    /// ```
    ///
    /// # Errors
    /// This will return an error if the specified sentence does not exist.
    pub fn remove_sentence(&mut self, id: usize) -> Result<(), CorporeumError> {
        if id >= self.sentences.len() {
            return Err(CorporeumError::ElementNotFound(format!(
                "Sentence with ID {id} does not exist"
            )));
        }

        self.sentences.remove(id);
        Ok(())
    }

    /// Get a reference to a vector containing all sentences in this `Document`.
    ///
    /// # Example
    /// ```no_run
    /// use corporum::Corpus;
    /// use std::fs::File;
    ///
    /// let src = File::open("some_file.ucf").unwrap();
    /// let mut corp = Corpus::load(src).unwrap();
    /// let mut doc = corp.get_doc_mut(0).unwrap();
    ///
    /// for sentence in doc.get_sentences() {
    ///     println!("{sentence:#?}");
    /// }
    /// ```
    pub const fn get_sentences(&self) -> &Vec<Sentence<Source>> {
        &self.sentences
    }

    /// Fetch a sentence by its `id` and return a reference to it if exists.
    ///
    //// # Example
    /// ```no_run
    /// use corporum::Corpus;
    ///
    /// let mut corp = Corpus::new();
    /// let mut doc = corp.create_doc();
    ///
    /// assert!(doc.get_sentence(0).is_none());
    /// ```
    pub fn get_sentence(&self, id: u32) -> Option<&Sentence<Source>> {
        self.sentences.iter().find(|&sent| sent.id == id)
    }

    /// Fetch a sentence by its `id` and return a reference to it if exists.
    ///
    //// # Example
    /// ```no_run
    /// use corporum::Corpus;
    ///
    /// let mut corp = Corpus::new();
    /// let mut doc = corp.create_doc();
    ///
    /// assert!(doc.get_sentence(0).is_none());
    /// ```
    pub fn get_sentence_mut(&mut self, id: u32) -> Option<&mut Sentence<Source>> {
        self.sentences.iter_mut().find(|sent| sent.id == id)
    }
}
