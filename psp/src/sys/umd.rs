/// UMD Info
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UmdInfo {
    /// This should hold the size of this struct.
    pub size: u32,
    pub type_: UmdType,
}

/// UMD type
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum UmdType {
    Game = 0x10,
    Video = 0x20,
    Audio = 0x40,
}

bitflags::bitflags! {
    /// UMD drive state
    pub struct UmdStateFlags: i32 {
        const NOT_PRESENT = 0x01;
        const PRESENT = 0x02;
        const CHANGED = 0x04;
        const INITING = 0x08;
        const INITED = 0x10;
        const READY = 0x20;
    }
}

/// Callback type
pub type UmdCallback = fn(unknown: i32, event: i32) -> i32;

psp_extern! {
    #![name = "sceUmdUser"]
    #![flags = 0x4001]
    #![version = (0x00, 0x11)]

    #[psp(0x46EBB729)]
    /// Check whether there is a disc in the UMD drive
    ///
    /// # Return Value
    ///
    /// 0 if no disc present, anything else indicates a disc is inserted.
    pub fn sce_umd_check_medium() -> i32;

    #[psp(0x340B7686)]
    /// Get the disc info
    ///
    /// # Parameters
    ///
    /// - `info`: An out pointer to a `UmdInfo` instance.
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_get_disc_info(info: *mut UmdInfo) -> i32;

    #[psp(0xC6183D47)]
    /// Activates the UMD drive
    ///
    /// # Parameters
    ///
    /// - `unit`: The unit to initialise (probably). Should be set to 1.
    /// - `drive`: A prefix string for the fs device to mount the UMD on (e.g. "disc0:")
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_activate(unit: i32, drive: *const u8) -> i32;

    #[psp(0xE83742BA)]
    /// Deativates the UMD drive
    ///
    /// # Parameters
    ///
    /// - `unit`: The unit to initialise (probably). Should be set to 1.
    /// - `drive`: A prefix string for the fs device to mount the UMD on (e.g. "disc0:")
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_deactivate(unit: i32, drive: *const u8) -> i32;

    #[psp(0x8EF08FCE)]
    /// Wait for the UMD drive to reach a certain state
    ///
    /// # Parameters
    ///
    /// - `state`: UMD state(s) to wait for
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_wait_drive_stat(state: UmdStateFlags) -> i32;

    #[psp(0x56202973)]
    /// Wait for the UMD drive to reach a certain state
    ///
    /// # Parameters
    ///
    /// - `state`: UMD state(s) to wait for
    ///
    /// # Parameters
    ///
    /// - `timeout`: Timeout value in microseconds
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_wait_drive_stat_with_timer(
        state: UmdStateFlags,
        timeout: u32,
    ) -> i32;

    #[psp(0x4A9E5E29)]
    /// Wait for the UMD drive to reach a certain state (plus callback)
    ///
    /// # Parameters
    ///
    /// - `state`: UMD state(s) to wait for
    ///
    /// # Parameters
    ///
    /// - `timeout`: Timeout value in microseconds
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_wait_drive_stat_cb(
        state: UmdStateFlags,
        timeout: u32,
    ) -> i32;

    #[psp(0x6AF9B50A)]
    /// Cancel a `sce_umd_wait_*` call
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_cancel_wait_drive_stat() -> i32;

    #[psp(0x6B4A146C)]
    /// Get (poll) the current state of the UMD drive
    ///
    /// # Return Value
    ///
    /// < 0 on error, one or more of `UmdStateFlags` on success
    pub fn sce_umd_get_drive_stat() -> i32;

    #[psp(0x20628E6F)]
    /// Get the error code associated with a failed event
    ///
    /// # Return Value
    ///
    /// < 0 on error, the error code on success
    pub fn sce_umd_get_error_stat() -> i32;

    #[psp(0xAEE7404D)]
    /// Register a callback for the UMD drive
    ///
    /// # Parameters
    ///
    /// - `cbid`: A callback ID created from `sce_kernel_create_callback`.
    ///   Callback should be of type `UmdCallback`.
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_register_umd_callback(cbid: i32) -> i32;

    #[psp(0xBD2BDE07)]
    /// Un-register a callback for the UMD drive
    ///
    /// # Parameters
    ///
    /// - `cbid`: A callback ID created from `sce_kernel_create_callback`
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_un_register_umd_callback(cbid: i32) -> i32;

    #[psp(0xCBE9F02A)]
    /// Permit UMD disc being replaced
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_replace_permit() -> i32;

    #[psp(0x87533940)]
    /// Prohibit UMD disc being replaced
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sce_umd_replace_prohibit() -> i32;
}
