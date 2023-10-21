use super::stream::*;
use encoding::{all::UTF_16LE, mem::decode_latin1, DecoderTrap, Encoding};
use std::borrow::Cow;

fn parse_str(bytes: &[u8], compressed: bool) -> Result<String, Cow<'static, str>> {
    if compressed {
        // Isomorphic Decode
        Ok(decode_latin1(bytes).into_owned())
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
