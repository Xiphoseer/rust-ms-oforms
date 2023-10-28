mod parser;
pub use parser::*;
use std::num::NonZeroU32;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StdFont {
    /// A signed integer that specifies the character set of the font.
    pub charset: i16,
    /// A [`FONTFLAGS`][`FontFlags`] that specifies style characteristics of the font.
    pub flags: FontFlags,
    /// A signed integer that specifies the weight of the font. MUST be in the range from zero through 1000.
    /// A value of zero specifies that the weight is to be determined by the application.
    /// A value in the range from 1 through 1000 specifies a weight, where 1 specifies the lightest type and 1000 specifies the darkest type.
    pub weight: i16,
    /// An unsigned integer that specifies the height, in ten-thousandths of a point, of the font.
    /// MUST be greater than zero and less than or equal to 655350000.
    pub height: NonZeroU32,
    /// An ASCII string that specifies the name of the font.
    pub font_face: String,
}

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FontFlags: u8 {
        /// Specifies whether the font style is bold. MUST be set to zero.
        const BOLD = 1 << 0;
        /// Specifies whether the font style is italic.
        const ITALIC = 1 << 1;
        /// Specifies whether the font style is underlined.
        const UNDERLINE = 1 << 2;
        /// Specifies whether the font style is strikethrough.
        const STRIKETHROUGH = 1 << 3;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextProps {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormFont {
    Empty,
    DdsForm21FontNew(DdsForm21FontNew),
    StdFont(StdFont),
    TextProps(TextProps),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DdsForm21FontNew {
    _d1: u32,
    _d2: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuidAndFont {
    pub guid: Uuid,
    pub font: FormFont,
}

impl GuidAndFont {
    pub const EMPTY: Self = GuidAndFont {
        guid: Uuid::nil(),
        font: FormFont::Empty,
    };
}
