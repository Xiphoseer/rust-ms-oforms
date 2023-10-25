use super::{ClipboardFormat, CompObj, CompObjHeader};
use nom::bytes::complete::take;
use nom::combinator::{map, map_opt, map_res, value, verify};
use nom::error::{FromExternalError, ParseError};
use nom::multi::length_data;
use nom::number::complete::{le_i16, le_i32, le_u16, le_u32, le_u8};
use nom::IResult;
use uuid::Uuid;

use std::cell::Cell;
use std::ffi::{CStr, FromBytesWithNulError};

pub fn check_guid<'a>(guid: Uuid) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Uuid> {
    verify(parse_guid, move |x| x == &guid)
}

pub fn parse_guid<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Uuid, E>
where
    E: ParseError<&'a [u8]>,
{
    let (input, d1) = le_u32(input)?;
    let (input, d2) = le_u16(input)?;
    let (input, d3) = le_u16(input)?;
    let mut d4 = [0u8; 8];
    let (input, _d4) = take(8usize)(input)?;
    d4.copy_from_slice(_d4);
    Ok((
        input,
        uuid::Builder::from_fields(d1, d2, d3, &d4).into_uuid(),
    ))
}

pub trait AlignedParser {
    fn align<'a, E: ParseError<&'a [u8]>>(
        &self,
        input: &'a [u8],
        count: usize,
    ) -> IResult<&'a [u8], usize, E>;
    fn inc(&self, by: usize);

    fn le_u32<'a, E: ParseError<&'a [u8]>>(&self, input: &'a [u8]) -> IResult<&'a [u8], u32, E> {
        let (_i, _o) = self.align(input, 4)?;
        let (_i, x) = le_u32(_i)?;
        self.inc(4);
        Ok((_i, x))
    }

    fn le_i32<'a, E: ParseError<&'a [u8]>>(&self, input: &'a [u8]) -> IResult<&'a [u8], i32, E> {
        let (_i, _o) = self.align(input, 4)?;
        let (_i, x) = le_i32(_i)?;
        self.inc(4);
        Ok((_i, x))
    }

    fn le_u16<'a, E: ParseError<&'a [u8]>>(&self, input: &'a [u8]) -> IResult<&'a [u8], u16, E> {
        let (_i, _o) = self.align(input, 2)?;
        let (_i, x) = le_u16(_i)?;
        self.inc(2);
        Ok((_i, x))
    }

    fn le_i16<'a, E: ParseError<&'a [u8]>>(&self, input: &'a [u8]) -> IResult<&'a [u8], i16, E> {
        let (_i, _o) = self.align(input, 2)?;
        let (_i, x) = le_i16(_i)?;
        self.inc(2);
        Ok((_i, x))
    }

    fn le_u8<'a, E: ParseError<&'a [u8]>>(&self, input: &'a [u8]) -> IResult<&'a [u8], u8, E> {
        let (_i, x) = le_u8(input)?;
        self.inc(1);
        Ok((_i, x))
    }

    /// Read an u32 bitfield
    fn bitfield32<'a, F, C, E: ParseError<&'a [u8]>>(
        &self,
        input: &'a [u8],
        func: F,
    ) -> IResult<&'a [u8], C, E>
    where
        F: Fn(u32) -> Option<C>,
    {
        map_opt(move |i| self.le_u32(i), func)(input)
    }

    /// Read an u16 bitfield
    fn bitfield16<'a, F, C, E: ParseError<&'a [u8]>>(
        &self,
        input: &'a [u8],
        func: F,
    ) -> IResult<&'a [u8], C, E>
    where
        F: Fn(u16) -> Option<C>,
    {
        map_opt(move |i| self.le_u16(i), func)(input)
    }

    /// Read an u8 bitfield
    fn bitfield8<'a, F, C, E: ParseError<&'a [u8]>>(
        &self,
        input: &'a [u8],
        func: F,
    ) -> IResult<&'a [u8], C, E>
    where
        F: Fn(u8) -> Option<C>,
    {
        map_opt(move |i| self.le_u8(i), func)(input)
    }
}

impl AlignedParser for Cell<usize> {
    fn align<'a, E: ParseError<&'a [u8]>>(
        &self,
        input: &'a [u8],
        align: usize,
    ) -> IResult<&'a [u8], usize, E> {
        let p0 = self.get();
        let p1 = p0 % align;
        let p2 = if p1 == 0 { 0 } else { align - p1 };
        let (rest, _pad) = take(p2)(input)?;
        let p3 = p0 + p2;
        self.set(p3);
        Ok((rest, p3))
    }

    fn inc(&self, by: usize) {
        let offset = self.get();
        self.set(offset + by);
    }
}

fn parse_comp_obj_header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], CompObjHeader, E>
where
    E: ParseError<&'a [u8]>,
{
    value(CompObjHeader, take(28usize))(input)
}

fn parse_length_prefixed_ansi_string<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], &'a CStr, E>
where
    E: ParseError<&'a [u8]>,
    E: FromExternalError<&'a [u8], FromBytesWithNulError>,
{
    map_res(length_data(le_u32), CStr::from_bytes_with_nul)(input)
}

fn parse_ansi_clipboard_format<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], ClipboardFormat, E>
where
    E: ParseError<&'a [u8]>,
    E: FromExternalError<&'a [u8], FromBytesWithNulError>,
{
    let (input, marker_or_length) = le_u32(input)?;
    match marker_or_length {
        0x00000000 => Ok((input, ClipboardFormat::None)),
        0xFFFFFFFE | 0xFFFFFFFF => map(le_u32, ClipboardFormat::Standard)(input),
        len => map(
            map_res(take(len), CStr::from_bytes_with_nul),
            ClipboardFormat::custom,
        )(input),
    }
}

pub fn parse_comp_obj<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], CompObj, E>
where
    E: ParseError<&'a [u8]>,
    E: FromExternalError<&'a [u8], FromBytesWithNulError>,
{
    let (input, _header) = parse_comp_obj_header(input)?;
    let (input, ansi_user_type) = map(parse_length_prefixed_ansi_string, CStr::to_owned)(input)?;
    let (input, ansi_clipboard_format) = parse_ansi_clipboard_format(input)?;
    Ok((
        input,
        CompObj {
            ansi_user_type,
            ansi_clipboard_format,
        },
    ))
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use crate::common::{
        parse_comp_obj,
        parser::{
            parse_ansi_clipboard_format, parse_comp_obj_header, parse_length_prefixed_ansi_string,
        },
        ClipboardFormat, CompObj, CompObjHeader,
    };

    const DATA: &[u8] = include_bytes!("comp_obj.bin");

    #[test]
    fn test() {
        let user_type = CStr::from_bytes_with_nul(b"Microsoft DDS Form 2.0\0").unwrap();
        let fmt = CStr::from_bytes_with_nul(b"Embedded Object\0").unwrap();
        assert_eq!(
            parse_comp_obj_header::<nom::error::Error<_>>(DATA).ok(),
            Some((&DATA[28..], CompObjHeader)),
        );
        assert_eq!(
            parse_length_prefixed_ansi_string::<nom::error::Error<_>>(&DATA[28..]).ok(),
            Some((&DATA[55..], user_type)),
        );
        assert_eq!(
            parse_ansi_clipboard_format::<nom::error::Error<_>>(&DATA[55..]).ok(),
            Some((&DATA[75..], ClipboardFormat::Custom(fmt.to_owned()))),
        );

        assert_eq!(
            parse_comp_obj::<nom::error::Error<_>>(DATA).ok(),
            Some((
                &DATA[75..],
                CompObj {
                    ansi_user_type: user_type.to_owned(),
                    ansi_clipboard_format: ClipboardFormat::Custom(fmt.to_owned()),
                }
            ))
        )
    }
}
