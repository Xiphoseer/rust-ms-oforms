//! Specifies properties of embedded controls in a FormControl as persisted to a stream.
use super::Clsid;

bitflags! {
    /// Specifies the properties of the control are not set to the file format default.
    ///
    /// For each bit, a value of zero specifies that the corresponding property is the file format default and is not stored in the file.
    pub struct SitePropMask: u32 {
        /// Specifies whether the size and compression flag of the Name property are stored in the DataBlock.NameData of the OleSiteConcreteControl that contains this SitePropMask and the Name string is stored in the ExtraDataBlock.Name of the OleSiteConcreteControl.
        const NAME                  = 0x00000001;
        /// Specifies whether the size and compression flag of the Tag property are stored in the DataBlock.TagData of the OleSiteConcreteControl that contains this SitePropMask and the Tag string is stored in the ExtraDataBlock.Tag of the OleSiteConcreteControl.
        const TAG                   = 0x00000002;
        /// Specifies whether the ID property is stored in the DataBlock.ID of the OleSiteConcreteControl that contains this SitePropMask.
        const ID                    = 0x00000004;
        /// Specifies whether the HelpContextID property is stored in the DataBlock.HelpContextID of the OleSiteConcreteControl that contains this SitePropMask.
        const HELP_CONTEXT_ID       = 0x00000008;
        /// Specifies whether the BitFlags property is stored in the DataBlock.BitFlags of the OleSiteConcreteControl that contains this SitePropMask.
        const BIT_FLAGS             = 0x00000010;
        /// Specifies whether the ObjectStreamSize property is stored in the DataBlock.ObjectStreamSize of the OleSiteConcreteControl that contains this SitePropMask.
        const OBJECT_STREAM_SIZE    = 0x00000020;
        /// Specifies whether the TabIndex property is stored in the DataBlock.TabIndex of the OleSiteConcreteControl that contains this SitePropMask.
        const TAB_INDEX             = 0x00000040;
        /// Specifies whether the ClsidCacheIndex property is stored in the DataBlock.ClsidCacheIndex of the OleSiteConcreteControl that contains this SitePropMask.
        const CLSID_CACHE_INDEX     = 0x00000080;
        /// Specifies whether the Position property is stored in the ExtraDataBlock.Position of the OleSiteConcreteControl that contains this SitePropMask.
        const POSITION              = 0x00000100;
        /// Specifies whether the GroupID property is stored in the DataBlock.GroupID of the OleSiteConcreteControl that contains this SitePropMask.
        const GROUP_ID              = 0x00000200;

        /// Specifies whether the size and compression flag of the Tooltip property are stored in the DataBlock.ControlTipTextData of the OleSiteConcreteControl that contains this SitePropMask and the Tooltip string is stored in the ExtraDataBlock.ControlTipText of the OleSiteConcreteControl.
        const CONTROL_TIP_TEXT      = 0x00000800;
        /// Specifies whether the size and compression flag of the RuntimeLicKey property are stored in the DataBlock.RuntimeLicKeyData of the OleSiteConcreteControl that contains this SitePropMask and the RuntimeLicKey string is stored in the ExtraDataBlock.RuntimeLicKeyData of the OleSiteConcreteControl.
        const RUNTIME_LIC_KEY       = 0x00001000;
        /// Specifies whether the size and compression flag of the ControlSource property are stored in the DataBlock.ControlSourceData of the OleSiteConcreteControl that contains this SitePropMask and the ControlSource string is stored in the ExtraDataBlock.ControlSource of the OleSiteConcreteControl.
        const CONTROL_SOURCE        = 0x00002000;
        /// Specifies whether the size and compression flag of the RowSource property are stored in the DataBlock.RowSourceData of the OleSiteConcreteControl that contains this SitePropMask and the RowSource string is stored in the ExtraDataBlock.RowSource of the OleSiteConcreteControl.
        const ROW_SOURCE            = 0x00004000;
    }
}

bitflags! {
    pub struct ClsidCacheIndex: u16 {
        const INVALID               = 0x7FFF;
        const INDEX_MASK            = 0x7FFF;
        const IS_FROM_CLASS_TABLE   = 0x8000;
    }
}

impl From<ClsidCacheIndex> for Clsid {
    fn from(v: ClsidCacheIndex) -> Self {
        let value: u16 = (v & ClsidCacheIndex::INDEX_MASK).bits();
        if v.contains(ClsidCacheIndex::IS_FROM_CLASS_TABLE) {
            Clsid::ClassTable(value)
        } else if v == ClsidCacheIndex::INVALID {
            Clsid::Invalid
        } else {
            Clsid::Global(value)
        }
    }
}
