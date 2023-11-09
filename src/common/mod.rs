mod parser;
pub use parser::*;
use uuid::{uuid, Uuid};

use std::{
    ffi::{CStr, CString},
    fmt::Debug,
};

/// `{0BE35203-8F91-11CE-9DE3-00AA004BB851}` StdFont
pub const CLSID_STD_FONT: Uuid = uuid!("0BE35203-8F91-11CE-9DE3-00AA004BB851");
/// `{AFC20920-DA4E-11CE-B943-00AA006887B4}` TextProps / NewFont
///
/// See also: <https://learn.microsoft.com/en-us/dotnet/api/microsoft.vbe.interop.forms.newfontclass>
pub const CLSID_TEXT_PROPS: Uuid = uuid!("AFC20920-DA4E-11CE-B943-00AA006887B4");
/// `{0BE35204-8F91-11CE-9DE3-00AA004BB851}` StdPicture
pub const CLSID_STD_PICTURE: Uuid = uuid!("0BE35204-8F91-11CE-9DE3-00AA004BB851");
/// `{105b80de-95f1-11d0-b0a0-00aa00bdcb5c}` Microsoft DT DDSform 2.1 FontNew
pub const CLSID_DT_DDSFORM_21_FONT_NEW: Uuid = uuid!("105b80de-95f1-11d0-b0a0-00aa00bdcb5c");
/// `{00020400-0000-0000-C000-000000000046}` IDispatch
pub const IID_IDISPATCH: Uuid = uuid!("00020400-0000-0000-C000-000000000046");

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

bitflags! {
    /// [MS-OAUT] 2.2.7 VARIANT Type Constants
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[doc(alias = "tagVARENUM")]
    pub struct VarType : u16 {
        const EMPTY = 0x0000;
        const NULL = 0x0001;
        const I2 = 0x0002;
        const I4 = 0x0003;
        const R4 = 0x0004;
        const R8 = 0x0005;
        const CY = 0x0006;
        const DATE = 0x0007;
        const BSTR = 0x0008;
        const DISPATCH = 0x0009;
        const ERROR = 0x000A;
        const BOOL = 0x000B;
        const VARIANT = 0x000C;
        const UNKNOWN = 0x000D;
        const DECIMAL = 0x000E;
        const I1 = 0x0010;
        const UI1 = 0x0011;
        const UI2 = 0x0012;
        const UI4 = 0x0013;
        const I8 = 0x0014;
        const UI8 = 0x0015;
        const INT = 0x0016;
        const UINT = 0x0017;
        const VOID = 0x0018;
        const HRESULT = 0x0019;
        const PTR = 0x001A;
        const SAFEARRAY = 0x001B;
        const CARRAY = 0x001C;
        const USERDEFINED = 0x001D;
        const LPSTR = 0x001E;
        const LPWSTR = 0x001F;
        const RECORD = 0x0024;
        const INTPTR = 0x0025;
        const UINTPTR = 0x0026;
        const ARRAY = 0x2000;
        const BYREF = 0x4000;
    }
}

/// [MS-OLEDS] 2.3.7 CompObjHeader
///
/// See: <https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oleds/5f368864-fdeb-4865-b298-67289f3c4e8e>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CompObjHeader;

/// [MS-OLEDS] 2.3.8 CompObjStream
///
/// See: <https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oleds/142e0420-2f74-4ed9-829b-0b3d5a684d01>
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct CompObj {
    pub ansi_user_type: CString,
    pub ansi_clipboard_format: ClipboardFormat,
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
