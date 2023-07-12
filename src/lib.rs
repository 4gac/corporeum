#![allow(clippy::must_use_candidate, clippy::module_name_repetitions)]
//
//! A library for working with text corpora.
//
pub use error::CorporeumError;
pub use schema::{Author, Corpus, Document, Metadata, Sentence, Token};

mod author;
mod corpus;
mod document;
mod error;
mod metadata;
mod schema;
mod sentence;
mod token;
