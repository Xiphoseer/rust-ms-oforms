mod parser;
pub use parser::*;

use std::{
    ffi::{CStr, CString},
    fmt::{Debug, Error as FmtError, Formatter},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct GUID(pub u32, pub u16, pub u16, pub u64);

impl GUID {
    pub const EMPTY: Self = Self(0, 0, 0, 0);
    /// {0BE35203-8F91-11CE-9DE3-00AA004BB851}
    pub const STD_FONT: Self = Self(0x0352E30B, 0x918F, 0xCE11, 0x51B84B00AA00E39D);
    /// {AFC20920-DA4E-11CE-B943-00AA006887B4}
    pub const TEXT_PROPS: Self = Self(0x2009C2AF, 0x4EDA, 0xCE11, 0xB4876800AA0043B9);
    /// {0BE35204-8F91-11CE-9DE3-00AA004BB851}
    pub const STD_PICTURE: Self = Self(0x0452E30B, 0x918F, 0xCE11, 0x51B84B00AA00E39D);
    /// What is this font?
    pub const WTF_FONT: Self = Self(0x105B80DE, 0x95F1, 0x11D0, 0x5CCBBD00AA00A0B0);
    /// {00020400-0000-0000-C000-000000000046}
    pub const DEFAULT: Self = Self(0x00020400, 0x0000, 0x0000, 0x000000000046C000);
}

impl Debug for GUID {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(
            fmt,
            "{{{:x}-{:x}-{:x}-{:x}}}",
            self.0, self.1, self.2, self.3
        )
    }
}

bitflags! {
    /// ## [MS-OAUT] VARFLAGS
    ///
    /// The VARFLAGS enumeration values are used in the wVarFlags field of a VARDESC to specify the
    /// features of a field, constant, or ODL dispinterface property, as specified in section 2.2.43.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct VarFlags: u16 {
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [readonly] attribute (see section 2.2.49.5.3).
        const READONLY = 0x1;
        /// MUST be set if the variable is a property member of an ODL interface that was declared with the [source] attribute (see section 2.2.49.8).
        const SOURCE = 0x2;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [bindable] attribute (see section 2.2.49.5.2).
        const BINDABLE = 0x4;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [requestedit] attribute (see section 2.2.49.5.2).
        const REQUEST_EDIT = 0x8;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [displaybind] attribute (see section 2.2.49.5.2).
        const DISPLAY_BIND = 0x10;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [defaultbind] attribute (see section 2.2.49.5.2).
        const DEFAULT_BIND = 0x20;
        /// MUST be set if the variable is a member of a type that was declared with the [hidden] attribute (see section 2.2.49.5.1).
        const HIDDEN = 0x40;
        /// MUST be set if the variable is a member of a type that was declared with the [restricted] attribute (see section 2.2.49.5.1).
        const RESTRICTED = 0x80;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [defaultcollelem] attribute (see section 2.2.49.5.1).
        const DEFAULT_COLL_ELEM = 0x100;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [uidefault] attribute (see section 2.2.49.5.1).
        const UI_DEFAULT = 0x200;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [nonbrowsable] attribute (see section 2.2.49.5.1).
        const NON_BROWSABLE = 0x400;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [replaceable] attribute (see section 2.2.49.5.1). MUST be ignored on receipt.
        const REPLACEABLE = 0x800;
        /// MUST be set if the variable is an ODL dispinterface property that was declared with the [immediatebind] attribute (see section 2.2.49.5.2).
        const IMMEDIATE_BIND = 0x1000;
    }
}

#[repr(u16)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum VarType {
    Empty = 0x0000,
    Null = 0x0001,
    I2 = 0x0002,
    I4 = 0x0003,
    R4 = 0x0004,
    R8 = 0x0005,
    Cy = 0x0006,
    Date = 0x0007,
    BStr = 0x0008,
    Dispatch = 0x0009,
    Error = 0x000A,
    Bool = 0x000B,
    Variant = 0x000C,
    Unknown = 0x000D,
    Decimal = 0x000E,
    I1 = 0x0010,
    UI1 = 0x0011,
    UI2 = 0x0012,
    UI4 = 0x0013,
    I8 = 0x0014,
    UI8 = 0x0015,
    Int = 0x0016,
    UInt = 0x0017,
    Void = 0x0018,
    HResult = 0x0019,
    Ptr = 0x001A,
    Safearray = 0x001B,
    CArray = 0x001C,
    UserDefined = 0x001D,
    LPStr = 0x001E,
    LPWStr = 0x001F,
    Record = 0x0024,
    IntPtr = 0x0025,
    UIntPtr = 0x0026,
    Array = 0x2000,
    ByRef = 0x4000,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CompObjHeader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompObj {
    ansi_user_type: CString,
    ansi_clipboard_format: ClipboardFormat,
}

/// Clipboard formats
///
/// ```c
/// #define CF_EMBEDSOURCE       "Embed Source"
/// #define CF_EMBEDDEDOBJECT    "Embedded Object"
/// #define CF_LINKSOURCE        "Link Source"
/// #define CF_CUSTOMLINKSOURCE  "Custom Link Source"
/// #define CF_OBJECTDESCRIPTOR  "Object Descriptor"
/// #define CF_LINKSRCDESCRIPTOR "Link Source Descriptor"
/// #define CF_OWNERLINK         "OwnerLink"
/// #define CF_FILENAME          "FileName"
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardFormat {
    None,
    Standard(u32),
    Custom(CString),
}

impl ClipboardFormat {
    pub fn custom(c: &CStr) -> Self {
        Self::Custom(c.to_owned())
    }
}
