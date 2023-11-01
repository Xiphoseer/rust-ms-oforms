pub type DXMode = u32;

#[repr(i8)]
#[derive(Debug)]
pub enum ClickControlMode {
    /// Use the same value as the client application design-time settings.
    Inherit = -2,
    /// Use the client application default value.
    Default = -1,
    /// Select the control under the cursor, and set the insertion point under the cursor, both on the same click.
    InsertionPoint = 0,
    /// If the control under the cursor is already selected, set the insertion point under the cursor; otherwise, select the control.
    SelectThenInsert = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum DblClickControlMode {
    /// Use the same value as the client application design-time settings.
    Inherit = 0xFE,
    /// Select any text that is under the cursor.
    SelectText = 0x00,
    /// Display and set focus to the code associated with the control that is under the cursor.
    EditCode = 0x01,
    /// Display the properties of the control that is under the cursor.
    EditProperties = 0x02,
}

#[derive(Debug)]
pub struct DesignExtender {
    /// default: 0x00015F55
    pub bit_flags: DXMode,
    /// default: InsertionPoint
    pub click_control_mode: ClickControlMode,
    /// default: SelectText
    pub double_click_control_mode: DblClickControlMode,

    pub grid_x: i32,

    pub grid_y: i32,
}
