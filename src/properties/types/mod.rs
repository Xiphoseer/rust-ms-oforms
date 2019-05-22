#[allow(unused_imports)]
use num_traits::{FromPrimitive, ToPrimitive};

pub mod color;
pub mod font;
pub mod picture;
pub mod parser;

/// An unsigned integer that specifies the type of icon displayed as the mouse pointer for the control.
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MousePointer {
    /// Standard pointer.
    Default = 0x00,
    /// Arrow.
    Arrow = 0x01,
    /// Cross-hair pointer.
    Cross = 0x02,
    /// I-beam.
    IBeam = 0x03,
    /// Double arrow pointing northeast and southwest.
    SizeNESW = 0x06,
    /// Double arrow pointing north and south.
    SizeNS = 0x07,
    /// Double arrow pointing northwest and southeast.
    SizeNWSE = 0x08,
    /// Double arrow pointing west and east.
    SizeWE = 0x09,
    /// Up arrow.
    UpArrow = 0x0A,
    /// Hourglass.
    HourGlass = 0x0B,
    /// "Not" symbol (circle with a diagonal line) on top of the object being dragged.
    NoDrop = 0x0C,
    /// Arrow with an hourglass.
    AppStarting = 0x0D,
    /// Arrow with a question mark.
    Help = 0x0E,
    /// "Size-all" cursor (arrows pointing north, south, east, and west).
    SizeAll = 0x0F,
    /// Uses the icon specified by the MouseIcon property.
    Custom = 0x63,
}

/// Specifies the visual appearance of the control.
///
/// In this enumeration, "form" refers to the surface on which the control appears.
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SpecialEffect {
    /// Control appears flat.
    Flat = 0x00,
    /// Control appears to be raised up from the form.
    Raised = 0x01,
    /// Control appears to be carved into the form.
    Sunken = 0x02,
    /// The control border appears to be carved into the form.
    Etched = 0x03,
    /// The control border appears to be raised up from the form.
    Bump = 0x06,
}

/// Specifies the alignment of the picture in the Form or Image.
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PictureAlignment {
    /// The top-left corner.
    TopLeft = 0x00,
    /// The top-right corner.
    TopRight = 0x01,
    /// The center.
    Center = 0x02,
    /// The bottom-left corner.
    BottomLeft = 0x03,
    /// The bottom-right corner.
    BottomRight = 0x04,
}

/// Specifies how to display the picture.
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PictureSizeMode {
    /// Crops any part of the picture that is larger than the control boundaries.
    Clip = 0x00,
    /// Stretches the picture to fill the control area. This setting distorts the picture in either the horizontal or vertical direction.
    Stretch = 0x01,
    /// Enlarges the picture, but does not distort the picture in either the horizontal or vertical direction.
    Zoom = 0x03,
}

pub type HiMetric = u32;

#[derive(Debug)]
/// Specifies a pair of signed integers that specify the size of a control.
pub struct Size {
    /// A signed integer that specifies the width, in HIMETRIC units, of the control.
    pub width: HiMetric,
    /// A signed integer that specifies the height, in HIMETRIC units, of the control.
    pub height: HiMetric,
}

pub type SignedHiMetric = i32;

#[derive(Debug)]
/// Specifies a pair of signed integers that specify a position relative to a reference point.
pub struct Position {
    /// A signed integer that specifies, in HIMETRIC units, a distance below the reference point.
    pub top: SignedHiMetric,
    /// A signed integer that specifies, in HIMETRIC units, a distance to the right of the reference point.
    pub left: SignedHiMetric,
}
