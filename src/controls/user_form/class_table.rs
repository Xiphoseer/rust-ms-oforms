use uuid::Uuid;

use crate::common::{VarFlags, VarType};

bitflags! {
    /// A bit field that specifies Boolean properties of a SiteClassInfo.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ClsTableFlags: u16 {
        /// Specifies whether the typeKind member of the TYPEATTR that describes this type
        /// information, as specified in [MS-OAUT] section 2.2.44, is set to TKIND_ALIAS, as
        /// specified in [MS-OAUT] section 2.2.17.
        const EXCLUSIVE_VALUE   = 0x0001;
        /// Specifies whether this type information implements a dual interface.
        const DUAL_INTERFACE    = 0x0002;
        /// Specifies whether this type information supports aggregation. A value of 1 specifies
        /// that the control does not support aggregation.
        const NO_AGGREGATION    = 0x0004;
    }
}

/// Specifies the structure, as persisted to a stream, of the type information of an embedded ActiveX control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteClassInfo {
    /// A CLSTABLE_FLAGS that specifies Boolean properties of the type information.
    ///
    /// The file format default is 0x0000.
    pub class_table_flags: ClsTableFlags,
    /// A VARFLAGS, as specified in [MS-OAUT] section 2.2.18, that specifies Boolean properties of
    /// the type information.
    ///
    /// The file format default is 0x0000.
    pub var_flags: VarFlags,
    /// An unsigned integer that specifies the number of methods on the default dual interface of
    /// the type information.
    ///
    /// The file format default is 0x00000000.
    pub count_of_methods: u32,
    /// An unsigned integer that specifies the IDispatch identifier (DispID) of the default
    /// bindable property, as specified in [MS-OAUT] section 2.2.49.5.2, in this type information.
    /// The value of this field is the memid field of the VARDESC of the function, as specified in
    /// [MS-OAUT] section 2.2.43. The VARDESC.wVarFlags field MUST be set to 0x00000014, or
    /// FUNCFLAG_FBINDABLE and FUNCFLAG_FDISPLAYBIND, as specified in [MS-OAUT] section 2.2.11.
    pub dispid_bind: u32,
    /// An unsigned integer that specifies the index of the "get" function of the default bindable
    /// property, as specified in [MS-OAUT] section 2.2.49.5.2, into the dynamic virtual table of a
    /// type information that implements a dual interface. The value of this field is the oVft
    /// field of the FUNCDESC that specifies the function, as specified in [MS-OAUT] section 2.2.42.
    /// The memid field of this FUNCDESC MUST NOT be set to DISPID_VALUE, as specified in [MS-OAUT]
    /// section 2.2.32.1. The invkind field of this FUNCDESC MUST be set to INVOKE_PROPERTYGET, as
    /// specified in [MS-OAUT] section 2.2.14.
    ///
    /// The file format default is 0x0000.
    pub get_bind_index: u16,
    /// An unsigned integer that specifies the index of the "put" function of the default bindable
    /// property, as specified in [MS-OAUT] section 2.2.49.5.2, into the dynamic virtual table of a
    /// type information that implements a dual interface. The value of this field is the oVft
    /// field of the FUNCDESC that specifies the function, as specified in [MS-OAUT] section 2.2.42.
    /// The memid field of this FUNCDESC MUST NOT be set to DISPID_VALUE, as specified in [MS-OAUT]
    /// section 2.2.32.1. The invkind field of this FUNCDESC MUST be set to INVOKE_PROPERTYPUT, as
    /// specified in [MS-OAUT] section 2.2.14.
    ///
    /// The file format default is 0x0000.
    pub put_bind_index: u16,
    /// A variant type that specifies the type of the default bindable property, as specified in
    /// [MS-OAUT] section 2.2.49.5.2. The value of this field is the vt field of the TYPEDESC, as
    /// specified in [MS-OAUT] section 2.2.37, of the FUNCDESC.elemdescFunc, as specified in
    /// [MS-OAUT] section 2.2.42, of the function referenced by GetBindIndex or PutBindIndex in
    /// this ClassInfoDataBlock.
    ///
    /// The file format default is 0x0000, VT_EMPTY.
    pub bind_type: VarType,
    /// An unsigned integer that specifies the index of the function that retrieves the value of
    /// the control into the dynamic virtual table of the class. The value of this field is the
    /// oVft field of the FUNCDESC that specifies the function, as specified in [MS-OAUT] section
    /// 2.2.42. The memid of the FUNCDESC MUST be set to DISPID_VALUE, as specified in [MS-OAUT]
    /// section 2.2.32.1. The invkind field of the FUNCDESC MUST be set to INVOKE_PROPERTYGET, as
    /// specified in [MS-OAUT] section 2.2.14.
    ///
    /// The file format default is 0x0000.
    pub get_value_index: u16,
    /// An unsigned integer that specifies the index of the function that sets the value of the
    /// control into the dynamic virtual table of the class. The value of this field is the oVft
    /// field of the FUNCDESC that specifies the function, as specified in [MS-OAUT] section
    /// 2.2.42. The memid of the FUNCDESC MUST be set to DISPID_VALUE, as specified in [MS-OAUT]
    /// section 2.2.32.1. The invkind field of the FUNCDESC MUST be set to INVOKE_PROPERTYPUT, as
    /// specified in [MS-OAUT] section 2.2.14.
    ///
    /// The file format default is 0x0000.
    pub put_value_index: u16,
    /// A variant type that specifies the type of the value that is returned in response to
    /// DISPID_VALUE. The value of this field is the vt field of the TYPEDESC, as specified in
    /// [MS-OAUT] section 2.2.37, of the FUNCDESC.elemdescFunc, as specified in [MS-OAUT] section
    /// 2.2.42, of the function referenced by GetValueIndex or PutValueIndex in this
    /// ClassInfoDataBlock.
    ///
    /// The file format default is 0x0000, VT_EMPTY.
    pub value_type: VarType,
    /// An unsigned integer that specifies the DispID of a property that supports a method for
    /// fetching rows sequentially, getting the data from those rows, and managing rows. The
    /// value of this field is the memid field of the FUNCDESC that specifies the property "set"
    /// method, as specified in [MS-OAUT] section 2.2.42, or of the VARDESC that specifies the
    /// property, as specified in [MS-OAUT] section 2.2.43. The value of memid can be determined by
    /// the algorithm specified in section 2.6.1.1.
    ///
    /// The file format default is 0xFFFFFFFF, DISPID_UNKNOWN.
    pub dispid_rowset: u32,
    /// An unsigned integer that specifies the index of the "set" function into the dynamic virtual
    /// table of the class, for a property that supports a method for fetching rows sequentially,
    /// getting the data from those rows, and managing rows. The value of this field is the oVft
    /// field of the FUNCDESC that specifies the property "set" method, as specified in [MS-OAUT]
    /// section 2.2.42. The value of oVft can be determined by the algorithm specified in section
    /// 2.6.1.2.
    ///
    /// The file format default is 0x0000.
    pub set_rowset: u16,
    /// A Uuid, as specified in [MS-DTYP], that specifies the CLSID of a control.
    ///
    /// The file format default is {00000000-0000-0000-0000-000000000000}.
    pub cls_id: Uuid,
    /// A Uuid, as specified in [MS-DTYP], that specifies the source interface, as specified in
    /// [MS-OAUT] section 2.2.49.8, in this type information.
    ///
    /// The file format default is {00020400-0000-0000-C000-000000000046}.
    pub disp_event: Uuid,
    /// A Uuid, as specified in [MS-DTYP], that specifies the default interface, as specified in
    /// [MS-OAUT] section 2.2.49.8, in this type information.
    ///
    /// The file format default is {00020400-0000-0000-C000-000000000046}.
    pub default_proc: Uuid,
}
