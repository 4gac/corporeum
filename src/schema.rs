use serde::Deserialize;
use serde::Serialize;
use std::marker::PhantomData;

/// Represents a text corpus.
///
/// This structure __cannot__ be created by the user!
/// To create it, you must create a [`Corporeum`](crate::corporeum::Corporeum)
/// first (using either [`new()`](crate::new) or [`load()`](crate::load)).
/// Then call [`corpus()`](crate::corporeum::Corporeum::corpus) (or [`corpus_mut()`](crate::corporeum::Corporeum::corpus_mut)).
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename = "corpus")]
pub struct Corpus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) documents: Vec<Document>,
}

/// Represents the metadata contents in a text corpus.
///
/// This structure __cannot__ be created by the user, to add
/// metadata to a [`Corpus`](Corpus), use [`add_metadata()`](Corpus::add_metadata) instead.
#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) authors: Vec<Author>,
    // this does not follow semantic versioning, it is just a number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) modified: Option<i64>,
    pub(crate) name: String,
    #[serde(rename = "version")]
    pub(crate) version: u16,
}

/// Represents an author of `Metadata`.
///
/// This structure __cannot__ be created by the user, to add an `Author`
/// to [`Metadata`](Metadata), use [`add_author()`](Metadata::add_author) instead.
#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mail: Option<String>,
}
/// Represents a document.
///
/// This structure __cannot__ be created by the user, to add a `Document`
/// to a [`Corpus`](Corpus), use [`add_doc()`](Corpus::add_doc) instead.
#[derive(Deserialize, Serialize, Debug)]
pub struct Document {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    pub(crate) id: u32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) sentences: Vec<Sentence<Source>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) source: Option<String>,
}

#[derive(Debug)]
pub struct Source;
#[derive(Debug)]
pub struct Target;

/// Represents a sentence.
///
/// This structure __cannot__ be created by the user, to add a `Sentence`
/// to a [`Document`](Document), use [`create_sentence()`](Document::create_sentence) and
/// [`add_sentence()`](Document::add_sentence) instead.
#[derive(Deserialize, Serialize, Debug)]
pub struct Sentence<Type> {
    pub(crate) t: PhantomData<Type>,
    pub(crate) id: u32,
    // TODO enum - could be either 'source' or 'target'
    pub(crate) lang: String, // TODO features, labels
    // pub(crate) sentence_type: SentenceType, // language identifier
    pub(crate) tokens: Vec<Token>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) translations: Vec<Sentence<Target>>,
}

/// Represents a token.
///
/// This structure __cannot__ be created by the user, to add a `Token`
/// to a [`Sentence`](Sentence), use [`create_token()`](Sentence::create_token) and
/// [`add_token()`](Sentence::add_token) instead.
#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub(crate) id: u32,
    pub(crate) form: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) lemma: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) upos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) xpos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) feats: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) head: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) deprel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) deps: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) misc: Option<String>, // this is here only for compatibility reasons with CoNLL-U
}
