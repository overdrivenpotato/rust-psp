use crate::sys::macros;
use crate::sys::SceStubLibraryEntry;
use core::ffi::c_void;

#[repr(u32)]
/// Display mode.
///
/// Display modes other than LCD are unknown.
pub enum DisplayMode {
    // https://github.com/hrydgard/ppsspp/blob/25197451e5cdb1b83dc69fea14c501bdb1e13b1a/Core/HLE/sceDisplay.cpp#L922
    Lcd = 0,

    // TODO: What are the other modes?
}

sys_lib! {
    #![name = "sceDisplay"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x0E20F177)]
    /// Set display mode
    ///
    /// # Parameters
    ///
    /// `mode` - Display mode, normally `DisplayMode::Lcd`.
    /// `width` - Width of screen in pixels.
    /// `height` - Height of screen in pixels.
    ///
    /// # Return value
    ///
    /// ???
    pub unsafe fn sce_display_set_mode(mode: DisplayMode, width: usize, height: usize) -> u32;

    #[psp(0x289D82FE)]
    pub unsafe fn sce_display_set_frame_buf(
        top_addr: *const u8,
        buffer_width: usize,
        pixel_format: u32,
        sync: u32,
    ) -> u32;

    #[psp(0x984C27E7)]
    /// Wait for vertical blank start
    pub unsafe fn sce_display_wait_vblank_start() -> i32; 
}


