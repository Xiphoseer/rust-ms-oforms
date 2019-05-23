use nom::{le_u8, le_u16, le_u32, le_u64, le_i16, le_i32};
use super::GUID;

named_args!(pub check_guid(guid: GUID)<GUID>,
    verify!(parse_guid, |x| x == guid)
);

named!(pub parse_guid<GUID>,
    do_parse!(
        d1: le_u32 >>
        d2: le_u16 >>
        d3: le_u16 >>
        d4: le_u64 >>
        (GUID(d1,d2,d3,d4))
    )
);

named_args!(pub align<'a>(offset: &'a mut usize, count: usize)<usize>,
    do_parse!(
        p1: value!(*offset % count) >>
        p2: value!(if p1 == 0 { 0 } else { count - p1 }) >>
        take!(p2) >>
        ({*offset += p2; *offset})
    )
);

named_args!(pub aligned_le_u32<'a>(offset: &'a mut usize)<u32>,
    do_parse!(
        call!(align, offset, 4) >>
        val: map!(le_u32, |x| { *offset += 4; x }) >>
        (val)
    )
);

named_args!(pub aligned_le_i32<'a>(offset: &'a mut usize)<i32>,
    do_parse!(
        call!(align, offset, 4) >>
        val: map!(le_i32, |x| { *offset += 4; x }) >>
        (val)
    )
);

named_args!(pub aligned_le_u16<'a>(offset: &'a mut usize)<u16>,
    do_parse!(
        call!(align, offset, 2) >>
        val: map!(le_u16, |x| { *offset += 2; x }) >>
        (val)
    )
);

named_args!(pub aligned_le_i16<'a>(offset: &'a mut usize)<i16>,
    do_parse!(
        call!(align, offset, 2) >>
        val: map!(le_i16, |x| { *offset += 2; x }) >>
        (val)
    )
);

named_args!(pub aligned_le_u8<'a>(offset: &'a mut usize)<u8>,
    map!(le_u8, |x| { *offset += 1; x })
);
