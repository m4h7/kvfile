#![deny(missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]
pub mod reader;
pub mod writer;
pub mod sreader;
pub mod swriter;
pub mod colwriter;
pub mod colreader;
pub mod twriter;
pub mod treader;
mod test;
