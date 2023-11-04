use serde::Deserialize;
use serde::Serialize;
use std::marker::PhantomData;

/// Represents a text corpus.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct Corpus {
    pub(crate) metadata: Option<Metadata>,
    pub(crate) documents: Vec<Document>,
}

/// Represents the metadata contents in a text corpus.
///
/// This structure __cannot__ be created by the user, to add
/// metadata to a [`Corpus`](Corpus), use [`add_metadata()`](Corpus::add_metadata) instead.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub(crate) authors: Vec<Author>,
    // this does not follow semantic versioning, it is just a number
    pub(crate) created: Option<i64>,
    pub(crate) description: Option<String>,
    pub(crate) modified: Option<i64>,
    pub(crate) name: String,
    pub(crate) version: u16,
}

/// Represents an author of `Metadata`.
///
/// This structure __cannot__ be created by the user, to add an `Author`
/// to [`Metadata`](Metadata), use [`add_author()`](Metadata::add_author) instead.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Author {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) mail: Option<String>,
}
/// Represents a document.
///
/// This structure __cannot__ be created by the user, to add a `Document`
/// to a [`Corpus`](Corpus), use [`add_doc()`](Corpus::add_doc) instead.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Document {
    pub(crate) description: Option<String>,
    pub(crate) id: u32,
    pub(crate) sentences: Vec<Sentence<Source>>,
    pub(crate) source: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Source;
#[derive(Debug, PartialEq, Eq)]
pub struct Target;

/// Represents a sentence.
///
/// This structure __cannot__ be created by the user, to add a `Sentence`
/// to a [`Document`](Document), use [`create_sentence()`](Document::create_sentence) and
/// [`add_sentence()`](Document::add_sentence) instead.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Sentence<Type> {
    pub(crate) t: PhantomData<Type>,
    pub(crate) id: u32,
    // TODO enum - could be either 'source' or 'target'
    pub(crate) lang: String, // TODO features, labels
    // pub(crate) sentence_type: SentenceType, // language identifier
    pub(crate) tokens: Vec<Token>,
    pub(crate) translations: Vec<Sentence<Target>>,
}

/// Represents a token.
///
/// This structure __cannot__ be created by the user, to add a `Token`
/// to a [`Sentence`](Sentence), use [`create_token()`](Sentence::create_token) and
/// [`add_token()`](Sentence::add_token) instead.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Token {
    pub(crate) id: u32,
    pub(crate) form: String,
    pub(crate) lemma: Option<String>,
    pub(crate) upos: Option<String>,
    pub(crate) xpos: Option<String>,
    pub(crate) feats: Option<String>,
    pub(crate) head: Option<String>,
    pub(crate) deprel: Option<String>,
    pub(crate) deps: Option<String>,
    pub(crate) misc: Option<String>, // this is here only for compatibility reasons with CoNLL-U
}
