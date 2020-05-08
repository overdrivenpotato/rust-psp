/// Enumeration for the digital controller buttons.
/// NOTE: Home, Note, Screen, VolUp, VolDown, Disc, WlanUp, Remote, MS can only be read in kernel mode
#[repr(u32)]
pub enum PspCtrlButtons {
    /// Select button. 
    Select     = 0x000001,
    /// Start button. 
    Start      = 0x000008,
    /// Up D-Pad button.
    Up         = 0x000010,
    /// Right D-Pad button.
    Right      = 0x000020,
    /// Down D-Pad button.
    Down      	= 0x000040,
    /// Left D-Pad button.
    Left      	= 0x000080,
    /// Left trigger.
    LTrigger   = 0x000100,
    /// Right trigger.
    RTrigger   = 0x000200,
    /// Triangle button.
    Triangle   = 0x001000,
    /// Circle button.
    Circle     = 0x002000,
    /// Cross button.
    Cross      = 0x004000,
    /// Square button.
    Square     = 0x008000,
    /// Home button. In user mode this bit is set if the exit dialog is visible.
    Home       = 0x010000,
    /// Hold button. 
    Hold       = 0x020000,
    /// Music Note button. 
    Note       = 0x800000,
    /// Screen button. 
    Screen     = 0x400000,
    /// Volume up button. 
    VolUp      = 0x100000,
    /// Volume down button. 
    VolDown    = 0x200000,
    /// Wlan switch up. 
    WlanUp    = 0x040000,
    /// Remote hold position.
    Remote     = 0x080000,	
    /// Disc present. 
    Disc       = 0x1000000,
    /// Memory stick present. 
    MS         = 0x2000000,
}

/// Controller mode.
#[repr(u32)]
pub enum PspCtrlMode
{
    /// Digital.
    Digital = 0,
    /// Analog.
    Analaog
}

#[repr(C)]
/// Returned controller data
pub struct SceCtrlData {
    /// The current read frame.
    pub TimeStamp: u32,
    /// Bit mask containing zero or more of ::PspCtrlButtons.
    pub Buttons: u32,
    /// Analogue stick, X axis.
    pub Lx: u8,
    /// Analogue stick, Y axis.
    pub Ly: u8,
    /// Reserved.
    pub Rsrv: [u8; 6usize],
}

#[repr(C)]
pub struct SceCtrlLatch {
    pub uiMake: u32,
    pub uiBreak: u32,
    pub uiPress: u32,
    pub uiRelease: u32,
}

sys_lib! {
    #![name = "sceCtrl"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x6A2774F3)]
    /// Set the controller cycle setting.
    ///
    /// Parameters
    ///
    /// `cycle` - Cycle. Normally set to 0.
    ///
    /// Return value
    ///
    /// The previous cycle setting.
    pub unsafe fn sce_ctrl_set_sampling_cycle(cycle: i32) -> i32;

    #[psp(0x02BAAD91)]
    /// Get the controller current cycle setting.
    /// 
    /// Parameters
    ///
    /// `pcycle` - Return value.
    ///
    /// Return value
    ///
    /// 0
    pub unsafe fn sce_ctrl_get_sampling_cycle(pcycle: *mut i32) -> i32;

    #[psp(0x1F4011E6)]
    /// Set the controller mode.
    ///
    /// Parameters
    ///
    /// `mode` - One of PspCtrlMode.
    ///
    /// Return Value
    ///
    /// The previous mode.
    pub unsafe fn sce_ctrl_set_sampling_mode(mode: i32) -> i32;
    
    #[psp(0xDA6B76A1)]
    /// Get the current controller mode.
    ///
    /// Parameters
    ///
    /// `pmode` - Return value.
    ///
    /// Return value
    ///
    /// 0
    pub unsafe fn sce_ctrl_get_sampling_mode(pmode: *mut i32) -> i32;

    #[psp(0x3A622550)]
    pub unsafe fn sce_ctrl_peek_buffer_positive(pad_data: *mut SceCtrlData, count: i32) -> i32;

    #[psp(0xC152080A)]
    pub unsafe fn sce_ctrl_peek_buffer_negative(pad_data: *mut SceCtrlData, count: i32) -> i32; 

    #[psp(0x1F803938)]
    /// Read buffer positive
    ///
    /// Parameters
    ///
    /// `pad_data` - Pointer to a SceCtrlData structure used to hold the returned pad data.
    /// `count` - Number of SceCtrlData buffers to read.
    pub unsafe fn sce_ctrl_read_buffer_positive(pad_data: *mut SceCtrlData, count: i32) -> i32;

    #[psp(0x60B81F86)]
    pub unsafe fn sce_ctrl_read_buffer_negative(pad_data: *mut SceCtrlData, count: i32) -> i32;

    #[psp(0xB1D0E5CD)]
    pub unsafe fn sce_ctrl_peek_latch(latch_data: *mut SceCtrlLatch) -> i32;

    #[psp(0x0B588501)]
    pub unsafe fn sce_ctrl_read_latch(latch_data: *mut SceCtrlLatch) -> i32;

    #[psp(0xA7144800)]
    /// Set analog threshold relating to the idle timer.
    ///
    /// Parameters
    ///
    /// `idlereset` -  Movement needed by the analog to reset the idle timer.
    /// `idleback` - Movement needed by the analog to bring the PSP back from an idle state.
    /// Set to -1 for analog to not cancel idle timer.
    /// Set to 0 for idle timer to be cancelled even if the analog is not moved.
    /// Set between 1-128 to specify the movement on either axis needed by the analog 
    /// to fire the  event.
    ///
    /// Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ctrl_set_idle_cancel_threshold(idlereset: i32, idleback: i32) -> i32;

    #[psp(0x687660FA)]
    /// Get the idle threshold values.
    ///
    /// Parameters
    ///
    /// `idlereset` - Movement needed by the analog to reset the idle timer.
    /// `idleback` - Movement needed by the analog to bring the PSP back from an idle state.
    ///
    /// Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ctrl_get_idle_cancel_threshold(idlereset: *mut i32, idleback: *mut i32) -> i32

}

