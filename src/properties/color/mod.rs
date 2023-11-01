mod parser;
pub use parser::*;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RgbColor {
    pub green: u8,
    pub blue: u8,
    pub red: u8,
}

impl RgbColor {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn from_gbr(green: u8, blue: u8, red: u8) -> Self {
        Self { green, blue, red }
    }
}

pub type PaletteEntry = u16;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
enum OleColorType {
    Default = 0x00,
    PaletteEntry = 0x01,
    RgbColor = 0x02,
    SystemPalette = 0x80,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OleColor {
    Default(RgbColor),
    PaletteEntry(PaletteEntry),
    RgbColor(RgbColor),
    SystemPalette(PaletteEntry),
}

impl TryFrom<u32> for OleColor {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let [a, b, c, d] = value.to_le_bytes();
        match d {
            0x00 => Ok(OleColor::Default(RgbColor::from_gbr(a, b, c))),
            0x01 => Ok(OleColor::PaletteEntry(u16::from_le_bytes([a, b]))),
            0x02 => Ok(OleColor::RgbColor(RgbColor::from_gbr(a, b, c))),
            0x80 => Ok(OleColor::SystemPalette(u16::from_le_bytes([a, b]))),
            _ => Err(value),
        }
    }
}

impl OleColor {
    pub const BTNFACE: Self = OleColor::SystemPalette(0x000f);
    pub const BTNTEXT: Self = OleColor::SystemPalette(0x0012);
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{parse_ole_color, OleColor, RgbColor};

    #[test]
    fn test_color() {
        assert_eq!(
            OleColor::try_from(u32::from_le_bytes([0, 0, 255, 0])),
            Ok(OleColor::Default(RgbColor {
                green: 0,
                blue: 0,
                red: 255,
            }))
        );
        assert_eq!(
            parse_ole_color::<nom::error::Error<&[u8]>>(&[0, 0, 255, 0]),
            Ok((
                &[][..],
                OleColor::Default(RgbColor {
                    green: 0,
                    blue: 0,
                    red: 255,
                })
            ))
        );
    }
}
