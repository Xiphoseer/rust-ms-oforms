//use crate::common::GUID;

#[derive(Debug)]
pub struct StdPicture {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum GuidAndPicture {
    Empty,
    StdPicture(StdPicture),
}

impl GuidAndPicture {
    pub const EMPTY: Self = GuidAndPicture::Empty;
}
