use serde::Deserialize;
use serde::Serialize;
// use serde_derive::Deserialize;
// use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename = "corpus")]
pub struct Corpus {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub(crate) metadata: Option<Metadata>,
    #[serde(rename = "documents")]
    pub(crate) documents: Vec<Document>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) authors: Option<Vec<Author>>,
    // this does not follow semantic versioning, it is just a number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) modified: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sentences: Option<Vec<Sentence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) source: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sentence {
    pub(crate) id: u32,
    // TODO enum - could be either 'source' or 'target'
    pub(crate) lang: String,                // TODO features, labels
    pub(crate) sentence_type: SentenceType, // language identifier
    pub(crate) tokens: Vec<Token>,          // not an Option, because sentence cannot be empty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) translations: Option<Vec<Translation>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub(crate) id: u32,
    // morpho, tags, lema
    pub(crate) text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Translations {
    #[serde(rename = "translation")]
    pub(crate) translations: Vec<Translation>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Translation {
    pub(crate) id: u32,
    pub(crate) lang: String,
    pub(crate) sentence_type: SentenceType,
    pub(crate) tokens: Vec<Token>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SentenceType {
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "translation")]
    Translation,
}
