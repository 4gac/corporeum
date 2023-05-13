use corporum::{Compression, Corporeum};
use std::path::Path;

fn main() {
    let corp = Corporeum::new(Path::new("hello.corp"));
    corp.save(false).unwrap();
    corp.save_as(&"hello.corp.xz", false, Compression::Lzma)
        .unwrap();
    corp.save_as(&"hello.corp.gz", false, Compression::Deflate)
        .unwrap();
}
