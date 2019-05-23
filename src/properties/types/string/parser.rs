use super::stream::*;
use encoding::{Encoding, DecoderTrap, all::UTF_16LE};
use std::borrow::Cow;

fn parse_str(bytes: &[u8], compressed: bool) -> Result<String, Cow<'static, str>> {
    if compressed {
        let mut new_bytes: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
        for byte in bytes {
            new_bytes.push(*byte);
            new_bytes.push(0);
        }
        UTF_16LE.decode(&new_bytes, DecoderTrap::Strict)
    } else {
        UTF_16LE.decode(bytes, DecoderTrap::Strict)
    }
}

named_args!(pub parse_string(length_and_compression: CountOfBytesWithCompressionFlag)<String>,
    map_res!(
        take!((length_and_compression & CountOfBytesWithCompressionFlag::COUNT_OF_BYTES).bits()),
        |x| parse_str(x, length_and_compression.contains(CountOfBytesWithCompressionFlag::COMPRESSION_FLAG))
    )
);
