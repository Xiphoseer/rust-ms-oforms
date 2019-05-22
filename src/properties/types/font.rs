use crate::common::GUID;

#[derive(Debug)]
pub struct StdFont {

}

#[derive(Debug)]
pub struct TextProps {

}

#[derive(Debug)]
pub enum FormFont {
    Empty,
    StdFont(StdFont),
    TextProps(TextProps),
}

#[derive(Debug)]
pub struct GuidAndFont {
    pub guid: GUID,
    pub font: FormFont,
}

impl GuidAndFont {
    pub const EMPTY: Self = GuidAndFont{ guid: GUID::EMPTY, font: FormFont::Empty};
}
