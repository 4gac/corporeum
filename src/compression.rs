#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    #[cfg(feature = "flate2-compression")]
    /// DEFLATE-based stream compression/decompression
    Flate2,
    #[cfg(feature = "zstd-compression")]
    /// Zstandard
    Zstd,
    #[cfg(feature = "lzma-compression")]
    /// Lempel–Ziv–Markov chain algorithm
    Lzma2,
}
