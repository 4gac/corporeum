use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, Default)]
pub struct Corpus {
    pub(crate) metadata: Option<Metadata>,
    pub(crate) documents: Vec<Document>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub(crate) authors: Vec<Author>,
    pub(crate) created: Option<i64>,
    pub(crate) description: Option<String>,
    pub(crate) modified: Option<i64>,
    pub(crate) name: String,
    pub(crate) version: u16,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Author {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) mail: Option<String>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Document {
    pub(crate) description: Option<String>,
    pub(crate) id: u32,
    pub(crate) sentences: Vec<Sentence>,
    pub(crate) source: Option<String>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]
pub struct Sentence {
    pub(crate) id: u32,
    pub(crate) lang: String, // TODO features, labels
    pub(crate) tokens: Vec<Token>,
    #[omit_bounds]
    pub(crate) translations: Vec<Sentence>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
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
    pub(crate) misc: Option<String>, // this is here only for compatibilty reasons with CoNLL-U
}
