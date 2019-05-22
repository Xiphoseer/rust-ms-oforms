#[derive(Debug)]
pub struct GUID(u32,u16,u16,u64);

impl GUID {
    pub const EMPTY: Self = Self(0,0,0,0);
}
