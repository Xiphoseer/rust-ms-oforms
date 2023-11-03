mod parser;
pub use parser::*;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColor {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn from_bgr(blue: u8, green: u8, red: u8) -> Self {
        Self { blue, green, red }
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
    /// See also:
    ///
    /// - <https://learn.microsoft.com/de-de/windows/win32/api/winuser/nf-winuser-getsyscolor>
    /// - <https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/system-color-constants>
    SystemPalette(PaletteEntry),
}

impl OleColor {
    pub fn from_u32(value: u32) -> Option<Self> {
        let [a, b, c, d] = value.to_le_bytes();
        match d {
            0x00 => Some(OleColor::Default(RgbColor::from_bgr(a, b, c))),
            0x01 => Some(OleColor::PaletteEntry(u16::from_le_bytes([a, b]))),
            0x02 => Some(OleColor::RgbColor(RgbColor::from_bgr(a, b, c))),
            0x80 => Some(OleColor::SystemPalette(u16::from_le_bytes([a, b]))),
            _ => None,
        }
    }
}

impl TryFrom<u32> for OleColor {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(value)
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

    #[test]
    fn test_system_colors() {
        assert_eq!(OleColor::from_u32(0x80000012).unwrap(), OleColor::BTNTEXT);
        assert_eq!(OleColor::from_u32(0x8000000f).unwrap(), OleColor::BTNFACE);
        assert_eq!(
            OleColor::from_u32(0xFFCC00).unwrap(),
            OleColor::Default(RgbColor {
                red: 0xFF,
                green: 0xCC,
                blue: 0,
            })
        );
    }
}
