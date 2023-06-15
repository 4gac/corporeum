#![allow(
    unused_assignments,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::useless_let_if_seq,
    clippy::module_name_repetitions
)]

pub use corporeum::Corporeum;

mod author;
mod corporeum;
mod corpus;
mod document;
mod error;
mod metadata;
mod schema;
mod sentence;
mod token;
