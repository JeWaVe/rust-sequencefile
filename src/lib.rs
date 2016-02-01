//! Prototype streaming library for reading Hadoop sequencefiles
//!
//! # Example
//! ```ignore
//! let path = Path::new("/path/to/seqfile");
//! let file = File::open(&path).unwrap();
//!
//! let seqfile = match sequencefile::Reader::new(file) {
//!   Ok(val) => val,
//!   Err(err) => panic!("Failed to open sequence file: {}", err),
//! };
//!
//! for kv in seqfile {
//!     println!("{:?}", kv);
//! }
//! ```

#![crate_name = "sequencefile"]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces)]

extern crate byteorder;
extern crate bzip2;
extern crate flate2;

use std::collections::HashMap;

/// Converts a Result<T,E> to an Option<T>
macro_rules! to_opt {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(_) => return None,
    });
}

/// Convenience typedef
pub type ByteString = Vec<u8>;

/// Sequencefile header, metadata about the file, e.g. key/value types, version, compression
/// and some internal state for properly decoding
#[derive(Debug)]
pub struct Header {
    version: u16,
    compression_type: CompressionType,
    compression_codec: Option<Codec>,
    key_class: String,
    value_class: String,
    metadata: HashMap<String, String>,
    sync_marker: ByteString,
}

// modules
mod compress;
mod errors;

pub mod reader;

// exports
pub use reader::*;
pub use compress::{Codec, CompressionType};
