//! ## 2.2.1 CommandButton Control

use nom::{
    bytes::complete::tag,
    combinator::map_opt,
    error::{FromExternalError, ParseError},
    multi::length_value,
    number::complete::{le_u16, le_u32},
    sequence::preceded,
    IResult,
};

use crate::{common::AlignedParser, properties::color::OleColor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandButtonControl {
    pub fore_color: OleColor,
    pub back_color: OleColor,
}

bitflags! {
    struct CommandButtonPropMask: u32 {
        /// A - fForeColor (1 bit): Specifies whether the ForeColor property is stored in the DataBlock.ForeColor of the CommandButtonControl that contains this CommandButtonPropMask.
        const FORE_COLOR = 1 << 0;
        /// B - fBackColor (1 bit): Specifies whether the BackColor property is stored in the DataBlock.BackColor of the CommandButtonControl that contains this CommandButtonPropMask.
        const BACK_COLOR = 1 << 1;
        /// C - fVariousPropertyBits (1 bit): Specifies whether the VariousPropertyBits property is stored in the DataBlock.VariousPropertyBits of the CommandButtonControl that contains this CommandButtonPropMask.
        const VARIOUS_PROPERTY_BITS = 1 << 2;
        /// D - fCaption (1 bit): Specifies whether the size and compression flag of the Caption property are stored in the DataBlock.Caption of the CommandButtonControl that contains this CommandButtonPropMask and the Caption string is stored in the ExtraDataBlock.Caption of the CommandButtonControl.
        const CAPTION = 1 << 3;
        /// E - fPicturePosition (1 bit): Specifies whether the PicturePosition property is stored in the DataBlock.PicturePosition of the CommandButtonControl that contains this CommandButtonPropMask.
        const PICTURE_POSITION = 1 << 4;
        /// F - fSize (1 bit): Specifies whether the Size property is stored in the ExtraDataBlock.Size of the CommandButtonControl that contains this CommandButtonPropMask. MUST be set to 1.
        const SIZE = 1 << 5;
        /// G - fMousePointer (1 bit): Specifies whether the MousePointer property is stored in the DataBlock.MousePointer of the CommandButtonControl that contains this CommandButtonPropMask.
        const MOUSE_POINTER = 1 << 6;
        /// H - fPicture (1 bit): Specifies whether the Picture property is stored in the StreamData.Picture of the CommandButtonControl that contains this CommandButtonPropMask. When this bit is set to 1, a value of 0xFFFF MUST be stored in the DataBlock.Picture of the CommandButtonControl.
        const PICTURE = 1 << 7;
        /// I - fAccelerator (1 bit): Specifies whether the Accelerator property is stored in the DataBlock.Accelerator of the CommandButtonControl that contains this CommandButtonPropMask.
        const ACCELERATOR = 1 << 8;
        /// J - fTakeFocusOnClick (1 bit): Specifies whether the value of the TakeFocusOnClick property is not the file format default.
        const TAKE_FOCUS_ON_CLICK = 1 << 9;
        /// K - fMouseIcon (1 bit): Specifies whether the MouseIcon property is stored in the StreamData.MouseIcon of the CommandButtonControl that contains this CommandButtonPropMask. When this bit is set to 1, a value of 0xFFFF MUST be stored in the DataBlock.MouseIcon of the CommandButtonControl.
        const MOUSE_ICON = 1 << 10;
    }
}

fn parse_command_button_header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], u16, E>
where
    E: ParseError<&'a [u8]>,
{
    preceded(tag([0x00, 0x02]), le_u16)(input)
}

pub fn parse_command_button<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], CommandButtonControl, E>
where
    E: ParseError<&'a [u8]>,
    E: FromExternalError<&'a [u8], u32>,
{
    length_value(parse_command_button_header, _parse_command_button)(input)
}

fn _parse_command_button<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], CommandButtonControl, E>
where
    E: ParseError<&'a [u8]>,
    E: FromExternalError<&'a [u8], u32>,
{
    let (input, mask) = map_opt(le_u32, CommandButtonPropMask::from_bits)(input)?;
    let ap = AlignedParser::new();

    let (input, fore_color) = match mask.contains(CommandButtonPropMask::FORE_COLOR) {
        true => ap.ole_color(input)?,
        false => (input, OleColor::BTNTEXT),
    };
    let (input, back_color) = match mask.contains(CommandButtonPropMask::BACK_COLOR) {
        true => ap.ole_color(input)?,
        false => (input, OleColor::BTNFACE),
    };

    Ok((
        input,
        CommandButtonControl {
            fore_color,
            back_color,
        },
    ))
}
