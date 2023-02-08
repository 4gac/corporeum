use serde::Deserialize;
use serde::Serialize;
// use serde_derive::Deserialize;
// use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename = "corpus")]
pub struct Corpus {
    #[serde(rename = "documents")]
    pub(crate) documents: Vec<Document>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub(crate) metadata: Option<Metadata>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub(crate) authors: Option<Vec<Author>>,
    // this does not follow semantic versioning, it is just a number
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub(crate) created: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<Description>,
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub(crate) modified: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<Name>,
    #[serde(rename = "version")]
    pub(crate) version: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Name {
    pub(crate) text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub(crate) text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Description {
    pub(crate) text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Document {
    pub(crate) description: Option<Description>,
    pub(crate) id: u32,
    #[serde(rename = "sentences")]
    pub(crate) sentences: Option<Vec<Sentence>>,
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
