use crate::common::{/*GUID, */parser::*};
use super::*;
use nom::{le_u16, le_u32};

named!(pub parse_guid_and_font<GuidAndFont>,
    switch!(parse_guid,
        GUID::WTF_FONT => do_parse!(
            _v_min: tag!([0x00]) >>
            _v_maj: tag!([0x00]) >>
            _cb_count: verify!(le_u16, |x| x == 8) >>
            d1: le_u32 >>
            d2: le_u32 >>
            (GuidAndFont{guid: GUID::WTF_FONT, font: FormFont::Unknown1(d1, d2)})
        ) | _ => value!(GuidAndFont::EMPTY)
    )
);
