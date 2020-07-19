use nom::error::{ErrorKind, ParseError};
use nom::number::complete::{le_u16, le_u32, le_u8};
use nom::IResult;
use nom_methods::call_m;

use super::stream::*;
use super::*;
use crate::common::{parser::*, VarFlags, VarType};
use crate::controls::ole_site_concrete::parser::parse_ole_site_concrete;
use crate::properties::types::{
    color::{parser::AlignedColorParser, OleColor},
    font::parser::parse_guid_and_font,
    parser::{parse_position, parse_size},
    string::{parser::parse_string, stream::CountOfBytesWithCompressionFlag},
};

use std::cell::Cell;

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
    let ap = Cell::new(0);
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
        ap.bitfield16(_i, VarType::from_u16)?
    } else {
        (_i, VarType::Empty)
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
        ap.bitfield16(_i, VarType::from_u16)?
    } else {
        (_i, VarType::Empty)
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
        (_i, GUID::EMPTY)
    };

    // Disp Event
    let (_i, disp_event) = if mask.contains(ClassInfoPropMask::DISP_EVENT) {
        parse_guid(_i)?
    } else {
        (_i, GUID::DEFAULT)
    };

    // Default Proc
    let (_i, default_proc) = if mask.contains(ClassInfoPropMask::DEFAULT_PROC) {
        parse_guid(_i)?
    } else {
        (_i, GUID::DEFAULT)
    };

    Ok((
        _i,
        ClassTable {
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

impl<T> AlignedFormClassParser for T where T: AlignedParser {}

pub trait AlignedFormClassParser: AlignedParser {
    fn parse_form_object_depth_type_count<'a>(
        &self,
        input: &'a [u8],
    ) -> IResult<&'a [u8], (u8, SiteType, u32)> {
        let _i = input;

        // Depth
        let (_i, depth) = le_u8(_i)?;

        // Value
        let (_ir, value_bits) = le_u8(_i)?;
        let (_i, value) = match TypeOrCount::from_bits(value_bits) {
            Some(value) => Ok((_ir, value)),
            None => Err(nom::Err::Error(<(&[u8], ErrorKind)>::from_error_kind(
                _i,
                ErrorKind::MapOpt,
            ))),
        }?;

        let type_or_count = (value & TypeOrCount::TYPE_OR_COUNT).bits();
        let (_i, res) = if value.contains(TypeOrCount::IS_COUNT) {
            let (_ir, type_bits) = le_u8(_i)?;
            let (_i, r#type) = match SiteType::from_u8(type_bits) {
                Some(r#type) => {
                    self.inc(3);
                    Ok((_ir, r#type))
                }
                None => Err(nom::Err::Error(<(&[u8], ErrorKind)>::from_error_kind(
                    _i,
                    ErrorKind::MapOpt,
                ))),
            }?;
            let count = u32::from(type_or_count);
            (_i, (depth, r#type, count))
        } else {
            let (_i, r#type) = match SiteType::from_u8(type_or_count) {
                Some(r#type) => {
                    self.inc(2);
                    Ok((_i, r#type))
                }
                None => Err(nom::Err::Error(<(&[u8], ErrorKind)>::from_error_kind(
                    _i,
                    ErrorKind::MapOpt,
                ))),
            }?;
            (_i, (depth, r#type, 1))
        };
        Ok((_i, res))
    }
}

pub fn parse_site_depths_and_types(
    input: &[u8],
    count_of_sites: u32,
) -> IResult<&[u8], Vec<SiteDepthAndType>> {
    let ap = Cell::new(0);

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

pub fn parse_sites(
    input: &[u8],
    site_depths_and_types: Vec<SiteDepthAndType>,
) -> IResult<&[u8], Vec<Site>> {
    let mut result = Vec::with_capacity(site_depths_and_types.len());
    let mut data = input;
    for site_depth_and_type in site_depths_and_types {
        let (rest, site) = match site_depth_and_type.r#type {
            SiteType::Ole => parse_ole_site_concrete(data).map(|(r, x)| (r, Site::Ole(x)))?,
        };
        result.push(site);
        data = rest;
    }
    return Ok((data, result));
}

pub fn parse_form_control(input: &[u8]) -> IResult<&[u8], FormControl> {
    let ap = Cell::new(0 as usize);
    let _i = input;

    // Form Control Header
    let (_i, _cb_form) = parse_form_control_header(_i)?;

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
        verify!(_i, call_m!(ap.le_u16), |x| *x == 0xFFFF)?
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
        let (_ir, x) = ap.le_u16(_i)?;
        if x == 0xFFFF {
            Ok((_ir, x))
        } else {
            Err(nom::Err::Error(<(&[u8], ErrorKind)>::from_error_kind(
                _i,
                ErrorKind::Verify,
            )))
        }
    } else {
        Ok((_i, 0))
    }?;

    // Picture
    let (_i, _picture) = if mask.contains(FormPropMask::PICTURE) {
        let (_ir, x) = ap.le_u16(_i)?;
        if x == 0xFFFF {
            Ok((_ir, x))
        } else {
            Err(nom::Err::Error(<(&[u8], ErrorKind)>::from_error_kind(
                _i,
                ErrorKind::Verify,
            )))
        }
    } else {
        Ok((_i, 0))
    }?;

    // Zoom
    let (_i, zoom) = if mask.contains(FormPropMask::ZOOM) {
        ap.le_u32(_i)?
    } else {
        (_i, 100 as u32)
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
        (_i, 0 as u32)
    };

    // Draw Buffer
    let (_i, draw_buffer) = if mask.contains(FormPropMask::DRAW_BUFFER) {
        ap.le_u32(_i)?
    } else {
        (_i, 0 as u32)
    };

    let (_i, _o) = ap.align(_i, 4)?;

    // Displayed Size
    let (_i, displayed_size) = if mask.contains(FormPropMask::DISPLAYED_SIZE) {
        let (_ir, displayed_size) = parse_size(_i)?;
        ap.inc(8);
        (_ir, displayed_size)
    } else {
        (
            _i,
            Size {
                width: 4000,
                height: 3000,
            },
        )
    };

    // Logical Size
    let (_i, logical_size) = if mask.contains(FormPropMask::LOGICAL_SIZE) {
        let (_ir, logical_size) = parse_size(_i)?;
        ap.inc(8);
        (_ir, logical_size)
    } else {
        (
            _i,
            Size {
                width: 4000,
                height: 3000,
            },
        )
    };

    // Scroll Position
    let (_i, scroll_position) = if mask.contains(FormPropMask::SCROLL_POSITION) {
        let (_ir, scroll_position) = parse_position(_i)?;
        ap.inc(8);
        (_ir, scroll_position)
    } else {
        (_i, Position { top: 0, left: 0 })
    };

    // Caption
    let (_i, caption) = if mask.contains(FormPropMask::CAPTION) {
        parse_string(_i, caption_length)?
    } else {
        (_i, String::from(""))
    };

    // Picture
    // TODO

    // Mouse Icon
    let (_i, mouse_icon) = (_i, GuidAndPicture::EMPTY);

    // Font
    let (_i, font) = if mask.contains(FormPropMask::FONT) {
        parse_guid_and_font(_i)?
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

    let (_i, site_classes) = count!(_i, parse_site_class_info, count_of_site_class_info)?;

    // TODO: DesignEx?
    let (_i, count_of_sites) = le_u32(_i)?;
    let (_i, _count_of_bytes) = le_u32(_i)?;

    let (_i, site_depths_and_types) = parse_site_depths_and_types(_i, count_of_sites)?;
    let (_i, sites) = parse_sites(_i, site_depths_and_types)?;

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
