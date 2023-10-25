mod parser;
pub use parser::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct StdFont {}

#[derive(Debug)]
pub struct TextProps {}

#[derive(Debug)]
pub enum FormFont {
    Empty,
    DdsForm21FontNew(u32, u32),
    StdFont(StdFont),
    TextProps(TextProps),
}

#[derive(Debug)]
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
