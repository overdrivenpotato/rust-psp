use crate::sys::SceUid;
use core::ffi::c_void;

/// For use with `sceUsbActivate` and `sceUsbDeactivate`.
pub const USB_CAM_PID: i32 = 0x282;

pub const USB_BUS_DRIVER_NAME: &str = "USBBusDriver";
pub const USB_CAM_DRIVER_NAME: &str = "USBCamDriver";
pub const USB_CAM_MIC_DRIVER_NAME: &str = "USBCamMicDriver";
pub const USB_STOR_DRIVER_NAME: &str = "USBStor_Driver";

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct UsbState: i32 {
        const ACTIVATED = 0x200;
        const CONNECTED = 0x020;
        const ESTABLISHED = 0x002;
    }
}

psp_extern! {
    #![name = "sceUsb"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0xAE5DE6AF)]
    /// Start a USB driver.
    ///
    /// # Parameters
    ///
    /// - `driver_name`: name of the USB driver to start. You probably want to
    ///   use `USB_BUS_DRIVER_NAME`. Other driver name constants are also
    ///   available for the camera.
    /// - `size`: Size of arguments to pass to USB driver start
    /// - `args`: Arguments to pass to USB driver start
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUsbStart(
        driver_name: *const u8,
        size: i32,
        args: *mut c_void,
    ) -> i32;

    #[psp(0xC2464FA0)]
    /// Stop a USB driver.
    ///
    /// # Parameters
    ///
    /// - `driver_name`: name of the USB driver to stop
    /// - `size`: Size of arguments to pass to USB driver start
    /// - `args`: Arguments to pass to USB driver start
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUsbStop(
        driver_name: *const u8,
        size: i32,
        args: *mut c_void,
    ) -> i32;

    #[psp(0x586DB82C)]
    /// Activate a USB driver.
    ///
    /// # Parameters
    ///
    /// - `pid`: Product ID for the default USB Driver. An example is the
    ///   constant `USB_CAM_PID`.
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUsbActivate(pid: u32) -> i32;

    #[psp(0xC572A9C8)]
    /// Deactivate USB driver.
    ///
    /// # Parameters
    ///
    /// - `pid`: Product ID for the default USB driver
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUsbDeactivate(pid: u32) -> i32;

    #[psp(0xC21645A4)]
    /// Get USB state
    ///
    /// # Return Value
    ///
    /// USB `State`.
    pub fn sceUsbGetState() -> UsbState;

    #[psp(0x112CC951)]
    /// Get state of a specific USB driver
    ///
    /// # Parameters
    ///
    /// - `driver_name`: name of USB driver to get status from
    ///
    /// # Return Value
    ///
    /// 1 if the driver has been started, 2 if it is stopped
    pub fn sceUsbGetDrvState(driver_name: *const u8) -> i32;
}

/// Structure for `sceUsbCamSetupStill`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsbCamSetupStillParam {
    /// Size of the `UsbCamSetupStillParam` structure.
    pub size: i32,
    /// Resolution.
    pub resolution: UsbCamResolution,
    /// Size of the jpeg image.
    pub jpeg_size: i32,
    /// Reverse effect to apply. Zero or more of `UsbCamReverseFlags`.
    pub reverse_flags: UsbCamReverseFlags,
    /// Delay to apply to take the picture.
    pub delay: UsbCamDelay,
    /// JPEG compression level, a value from 1-63.
    ///
    /// 1 -> less compression, better quality; 63 -> max compression, worse quality.
    pub comp_level: i32,
}

/// Structure for `sceUsbCamSetupStillEx`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsbCamSetupStillExParam {
    /// Size of the `UsbCamSetupStillExParam` structure.
    pub size: i32,
    /// Unknown, set it to 9 at the moment.
    pub unk: u32,
    /// Resolution.
    pub resolution: UsbCamResolutionEx,
    /// Size of the jpeg image.
    pub jpeg_size: i32,
    /// JPEG compression level, a value from 1-63.
    ///
    /// 1 -> less compression, better quality; 63 -> max compression, worse quality.
    pub comp_level: i32,
    /// Unknown, set it to 0 at the moment.
    pub unk2: u32,
    /// Unknown, set it to 1 at the moment.
    pub unk3: u32,
    /// Flag that indicates whether to flip the image.
    pub flip: i32,
    /// Flag that indicates whether to mirror the image.
    pub mirror: i32,
    /// Delay to apply to take the picture.
    pub delay: UsbCamDelay,
    /// Unknown, set it to 0 at the moment.
    pub unk4: [u32; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsbCamSetupVideoParam {
    /// Size of the `UsbCamSetupVideoParam` structure.
    pub size: i32,
    /// Resolution.
    pub resolution: UsbCamResolution,
    /// Framerate.
    pub framerate: UsbCamFrameRate,
    /// White balance.
    pub white_balance: UsbCamWb,
    /// Saturation (0-255).
    pub saturation: i32,
    /// Brightness (0-255).
    pub brightness: i32,
    /// Contrast (0-255).
    pub contrast: i32,
    /// Sharpness (0-255).
    pub sharpness: i32,
    /// Effect mode.
    pub effect_mode: UsbCamEffectMode,
    /// Size of jpeg video frame.
    pub frame_size: i32,
    /// Unknown. Set it to 0 at the moment.
    pub unk: u32,
    /// Exposure value.
    pub evl_evel: UsbCamEvLevel,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsbCamSetupVideoExParam {
    /// Size of the `UsbCamSetupVideoParam` structure
    pub size: i32,
    pub unk: u32,
    /// Resolution.
    pub resolution: UsbCamResolutionEx,
    /// Framerate.
    pub framerate: UsbCamFrameRate,
    /// Unknown. Set it to 2 at the moment
    pub unk2: u32,
    /// Unknown. Set it to 3 at the moment
    pub unk3: u32,
    /// White balance.
    pub white_balance: UsbCamWb,
    /// Saturation (0-255)
    pub saturation: i32,
    /// Brightness (0-255)
    pub brightness: i32,
    /// Contrast (0-255)
    pub contrast: i32,
    /// Sharpness (0-255)
    pub sharpness: i32,
    /// Unknown. Set it to 0 at the moment
    pub unk4: u32,
    /// Unknown. Set it to 1 at the moment
    pub unk5: u32,
    /// Unknown. Set it to 0 at the moment
    pub unk6: [u32; 3usize],
    /// Effect mode.
    pub effect_mode: UsbCamEffectMode,
    /// Unknown. Set it to 1 at the moment
    pub unk7: u32,
    /// Unknown. Set it to 10 at the moment
    pub unk8: u32,
    /// Unknown. Set it to 2 at the moment
    pub unk9: u32,
    /// Unknown. Set it to 500 at the moment
    pub unk10: u32,
    /// Unknown. Set it to 1000 at the moment
    pub unk11: u32,
    /// Size of jpeg video frame
    pub frame_size: i32,
    /// Unknown. Set it to 0 at the moment
    pub unk12: u32,
    /// Exposure value.
    pub ev_level: UsbCamEvLevel,
}

/// Resolutions for `sceUsbCamSetupStill` & `sceUsbCamSetupVideo`
///
/// DO NOT use on `sceUsbCamSetupStillEx` & `sceUsbCamSetupVideoEx`
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum UsbCamResolution {
    Px160_120 = 0,
    Px176_144 = 1,
    Px320_240 = 2,
    Px352_288 = 3,
    Px640_480 = 4,
    Px1024_768 = 5,
    Px1280_960 = 6,
    Px480_272 = 7,
    Px360_272 = 8,
}

/// Resolutions for `sceUsbCamSetupStillEx` & `sceUsbCamSetupVideoEx`
///
/// DO NOT use on `sceUsbCamSetupStill` & `sceUsbCamSetupVideo`
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum UsbCamResolutionEx {
    Px160_120 = 0,
    Px176_144 = 1,
    Px320_240 = 2,
    Px352_288 = 3,
    Px360_272 = 4,
    Px480_272 = 5,
    Px640_480 = 6,
    Px1024_768 = 7,
    Px1280_960 = 8,
}

bitflags::bitflags! {
    /// Flags for reverse effects.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct UsbCamReverseFlags: i32 {
        const FLIP = 1;
        const MIRROR = 0x100;
    }
}

/// Delay to take pictures
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum UsbCamDelay {
    NoDelay = 0,
    Delay10Sec = 1,
    Delay20Sec = 2,
    Delay30Sec = 3,
}

/// Usbcam framerates
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum UsbCamFrameRate {
    /// 3.75 FPS
    Fps3_75 = 0,
    /// 5 FPS
    Fps5 = 1,
    /// 7.5 FPS
    Fps7_5 = 2,
    /// 10 FPS
    Fps10 = 3,
    /// 15 FPS
    Fps15 = 4,
    /// 20 FPS
    Fps20 = 5,
    /// 30 FPS
    Fps30 = 6,
    /// 60 FPS
    Fps60 = 7,
}

/// White balance values
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum UsbCamWb {
    Auto = 0,
    Daylight = 1,
    Fluorescent = 2,
    Incadescent = 3,
}

/// Effect modes
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum UsbCamEffectMode {
    Normal = 0,
    Negative = 1,
    Blackwhite = 2,
    Sepia = 3,
    Blue = 4,
    Red = 5,
    Green = 6,
}

/// Exposure levels
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum UsbCamEvLevel {
    /// +2.0
    Pos2_0 = 0,
    /// +1.7
    Pos1_7 = 1,
    /// +1.5
    Pos1_5 = 2,
    /// +1.3
    Pos1_3 = 3,
    /// +1.0
    Pos1_0 = 4,
    /// +0.7
    Pos0_7 = 5,
    /// +0.5
    Pos0_5 = 6,
    /// +0.3
    Pos0_3 = 7,
    /// 0.0
    Zero = 8,
    /// -0.3
    Neg0_3,
    /// -0.5
    Neg0_5,
    /// -0.7
    Neg0_7,
    /// -1.0
    Neg1_0,
    /// -1.3
    Neg1_3,
    /// -1.5
    Neg1_5,
    /// -1.7
    Neg1_7,
    /// -2.0
    Neg2_0,
}

psp_extern! {
    #![name = "sceUsbCam"]
    #![flags = 0x4009]
    #![version = (0x00, 0x00)]

    #[psp(0x3F0CF289)]
    /// Setups the parameters to take a still image.
    ///
    /// # Parameters
    ///
    /// - `param`: pointer to a `UsbCamSetupStillParam`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetupStill(param: *mut UsbCamSetupStillParam) -> i32;

    #[psp(0x0A41A298)]
    /// Setups the parameters to take a still image (with more options)
    ///
    /// # Parameters
    ///
    /// - `param`: pointer to a `UsbCamSetupStillExParam`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetupStillEx(param: *mut UsbCamSetupStillExParam) -> i32;

    #[psp(0x61BE5CAC)]
    /// Gets a still image. The function doesn't return until the image
    /// has been acquired.
    ///
    /// # Parameters
    ///
    /// - `buf`: The buffer that receives the image jpeg data
    /// - `size`: The size of the buffer.
    ///
    /// # Return Value
    ///
    /// size of acquired image on success, < 0 on error
    pub fn sceUsbCamStillInputBlocking(buf: *mut u8, size: usize) -> i32;

    #[psp(0xFB0A6C5D)]
    /// Gets a still image.
    ///
    /// The function returns inmediately, and the completion has to be handled
    /// by calling `sceUsbCamStillWaitInputEnd` or
    /// `sceUsbCamStillPollInputEnd`.
    ///
    /// # Parameters
    ///
    /// - `buf`: The buffer that receives the image jpeg data
    /// - `size`: The size of the buffer.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamStillInput(buf: *mut u8, size: usize) -> i32;

    #[psp(0x7563AFA1)]
    /// Waits untils still input has been finished.
    ///
    /// # Return Value
    ///
    /// the size of the acquired image on success, < 0 on error
    pub fn sceUsbCamStillWaitInputEnd() -> i32;

    #[psp(0x1A46CFE7)]
    /// Polls the status of still input completion.
    ///
    /// # Return Value
    ///
    /// the size of the acquired image if still input has ended, 0 if the input
    /// has not ended, < 0 on error.
    pub fn sceUsbCamStillPollInputEnd() -> i32;

    #[psp(0xA720937C)]
    /// Cancels the still input.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamStillCancelInput() -> i32;

    #[psp(0xE5959C36)]
    /// Gets the size of the acquired still image.
    ///
    /// # Return Value
    ///
    /// the size of the acquired image on success, < 0 on error
    pub fn sceUsbCamStillGetInputLength() -> i32;

    #[psp(0x17F7B2FB)]
    /// Set ups the parameters for video capture.
    ///
    /// # Parameters
    ///
    /// - `param`: Pointer to a `UsbCamSetupVideoParam` structure.
    /// - `work_area`: Pointer to a buffer used as work area by the driver.
    /// - `work_area_size`: Size of the work area.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetupVideo(
        param: *mut UsbCamSetupVideoParam,
        work_area: *mut c_void,
        work_area_size: i32,
    ) -> i32;

    #[psp(0xCFE9E999)]
    /// Set ups the parameters for video capture (with more options)
    ///
    /// # Parameters
    ///
    /// - `param`: Pointer to a `UsbCamSetupVideoExParam` structure.
    /// - `work_area`: Pointer to a buffer used as work area by the driver.
    /// - `work_area_size`: Size of the work area.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetupVideoEx(
        param: *mut UsbCamSetupVideoExParam,
        work_area: *mut c_void,
        work_area_size: i32,
    ) -> i32;

    #[psp(0x574A8C3F)]
    /// Starts video input from the camera.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamStartVideo() -> i32;

    #[psp(0x6CF32CB9)]
    /// Stops video input from the camera.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamStopVideo() -> i32;

    #[psp(0x7DAC0C71)]
    /// Reads a video frame. The function doesn't return until the frame
    /// has been acquired.
    ///
    /// # Parameters
    ///
    /// - `buf`: The buffer that receives the frame jpeg data
    /// - `size`: The size of the buffer.
    ///
    /// # Return Value
    ///
    /// size of acquired frame on success, < 0 on error
    pub fn sceUsbCamReadVideoFrameBlocking(buf: *mut u8, size: usize) -> i32;

    #[psp(0x99D86281)]
    /// Reads a video frame. The function returns inmediately, and
    /// the completion has to be handled by calling `sceUsbCamWaitReadVideoFrameEnd`
    /// or `sceUsbCamPollReadVideoFrameEnd`.
    ///
    /// # Parameters
    ///
    /// - `buf`: The buffer that receives the frame jpeg data
    /// - `size`: The size of the buffer.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamReadVideoFrame(buf: *mut u8, size: usize) -> i32;

    #[psp(0xF90B2293)]
    /// Waits untils the current frame has been read.
    ///
    /// # Return Value
    ///
    /// the size of the acquired frame on sucess, < 0 on error
    pub fn sceUsbCamWaitReadVideoFrameEnd() -> i32;

    #[psp(0x41E73E95)]
    /// Polls the status of video frame read completion.
    ///
    /// # Return Value
    ///
    /// the size of the acquired frame if it has been read,
    /// 0 if the frame has not yet been read, < 0 on error.
    pub fn sceUsbCamPollReadVideoFrameEnd() -> i32;

    #[psp(0xDF9D0C92)]
    /// Gets the size of the acquired frame.
    ///
    /// # Return Value
    ///
    /// the size of the acquired frame on success, < 0 on error
    pub fn sceUsbCamGetReadVideoFrameSize() -> i32;

    #[psp(0x6E205974)]
    /// Sets the saturation
    ///
    /// # Parameters
    ///
    /// - `saturation`: The saturation (0-255)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetSaturation(saturation: i32) -> i32;

    #[psp(0x4F3D84D5)]
    /// Sets the brightness
    ///
    /// # Parameters
    ///
    /// - `brightness`: The brightness (0-255)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetBrightness(brightness: i32) -> i32;

    #[psp(0x09C26C7E)]
    /// Sets the contrast
    ///
    /// # Parameters
    ///
    /// - `contrast`: The contrast (0-255)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetContrast(contrast: i32) -> i32;

    #[psp(0x622F83CC)]
    /// Sets the sharpness
    ///
    /// # Parameters
    ///
    /// - `sharpness`: The sharpness (0-255)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetSharpness(sharpness: i32) -> i32;

    #[psp(0xD4876173)]
    /// Sets the image effect mode
    ///
    /// # Parameters
    ///
    /// - `effect_mode`: The effect mode, one of `UsbCamEffectMode`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetImageEffectMode(effect_mode: UsbCamEffectMode) -> i32;

    #[psp(0x1D686870)]
    /// Sets the exposure level
    ///
    /// # Parameters
    ///
    /// - `exposure_level`: The exposure level, one of `UsbCamEvLevel`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetEvLevel(exposure_level: UsbCamEvLevel) -> i32;

    #[psp(0x951BEDF5)]
    /// Sets the reverse mode
    ///
    /// # Parameters
    ///
    /// - `reverse_flags`: The reverse flags, zero or more of `UsbCamReverseFlags`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamSetReverseMode(reverse_flags: UsbCamReverseFlags) -> i32;

    #[psp(0xC484901F)]
    /// Sets the zoom.
    ///
    /// # Parameters
    ///
    /// - `zoom`: The zoom level starting by 10. (10 = 1X, 11 = 1.1X, etc)
    ///
    /// @returns 0 on success, < 0 on error
    pub fn sceUsbCamSetZoom(zoom: i32) -> i32;

    #[psp(0x383E9FA8)]
    /// Gets the current saturation
    ///
    /// # Parameters
    ///
    /// - `saturation`: pointer to a variable that receives the current saturation
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetSaturation(saturation: *mut i32) -> i32;

    #[psp(0x70F522C5)]
    /// Gets the current brightness
    ///
    /// # Parameters
    ///
    /// - `brightness`: pointer to a variable that receives the current brightness
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetBrightness(brightness: *mut i32) -> i32;

    #[psp(0xA063A957)]
    /// Gets the current contrast
    ///
    /// # Parameters
    ///
    /// - `contrast`: pointer to a variable that receives the current contrast
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetContrast(contrast: *mut i32) -> i32;

    #[psp(0xFDB68C23)]
    /// Gets the current sharpness
    ///
    /// # Parameters
    ///
    /// - `sharpness`: pointer to a variable that receives the current sharpness
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetSharpness(sharpness: *mut i32) -> i32;

    #[psp(0x994471E0)]
    /// Gets the current image efect mode
    ///
    /// # Parameters
    ///
    /// - `effect_mode`: pointer to a variable that receives the current effect mode
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetImageEffectMode(
        effect_mode: *mut UsbCamEffectMode,
    ) -> i32;

    #[psp(0x2BCD50C0)]
    /// Gets the current exposure level.
    ///
    /// # Parameters
    ///
    /// - `exposure_level`: pointer to a variable that receives the current exposure level
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetEvLevel(exposure_level: *mut UsbCamEvLevel) -> i32;

    #[psp(0xD5279339)]
    /// Gets the current reverse mode.
    ///
    /// # Parameters
    ///
    /// - `reverse_flags`: pointer to a variable that receives the current reverse mode flags
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetReverseMode(
        reverse_flags: *mut UsbCamReverseFlags,
    ) -> i32;

    #[psp(0x9E8AAF8D)]
    /// Gets the current zoom.
    ///
    /// # Parameters
    ///
    /// - `zoom`: pointer to a variable that receives the current zoom
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamGetZoom(zoom: *mut i32) -> i32;

    #[psp(0xF93C4669)]
    /// Sets if the image should be automatically reversed, depending of the position
    /// of the camera.
    ///
    /// # Parameters
    ///
    /// - `on`: 1 to set the automatic reversal of the image, 0 to set it off
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUsbCamAutoImageReverseSW(on: i32) -> i32;

    #[psp(0x11A1F128)]
    /// Gets the state of the autoreversal of the image.
    ///
    /// # Return Value
    ///
    /// 1 if it is set to automatic, 0 otherwise
    pub fn sceUsbCamGetAutoImageReverseState() -> i32;

    #[psp(0x4C34F553)]
    /// Gets the direction of the camera lens
    ///
    /// # Return Value
    ///
    /// 1 if the camera is "looking to you", 0 if the camera
    /// is "looking to the other side".
    pub fn sceUsbCamGetLensDirection() -> i32;
}

psp_extern! {
    #![name = "sceUsbstorBoot"]
    #![flags = 0x4009]
    #![version = (0x00, 0x00)]

    #[psp(0x1F080078)]
    /// Register an eventFlag to send notifications to.
    ///
    /// # Parameters
    ///
    /// - `event_flag`: Event flag created with `sceKernelCreateEventFlag`
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUsbstorBootRegisterNotify(event_flag: SceUid) -> i32;

    #[psp(0xA55C9E16)]
    /// Unregister a previously registered event flag.
    ///
    /// # Parameters
    ///
    /// - `event_flag`: event flag created with `sceKernelCreateEventFlag`
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUsbstorBootUnregisterNotify(event_flag: u32) -> i32;

    #[psp(0xE58818A8)]
    /// Tell the USBstorBoot driver the size of MS
    ///
    /// # Note
    ///
    /// I'm not sure if this is the actual size of the media or not as it seems
    /// to have no bearing on what size windows detects. PSPPET passes 0x800000.
    ///
    /// # Parameters
    ///
    /// - `size`: capacity of memory stick
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUsbstorBootSetCapacity(size: u32) -> i32;
}
