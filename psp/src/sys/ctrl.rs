bitflags::bitflags! {
    /// Enumeration for the digital controller buttons.
    ///
    /// # Note
    ///
    /// Home, Note, Screen, VolUp, VolDown, Disc, WlanUp, Remote, and MS can only be
    /// read in kernel mode.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct CtrlButtons: u32 {
        /// Select button.
        const SELECT = 0x000001;
        /// Start button.
        const START = 0x000008;
        /// Up D-Pad button.
        const UP = 0x000010;
        /// Right D-Pad button.
        const RIGHT = 0x000020;
        /// Down D-Pad button.
        const DOWN = 0x000040;
        /// Left D-Pad button.
        const LEFT = 0x000080;
        /// Left trigger.
        const LTRIGGER = 0x000100;
        /// Right trigger.
        const RTRIGGER = 0x000200;
        /// Triangle button.
        const TRIANGLE = 0x001000;
        /// Circle button.
        const CIRCLE = 0x002000;
        /// Cross button.
        const CROSS = 0x004000;
        /// Square button.
        const SQUARE = 0x008000;
        /// Home button. In user mode this bit is set if the exit dialog is visible.
        const HOME = 0x010000;
        /// Hold button.
        const HOLD = 0x020000;
        /// Music Note button.
        const NOTE = 0x800000;
        /// Screen button.
        const SCREEN = 0x400000;
        /// Volume up button.
        const VOL_UP = 0x100000;
        /// Volume down button.
        const VOL_DOWN = 0x200000;
        /// Wlan switch up.
        const WLAN_UP = 0x040000;
        /// Remote hold position.
        const REMOTE = 0x080000;
        /// Disc present.
        const DISC = 0x1000000;
        /// Memory stick present.
        const MEM_STICK = 0x2000000;
    }
}

/// Controller mode.
#[repr(u32)]
pub enum CtrlMode {
    /// Digital.
    Digital = 0,
    /// Analog.
    Analog,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
/// Returned controller data
pub struct SceCtrlData {
    /// The current read frame.
    pub timestamp: u32,
    /// Bit mask containing zero or more of `CtrlButtons`.
    pub buttons: CtrlButtons,
    /// Analogue stick, X axis.
    pub lx: u8,
    /// Analogue stick, Y axis.
    pub ly: u8,
    /// Reserved.
    pub rsrv: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SceCtrlLatch {
    pub ui_make: u32,
    pub ui_break: u32,
    pub ui_press: u32,
    pub ui_release: u32,
}

psp_extern! {
    #![name = "sceCtrl"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x6A2774F3)]
    /// Set the controller cycle setting.
    ///
    /// # Parameters
    ///
    /// - `cycle`: Cycle. Normally set to 0.
    ///
    /// # Return value
    ///
    /// The previous cycle setting.
    pub fn sceCtrlSetSamplingCycle(cycle: i32) -> i32;

    #[psp(0x02BAAD91)]
    /// Get the controller current cycle setting.
    ///
    /// # Parameters
    ///
    /// - `pcycle`: Return value.
    ///
    /// # Return value
    ///
    /// 0
    pub fn sceCtrlGetSamplingCycle(pcycle: *mut i32) -> i32;

    #[psp(0x1F4011E6)]
    /// Set the controller mode.
    ///
    /// # Parameters
    ///
    /// - `mode`: One of `CtrlMode`.
    ///
    /// # Return Value
    ///
    /// The previous mode.
    pub fn sceCtrlSetSamplingMode(mode: CtrlMode) -> i32;

    #[psp(0xDA6B76A1)]
    /// Get the current controller mode.
    ///
    /// # Parameters
    ///
    /// - `pmode`: Return value.
    ///
    /// # Return value
    ///
    /// 0
    pub fn sceCtrlGetSamplingMode(pmode: *mut i32) -> i32;

    #[psp(0x3A622550)]
    pub fn sceCtrlPeekBufferPositive(pad_data: *mut SceCtrlData, count: i32) -> i32;

    #[psp(0xC152080A)]
    pub fn sceCtrlPeekBufferNegative(pad_data: *mut SceCtrlData, count: i32) -> i32;

    #[psp(0x1F803938)]
    /// Read buffer positive
    ///
    /// # Parameters
    ///
    /// - `pad_data`: Pointer to a `SceCtrlData` structure used to hold the returned pad data.
    /// - `count`: Number of `SceCtrlData` buffers to read.
    pub fn sceCtrlReadBufferPositive(pad_data: *mut SceCtrlData, count: i32) -> i32;

    #[psp(0x60B81F86)]
    pub fn sceCtrlReadBufferNegative(pad_data: *mut SceCtrlData, count: i32) -> i32;

    #[psp(0xB1D0E5CD)]
    pub fn sceCtrlPeekLatch(latch_data: *mut SceCtrlLatch) -> i32;

    #[psp(0x0B588501)]
    pub fn sceCtrlReadLatch(latch_data: *mut SceCtrlLatch) -> i32;

    #[psp(0xA7144800)]
    /// Set analog threshold relating to the idle timer.
    ///
    /// # Parameters
    ///
    /// - `idlereset`:  Movement needed by the analog to reset the idle timer.
    /// - `idleback`: Movement needed by the analog to bring the PSP back from
    ///   an idle state.
    ///
    ///   Set to -1 for analog to not cancel idle timer.
    ///
    ///   Set to 0 for idle timer to be cancelled even if the analog is not moved.
    ///
    ///   Set between 1-128 to specify the movement on either axis needed by the analog
    ///   to fire the  event.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceCtrlSetIdleCancelThreshold(idlereset: i32, idleback: i32) -> i32;

    #[psp(0x687660FA)]
    /// Get the idle threshold values.
    ///
    /// # Parameters
    ///
    /// - `idlereset`: Movement needed by the analog to reset the idle timer.
    /// - `idleback`: Movement needed by the analog to bring the PSP back from
    ///   an idle state.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceCtrlGetIdleCancelThreshold(idlereset: *mut i32, idleback: *mut i32) -> i32;
}
