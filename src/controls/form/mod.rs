//! ## The `FromControl` binary format
//!
//! Excerpt from the docs:
//!
//! > All parent controls MUST contain a FormControl. The FormControl properties are persisted to a stream as specified in section 2.1.1.2. The name of this stream MUST be "f". An OleSiteConcrete is persisted in this stream for each embedded control, as specified by the FormControl in section 2.2.10.12. The FormControl can also contain a DesignExtender, as specified in section 2.2.10.11.

#[allow(unused_imports)]
use num_traits::{FromPrimitive, ToPrimitive};

use crate::properties::types::{
    font::GuidAndFont,
    picture::GuidAndPicture,
    color::OleColor,
    SpecialEffect,
    PictureAlignment,
    PictureSizeMode,
    MousePointer,
    Size, Position,
};

pub mod parser;
pub mod stream;
pub mod designex;

bitflags!{
    /// A bit field that specifies Boolean properties of a form.
    pub struct FormFlags: u32 {
        /// Specifies whether the form is enabled.
        const ENABLED               = 0x00000004;
        /// Specifies whether Design Extender properties are persisted with this form.
        const DESINKPERSISTED       = 0x00004000;
        /// Specifies whether the Class Table of a form is not persisted. A value of zero specifies that the Class Table is persisted if it is not empty.
        const DONTSAVECLASSTABLE    = 0x00008000;
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum BorderStyle {
    /// The control has no visible border line.
    None = 0x00,
    /// The control has a single-line border.
    Single = 0x01,
}

/// Specifies the behavior of the TAB key in the last control of a form
#[repr(u8)]
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Cycle {
    /// The focus is next set to the first control on the next form, returning to the first control of this form only after all controls on all other forms have been reached.
    AllForms = 0x00,
    /// The focus is next set to the first control on this form, ignoring other forms.
    CurrentForm = 0x02,
}

bitflags! {
    /// A bit field that specifies the location of the scroll bars of a form.
    pub struct FormScrollBarFlags: u8 {
        /// Specifies whether the horizontal scroll bar is displayed.
        const HORIZONTAL = 0x01;
        /// Specifies whether the vertical scroll bar is displayed.
        const VERTICAL = 0x02;
        /// Specifies whether to display the horizontal scroll bar at all times, even when all controls are visible without scrolling.
        const KEEP_HORIZONTAL = 0x04;
        /// Specifies whether to display the vertical scroll bar at all times, even when all controls are visible without scrolling.
        const KEEP_VERTICAL = 0x08;
        /// Specifies whether to display the vertical scroll bar on the left side of the form.
        const KEEP_LEFT = 0x10;
        /// The default value
        const DEFAULT = Self::KEEP_HORIZONTAL.bits | Self::KEEP_VERTICAL.bits;
    }
}

#[derive(Debug)]
pub struct FormControl {
    /// default: 0x8000000F = COLOR_BTNFACE from the system palette.
    pub back_color: OleColor,
    /// default: 0x00000004 = FORM_FLAG_ENABLED
    pub boolean_properties: FormFlags,
    /// default: 0x80000012 = COLOR_BTNTEXT from the system palette.
    pub border_color: OleColor,
    /// default: 0x00 = None
    pub border_style: BorderStyle,
    /// default: ""
    pub caption: String,
    /// default: AllForms
    pub cycle: Cycle,
    /// default: (4000, 3000)
    pub displayed_size: Size,
    /// REQUIRED
    pub draw_buffer: u32,
    /// default: no font
    pub font: GuidAndFont,
    /// default: 0x80000012 = COLOR_BTNTEXT from the system palette
    pub fore_color: OleColor,
    /// An unsigned integer that specifies the number of control groups on a form. default: 0
    pub group_count: u32,
    /// An fmSize that specifies the full scrollable size, in HIMETRIC units, of a form, including all controls. A value of zero in either width or height specifies that the scrollable size is equivalent to the value of the corresponding portion of DisplayedSize.
    ///
    /// default: (4000, 3000)
    pub logical_size: Size,
    /// A GuidAndPicture that specifies a custom icon to display as the mouse pointer for the control, which is used when the value of the MousePointer property is set to 99, fmMousePointerCustom.
    ///
    /// The file format default is no custom icon.
    pub mouse_icon: GuidAndPicture,
    /// An unsigned integer that specifies the type of icon displayed as the mouse pointer for the control. SHOULD be a value from the fmMousePointer enumeration.<8>
    ///
    /// The file format default is 0x00, fmMousePointerDefault.
    pub mouse_pointer: MousePointer,
    /// An unsigned integer that specifies the largest ID that has been used by an embedded control on a form. The value of this property can be used by the client application to determine the next valid ID for a new control.
    ///
    /// The file format default is 0x00000000.
    pub next_available_id: u32,
    /// A GuidAndPicture that specifies the picture to display on a control.
    ///
    /// The file format default is no picture.
    pub picture: GuidAndPicture,

    /// An fmPictureAlignment that specifies the alignment of the picture in the Form or Image.
    ///
    /// The file format default is 0x02, fmPictureAlignmentCenter.
    pub picture_alignment: PictureAlignment,
    /// An fmPictureSizeMode that specifies how to display the picture.
    ///
    /// The file format default is 0x00, fmPictureSizeModeClip.
    pub picture_size_mode: PictureSizeMode,
    /// A Boolean value that specifies whether the picture is tiled across the background.
    ///
    /// The file format default is FALSE.
    pub picture_tiling: bool,
    /// A FormScrollBarFlags that specifies whether a form has vertical or horizontal scroll bars and when to display them.
    ///
    /// The file format default is 0x0000000C, fScrollBarsKeepHorizontal and fScrollBarsKeepVertical.
    pub scroll_bars: FormScrollBarFlags,
    /// An fmPosition that specifies, in HIMETRIC units, the coordinates of the first point in the LogicalSize of the form that is visible.
    ///
    /// The file format default is a position of (0, 0), which specifies that the form has not been scrolled.
    pub scroll_position: Position,
    /// An unsigned integer that specifies the number of times the dynamic type information of a form has changed. The value of this property can be used to determine whether a form being loaded still matches the dynamic type information against which it was compiled.
    ///
    /// The file format default is 0x00000000.
    pub shape_cookie: u32,
    /// An fmSpecialEffect that specifies the visual appearance of the control.
    ///
    /// The file format default is  0x00, fmSpecialEffectFlat
    pub special_effect: SpecialEffect,
    /// A signed integer that specifies the magnification of embedded controls, in percentage points of the size of the parent control. MUST be greater than or equal to 10 (10 percent of actual size) and less than or equal to 400 (four times or 400 percent of actual size).
    ///
    /// The file format default is 100, or actual size.
    pub zoom: u32,
}
