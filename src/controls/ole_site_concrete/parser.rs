use super::stream::*;
use super::*;
use crate::common::AlignedParser;
use crate::properties::types::string::{parse_string, stream::CountOfBytesWithCompressionFlag};
use nom::bytes::complete::tag;
use nom::combinator::{map, map_opt};
use nom::error::ParseError;
use nom::number::complete::le_u16;
use nom::sequence::preceded;
use nom::IResult;

use std::cell::Cell;

pub fn parse_ole_site_concrete_header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], u16, E>
where
    E: ParseError<&'a [u8]>,
{
    preceded(tag([0x00, 0x00]), le_u16)(input)
}

pub trait AlignedOleSiteParser: AlignedParser {
    fn parse_cobwcf<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
    ) -> IResult<&'a [u8], CountOfBytesWithCompressionFlag, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            map_opt(
                |i| self.le_u32(i),
                CountOfBytesWithCompressionFlag::from_bits,
            )(input)
        } else {
            Ok((input, CountOfBytesWithCompressionFlag::EMPTY))
        }
    }

    fn parse_clsid_cache_index<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
    ) -> IResult<&'a [u8], ClsidCacheIndex, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            let (input, bits) = self.le_u16(input)?;
            match ClsidCacheIndex::from_bits(bits) {
                Some(x) => Ok((input, x)),
                None => Ok((input, ClsidCacheIndex::INVALID)),
            }
        } else {
            Ok((input, ClsidCacheIndex::INVALID))
        }
    }

    fn parse_id<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
    ) -> IResult<&'a [u8], i32, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            self.le_i32(input)
        } else {
            Ok((input, 0x00000000))
        }
    }

    fn parse_u32<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
        default: u32,
    ) -> IResult<&'a [u8], u32, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            self.le_u32(input)
        } else {
            Ok((input, default))
        }
    }

    fn parse_u16<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
        default: u16,
    ) -> IResult<&'a [u8], u16, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            self.le_u16(input)
        } else {
            Ok((input, default))
        }
    }

    fn parse_i16<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
        default: i16,
    ) -> IResult<&'a [u8], i16, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            self.le_i16(input)
        } else {
            Ok((input, default))
        }
    }

    fn parse_str<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
        length_and_compression: CountOfBytesWithCompressionFlag,
    ) -> IResult<&'a [u8], String, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            let (input, s) = parse_string(length_and_compression)(input)?;
            self.inc(s.len());
            Ok((input, s))
        } else {
            Ok((input, String::from("")))
        }
    }

    fn parse_position<'a, E>(
        &self,
        input: &'a [u8],
        mask: SitePropMask,
        flag: SitePropMask,
    ) -> IResult<&'a [u8], Position, E>
    where
        E: ParseError<&'a [u8]>,
    {
        if mask.contains(flag) {
            let (input, top) = self.le_i32(input)?;
            let (input, left) = self.le_i32(input)?;
            Ok((input, Position { top, left }))
        } else {
            Ok((input, Position { top: 0, left: 0 }))
        }
    }
}

impl<T> AlignedOleSiteParser for T where T: AlignedParser {}

pub fn parse_ole_site_concrete<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], OleSiteConcrete, E>
where
    E: ParseError<&'a [u8]>,
{
    let ap = Cell::new(0);
    let _i = input;

    // Header
    let (_i, _cb_site) = parse_ole_site_concrete_header(_i)?;

    // Mask
    let (_i, mask) = map_opt(|i| ap.le_u32(i), SitePropMask::from_bits)(_i)?;

    // Name Data
    let (_i, name_data) = ap.parse_cobwcf(_i, mask, SitePropMask::NAME)?;

    // Tag Data
    let (_i, tag_data) = ap.parse_cobwcf(_i, mask, SitePropMask::TAG)?;
    // ID
    let (_i, id) = ap.parse_id(_i, mask, SitePropMask::ID)?;
    // Help Context ID
    let (_i, help_context_id) = ap.parse_id(_i, mask, SitePropMask::HELP_CONTEXT_ID)?;

    // Bit Flags
    let (_i, bit_flags) = if mask.contains(SitePropMask::BIT_FLAGS) {
        map_opt(|i| ap.le_u32(i), SiteFlags::from_bits)(_i)?
    } else {
        (
            _i,
            SiteFlags::TAB_STOP | SiteFlags::VISIBLE | SiteFlags::STREAMED | SiteFlags::AUTO_SIZE,
        )
    };

    // Object Stream Size
    let (_i, object_stream_size) =
        ap.parse_u32(_i, mask, SitePropMask::OBJECT_STREAM_SIZE, 0x00000000)?;
    // Tab Index
    let (_i, tab_index) = ap.parse_i16(_i, mask, SitePropMask::TAB_INDEX, -1)?;
    // CLSID Cache Index
    let (_i, clsid_cache_index) = map(
        |i| ap.parse_clsid_cache_index(i, mask, SitePropMask::CLSID_CACHE_INDEX),
        Clsid::from,
    )(_i)?;

    // Group ID
    let (_i, group_id) = ap.parse_u16(_i, mask, SitePropMask::GROUP_ID, 0x0000)?;

    let (_i, control_tip_text_data) = ap.parse_cobwcf(_i, mask, SitePropMask::CONTROL_TIP_TEXT)?;
    let (_i, runtime_lic_key_data) = ap.parse_cobwcf(_i, mask, SitePropMask::RUNTIME_LIC_KEY)?;
    let (_i, control_source_data) = ap.parse_cobwcf(_i, mask, SitePropMask::CONTROL_SOURCE)?;
    let (_i, row_source_data) = ap.parse_cobwcf(_i, mask, SitePropMask::ROW_SOURCE)?;

    ap.align(_i, 4)?;
    let (_i, name) = ap.parse_str(_i, mask, SitePropMask::NAME, name_data)?;

    ap.align(_i, 4)?;
    let (_i, tag) = ap.parse_str(_i, mask, SitePropMask::TAG, tag_data)?;

    ap.align(_i, 4)?;
    let (_i, site_position) = ap.parse_position(_i, mask, SitePropMask::POSITION)?;

    ap.align(_i, 4)?;
    let (_i, control_tip_text) = ap.parse_str(
        _i,
        mask,
        SitePropMask::CONTROL_TIP_TEXT,
        control_tip_text_data,
    )?;

    ap.align(_i, 4)?;
    let (_i, runtime_lic_key) = ap.parse_str(
        _i,
        mask,
        SitePropMask::RUNTIME_LIC_KEY,
        runtime_lic_key_data,
    )?;

    ap.align(_i, 4)?;
    let (_i, control_source) =
        ap.parse_str(_i, mask, SitePropMask::CONTROL_SOURCE, control_source_data)?;

    ap.align(_i, 4)?;
    let (_i, row_source) = ap.parse_str(_i, mask, SitePropMask::ROW_SOURCE, row_source_data)?;

    Ok((
        _i,
        OleSiteConcrete {
            id,
            help_context_id,
            bit_flags,
            object_stream_size,
            tab_index,
            clsid_cache_index,
            group_id,
            name,
            tag,
            site_position,
            control_tip_text,
            runtime_lic_key,
            control_source,
            row_source,
        },
    ))
}
