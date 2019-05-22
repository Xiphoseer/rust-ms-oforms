pub mod parser;

#[derive(Debug)]
pub struct RgbColor {
    green: u8,
    blue: u8,
    red: u8,
}

pub type PaletteEntry = u16;

#[repr(u8)]
#[derive(Debug)]
pub enum OleColorType {
    Default = 0x00,
    PaletteEntry = 0x01,
    RgbColor = 0x02,
    SystemPalette = 0x80,
}

#[derive(Debug)]
#[repr(u32)]
pub enum OleColor {
    Default(RgbColor),
    PaletteEntry(PaletteEntry),
    RgbColor(RgbColor),
    SystemPalette(PaletteEntry),
}

impl OleColor {
    pub const BTNFACE: Self = OleColor::SystemPalette(0x000f);
    pub const BTNTEXT: Self = OleColor::SystemPalette(0x0012);
}
