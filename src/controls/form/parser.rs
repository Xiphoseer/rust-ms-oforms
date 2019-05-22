use nom::{le_u8, le_u16, le_u32, IResult};
use super::*;
use super::stream::*;
use crate::properties::types::{
    color::{OleColor, parser::parse_ole_color},
    parser::{parse_size, parse_position},
};
use encoding::{Encoding, DecoderTrap, all::UTF_16LE};
use std::borrow::Cow;

named_args!(align<'a>(offset: &'a mut usize, count: usize)<usize>,
    do_parse!(
        p1: value!(*offset % count) >>
        p2: value!(if p1 == 0 { 0 } else { count - p1 }) >>
        take!(p2) >>
        ({*offset += p2; *offset})
    )
);

named_args!(aligned_le_u32<'a>(offset: &'a mut usize)<u32>,
    do_parse!(
        call!(align, offset, 4) >>
        val: map!(le_u32, |x| { *offset += 4; x }) >>
        (val)
    )
);

named_args!(aligned_le_u16<'a>(offset: &'a mut usize)<u16>,
    do_parse!(
        call!(align, offset, 2) >>
        val: map!(le_u16, |x| { *offset += 2; x }) >>
        (val)
    )
);

named_args!(aligned_le_u8<'a>(offset: &'a mut usize)<u8>,
    map!(le_u8, |x| { *offset += 1; x })
);

named_args!(aligned_ole_color<'a>(offset: &'a mut usize)<OleColor>,
    do_parse!(
        call!(align, offset, 4) >>
        val: map!(parse_ole_color, |x| { *offset += 4; x }) >>
        (val)
    )
);

named!(pub parse_form_control_header<u16>,
    do_parse!(
        tag!([0x00, 0x04]) >>
        byte_count: le_u16 >>
        (byte_count)
    )
);

fn parse_str(bytes: &[u8], compressed: bool) -> Result<String, Cow<'static, str>> {
    if compressed {
        let mut new_bytes: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
        for byte in bytes {
            new_bytes.push(*byte);
            new_bytes.push(0);
        }
        UTF_16LE.decode(&new_bytes, DecoderTrap::Strict)
    } else {
        UTF_16LE.decode(bytes, DecoderTrap::Strict)
    }
}

named_args!(parse_string(length_and_compression: CountOfBytesWithCompressionFlag)<String>,
    map_res!(
        take!((length_and_compression & CountOfBytesWithCompressionFlag::COUNT_OF_BYTES).bits()),
        |x| parse_str(x, length_and_compression.contains(CountOfBytesWithCompressionFlag::COMPRESSION_FLAG))
    )
);

pub fn parse_form_control(input: &[u8]) -> IResult<&[u8], FormControl> {
    let mut offset: usize = 0;
    do_parse!(input,
        _cb_form: parse_form_control_header >>
        mask: dbg!(map_opt!(
            call!(aligned_le_u32, &mut offset),
            FormPropMask::from_bits
        )) >>
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
        ), |x| { offset += 1; x.unwrap_or(BorderStyle::None)}) >>
        mouse_pointer: map!(cond!(
            mask.contains(FormPropMask::MOUSE_POINTER),
            map_opt!(call!(aligned_le_u8, &mut offset), MousePointer::from_u8)
        ), |x| { offset += 1; x.unwrap_or(MousePointer::Default)}) >>
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
        ), |x| { offset += 2; x.unwrap_or(0)}) >>
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
            parse_size
        ), |x| x.unwrap_or(Size{width: 4000, height: 3000})) >>
        logical_size: map!(cond!(
            mask.contains(FormPropMask::LOGICAL_SIZE),
            parse_size
        ), |x| x.unwrap_or(Size{width: 4000, height: 3000})) >>
        scroll_position: map!(cond!(
            mask.contains(FormPropMask::SCROLL_POSITION),
            parse_position
        ), |x| x.unwrap_or(Position{top: 0, left: 0})) >>
        caption: map!(cond!(
            mask.contains(FormPropMask::CAPTION),
            call!(parse_string, length_and_compression)
        ), |x| x.unwrap_or_else(|| "".to_string())) >>
        mouse_icon: value!(GuidAndPicture::EMPTY) >>
        font: value!(GuidAndFont::EMPTY) >>
        picture: value!(GuidAndPicture::EMPTY) >>
        (FormControl{
            back_color, fore_color, next_available_id, boolean_properties,
            border_style, mouse_pointer, scroll_bars, group_count,
            cycle, special_effect, border_color, zoom, draw_buffer,
            picture_alignment, picture_size_mode, shape_cookie,
            displayed_size, logical_size, scroll_position, caption,
            mouse_icon, font, picture, picture_tiling,
        })
    )
}

/*
p1: value!(self.offset % 4) >>
p2: value!(if p1 == 0 { 0 } else { 4 - p1 }) >>
take!(p1) >>
*/

/*
displayed_size: cond!(mask.contains(FormPropMask::J), tuple!(le_u32, le_u32)) >>
logical_size: cond!(mask.contains(FormPropMask::K), tuple!(le_u32, le_u32)) >>
scroll_position: cond!(mask.contains(FormPropMask::L), tuple!(le_i32, le_i32)) >>
caption: value!(None) >> // TODO implement caption
*/
