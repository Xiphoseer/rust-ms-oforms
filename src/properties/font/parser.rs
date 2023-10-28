use std::num::NonZeroU32;

use super::{FontFlags, FormFont, GuidAndFont, StdFont};
use crate::common::{parse_guid, CLSID_DT_DDSFORM_21_FONT_NEW};
use nom::bytes::complete::tag;
use nom::combinator::{map, map_opt, verify};
use nom::error::ParseError;
use nom::multi::length_data;
use nom::number::complete::{le_i16, le_u8};
use nom::{
    number::complete::{le_u16, le_u32},
    IResult,
};

pub fn parse_std_font<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], StdFont, E>
where
    E: ParseError<&'a [u8]>,
{
    let (input, _version) = tag([1u8])(input)?;
    let (input, charset) = le_i16(input)?;
    let (input, flags) = map_opt(le_u8, FontFlags::from_bits)(input)?;
    let (input, weight) = le_i16(input)?;
    let (input, height) = map_opt(le_u32, NonZeroU32::new)(input)?;
    let (input, font_face) = map(
        verify(length_data(verify(le_u8, |x| *x < 32)), <[u8]>::is_ascii),
        |b: &[u8]| unsafe { String::from_utf8_unchecked(b.to_vec()) },
    )(input)?;
    Ok((
        input,
        StdFont {
            charset,
            flags,
            weight,
            height,
            font_face,
        },
    ))
}

pub fn parse_guid_and_font<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], GuidAndFont, E>
where
    E: ParseError<&'a [u8]>,
{
    let (input, guid) = parse_guid(input)?;
    match guid {
        CLSID_DT_DDSFORM_21_FONT_NEW => {
            let (input, _v_min) = tag([0x00])(input)?;
            let (input, _v_maj) = tag([0x00])(input)?;
            let (input, _cb_count) = verify(le_u16, |x: &u16| *x == 8)(input)?;
            let (input, d1) = le_u32(input)?;
            let (input, d2) = le_u32(input)?;
            Ok((
                input,
                GuidAndFont {
                    guid: CLSID_DT_DDSFORM_21_FONT_NEW,
                    font: FormFont::DdsForm21FontNew(d1, d2),
                },
            ))
        }
        _ => unimplemented!("{}", guid),
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::{
        super::{FontFlags, StdFont},
        parse_std_font,
    };

    #[test]
    fn test_parse_std_font() {
        let bytes = [
            0x01, 0x00, 0x00, 0x00, 0x90, 0x01, 0x44, 0x42, 0x01, 0x00, 0x06, 0x54, 0x61, 0x68,
            0x6f, 0x6d, 0x61,
        ];
        assert_eq!(
            parse_std_font::<nom::error::Error<_>>(&bytes),
            Ok((
                &[][..],
                StdFont {
                    charset: 0,
                    flags: FontFlags::empty(),
                    weight: 0x190,
                    height: NonZeroU32::new(0x14244).unwrap(),
                    font_face: String::from("Tahoma"),
                }
            ))
        )
    }
}
