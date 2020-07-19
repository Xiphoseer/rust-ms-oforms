use nom::number::complete::{le_u16, le_u8};
use nom::IResult;
//use nom_methods::call_m;
use super::*;
use crate::common::parser::AlignedParser;

named!(pub parse_rgb_color<RgbColor>,
    do_parse!(
        green: le_u8 >>
        blue: le_u8 >>
        red: le_u8 >>
        (RgbColor{red, green, blue})
    )
);

named!(pub parse_palette_entry<PaletteEntry>,
    call!(le_u16)
);

named!(pub parse_default_ole_color<OleColor>,
    do_parse!(
        rgb: parse_rgb_color >>
        tag!([OleColorType::Default as u8]) >>
        (OleColor::Default(rgb))
    )
);

named!(pub parse_rgb_ole_color<OleColor>,
    do_parse!(
        rgb: parse_rgb_color >>
        tag!([OleColorType::RgbColor as u8]) >>
        (OleColor::RgbColor(rgb))
    )
);

named!(pub parse_palette_ole_color<OleColor>,
    do_parse!(
        entry: parse_palette_entry >>
        take!(1) >>
        tag!([OleColorType::PaletteEntry as u8]) >>
        (OleColor::PaletteEntry(entry))
    )
);

named!(pub parse_system_ole_color<OleColor>,
    do_parse!(
        entry: parse_palette_entry >>
        take!(1) >>
        tag!([OleColorType::SystemPalette as u8]) >>
        (OleColor::SystemPalette(entry))
    )
);

named!(pub parse_ole_color<OleColor>,
    alt!( parse_default_ole_color
        | parse_palette_ole_color
        | parse_rgb_ole_color
        | parse_system_ole_color
    )
);

/// Trait to parse a color
pub trait AlignedColorParser: AlignedParser {
    fn ole_color<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], OleColor> {
        let (input, _) = self.align(input, 4)?;
        let (input, x) = parse_ole_color(input)?;
        self.inc(4);
        Ok((input, x))
    }
}

// Default implementation
impl<T> AlignedColorParser for T where T: AlignedParser {}
