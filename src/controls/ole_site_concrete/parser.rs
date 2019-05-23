use nom::{le_u16, IResult};
use crate::properties::types::string::{
    stream::CountOfBytesWithCompressionFlag,
    parser::parse_string,
};
use crate::properties::types::parser::parse_position;
use crate::common::parser::*;
use super::*;
use super::stream::*;

named!(pub parse_ole_site_concrete_header<u16>,
    do_parse!(
        tag!([0x00, 0x00]) >>
        cb_site: le_u16 >>
        (cb_site)
    )
);

named_args!(parse_cobwcf<'a>(mask: SitePropMask, flag: SitePropMask, offset: &'a mut usize)<CountOfBytesWithCompressionFlag>,
    map!(cond!(
        mask.contains(flag),
        map_opt!(
            call!(aligned_le_u32, offset),
            CountOfBytesWithCompressionFlag::from_bits
        )
    ), |x| x.unwrap_or(CountOfBytesWithCompressionFlag::EMPTY))
);

named_args!(parse_clsid_cache_index<'a>(mask: SitePropMask, flag: SitePropMask, offset: &'a mut usize)<ClsidCacheIndex>,
    map!(cond!(
        mask.contains(flag),
        map_opt!(
            call!(aligned_le_u16, offset),
            ClsidCacheIndex::from_bits
        )
    ), |x| x.unwrap_or(ClsidCacheIndex::INVALID))
);

named_args!(parse_id<'a>(mask: SitePropMask, flag: SitePropMask, offset: &'a mut usize)<i32>,
    map!(cond!(mask.contains(flag), call!(aligned_le_i32, offset)), |x| x.unwrap_or(0x00000000))
);

named_args!(parse_u32<'a>(mask: SitePropMask, flag: SitePropMask, default: u32, offset: &'a mut usize)<u32>,
    map!(cond!(mask.contains(flag), call!(aligned_le_u32, offset)), |x| x.unwrap_or(default))
);

named_args!(parse_u16<'a>(mask: SitePropMask, flag: SitePropMask, default: u16, offset: &'a mut usize)<u16>,
    map!(cond!(mask.contains(flag), call!(aligned_le_u16, offset)), |x| x.unwrap_or(default))
);

named_args!(parse_i16<'a>(mask: SitePropMask, flag: SitePropMask, default: i16, offset: &'a mut usize)<i16>,
    map!(cond!(mask.contains(flag), call!(aligned_le_i16, offset)), |x| x.unwrap_or(default))
);

named_args!(parse_str<'a>(mask: SitePropMask, flag: SitePropMask, length_and_compression: CountOfBytesWithCompressionFlag, offset: &'a mut usize)<String>,
    map!(cond!(
        mask.contains(flag),
        call!(parse_string, length_and_compression)
    ), |x| x.map(|s| {*offset += s.len(); s}).unwrap_or_else(|| "".to_string()))
);

pub fn parse_ole_site_concrete(input: &[u8]) -> IResult<&[u8], OleSiteConcrete> {
    let mut offset: usize = 0;
    do_parse!(input,
        _cb_site: parse_ole_site_concrete_header >>
        mask: map_opt!(call!(aligned_le_u32, &mut offset), SitePropMask::from_bits) >>
        name_data: call!(parse_cobwcf, mask, SitePropMask::NAME, &mut offset) >>
        tag_data: call!(parse_cobwcf, mask, SitePropMask::TAG, &mut offset) >>
        id: call!(parse_id, mask, SitePropMask::ID, &mut offset) >>
        help_context_id: call!(parse_id, mask, SitePropMask::HELP_CONTEXT_ID, &mut offset) >>
        bit_flags: map!(cond!(
            mask.contains(SitePropMask::BIT_FLAGS),
            map_opt!(call!(aligned_le_u32, &mut offset), SiteFlags::from_bits)
        ), |x| x.unwrap_or(SiteFlags::TAB_STOP | SiteFlags::VISIBLE | SiteFlags::STREAMED | SiteFlags::AUTO_SIZE)) >>
        object_stream_size: call!(parse_u32, mask, SitePropMask::OBJECT_STREAM_SIZE, 0x00000000, &mut offset) >>
        tab_index: call!(parse_i16, mask, SitePropMask::TAB_INDEX, -1, &mut offset) >>
        clsid_cache_index: map!(call!(parse_clsid_cache_index, mask, SitePropMask::CLSID_CACHE_INDEX, &mut offset), |x| x.into()) >>
        group_id: call!(parse_u16, mask, SitePropMask::GROUP_ID, 0x0000, &mut offset) >>
        control_tip_text_data: call!(parse_cobwcf, mask, SitePropMask::CONTROL_TIP_TEXT, &mut offset) >>
        runtime_lic_key_data: call!(parse_cobwcf, mask, SitePropMask::RUNTIME_LIC_KEY, &mut offset) >>
        control_source_data: call!(parse_cobwcf, mask, SitePropMask::CONTROL_SOURCE, &mut offset) >>
        row_source_data: call!(parse_cobwcf, mask, SitePropMask::ROW_SOURCE, &mut offset) >>
        call!(align, &mut offset, 4) >>
        name: call!(parse_str, mask, SitePropMask::NAME, name_data, &mut offset) >>
        call!(align, &mut offset, 4) >>
        tag: call!(parse_str, mask, SitePropMask::TAG, tag_data, &mut offset) >>
        call!(align, &mut offset, 4) >>
        site_position: map!(cond!(
            mask.contains(SitePropMask::POSITION),
            map!(parse_position, |x| { offset += 8; x})
        ), |x| x.unwrap_or(Position{top: 0, left: 0})) >>
        call!(align, &mut offset, 4) >>
        control_tip_text: call!(parse_str, mask, SitePropMask::CONTROL_TIP_TEXT, control_tip_text_data, &mut offset) >>
        call!(align, &mut offset, 4) >>
        runtime_lic_key: call!(parse_str, mask, SitePropMask::RUNTIME_LIC_KEY, runtime_lic_key_data, &mut offset) >>
        call!(align, &mut offset, 4) >>
        control_source: call!(parse_str, mask, SitePropMask::CONTROL_SOURCE, control_source_data, &mut offset) >>
        call!(align, &mut offset, 4) >>
        row_source: call!(parse_str, mask, SitePropMask::ROW_SOURCE, row_source_data, &mut offset) >>
        ({
            let s = OleSiteConcrete {
                id, help_context_id, bit_flags, object_stream_size,
                tab_index, clsid_cache_index, group_id,
                name, tag, site_position, control_tip_text, runtime_lic_key,
                control_source, row_source,
            };
            //println!("{:?}", s);
            s
        })
    )
}
