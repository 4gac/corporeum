/// A corpus file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    #[cfg(feature = "cbor-format")]
    /// Concise Binary Object Representation
    Cbor,
    #[cfg(feature = "json-format")]
    /// JavaScript Object Notation
    Json,
    #[cfg(feature = "xml-format")]
    /// Extensible Markup Language
    Xml,
    #[cfg(feature = "rmp-format")]
    /// Rust MessagePack
    Rmp,
    #[cfg(feature = "bincode-format")]
    /// Bincode
    Bincode,
}
