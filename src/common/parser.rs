use super::GUID;
use nom::number::complete::{le_i16, le_i32, le_u16, le_u32, le_u64, le_u8};
use nom::IResult;
use nom_methods::call_m;

use std::cell::Cell;

named_args!(pub check_guid(guid: GUID)<GUID>,
    verify!(parse_guid, |x| x == &guid)
);

pub fn parse_guid(input: &[u8]) -> IResult<&[u8], GUID> {
    let (input, d1) = le_u32(input)?;
    let (input, d2) = le_u16(input)?;
    let (input, d3) = le_u16(input)?;
    let (input, d4) = le_u64(input)?;
    Ok((input, GUID(d1, d2, d3, d4)))
}

pub trait AlignedParser {
    fn align<'a>(&self, input: &'a [u8], count: usize) -> IResult<&'a [u8], usize>;
    fn inc(&self, by: usize);

    fn le_u32<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], u32> {
        let (_i, _o) = self.align(input, 4)?;
        let (_i, x) = le_u32(_i)?;
        self.inc(4);
        Ok((_i, x))
    }

    fn le_i32<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], i32> {
        let (_i, _o) = self.align(input, 4)?;
        let (_i, x) = le_i32(_i)?;
        self.inc(4);
        Ok((_i, x))
    }

    fn le_u16<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], u16> {
        let (_i, _o) = self.align(input, 2)?;
        let (_i, x) = le_u16(_i)?;
        self.inc(2);
        Ok((_i, x))
    }

    fn le_i16<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], i16> {
        let (_i, _o) = self.align(input, 2)?;
        let (_i, x) = le_i16(_i)?;
        self.inc(2);
        Ok((_i, x))
    }

    fn le_u8<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], u8> {
        let (_i, x) = le_u8(input)?;
        self.inc(1);
        Ok((_i, x))
    }

    /// Read an u32 bitfield
    fn bitfield32<'a, F, C>(&self, input: &'a [u8], func: F) -> IResult<&'a [u8], C>
    where
        F: Fn(u32) -> Option<C>,
    {
        map_opt!(input, call_m!(self.le_u32), func)
    }

    /// Read an u16 bitfield
    fn bitfield16<'a, F, C>(&self, input: &'a [u8], func: F) -> IResult<&'a [u8], C>
    where
        F: Fn(u16) -> Option<C>,
    {
        map_opt!(input, call_m!(self.le_u16), func)
    }

    /// Read an u8 bitfield
    fn bitfield8<'a, F, C>(&self, input: &'a [u8], func: F) -> IResult<&'a [u8], C>
    where
        F: Fn(u8) -> Option<C>,
    {
        map_opt!(input, call_m!(self.le_u8), func)
    }
}

impl AlignedParser for Cell<usize> {
    fn align<'a>(&self, input: &'a [u8], align: usize) -> IResult<&'a [u8], usize> {
        let p0 = self.get();
        let p1 = p0 % align;
        let p2 = if p1 == 0 { 0 } else { align - p1 };
        let (rest, _pad) = take!(input, p2)?;
        let p3 = p0 + p2;
        self.set(p3);
        Ok((rest, p3))
    }

    fn inc(&self, by: usize) {
        let offset = self.get();
        self.set(offset + by);
    }
}
