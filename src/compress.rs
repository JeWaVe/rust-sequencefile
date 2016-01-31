use std::io::prelude::*;
use std::convert::AsRef;
use flate2::read::{GzDecoder, ZlibDecoder};
use errors::{Error, Result};
use Codec;

pub const DEFAULT_CODEC: &'static str = "org.apache.hadoop.io.compress.DefaultCodec";
pub const GZIP_CODEC: &'static str = "org.apache.hadoop.io.compress.GzipCodec";
pub const SNAPPY_CODEC: &'static str = "org.apache.hadoop.io.compress.SnappyCodec";

pub fn codec(codec: &String) -> Option<Codec> {
    match codec.as_ref() {
        DEFAULT_CODEC => Some(Codec::Default),
        GZIP_CODEC => Some(Codec::Gzip),
        SNAPPY_CODEC => Some(Codec::Snappy),
        _ => None,
    }
}

pub fn decompressor(codec: &Codec, buffer: &[u8]) -> Result<Vec<u8>> {
    match *codec {
        Codec::Default => {
            let mut decoder = ZlibDecoder::new(buffer);

            let mut buf = Vec::new();
            try!(decoder.read_to_end(&mut buf));

            Ok(buf)
        }
        Codec::Gzip => {
            let mut decoder = try!(GzDecoder::new(buffer));

            let mut buf = Vec::new();
            try!(decoder.read_to_end(&mut buf));

            Ok(buf)
        }
        _ => Err(Error::CompressionTypeUnknown(format!("codec not implemented: {:?}", codec))),
    }
}
