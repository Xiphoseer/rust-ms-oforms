bitflags! {
    /// Specifies the properties of the control that are not set to the file format default.
    ///
    /// For each bit, a value of zero specifies that the corresponding property is the file format default and is not stored in the file.
    pub struct FormPropMask: u32 {
        /// Specifies whether the BackColor property is stored in the DataBlock.BackColor of the FormControl that contains this FormPropMask.
        const BACK_COLOR            = 0x00000002;
        /// Specifies whether the ForeColor property is stored in the DataBlock.ForeColor of the FormControl that contains this FormPropMask.
        const FORE_COLOR            = 0x00000004;
        /// Specifies whether the NextAvailableID property is stored in the DataBlock.NextAvailableID of the FormControl that contains this FormPropMask.
        const NEXT_AVAILABLE_ID     = 0x00000008;

        /// Specifies whether the BooleanProperties property is stored in the DataBlock.BooleanProperties of the FormControl that contains this FormPropMask.
        const BOOLEAN_PROPERTIES    = 0x00000040;
        /// Specifies whether the BorderStyle property is stored in the DataBlock.BorderStyle of the FormControl that contains this FormPropMask.
        const BORDER_STYLE          = 0x00000080;
        /// Specifies whether the MousePointer property is stored in the DataBlock.MousePointer of the FormControl that contains this FormPropMask.
        const MOUSE_POINTER         = 0x00000100;
        /// Specifies whether the ScrollBars property is stored in the DataBlock.ScrollBars of the FormControl that contains this FormPropMask.
        const SCROLL_BARS           = 0x00000200;
        /// Specifies whether the DisplayedSize property is stored in the ExtraDataBlock.DisplayedSize of the FormControl that contains this FormPropMask.
        const DISPLAYED_SIZE        = 0x00000400;
        /// Specifies whether the LogicalSize property is stored in the ExtraDataBlock.LogicalSize of the FormControl that contains this FormPropMask.
        const LOGICAL_SIZE          = 0x00000800;
        /// Specifies whether the ScrollPosition property is stored in the ExtraDataBlock.ScrollPosition of the FormControl that contains this FormPropMask.
        const SCROLL_POSITION       = 0x00001000;
        /// Specifies whether the GroupCount property is stored in the DataBlock.GroupCnt of the FormControl that contains this FormPropMask.
        const GROUP_CNT             = 0x00002000;

        /// Specifies whether the MouseIcon property is stored in the StreamData.MouseIcon of the FormControl that contains this FormPropMask. When this bit is set to 1, a value of 0xFFFF MUST be stored in the DataBlock.MouseIcon of the FormControl.
        const MOUSE_ICON            = 0x00008000;
        /// Specifies whether the Cycle property is stored in the DataBlock.Cycle of the FormControl that contains this FormPropMask.
        const CYCLE                 = 0x00010000;
        /// Specifies whether the SpecialEffect property is stored in the DataBlock.SpecialEffect of the FormControl that contains this FormPropMask.
        const SPECIAL_EFFECT        = 0x00020000;
        /// Specifies whether the BorderColor property is stored in the DataBlock.BorderColor of the FormControl that contains this FormPropMask.
        const BORDER_COLOR          = 0x00040000;
        /// Specifies whether the size and compression flag of the Caption property are stored in the DataBlock.LengthAndCompression of the FormControl that contains this FormPropMask and the Caption string is stored in the ExtraDataBlock.Caption of the FormControl.
        const CAPTION               = 0x00080000;
        /// Specifies whether the Font property is stored in the StreamData.GuidAndFont of the FormControl that contains this FormPropMask.
        const FONT                  = 0x00100000;
        /// Specifies whether the Picture property is stored in the StreamData.Picture of the FormControl that contains this FormPropMask.
        const PICTURE               = 0x00200000;
        /// Specifies whether the Zoom property is stored in the DataBlock.Zoom of the FormControl that contains this FormPropMask.
        const ZOOM                  = 0x00400000;
        /// Specifies whether the PictureAlignment property is stored in the DataBlock.PictureAlignment of the FormControl that contains this FormPropMask.
        const PICTURE_ALIGNMENT     = 0x00800000;
        /// Specifies whether the value of the PictureTiling property is not the file format default.
        const PICTURE_TILING        = 0x01000000;
        /// Specifies whether the PictureSizeMode property is stored in the DataBlock.PictureSizeMode of the FormControl that contains this FormPropMask.
        const PICTURE_SIZE_MODE     = 0x02000000;
        /// Specifies whether the ShapeCookie property is stored in the DataBlock.ShapeCookie of the FormControl that contains this FormPropMask.
        const SHAPE_COOKIE          = 0x04000000;
        /// Specifies whether the DrawBuffer property is stored in the DataBlock.DrawBuffer of the FormControl that contains this FormPropMask. MUST be set to 1.
        const DRAW_BUFFER           = 0x08000000;
    }
}

bitflags! {
    /// Specifies the properties of the SiteClassInfo that contains this ClassInfoPropMask that are
    /// not set to the file format default.
    ///
    /// For each bit, a value of zero specifies that the corresponding property is the file format
    /// default and is not stored in the file.
    pub struct ClassInfoPropMask: u32 {
        /// Specifies whether ExtraDataBlock.ClsID is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const CLS_ID            = 0x00000001;
        /// Specifies whether ExtraDataBlock.DispEvent is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const DISP_EVENT        = 0x00000002;

        /// Specifies whether ExtraDataBlock.DefaultProg is stored in the SiteClassInfo that
        /// contains this ClassInfoPropMask.
        const DEFAULT_PROC      = 0x00000008;
        /// Specifies whether DataBlock.ClassTableFlags and DataBlock.VarFlags are stored in the
        /// SiteClassInfo that contains this ClassInfoPropMask.
        const CLASS_FLAGS       = 0x00000010;
        /// Specifies whether DataBlock.CountOfMethods is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const COUNT_OF_METHODS  = 0x00000020;
        /// Specifies whether DataBlock.DispidBind is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const DISPID_BIND       = 0x00000040;
        /// Specifies whether DataBlock.GetBindIndex is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const GET_BIND_INDEX    = 0x00000080;
        /// Specifies whether DataBlock.PutBindIndex is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const PUT_BIND_INDEX    = 0x00000100;
        /// Specifies whether DataBlock.BindType is stored in the SiteClassInfo that contains this
        /// ClassInfoPropMask.
        const BIND_TYPE         = 0x00000200;
        /// Specifies whether DataBlock.GetValueIndex is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const GET_VALUE_INDEX   = 0x00000400;
        /// Specifies whether DataBlock.PutValueIndex is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const PUT_VALUE_INDEX   = 0x00000800;
        /// Specifies whether DataBlock.ValueType is stored in the SiteClassInfo that contains this
        /// ClassInfoPropMask.
        const VALUE_TYPE        = 0x00001000;
        /// Specifies whether DataBlock.DispidRowset is stored in the SiteClassInfo that contains
        /// this ClassInfoPropMask.
        const DISPID_ROWSET     = 0x00002000;
        /// Specifies whether DataBlock.SetRowset is stored in the SiteClassInfo that contains this
        /// ClassInfoPropMask.
        const SET_ROWSET        = 0x00004000;
    }
}

bitflags! {
    /// Specifies the type of this site or the count of sites with a following type
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct TypeOrCount: u8 {
        /// Mask for the data part fo this field
        const TYPE_OR_COUNT = 0x7F;
        /// Flag fCount
        const IS_COUNT      = 0x80;
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
/// The type of a concrete site
pub enum SiteType {
    /// An OleSiteConcrete
    Ole = 0x01,
}

/// The depth and type of a site
#[derive(Debug, Copy, Clone)]
pub struct SiteDepthAndType {
    /// The depth of the site
    pub depth: u8,
    /// The type of the site
    pub r#type: SiteType,
}
