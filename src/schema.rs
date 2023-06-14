use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename = "corpus")]
pub struct Corpus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) documents: Vec<Document>,
}

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mail: Option<String>,
}

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Sentence<Type> {
    pub(crate) _t: std::marker::PhantomData<Type>,
    pub(crate) id: u32,
    // TODO enum - could be either 'source' or 'target'
    pub(crate) lang: String, // TODO features, labels
    // pub(crate) sentence_type: SentenceType, // language identifier
    pub(crate) tokens: Vec<Token>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) translations: Vec<Sentence<Target>>,
}

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
    pub(crate) misc: Option<String>, // this is here only for compatibilty reasons with CoNLL-U
}
