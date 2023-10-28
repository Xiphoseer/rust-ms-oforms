#![allow(clippy::bad_bit_mask)] // fixme: update bitflags

bitflags! {
    /// Specifies the size of an fmString and whether the string is compressed.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CountOfBytesWithCompressionFlag: u32 {
        /// Specifies whether the string is compressed.
        const COMPRESSION_FLAG = 0x80000000;
        /// An unsigned integer that specifies the size of the string in bytes. The size of a compressed string is the size after compression.
        const COUNT_OF_BYTES   = 0x7FFFFFFF;
        /// An empty string
        const EMPTY            = 0x00000000;
    }
}

impl CountOfBytesWithCompressionFlag {
    pub fn len(&self) -> u32 {
        (*self & Self::COUNT_OF_BYTES).bits()
    }

    pub fn compressed(&self) -> bool {
        self.contains(Self::COMPRESSION_FLAG)
    }
}
