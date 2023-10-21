use super::{Position, Size};
use nom::{
    number::complete::{le_i32, le_u32},
    IResult,
};

pub fn parse_size(input: &[u8]) -> IResult<&[u8], Size> {
    let (input, width) = le_u32(input)?;
    let (input, height) = le_u32(input)?;
    Ok((input, Size { width, height }))
}

pub fn parse_position(input: &[u8]) -> IResult<&[u8], Position> {
    let (input, top) = le_i32(input)?;
    let (input, left) = le_i32(input)?;
    Ok((input, Position { top, left }))
}
