use crate::schema::{Document, Sentence, Source};

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
    pub const fn doc_id(&self) -> u32 {
        self.id
    }

    /// Add already tokenized sentence to the document.
    ///
    /// To add this sentence to this document, use [`add_sentence()`](Self::add_sentence).
    ///
    /// # Example
    /// ```no_run
    /// use corporum::new;
    ///
    /// let mut corp = new("...").unwrap();
    /// let mut doc = corp.corpus_mut().create_doc();
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
    /// use corporum::new;
    ///
    /// let mut corp = new("...").unwrap();
    /// let mut doc = corp.corpus_mut().create_doc();
    /// let sentence = doc.create_sentence("en");
    /// let sentence_id = sentence.sentence_id();
    /// doc.add_sentence(sentence);
    ///
    /// // This sentence is empty, so it will *not* be added
    /// assert!(doc.sentence_by_id(sentence_id).is_none());
    /// ```
    pub fn add_sentence(&mut self, sent: Sentence<Source>) {
        if sent.tokens.is_empty() {
            return;
        }
        self.sentences.push(sent);
    }

    pub fn remove_sentence(&mut self, id: usize) {
        self.sentences.remove(id);
    }

    pub fn get_sentences(&self) -> &Vec<Sentence<Source>> {
        &self.sentences
    }

    /// Fetch a sentence by its `id` and return a reference to it if exists.
    ///
    //// # Example
    /// ```no_run
    /// use corporum::new;
    ///
    /// let mut corp = new("...").unwrap();
    /// let mut doc = corp.corpus_mut().create_doc();
    ///
    /// assert!(doc.sentence_by_id(0).is_none());
    /// ```
    pub fn get_sentence(&self, id: u32) -> Option<&Sentence<Source>> {
        self.sentences.iter().find(|&sent| sent.id == id)
    }

    /// Fetch a sentence by its `id` and return a reference to it if exists.
    ///
    //// # Example
    /// ```no_run
    /// use corporum::new;
    ///
    /// let mut corp = new("...").unwrap();
    /// let mut doc = corp.corpus_mut().create_doc();
    ///
    /// assert!(doc.sentence_by_id_mut(0).is_none());
    /// ```
    pub fn sentence_mut(&mut self, id: u32) -> Option<&mut Sentence<Source>> {
        self.sentences.iter_mut().find(|sent| sent.id == id)
    }
}
