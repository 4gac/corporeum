pub mod corpus;

mod corporeum;
mod document;
mod metadata;
pub(crate) mod sentence;
mod token;
pub(crate) mod translation;
pub use corporeum::Corporeum;

mod schema;
pub use crate::schema::Corpus;
pub use crate::schema::Metadata;
