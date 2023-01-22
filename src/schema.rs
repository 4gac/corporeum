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
    pub(crate) documents: Documents,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<Name>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub(crate) authors: Option<Vec<Author>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<Description>,
    #[serde(rename = "version")]
    pub(crate) version: u16, // this does not follow semantic versioning, it is just a number
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub(crate) created: Option<String>,
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub(crate) modified: Option<String>,
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

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Documents {
    #[serde(rename = "document")]
    pub(crate) docs: Option<Vec<Document>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Document {
    pub(crate) id: u32,
    pub(crate) source: Option<String>,
    pub(crate) description: Option<Description>,
    #[serde(rename = "sentences")]
    pub(crate) sentences: Option<Vec<Sentence>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sentences {
    #[serde(rename = "sentence")]
    pub(crate) sents: Option<Vec<Sentence>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sentence {
    pub(crate) id: u32,
    // TODO features, labels
    pub(crate) sentence_type: SentenceType, // TODO enum - could be either 'source' or 'target'
    pub(crate) lang: String,                // language identifier
    #[serde(rename = "tokens")]
    pub(crate) tokens: Tokens, // not an Option, because sentence cannot be empty
    pub(crate) translations: Option<Vec<Translation>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tokens {
    #[serde(rename = "token")]
    pub(crate) token: Vec<Token>, // sentence cannot be empty
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
    pub(crate) sentence_type: SentenceType,
    pub(crate) lang: String,
    pub(crate) tokens: Tokens,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SentenceType {
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "translation")]
    Translation,
}
