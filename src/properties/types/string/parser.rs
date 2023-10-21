use super::stream::*;
use encoding_rs::{UTF_16LE, mem::decode_latin1};
use std::borrow::Cow;

fn parse_str(bytes: &[u8], compressed: bool) -> String {
    if compressed {
        // Isomorphic Decode
        decode_latin1(bytes)
    } else {
        UTF_16LE.decode(bytes).0
    }.into_owned()
}

named_args!(pub parse_string(length_and_compression: CountOfBytesWithCompressionFlag)<String>,
    map!(
        take!((length_and_compression & CountOfBytesWithCompressionFlag::COUNT_OF_BYTES).bits()),
        |x| parse_str(x, length_and_compression.contains(CountOfBytesWithCompressionFlag::COMPRESSION_FLAG))
    )
);
