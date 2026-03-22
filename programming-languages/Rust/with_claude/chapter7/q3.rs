// src/lib.rs
mod catalog;
mod io;

pub use catalog::GlueCatalog;
pub use io::S3FileIO;

// src/catalog.rs
pub struct GlueCatalog {
    pub name: String,
}

// src/io.rs
pub struct S3FileIO {
    pub bucket: String,
}

