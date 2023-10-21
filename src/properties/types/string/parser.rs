use std::borrow::Cow;

use super::stream::*;
use encoding_rs::{mem::decode_latin1, UTF_16LE};
use nom::{bytes::complete::take, combinator::map, error::ParseError, IResult};

fn decode_utf16_le(bytes: &[u8]) -> Cow<'_, str> {
    UTF_16LE.decode(bytes).0
}

pub fn parse_string<'a, E: ParseError<&'a [u8]>>(
    length_and_compression: CountOfBytesWithCompressionFlag,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], String, E> {
    map(
        map(
            take(length_and_compression.len()),
            match length_and_compression.compressed() {
                true => decode_latin1, // Isomorphic Decode
                false => decode_utf16_le,
            },
        ),
        Cow::into_owned,
    )
}
