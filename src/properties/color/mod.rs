mod parser;
use core::fmt;
use num_traits::FromPrimitive;
pub use parser::*;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RgbColor {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl RgbColor {
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub const fn from_bgr(blue: u8, green: u8, red: u8) -> Self {
        Self { blue, green, red }
    }
}

pub type PaletteIndex = u16;

/// 16-bit integer that may be a valid [`SystemColor`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemPaletteIndex(u16);

impl fmt::Debug for SystemPaletteIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match SystemColor::from_u16(self.0) {
            Some(s) => <SystemColor as fmt::Debug>::fmt(&s, f),
            None => f.debug_tuple("SystemPaletteIndex").field(&self.0).finish(),
        }
    }
}

/// Windows System Colors
///
/// See: <https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/system-color-constants>
#[repr(u16)]
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum SystemColor {
    /// Scroll bar color
    ScrollBars = 0x00,
    /// Desktop color
    Desktop = 0x01,
    /// Color of the title bar for the active window
    ActiveTitleBar = 0x02,
    /// Color of the title bar for the inactive window
    InactiveTitleBar = 0x03,
    /// Menu background color
    MenuBar = 0x04,
    /// Window background color
    WindowBackground = 0x05,
    /// Window frame color
    WindowFrame = 0x06,
    /// Color of text on menus
    MenuText = 0x07,
    /// Color of text in windows
    WindowText = 0x08,
    /// Color of text in caption, size box, and scroll arrow
    TitleBarText = 0x09,
    /// Border color of active window
    ActiveBorder = 0x0A,
    /// Border color of inactive window
    InactiveBorder = 0x0B,
    /// Background color of multiple-document interface (MDI) applications
    ApplicationWorkspace = 0x0C,
    /// Background color of items selected in a control
    Highlight = 0x0D,
    /// Text color of items selected in a control
    HighlightText = 0x0E,
    /// Color of shading on the face of command buttons
    ButtonFace = 0x0F,
    /// Color of shading on the edge of command buttons
    ButtonShadow = 0x10,
    /// Grayed (disabled) text
    GrayText = 0x11,
    /// Text color on push buttons
    ButtonText = 0x12,
    /// Color of text in an inactive caption
    InactiveCaptionText = 0x13,
    /// Highlight color for 3-D display elements (aka `ButtonHilight`)
    _3DHighlight = 0x14,
    /// Darkest shadow color for 3-D display elements (aka `ButtonDkShadow`)
    _3DDKShadow = 0x15,
    /// Second lightest 3-D color after vb3DHighlight (aka `ButtonLight`)
    _3DLight = 0x16,
    /// Color of text in ToolTips
    InfoText = 0x17,
    /// Background color of ToolTips (aka `InfoWindow`)
    InfoBackground = 0x18,
}

impl From<SystemColor> for RgbColor {
    fn from(value: SystemColor) -> Self {
        match value {
            SystemColor::ScrollBars => RgbColor::from_rgb(200, 200, 200),
            SystemColor::Desktop => RgbColor::from_rgb(0, 0, 0),
            SystemColor::ActiveTitleBar => RgbColor::from_rgb(153, 180, 209),
            SystemColor::InactiveTitleBar => RgbColor::from_rgb(191, 205, 219),
            SystemColor::MenuBar => RgbColor::from_rgb(255, 255, 255),
            SystemColor::WindowBackground => RgbColor::from_rgb(255, 255, 255),
            SystemColor::WindowFrame => RgbColor::from_rgb(100, 100, 100),
            SystemColor::MenuText => RgbColor::from_rgb(0, 0, 0),
            SystemColor::WindowText => RgbColor::from_rgb(0, 0, 0),
            SystemColor::TitleBarText => RgbColor::from_rgb(0, 0, 0),
            SystemColor::ActiveBorder => RgbColor::from_rgb(180, 180, 180),
            SystemColor::InactiveBorder => RgbColor::from_rgb(244, 247, 252),
            SystemColor::ApplicationWorkspace => RgbColor::from_rgb(171, 171, 171),
            SystemColor::Highlight => RgbColor::from_rgb(0, 120, 215),
            SystemColor::HighlightText => RgbColor::from_rgb(255, 255, 255),
            SystemColor::ButtonFace => RgbColor::from_rgb(255, 255, 255),
            SystemColor::ButtonShadow => RgbColor::from_rgb(160, 160, 160),
            SystemColor::GrayText => RgbColor::from_rgb(109, 109, 109),
            SystemColor::ButtonText => RgbColor::from_rgb(0, 0, 0),
            SystemColor::InactiveCaptionText => RgbColor::from_rgb(0, 0, 0),
            SystemColor::_3DHighlight => RgbColor::from_rgb(255, 255, 255),
            SystemColor::_3DDKShadow => RgbColor::from_rgb(105, 105, 105),
            SystemColor::_3DLight => RgbColor::from_rgb(227, 227, 227),
            SystemColor::InfoText => RgbColor::from_rgb(0, 0, 0),
            SystemColor::InfoBackground => RgbColor::from_rgb(255, 255, 225),
            /*
            SystemColor::ButtonAlternateFace => RgbColor::from_rgb(255, 255, 255),
            SystemColor::HotTrackingColor => RgbColor::from_rgb(0, 102, 204),
            SystemColor::GradientActiveTitle => RgbColor::from_rgb(185, 209, 234),
            SystemColor::GradientInactiveTitle => RgbColor::from_rgb(215, 228, 242),
            SystemColor::MenuHilight => RgbColor::from_rgb(51, 153, 255),
            SystemColor::MenuBar => RgbColor::from_rgb(255, 255, 255)
            */
        }
    }
}

impl SystemColor {
    pub const fn as_index(&self) -> SystemPaletteIndex {
        SystemPaletteIndex(*self as u16)
    }

    pub const fn as_ole_color(&self) -> OleColor {
        self.as_index().as_ole_color()
    }
}

impl SystemPaletteIndex {
    pub const fn as_ole_color(&self) -> OleColor {
        OleColor::SystemPalette(*self)
    }

    pub fn as_system_color(&self) -> Option<SystemColor> {
        SystemColor::from_u16(self.0)
    }
}

impl From<SystemColor> for SystemPaletteIndex {
    fn from(value: SystemColor) -> Self {
        value.as_index()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum OleColor {
    Default(RgbColor),
    PaletteEntry(PaletteIndex),
    RgbColor(RgbColor),
    /// See also:
    ///
    /// - <https://learn.microsoft.com/de-de/windows/win32/api/winuser/nf-winuser-getsyscolor>
    /// - <https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/system-color-constants>
    SystemPalette(SystemPaletteIndex),
}

impl OleColor {
    const TAG_DEFAULT: u8 = 0x00;
    const TAG_PALETTE_ENTRY: u8 = 0x01;
    const TAG_RGB_COLOR: u8 = 0x02;
    const TAG_SYSTEM_PALETTE: u8 = 0x80;

    pub const fn from_u32(value: u32) -> Option<Self> {
        let [a, b, c, d] = value.to_le_bytes();
        match d {
            Self::TAG_DEFAULT => Some(OleColor::Default(RgbColor::from_bgr(a, b, c))),
            Self::TAG_PALETTE_ENTRY => {
                Some(OleColor::PaletteEntry(PaletteIndex::from_le_bytes([a, b])))
            }
            Self::TAG_RGB_COLOR => Some(OleColor::RgbColor(RgbColor::from_bgr(a, b, c))),
            Self::TAG_SYSTEM_PALETTE => Some(OleColor::SystemPalette(SystemPaletteIndex(
                u16::from_le_bytes([a, b]),
            ))),
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
    pub const _3DDKSHADOW: Self = SystemColor::_3DDKShadow.as_ole_color();
    pub const _3DFACE: Self = SystemColor::ButtonFace.as_ole_color();
    pub const _3DHIGHLIGHT: Self = SystemColor::_3DHighlight.as_ole_color();
    pub const BTNFACE: Self = SystemColor::ButtonFace.as_ole_color();
    pub const BTNTEXT: Self = SystemColor::ButtonText.as_ole_color();
    pub const WINDOWTEXT: Self = SystemColor::WindowText.as_ole_color();
    pub const WINDOW: Self = SystemColor::WindowBackground.as_ole_color();
    pub const WINDOWFRAME: Self = SystemColor::WindowFrame.as_ole_color();
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

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<OleColor>(), 4);
    }
}
