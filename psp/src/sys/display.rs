use core::ffi::c_void;
use num_enum::TryFromPrimitive;

#[repr(u32)]
/// Display mode.
///
/// Display modes other than LCD are unknown.
pub enum DisplayMode {
    // https://github.com/hrydgard/ppsspp/blob/25197451e5cdb1b83dc69fea14c501bdb1e13b1a/Core/HLE/sceDisplay.cpp#L922
    Lcd = 0,
    // TODO: What are the other modes?
}

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u32)]
/// Framebuffer pixel formats.
pub enum DisplayPixelFormat {
    /// 16-bit RGB 5:6:5
    Psm5650 = 0,
    /// 16-bit RGBA 5:5:5:1
    Psm5551 = 1,
    /// 16-bit RGBA 4:4:4:4
    Psm4444 = 2,
    /// 32-bit RGBA 8:8:8:8
    Psm8888 = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DisplaySetBufSync {
    /// Buffer change effective immediately
    Immediate = 0,
    /// Buffer change effective next frame
    NextFrame = 1,
}

psp_extern! {
    #![name = "sceDisplay"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x0E20F177)]
    /// Set display mode
    ///
    /// # Parameters
    ///
    /// - `mode`: Display mode, normally `DisplayMode::Lcd`.
    /// - `width`: Width of screen in pixels.
    /// - `height`: Height of screen in pixels.
    ///
    /// # Return value
    ///
    /// ???
    pub fn sceDisplaySetMode(mode: DisplayMode, width: usize, height: usize) -> u32;

    #[psp(0xDEA197D4)]
    /// Get display mode
    ///
    /// # Parameters
    ///
    /// - `pmode`: Pointer to an integer to receive the current mode.
    /// - `pwidth`: Pointer to an integer to receive the current width.
    /// - `pheight`: Pointer to an integer to receive the current height.
    ///
    /// # Return value
    ///
    /// 0 on success
    pub fn sceDisplayGetMode(pmode: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> i32;

    #[psp(0x289D82FE)]
    /// Display set framebuffer
    ///
    /// # Parameters
    ///
    /// - `top_addr`: Address of start of framebuffer
    /// - `buffer_width`: Buffer width (must be power of 2)
    /// - `pixel_format`: One of `PspDisplayPixelFormats`.
    /// - `sync`: One of `PspDisplaySetBufSync`
    ///
    /// # Return value
    ///
    /// 0 on success
    pub fn sceDisplaySetFrameBuf(
        top_addr: *const u8,
        buffer_width: usize,
        pixel_format: DisplayPixelFormat,
        sync: DisplaySetBufSync,
    ) -> u32;

    #[psp(0xEEDA2E54)]
    /// Get display framebuffer information
    ///
    /// # Parameters
    ///
    /// - `top_addr`: Pointer to void* to receive address of start of framebuffer
    /// - `buffer_width`: Pointer to usize to receive buffer width (must be power of 2)
    /// - `pixelformat`: Pointer to receive DisplayPixelFormat.
    /// - `sync`: One of `DisplaySetBufSync`
    pub fn sceDisplayGetFrameBuf(
        top_addr: *mut *mut c_void,
        buffer_width: *mut usize,
        pixel_format: *mut DisplayPixelFormat,
        sync: DisplaySetBufSync,
    ) -> i32;

    #[psp(0x9C6EAAD7)]
    /// Number of vertical blank pulses up to now
    pub fn sceDisplayGetVcount() -> u32;

    #[psp(0x36CDFADE)]
    /// Wait for vertical blank
    pub fn sceDisplayWaitVblank() -> i32;

    #[psp(0x8EB9EC49)]
    /// Wait for vertical blank with callback
    pub fn sceDisplayWaitVblankCB() -> i32;

    #[psp(0x984C27E7)]
    /// Wait for vertical blank start
    pub fn sceDisplayWaitVblankStart() -> i32;

    #[psp(0x46F186C3)]
    /// Wait for vertical blank start with callback
    pub fn sceDisplayWaitVblankStartCB() -> i32;

    #[psp(0x210EAB3A)]
    /// Get accumulated HSYNC count
    pub fn sceDisplayGetAccumulatedHcount() -> i32;

    #[psp(0x773DD3A3)]
    /// Get current HSYNC count
    pub fn sceDisplayGetCurrentHcount() -> i32;

    #[psp(0xDBA6C4C4)]
    /// Get number of frames per second
    pub fn sceDisplayGetFramePerSec() -> f32;

    #[psp(0xB4F378FA)]
    /// Get whether or not frame buffer is being displayed
    pub fn sceDisplayIsForeground() -> i32;

    #[psp(0x4D4E10EC)]
    /// Test whether vblank is active
    pub fn sceDisplayIsVblank() -> i32;
}
