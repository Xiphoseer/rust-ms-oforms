use super::{Position, Size};
use nom::{
    error::ParseError,
    number::complete::{le_i32, le_u32},
    IResult,
};

pub fn parse_size<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Size, E>
where
    E: ParseError<&'a [u8]>,
{
    let (input, width) = le_u32(input)?;
    let (input, height) = le_u32(input)?;
    Ok((input, Size { width, height }))
}

pub fn parse_position<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Position, E>
where
    E: ParseError<&'a [u8]>,
{
    let (input, top) = le_i32(input)?;
    let (input, left) = le_i32(input)?;
    Ok((input, Position { top, left }))
}
