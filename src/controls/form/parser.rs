use nom::{le_u8, le_u16, le_u32, IResult};
use super::*;
use super::stream::*;
use crate::properties::types::{
    color::{OleColor, parser::aligned_ole_color},
    font::parser::parse_guid_and_font,
    parser::{parse_size, parse_position},
    string::{stream::CountOfBytesWithCompressionFlag, parser::parse_string},
};
use crate::controls::ole_site_concrete::parser::parse_ole_site_concrete;
use crate::common::{VarFlags, VarType, parser::*};
use std::convert::TryFrom;

named!(pub parse_form_control_header<u16>,
    do_parse!(
        tag!([0x00, 0x04]) >>
        byte_count: le_u16 >>
        (byte_count)
    )
);

named!(pub parse_site_class_info_header<u16>,
    do_parse!(
        tag!([0x00, 0x00]) >>
        byte_count: le_u16 >>
        (byte_count)
    )
);

pub fn parse_site_class_info(input: &[u8]) -> IResult<&[u8], ClassTable> {
    let mut offset: usize = 0;
    do_parse!(input,
        _cb_class_table: parse_site_class_info_header >>
        mask: map_opt!(
            call!(aligned_le_u32, &mut offset),
            ClassInfoPropMask::from_bits
        ) >>
        class_table_flags: map!(cond!(
            mask.contains(ClassInfoPropMask::CLASS_FLAGS),
            map_opt!(call!(aligned_le_u16, &mut offset), ClsTableFlags::from_bits)
        ), |x| x.unwrap_or(ClsTableFlags::empty())) >>
        var_flags: map!(cond!(
            mask.contains(ClassInfoPropMask::CLASS_FLAGS),
            map_opt!(call!(aligned_le_u16, &mut offset), VarFlags::from_bits)
        ), |x| x.unwrap_or(VarFlags::empty())) >>
        count_of_methods: map!(cond!(
            mask.contains(ClassInfoPropMask::COUNT_OF_METHODS),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(0x00000000)) >>
        dispid_bind: map!(cond!(
            mask.contains(ClassInfoPropMask::DISPID_BIND),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(0xFFFFFFFF)) >>
        get_bind_index: map!(cond!(
            mask.contains(ClassInfoPropMask::GET_BIND_INDEX),
            call!(aligned_le_u16, &mut offset)
        ), |x| x.unwrap_or(0x0000)) >>
        put_bind_index: map!(cond!(
            mask.contains(ClassInfoPropMask::PUT_BIND_INDEX),
            call!(aligned_le_u16, &mut offset)
        ), |x| x.unwrap_or(0x0000)) >>
        bind_type: map!(cond!(
            mask.contains(ClassInfoPropMask::BIND_TYPE),
            map_opt!(
                call!(aligned_le_u16, &mut offset),
                VarType::from_u16
            )
        ), |x| x.unwrap_or(VarType::Empty)) >>
        get_value_index: map!(cond!(
            mask.contains(ClassInfoPropMask::GET_VALUE_INDEX),
            call!(aligned_le_u16, &mut offset)
        ), |x| x.unwrap_or(0x0000)) >>
        put_value_index: map!(cond!(
            mask.contains(ClassInfoPropMask::PUT_VALUE_INDEX),
            call!(aligned_le_u16, &mut offset)
        ), |x| x.unwrap_or(0x0000)) >>
        value_type: map!(cond!(
            mask.contains(ClassInfoPropMask::VALUE_TYPE),
            map_opt!(
                call!(aligned_le_u16, &mut offset),
                VarType::from_u16
            )
        ), |x| x.unwrap_or(VarType::Empty)) >>
        dispid_rowset: map!(cond!(
            mask.contains(ClassInfoPropMask::DISPID_ROWSET),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(0xFFFFFFFF)) >>
        set_rowset: map!(cond!(
            mask.contains(ClassInfoPropMask::SET_ROWSET),
            call!(aligned_le_u16, &mut offset)
        ), |x| x.unwrap_or(0x0000)) >>
        call!(align, &mut offset, 4) >>
        cls_id: map!(cond!(
            mask.contains(ClassInfoPropMask::CLS_ID),
            parse_guid
        ), |x| x.unwrap_or(GUID::EMPTY)) >>
        disp_event: map!(cond!(
            mask.contains(ClassInfoPropMask::DISP_EVENT),
            parse_guid
        ), |x| x.unwrap_or(GUID::DEFAULT)) >>
        default_proc: map!(cond!(
            mask.contains(ClassInfoPropMask::DEFAULT_PROC),
            parse_guid
        ), |x| x.unwrap_or(GUID::DEFAULT)) >>
        (ClassTable{
            class_table_flags, var_flags, count_of_methods,
            dispid_bind,
            get_bind_index, put_bind_index, bind_type,
            get_value_index, put_value_index, value_type,
            dispid_rowset, set_rowset,
            cls_id, disp_event, default_proc,
        })
    )
}

named_args!(pub parse_form_object_depth_type_count<'a>(offset: &'a mut usize)<(u8, SiteType, u32)>,
    do_parse!(
        depth: le_u8 >>
        value: map_opt!(le_u8, TypeOrCount::from_bits) >>
        type_or_count: value!((value & TypeOrCount::TYPE_OR_COUNT).bits()) >>
        is_count: value!(value.contains(TypeOrCount::IS_COUNT)) >>
        res: switch!(value!(is_count),
            true => do_parse!(
                r#type: map_opt!(le_u8, |x| { *offset += 3; SiteType::from_u8(x) }) >>
                count: value!(u32::from(type_or_count)) >>
                ((depth, r#type, count))
            ) |
            false => do_parse!(
                r#type: map_opt!(value!(type_or_count), |x| { *offset += 2; SiteType::from_u8(x) }) >>
                ((depth, r#type, 1))
            )
        ) >>
        (res)
    )
);

pub fn parse_site_depths_and_types(input: &[u8], count_of_sites: u32) -> IResult<&[u8], Vec<SiteDepthAndType>> {
    let mut site_count: u32 = 0;
    let mut offset: usize = 0;
    let (input, ucount) = map_res!(input, value!(site_count), usize::try_from)?;
    let mut data = input;
    let mut result = Vec::with_capacity(ucount);
    while site_count < count_of_sites {
        let (rest, (depth, r#type, count)) = parse_form_object_depth_type_count(data, &mut offset)?;
        site_count += count;
        let depth_and_type = SiteDepthAndType{depth, r#type};
        for _i in 0..count {
            result.push(depth_and_type);
        }
        data = rest;
    }
    let (rest, _) = align(data, &mut offset, 4)?;
    Ok((rest, result))
}

pub fn parse_sites(input: &[u8], site_depths_and_types: Vec<SiteDepthAndType>)
-> IResult<&[u8], Vec<Site>> {
    let mut result = Vec::with_capacity(site_depths_and_types.len());
    let mut data = input;
    for site_depth_and_type in site_depths_and_types {
        let (rest, site) = match site_depth_and_type.r#type {
            SiteType::Ole => parse_ole_site_concrete(data).map(|(r,x)| (r, Site::Ole(x)))?
        };
        result.push(site);
        data = rest;
    }
    return Ok((data, result));
}


pub fn parse_form_control(input: &[u8]) -> IResult<&[u8], FormControl> {
    let mut offset: usize = 0;
    do_parse!(input,
        _cb_form: parse_form_control_header >>
        mask: map_opt!(
            call!(aligned_le_u32, &mut offset),
            FormPropMask::from_bits
        ) >>
        picture_tiling: value!(mask.contains(FormPropMask::PICTURE_TILING)) >>
        back_color: map!(cond!(
            mask.contains(FormPropMask::BACK_COLOR),
            call!(aligned_ole_color, &mut offset)
        ), |x| x.unwrap_or(OleColor::BTNFACE)) >>
        fore_color: map!(cond!(
            mask.contains(FormPropMask::FORE_COLOR),
            call!(aligned_ole_color, &mut offset)
        ), |x| x.unwrap_or(OleColor::BTNTEXT)) >>
        next_available_id: map!(cond!(
            mask.contains(FormPropMask::NEXT_AVAILABLE_ID),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(0x00000000)) >>
        boolean_properties: map!(cond!(
            mask.contains(FormPropMask::BOOLEAN_PROPERTIES),
            map_opt!(call!(aligned_le_u32, &mut offset), FormFlags::from_bits)
        ), |x| x.unwrap_or(FormFlags::ENABLED)) >>
        border_style: map!(cond!(
            mask.contains(FormPropMask::BORDER_STYLE),
            map_opt!(call!(aligned_le_u8, &mut offset), BorderStyle::from_u8)
        ), |x| x.unwrap_or(BorderStyle::None)) >>
        mouse_pointer: map!(cond!(
            mask.contains(FormPropMask::MOUSE_POINTER),
            map_opt!(call!(aligned_le_u8, &mut offset), MousePointer::from_u8)
        ), |x| x.unwrap_or(MousePointer::Default)) >>
        scroll_bars: map!(cond!(
            mask.contains(FormPropMask::SCROLL_BARS),
            map_opt!(call!(aligned_le_u8, &mut offset), FormScrollBarFlags::from_bits)
        ), |x| x.unwrap_or(FormScrollBarFlags::DEFAULT)) >>
        group_count: map!(cond!(
            mask.contains(FormPropMask::GROUP_CNT),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(0x00000000)) >>
        _mouse_icon: map!(cond!(
            mask.contains(FormPropMask::MOUSE_ICON),
            verify!(call!(aligned_le_u16, &mut offset), |x: u16| x == 0xFFFF)
        ), |x| x.unwrap_or(0)) >>
        cycle: map!(cond!(
            mask.contains(FormPropMask::CYCLE),
            map_opt!(call!(aligned_le_u8, &mut offset), Cycle::from_u8)
        ), |x| { x.unwrap_or(Cycle::AllForms)}) >>
        special_effect: map!(cond!(
            mask.contains(FormPropMask::SPECIAL_EFFECT),
            map_opt!(call!(aligned_le_u8, &mut offset), SpecialEffect::from_u8)
        ), |x| x.unwrap_or(SpecialEffect::Flat)) >>
        border_color: map!(cond!(
            mask.contains(FormPropMask::BORDER_COLOR),
            call!(aligned_ole_color, &mut offset)
        ), |x| x.unwrap_or(OleColor::BTNTEXT)) >>
        length_and_compression: map!(cond!(
            mask.contains(FormPropMask::CAPTION),
            map_opt!(
                call!(aligned_le_u32, &mut offset),
                CountOfBytesWithCompressionFlag::from_bits
            )
        ), |x| x.unwrap_or(CountOfBytesWithCompressionFlag::EMPTY)) >>
        _font: map!(cond!(
            mask.contains(FormPropMask::FONT),
            verify!(call!(aligned_le_u16, &mut offset), |x: u16| x == 0xFFFF)
        ), |x| x.unwrap_or(0)) >>
        _picture: map!(cond!(
            mask.contains(FormPropMask::PICTURE),
            verify!(call!(aligned_le_u16, &mut offset), |x: u16| x == 0xFFFF)
        ), |x| x.unwrap_or(0)) >>
        zoom: map!(cond!(
            mask.contains(FormPropMask::ZOOM),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(100 as u32)) >>
        picture_alignment: map!(cond!(
            mask.contains(FormPropMask::PICTURE_ALIGNMENT),
            map_opt!(
                call!(aligned_le_u8, &mut offset),
                PictureAlignment::from_u8
            )
        ), |x| x.unwrap_or(PictureAlignment::Center)) >>
        picture_size_mode: map!(cond!(
            mask.contains(FormPropMask::PICTURE_SIZE_MODE),
            map_opt!(
                call!(aligned_le_u8, &mut offset),
                PictureSizeMode::from_u8
            )
        ), |x| x.unwrap_or(PictureSizeMode::Clip)) >>
        shape_cookie: map!(cond!(
            mask.contains(FormPropMask::SHAPE_COOKIE),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(0 as u32)) >>
        draw_buffer: map!(cond!(
            mask.contains(FormPropMask::DRAW_BUFFER),
            call!(aligned_le_u32, &mut offset)
        ), |x| x.unwrap_or(0 as u32)) >>
        call!(align, &mut offset, 4) >>
        displayed_size: map!(cond!(
            mask.contains(FormPropMask::DISPLAYED_SIZE),
            map!(parse_size, |x| { offset += 8; x})
        ), |x| x.unwrap_or(Size{width: 4000, height: 3000})) >>
        logical_size: map!(cond!(
            mask.contains(FormPropMask::LOGICAL_SIZE),
            map!(parse_size, |x| { offset += 8; x})
        ), |x| x.unwrap_or(Size{width: 4000, height: 3000})) >>
        scroll_position: map!(cond!(
            mask.contains(FormPropMask::SCROLL_POSITION),
            map!(parse_position, |x| { offset += 8; x})
        ), |x| x.unwrap_or(Position{top: 0, left: 0})) >>
        caption: map!(cond!(
            mask.contains(FormPropMask::CAPTION),
            call!(parse_string, length_and_compression)
        ), |x| x.unwrap_or_else(|| "".to_string())) >>
        // TODO: Picture
        mouse_icon: value!(GuidAndPicture::EMPTY) >>
        font: map!(cond!(
            mask.contains(FormPropMask::FONT),
            parse_guid_and_font
        ), |x| x.unwrap_or(GuidAndFont::EMPTY)) >>
        // TODO: Picture
        picture: value!(GuidAndPicture::EMPTY) >>
        count_of_site_class_info: map!(cond!(
            !boolean_properties.contains(FormFlags::DONTSAVECLASSTABLE),
            map_res!(le_u16, usize::try_from)
        ), |x| x.unwrap_or(0)) >>
        site_classes: count!(parse_site_class_info, count_of_site_class_info) >>
        // TODO: DesignEx?
        count_of_sites: le_u32 >>
        _count_of_bytes: le_u32 >>
        site_depths_and_types: call!(parse_site_depths_and_types, count_of_sites) >>
        sites: call!(parse_sites, site_depths_and_types) >>
        // map!(value!(0), |_| println!("{:#?}", _sites)) >>
        (FormControl{
            back_color, fore_color, next_available_id, boolean_properties,
            border_style, mouse_pointer, scroll_bars, group_count,
            cycle, special_effect, border_color, zoom, draw_buffer,
            picture_alignment, picture_size_mode, shape_cookie,
            displayed_size, logical_size, scroll_position, caption,
            mouse_icon, font, picture, picture_tiling,
            sites, site_classes,
        })
    )
}
