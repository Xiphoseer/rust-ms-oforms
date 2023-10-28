//! Specifies properties stored for each embedded control in a UserForm control.
mod parser;
use std::num::NonZeroU16;

pub use parser::*;
pub mod stream;

use crate::properties::Position;

bitflags! {
    /// Specifies Boolean properties of an embedded control on a form.
    ///
    /// Unless otherwise specified,
    /// each bit applies to all control types. All bits that do not apply to a particular type of
    /// control MUST be set to zero for that control.
    #[derive(Debug)]
    pub struct SiteFlags: u32 {
        /// Specifies whether the control can receive focus while the user is navigating
        /// controls using the TAB key.
        const TAB_STOP          = 0x00000001;
        /// Specifies whether the control is displayed.
        const VISIBLE           = 0x00000002;
        /// Specifies whether the control is the default option on the form.
        const DEFAULT           = 0x00000004;
        /// Specifies whether the control is the cancel option on the form.
        const CANCEL            = 0x00000008;
        /// Specifies whether the control is stored in the Object stream of the form. A value of
        /// zero specifies that the control has its own storage.
        const STREAMED          = 0x00000010;
        /// Specifies whether the control automatically resizes to display its entire contents.
        const AUTO_SIZE         = 0x00000020;

        /// Specifies whether to preserve the height of a control when resizing. Applies to ListBox.
        const PRESERVE_HEIGHT   = 0x00000100;
        /// Specifies whether to adjust the size of a control when the size of its parent changes.
        const FIT_TO_PARENT     = 0x00000200;

        /// Specifies whether to select the first child of a container control when the container
        /// control is the next control to which the user is navigating.
        const SELECT_CHILD      = 0x00002000;

        /// Specifies whether child controls are identified as child objects of the
        /// control or as child objects of the parent of the control. MUST be set to 1 for the
        /// following controls: Frame, MultiPage and Page. MUST be set to zero for all other
        /// controls.
        const PROMOTE_CONTROLS  = 0x00040000;
    }
}

#[derive(Debug)]
pub enum Clsid {
    ClassTable(u16),
    Invalid,
    Global(u16),
}

/// Specifies properties stored for each embedded control in a UserForm control.
#[derive(Debug)]
pub struct OleSiteConcrete {
    pub id: i32,
    pub help_context_id: i32,
    /// A SITE_FLAG that specifies Boolean properties of an embedded control on a form.
    ///
    /// The file format default is 0x00000033, which means that the following flags are set to TRUE:
    /// fTabStop, fVisible, fStreamed, and fAutoSize.
    pub bit_flags: SiteFlags,
    /// An unsigned integer that specifies the size, in bytes, of an embedded control that is
    /// persisted to the Object stream of a Form.
    ///
    /// The file format default is 0x00000000.
    pub object_stream_size: u32,
    /// A signed integer that specifies the index of an embedded control in the tab order of a form.
    /// Values less than zero specify an invalid index in the tab order.
    ///
    /// The file format default is 0xFFFF, or –1, an invalid index.
    pub tab_index: i16,
    /// An unsigned integer that specifies the type of a FormEmbeddedActiveXControl on a parent control. A
    /// value less than 0x7FFF specifies an index value for FormEmbeddedActiveXControlCached. A value of
    /// 0x7FFF specifies that the index is invalid. A value greater than or equal to 0x8000 specifies an index
    /// into the FormSiteData.ClassTable of the FormControl in which the control is embedded, where
    /// information about the control is specified by the entry in ClassTable that corresponds to the value of
    /// this property minus 0x8000.
    ///
    /// The file format default is 0x7FFF, an invalid index.
    pub clsid_cache_index: Clsid,
    /// An unsigned integer that specifies the control group of a control. A value of zero specifies that the
    /// control is not in a control group. A value greater than zero specifies the unique identifier of the control
    /// group to which the control belongs. All controls that have the same value for this property are in the
    /// same control group.
    ///
    /// The file format default is 0x0000.
    pub group_id: Option<NonZeroU16>,

    /// An fmString that specifies the name of a control.
    ///
    /// The file format default is a zero-length string.
    pub name: String,

    /// An fmString that is associated with a control and that contains data entered by the user. SHOULD be
    /// ignored.<13>
    ///
    /// The file format default is a zero-length string.
    pub tag: String,

    /// An fmPosition that specifies the location of the top-left corner of an embedded control on a form,
    /// relative to the top-left corner of the LogicalSize of the form.
    /// The file format default is (0, 0), which specifies that the top-left corner of the embedded control is at
    /// the top-left corner of the form.
    pub site_position: Position,

    /// An fmString that specifies the tooltip for the control.
    ///
    /// The file format default is a zero-length string.
    pub control_tip_text: String,

    /// An fmString that specifies the license key of a control.
    ///
    /// The file format default is a zero-length string.
    pub runtime_lic_key: String,

    /// An fmString that specifies a cell in a worksheet that sets the Value property of a control when the
    /// control is loaded and to which the new value of the Value property is stored after it changes in the
    /// control.
    ///
    /// The file format default is a zero-length string.
    pub control_source: String,

    /// An fmString that specifies the source for the list of values in a ComboBox or ListBox that is embedded
    /// in a form. This property MUST NOT be set for other controls. The format of the string is a range of
    /// cells in a worksheet.
    ///
    /// The file format default is a zero-length string.
    pub row_source: String,
}
