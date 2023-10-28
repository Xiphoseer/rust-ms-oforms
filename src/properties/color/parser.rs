use super::*;
use crate::common::AlignedParser;
use nom::combinator::map_res;
use nom::error::{FromExternalError, ParseError};
use nom::number::complete::le_u32;
use nom::IResult;

pub fn parse_ole_color<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], OleColor, E>
where
    E: FromExternalError<&'a [u8], u32>,
{
    map_res(le_u32, OleColor::try_from)(input)
}

/// Trait to parse a color
pub trait AlignedColorParser: AlignedParser {
    fn ole_color<'a, E>(&self, input: &'a [u8]) -> IResult<&'a [u8], OleColor, E>
    where
        E: ParseError<&'a [u8]>,
        E: FromExternalError<&'a [u8], u32>,
    {
        let (input, _) = self.align(input, 4)?;
        let (input, x) = parse_ole_color(input)?;
        self.inc(4);
        Ok((input, x))
    }
}

// Default implementation
impl<T> AlignedColorParser for T where T: AlignedParser {}
