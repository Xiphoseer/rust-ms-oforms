bitflags! {
    /// Specifies the size of an fmString and whether the string is compressed.
    pub struct CountOfBytesWithCompressionFlag: u32 {
        /// Specifies whether the string is compressed.
        const COMPRESSION_FLAG = 0x80000000;
        /// An unsigned integer that specifies the size of the string in bytes. The size of a compressed string is the size after compression.
        const COUNT_OF_BYTES   = 0x7FFFFFFF;
        /// An empty string
        const EMPTY            = 0x00000000;
    }
}
