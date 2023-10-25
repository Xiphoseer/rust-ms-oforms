use super::{FormFont, GuidAndFont};
use crate::common::{parse_guid, CLSID_DT_DDSFORM_21_FONT_NEW};
use nom::bytes::complete::tag;
use nom::combinator::verify;
use nom::error::ParseError;
use nom::{
    number::complete::{le_u16, le_u32},
    IResult,
};

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
