use nom::bytes::complete::tag;
use nom::combinator::{map, map_opt, success, verify};
use nom::error::{context, ContextError, FromExternalError, ParseError};
use nom::multi::count;
use nom::number::complete::{le_u16, le_u32, le_u8};
use nom::sequence::preceded;
use nom::IResult;
use num_traits::FromPrimitive;
use uuid::Uuid;

use super::ole_site_concrete::parse_ole_site_concrete;
use super::{
    stream::*, BorderStyle, ClsTableFlags, Cycle, FormControl, FormFlags, FormScrollBarFlags, Site,
    SiteClassInfo, SiteKind,
};
use crate::common::{parse_guid, AlignedParser, VarFlags, VarType, IID_IDISPATCH};
use crate::properties::font::GuidAndFont;
use crate::properties::picture::GuidAndPicture;
use crate::properties::{
    color::OleColor,
    font::parse_guid_and_font,
    string::{parse_string, stream::CountOfBytesWithCompressionFlag},
};
use crate::properties::{
    MousePointer, PictureAlignment, PictureSizeMode, Position, Size, SpecialEffect,
};

pub fn parse_form_control_header<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], u16, E> {
    preceded(tag([0x00, 0x04]), le_u16)(input)
}

pub fn parse_site_class_info_header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], u16, E>
where
    E: ParseError<&'a [u8]>,
{
    preceded(tag([0x00, 0x00]), le_u16)(input)
}

pub fn parse_site_class_info<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], SiteClassInfo, E>
where
    E: ParseError<&'a [u8]>,
{
    let ap = AlignedParser::new();
    let _i = input;

    // Class Header
    let (_i, _cb_class_table) = parse_site_class_info_header(_i)?;

    // Mask
    let (_i, mask) = ap.bitfield32(_i, ClassInfoPropMask::from_bits)?;

    // Class Table Flags
    let (_i, class_table_flags) = if mask.contains(ClassInfoPropMask::CLASS_FLAGS) {
        ap.bitfield16(_i, ClsTableFlags::from_bits)?
    } else {
        (_i, ClsTableFlags::empty())
    };

    // Var Flags
    let (_i, var_flags) = if mask.contains(ClassInfoPropMask::CLASS_FLAGS) {
        ap.bitfield16(_i, VarFlags::from_bits)?
    } else {
        (_i, VarFlags::empty())
    };

    // Count of Methods
    let (_i, count_of_methods) = if mask.contains(ClassInfoPropMask::COUNT_OF_METHODS) {
        ap.le_u32(_i)?
    } else {
        (_i, 0x00000000)
    };

    // DispID Bind
    let (_i, dispid_bind) = if mask.contains(ClassInfoPropMask::DISPID_BIND) {
        ap.le_u32(_i)?
    } else {
        (_i, 0xFFFFFFFF)
    };

    // Get Bind Index
    let (_i, get_bind_index) = if mask.contains(ClassInfoPropMask::GET_BIND_INDEX) {
        ap.le_u16(_i)?
    } else {
        (_i, 0x0000)
    };

    // Put Bind Index
    let (_i, put_bind_index) = if mask.contains(ClassInfoPropMask::PUT_BIND_INDEX) {
        ap.le_u16(_i)?
    } else {
        (_i, 0x0000)
    };

    // Bind Type
    let (_i, bind_type) = if mask.contains(ClassInfoPropMask::BIND_TYPE) {
        ap.bitfield16(_i, VarType::from_bits)?
    } else {
        (_i, VarType::EMPTY)
    };

    // Get Value Index
    let (_i, get_value_index) = if mask.contains(ClassInfoPropMask::GET_VALUE_INDEX) {
        ap.le_u16(_i)?
    } else {
        (_i, 0x0000)
    };

    // Put Value Index
    let (_i, put_value_index) = if mask.contains(ClassInfoPropMask::PUT_VALUE_INDEX) {
        ap.le_u16(_i)?
    } else {
        (_i, 0x0000)
    };

    // Value Type
    let (_i, value_type) = if mask.contains(ClassInfoPropMask::VALUE_TYPE) {
        ap.bitfield16(_i, VarType::from_bits)?
    } else {
        (_i, VarType::EMPTY)
    };

    // DispID Rowset
    let (_i, dispid_rowset) = if mask.contains(ClassInfoPropMask::DISPID_ROWSET) {
        ap.le_u32(_i)?
    } else {
        (_i, 0xFFFFFFFF)
    };

    // Set Rowset
    let (_i, set_rowset) = if mask.contains(ClassInfoPropMask::SET_ROWSET) {
        ap.le_u16(_i)?
    } else {
        (_i, 0x0000)
    };

    let (_i, _o) = ap.align(_i, 4)?;

    // CLS ID
    let (_i, cls_id) = if mask.contains(ClassInfoPropMask::CLS_ID) {
        parse_guid(_i)?
    } else {
        (_i, Uuid::nil())
    };

    // Disp Event
    let (_i, disp_event) = if mask.contains(ClassInfoPropMask::DISP_EVENT) {
        parse_guid(_i)?
    } else {
        (_i, IID_IDISPATCH)
    };

    // Default Proc
    let (_i, default_proc) = if mask.contains(ClassInfoPropMask::DEFAULT_PROC) {
        parse_guid(_i)?
    } else {
        (_i, IID_IDISPATCH)
    };

    Ok((
        _i,
        SiteClassInfo {
            class_table_flags,
            var_flags,
            count_of_methods,
            dispid_bind,
            get_bind_index,
            put_bind_index,
            bind_type,
            get_value_index,
            put_value_index,
            value_type,
            dispid_rowset,
            set_rowset,
            cls_id,
            disp_event,
            default_proc,
        },
    ))
}

impl AlignedParser {
    fn parse_form_object_depth_type_count<'a, E>(
        &self,
        input: &'a [u8],
    ) -> IResult<&'a [u8], (u8, SiteType, u32), E>
    where
        E: ParseError<&'a [u8]>,
    {
        let _i = input;

        // Depth
        let (_i, depth) = le_u8(_i)?;

        // Value
        let (_i, value) = map_opt(le_u8, TypeOrCount::from_bits)(_i)?;

        let type_or_count = (value & TypeOrCount::TYPE_OR_COUNT).bits();
        let (_i, res) = if value.contains(TypeOrCount::IS_COUNT) {
            let (_i, r#type) = map_opt(le_u8, SiteType::from_u8)(_i)?;
            self.inc(3);
            let count = u32::from(type_or_count);
            (_i, (depth, r#type, count))
        } else {
            let (_i, r#type) = map_opt(success(type_or_count), SiteType::from_u8)(_i)?;
            self.inc(2);
            (_i, (depth, r#type, 1))
        };
        Ok((_i, res))
    }
}

pub fn parse_site_depths_and_types<'a, E>(
    count_of_sites: u32,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<SiteDepthAndType>, E>
where
    E: ParseError<&'a [u8]>,
{
    move |input: &'a [u8]| {
        let ap = AlignedParser::new();

        let mut site_count: u32 = 0;
        let ucount = count_of_sites as usize;
        let mut data = input;
        let mut result = Vec::with_capacity(ucount);
        while site_count < count_of_sites {
            let (rest, (depth, r#type, count)) = ap.parse_form_object_depth_type_count(data)?;
            site_count += count;
            let depth_and_type = SiteDepthAndType { depth, r#type };
            for _i in 0..count {
                result.push(depth_and_type);
            }
            data = rest;
        }
        let (rest, _) = ap.align(data, 4)?;
        Ok((rest, result))
    }
}

pub fn parse_sites<'a, E>(
    site_depths_and_types: Vec<SiteDepthAndType>,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<Site>, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    move |input: &'a [u8]| {
        let mut result = Vec::with_capacity(site_depths_and_types.len());
        let mut data = input;
        for site_depth_and_type in &site_depths_and_types {
            let (rest, site) = match site_depth_and_type.r#type {
                SiteType::Ole => map(context("ole_site_concrete", parse_ole_site_concrete), |x| {
                    Site {
                        kind: SiteKind::Ole(x),
                        depth: site_depth_and_type.depth,
                    }
                })(data)?,
            };
            result.push(site);
            data = rest;
        }
        Ok((data, result))
    }
}

pub fn parse_form_control<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], FormControl, E>
where
    E: ParseError<&'a [u8]>,
    E: FromExternalError<&'a [u8], u32>,
    E: ContextError<&'a [u8]>,
{
    let ap = AlignedParser::new();
    let _i = input;

    // Form Control Header
    let (_i, _cb_form) = context("form_control_header", parse_form_control_header)(_i)?;

    // Mask
    let (_i, mask) = ap.bitfield32(_i, FormPropMask::from_bits)?;

    // Picture Tiling
    let picture_tiling = mask.contains(FormPropMask::PICTURE_TILING);

    // Back Color
    let (_i, back_color) = if mask.contains(FormPropMask::BACK_COLOR) {
        ap.ole_color(_i)?
    } else {
        (_i, OleColor::BTNFACE)
    };

    // Fore color
    let (_i, fore_color) = if mask.contains(FormPropMask::FORE_COLOR) {
        ap.ole_color(_i)?
    } else {
        (_i, OleColor::BTNTEXT)
    };

    // Next available ID
    let (_i, next_available_id) = if mask.contains(FormPropMask::NEXT_AVAILABLE_ID) {
        ap.le_u32(_i)?
    } else {
        (_i, 0x00000000)
    };

    // Boolean Properties
    let (_i, boolean_properties) = if mask.contains(FormPropMask::BOOLEAN_PROPERTIES) {
        ap.bitfield32(_i, FormFlags::from_bits)?
    } else {
        (_i, FormFlags::ENABLED)
    };

    // Border style
    let (_i, border_style) = if mask.contains(FormPropMask::BORDER_STYLE) {
        ap.bitfield8(_i, BorderStyle::from_u8)?
    } else {
        (_i, BorderStyle::None)
    };

    // Mouse Pointer
    let (_i, mouse_pointer) = if mask.contains(FormPropMask::MOUSE_POINTER) {
        ap.bitfield8(_i, MousePointer::from_u8)?
    } else {
        (_i, MousePointer::Default)
    };

    // Scroll Bars
    let (_i, scroll_bars) = if mask.contains(FormPropMask::SCROLL_BARS) {
        ap.bitfield8(_i, FormScrollBarFlags::from_bits)?
    } else {
        (_i, FormScrollBarFlags::DEFAULT)
    };

    // Group Count
    let (_i, group_count) = if mask.contains(FormPropMask::GROUP_CNT) {
        ap.le_u32(_i)?
    } else {
        (_i, 0x00000000)
    };

    // Mouse Icon
    let (_i, _mouse_icon) = if mask.contains(FormPropMask::MOUSE_ICON) {
        verify(|i| ap.le_u16(i), |x| *x == 0xFFFF)(_i)?
    } else {
        (_i, 0)
    };

    // Cycle
    let (_i, cycle) = if mask.contains(FormPropMask::CYCLE) {
        ap.bitfield8(_i, Cycle::from_u8)?
    } else {
        (_i, Cycle::AllForms)
    };

    // Special Effect
    let (_i, special_effect) = if mask.contains(FormPropMask::SPECIAL_EFFECT) {
        ap.bitfield8(_i, SpecialEffect::from_u8)?
    } else {
        (_i, SpecialEffect::Flat)
    };

    // Border Color
    let (_i, border_color) = if mask.contains(FormPropMask::BORDER_COLOR) {
        ap.ole_color(_i)?
    } else {
        (_i, OleColor::BTNTEXT)
    };

    // Caption (length and compression)
    let (_i, caption_length) = if mask.contains(FormPropMask::CAPTION) {
        ap.bitfield32(_i, CountOfBytesWithCompressionFlag::from_bits)?
    } else {
        (_i, CountOfBytesWithCompressionFlag::EMPTY)
    };

    // Mouse Icon
    let (_i, _font) = if mask.contains(FormPropMask::FONT) {
        verify(|i| ap.le_u16(i), |x| *x == 0xFFFF)(_i)
    } else {
        Ok((_i, 0))
    }?;

    // Picture
    let (_i, _picture) = if mask.contains(FormPropMask::PICTURE) {
        verify(|i| ap.le_u16(i), |x| *x == 0xFFFF)(_i)
    } else {
        Ok((_i, 0))
    }?;

    // Zoom
    let (_i, zoom) = if mask.contains(FormPropMask::ZOOM) {
        ap.le_u32(_i)?
    } else {
        (_i, 100u32)
    };

    // Picture Alignment
    let (_i, picture_alignment) = if mask.contains(FormPropMask::PICTURE_ALIGNMENT) {
        ap.bitfield8(_i, PictureAlignment::from_u8)?
    } else {
        (_i, PictureAlignment::Center)
    };

    // Picture Size Mode
    let (_i, picture_size_mode) = if mask.contains(FormPropMask::PICTURE_SIZE_MODE) {
        ap.bitfield8(_i, PictureSizeMode::from_u8)?
    } else {
        (_i, PictureSizeMode::Clip)
    };

    // Shape Cookie
    let (_i, shape_cookie) = if mask.contains(FormPropMask::SHAPE_COOKIE) {
        ap.le_u32(_i)?
    } else {
        (_i, 0u32)
    };

    // Draw Buffer
    let (_i, draw_buffer) = if mask.contains(FormPropMask::DRAW_BUFFER) {
        ap.le_u32(_i)?
    } else {
        (_i, 0u32)
    };

    let (_i, _o) = ap.align(_i, 4)?;

    // Displayed Size
    let (_i, displayed_size) = if mask.contains(FormPropMask::DISPLAYED_SIZE) {
        let (_ir, displayed_size) = Size::parse(_i)?;
        ap.inc(8);
        (_ir, displayed_size)
    } else {
        (_i, Size::new(4000, 3000))
    };

    // Logical Size
    let (_i, logical_size) = if mask.contains(FormPropMask::LOGICAL_SIZE) {
        let (_ir, logical_size) = Size::parse(_i)?;
        ap.inc(8);
        (_ir, logical_size)
    } else {
        (_i, Size::new(4000, 3000))
    };

    // Scroll Position
    let (_i, scroll_position) = if mask.contains(FormPropMask::SCROLL_POSITION) {
        let (_ir, scroll_position) = Position::parse(_i)?;
        ap.inc(8);
        (_ir, scroll_position)
    } else {
        (_i, Position::default())
    };

    // Caption
    let (_i, caption) = if mask.contains(FormPropMask::CAPTION) {
        context("caption", parse_string(caption_length))(_i)?
    } else {
        (_i, String::from(""))
    };

    // Picture
    // TODO

    // Mouse Icon
    let (_i, mouse_icon) = (_i, GuidAndPicture::EMPTY);

    // Font
    let (_i, font) = if mask.contains(FormPropMask::FONT) {
        context("font", parse_guid_and_font)(_i)?
    } else {
        (_i, GuidAndFont::EMPTY)
    };

    // Picture
    // TODO

    let (_i, picture) = (_i, GuidAndPicture::EMPTY);

    // Size Class Info (count)
    let (_i, count_of_site_class_info) =
        if boolean_properties.contains(FormFlags::DONTSAVECLASSTABLE) {
            (_i, 0)
        } else {
            let (_ir, x) = le_u16(_i)?;
            (_ir, x as usize)
        };

    let (_i, site_classes) = context(
        "site_classes",
        count(parse_site_class_info, count_of_site_class_info),
    )(_i)?;

    // TODO: DesignEx?
    let (_i, count_of_sites) = le_u32(_i)?;
    let (_i, _count_of_bytes) = le_u32(_i)?;

    let (_i, site_depths_and_types) = context(
        "site_depths_and_types",
        parse_site_depths_and_types(count_of_sites),
    )(_i)?;
    let (_i, sites) = context("sites", parse_sites(site_depths_and_types))(_i)?;

    Ok((
        _i,
        FormControl {
            back_color,
            fore_color,
            next_available_id,
            boolean_properties,
            border_style,
            mouse_pointer,
            scroll_bars,
            group_count,
            cycle,
            special_effect,
            border_color,
            zoom,
            draw_buffer,
            picture_alignment,
            picture_size_mode,
            shape_cookie,
            displayed_size,
            logical_size,
            scroll_position,
            caption,
            mouse_icon,
            font,
            picture,
            picture_tiling,
            sites,
            site_classes,
        },
    ))
}
