use nom::number::complete::{le_u32, le_i32};
use super::{Size, Position};

named!(pub parse_size<Size>,
    do_parse!(
        width: le_u32 >>
        height: le_u32 >>
        (Size{width, height})
    )
);

named!(pub parse_position<Position>,
    do_parse!(
        top: le_i32 >>
        left: le_i32 >>
        (Position{top, left})
    )
);
