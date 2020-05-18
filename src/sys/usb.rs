use core::ffi::c_void;

pub const USBBUS_DRIVERNAME: &str = "USBBusDriver";

sys_lib! {
    #![name = "sceUsb"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0xAE5DE6AF)]
    /// Start a USB driver.
    ///
    /// # Parameters
    ///
    /// - `driver_name`: name of the USB driver to start
    /// - `size`: Size of arguments to pass to USB driver start
    /// - `args`: Arguments to pass to USB driver start
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_usb_start(
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
    pub unsafe fn sce_usb_stop(
        driver_name: *const u8,
        size: i32,
        args: *mut c_void,
    ) -> i32;

    #[psp(0x586DB82C)]
    /// Activate a USB driver.
    ///
    /// # Parameters
    ///
    /// - `pid`: Product ID for the default USB Driver
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_usb_activate(pid: u32) -> i32;

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
    pub unsafe fn sce_usb_deactivate(pid: u32) -> i32;

    #[psp(0xC21645A4)]
    /// Get USB state
    ///
    /// # Return Value
    ///
    /// OR'd PSP_USB_* constants
    pub unsafe fn sce_usb_get_state() -> i32;

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
    pub unsafe fn sce_usb_get_drv_state(driver_name: *const u8) -> i32;
}

/// Structure for `sce_usb_cam_setup_still`
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CamSetupStillParam {
    /// Size of the `CamSetupStillParam` structure
    pub size: i32,
    /// Resolution.
    pub resolution: CamResolution,
    /// Size of the jpeg image
    pub jpegsize: i32,
    /// Reverse effect to apply. Zero or more of `CamReverseFlags`
    pub reverseflags: CamReverseFlags,
    /// Delay to apply to take the picture.
    pub delay: CamDelay,
    /// JPEG compression level, a value from 1-63.
    ///
    /// 1 -> less compression, better quality; 63 -> max compression, worse quality
    pub complevel: i32,
}

/// Structure for sceUsbCamSetupStillEx
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CamSetupStillExParam {
    /// Size of the `CamSetupStillExParam` structure
    pub size: i32,
    /// Unknown, set it to 9 at the moment.
    pub unk: u32,
    /// Resolution.
    pub resolution: CamResolutionEx,
    /// Size of the jpeg image
    pub jpegsize: i32,
    /// JPEG compression level, a value from 1-63.
    ///
    /// 1 -> less compression, better quality; 63 -> max compression, worse quality
    pub complevel: i32,
    /// Unknown, set it to 0 at the moment
    pub unk2: u32,
    /// Unknown, set it to 1 at the moment
    pub unk3: u32,
    /// Flag that indicates whether to flip the image
    pub flip: i32,
    /// Flag that indicates whether to mirror the image
    pub mirror: i32,
    /// Delay to apply to take the picture.
    pub delay: CamDelay,
    /// Unknown, set it to 0 at the moment
    pub unk4: [u32; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CamSetupVideoParam {
    /// Size of the `CamSetupVideoParam` structure
    pub size: i32,
    /// Resolution.
    pub resolution: CamResolution,
    /// Framerate.
    pub framerate: CamFrameRate,
    /// White balance.
    pub wb: CamWb,
    /// Saturarion (0-255)
    pub saturation: i32,
    /// Brightness (0-255)
    pub brightness: i32,
    /// Contrast (0-255)
    pub contrast: i32,
    /// Sharpness (0-255)
    pub sharpness: i32,
    /// Effect mode.
    pub effectmode: CamEffectMode,
    /// Size of jpeg video frame
    pub framesize: i32,
    /// Unknown. Set it to 0 at the moment.
    pub unk: u32,
    /// Exposure value.
    pub evlevel: CamEvLevel,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CamSetupVideoExParam {
    /// Size of the ::CamSetupVideoParam structure
    pub size: i32,
    pub unk: u32,
    /// Resolution.
    pub resolution: CamResolutionEx,
    /// Framerate.
    pub framerate: CamFrameRate,
    /// Unknown. Set it to 2 at the moment
    pub unk2: u32,
    /// Unknown. Set it to 3 at the moment
    pub unk3: u32,
    /// White balance.
    pub wb: CamWb,
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
    pub effectmode: CamEffectMode,
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
    pub framesize: i32,
    /// Unknown. Set it to 0 at the moment
    pub unk12: u32,
    /// Exposure value.
    pub evlevel: CamEvLevel,
}

/// Resolutions for `sce_usb_cam_setup_still` & `sce_usb_cam_setup_video`
///
/// DO NOT use on `sce_usb_cam_setup_still_ex` & `sce_usb_cam_setup_video_ex`
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum CamResolution {
    _160_120  = 0,
    _176_144  = 1,
    _320_240  = 2,
    _352_288  = 3,
    _640_480  = 4,
    _1024_768 = 5,
    _1280_960 = 6,
    _480_272  = 7,
    _360_272  = 8,
}

/// Resolutions for `sce_usb_cam_setup_still_ex` & `sce_usb_cam_setup_video_ex`
///
/// DO NOT use on `sce_usb_cam_setup_still` & `sce_usb_cam_setup_video`
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum CamResolutionEx {
    _160_120  = 0,
    _176_144  = 1,
    _320_240  = 2,
    _352_288  = 3,
    _360_272  = 4,
    _480_272  = 5,
    _640_480  = 6,
    _1024_768 = 7,
    _1280_960 = 8,
}

bitflags::bitflags! {
    /// Flags for reverse effects.
    pub struct CamReverseFlags: i32 {
	const FLIP = 1;
	const MIRROR = 0x100;
    }
}

/// Delay to take pictures
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum CamDelay {
    NoDelay = 0,
    Delay10Sec = 1,
    Delay20Sec = 2,
    Delay30Sec = 3,
}

/// Usbcam framerates
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum CamFrameRate {
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
pub enum CamWb {
    Auto = 0,
    Daylight = 1,
    Fluorescent = 2,
    Incadescent = 3,
}

/// Effect modes
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum CamEffectMode {
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
pub enum CamEvLevel {
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

sys_lib! {
    #![name = "sceUsbCam"]
    #![flags = 0x4009]
    #![version = (0x00, 0x00)]

    #[psp(0x3F0CF289)]
    /// Setups the parameters to take a still image.
    ///
    /// # Parameters
    ///
    /// - `param`: pointer to a `CamSetupStillParam`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_setup_still(param: *mut CamSetupStillParam) -> i32;

    #[psp(0x0A41A298)]
    /// Setups the parameters to take a still image (with more options)
    ///
    /// # Parameters
    ///
    /// - `param`: pointer to a `CamSetupStillExParam`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_setup_still_ex(param: *mut CamSetupStillExParam) -> i32;

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
    pub unsafe fn sce_usb_cam_still_input_blocking(buf: *mut u8,size: usize) -> i32;

    #[psp(0xFB0A6C5D)]
    /// Gets a still image. The function returns inmediately, and
    /// the completion has to be handled by calling `sce_usb_cam_still_wait_input_end`
    /// or `sce_usb_cam_still_poll_input_end`.
    ///
    /// # Parameters
    ///
    /// - `buf`: The buffer that receives the image jpeg data
    /// - `size`: The size of the buffer.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_still_input(buf: *mut u8,size: usize) -> i32;

    #[psp(0x7563AFA1)]
    /// Waits untils still input has been finished.
    ///
    /// # Return Value
    ///
    /// the size of the acquired image on success, < 0 on error
    pub unsafe fn sce_usb_cam_still_wait_input_end() -> i32;

    #[psp(0x1A46CFE7)]
    /// Polls the status of still input completion.
    ///
    /// # Return Value
    ///
    /// the size of the acquired image if still input has ended,
    /// 0 if the input has not ended, < 0 on error.
    pub unsafe fn sce_usb_cam_still_poll_input_end() -> i32;

    #[psp(0xA720937C)]
    /// Cancels the still input.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_still_cancel_input() -> i32;

    #[psp(0xE5959C36)]
    /// Gets the size of the acquired still image.
    ///
    /// # Return Value
    ///
    /// the size of the acquired image on success, < 0 on error
    pub unsafe fn sce_usb_cam_still_get_input_length() -> i32;

    #[psp(0x17F7B2FB)]
    /// Set ups the parameters for video capture.
    ///
    /// # Parameters
    ///
    /// - `param`: Pointer to a `CamSetupVideoParam` structure.
    /// - `workarea`: Pointer to a buffer used as work area by the driver.
    /// - `wasize`: Size of the work area.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_setup_video(
        param: *mut CamSetupVideoParam,
        workarea: *mut c_void,
        wasize: i32,
    ) -> i32;

    #[psp(0xCFE9E999)]
    /// Set ups the parameters for video capture (with more options)
    ///
    /// # Parameters
    ///
    /// - `param`: Pointer to a `CamSetupVideoExParam` structure.
    /// - `workarea`: Pointer to a buffer used as work area by the driver.
    /// - `wasize`: Size of the work area.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_setup_video_ex(
        param: *mut CamSetupVideoExParam,
        workarea: *mut c_void,
        wasize: i32,
    ) -> i32;

    #[psp(0x574A8C3F)]
    /// Starts video input from the camera.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_start_video() -> i32;

    #[psp(0x6CF32CB9)]
    /// Stops video input from the camera.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_stop_video() -> i32;

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
    pub unsafe fn sce_usb_cam_read_video_frame_blocking(buf: *mut u8,size: usize) -> i32;

    #[psp(0x99D86281)]
    /// Reads a video frame. The function returns inmediately, and
    /// the completion has to be handled by calling `sce_usb_cam_wait_read_video_frame_end`
    /// or `sce_usb_cam_poll_read_video_frame_end`.
    ///
    /// # Parameters
    ///
    /// - `buf`: The buffer that receives the frame jpeg data
    /// - `size`: The size of the buffer.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_read_video_frame(buf: *mut u8,size: usize) -> i32;

    #[psp(0xF90B2293)]
    /// Waits untils the current frame has been read.
    ///
    /// # Return Value
    ///
    /// the size of the acquired frame on sucess, < 0 on error
    pub unsafe fn sce_usb_cam_wait_read_video_frame_end() -> i32;

    #[psp(0x41E73E95)]
    /// Polls the status of video frame read completion.
    ///
    /// # Return Value
    ///
    /// the size of the acquired frame if it has been read,
    /// 0 if the frame has not yet been read, < 0 on error.
    pub unsafe fn sce_usb_cam_poll_read_video_frame_end() -> i32;

    #[psp(0xDF9D0C92)]
    /// Gets the size of the acquired frame.
    ///
    /// # Return Value
    ///
    /// the size of the acquired frame on success, < 0 on error
    pub unsafe fn sce_usb_cam_get_read_video_frame_size() -> i32;

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
    pub unsafe fn sce_usb_cam_set_saturation(saturation: i32) -> i32;

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
    pub unsafe fn sce_usb_cam_set_brightness(brightness: i32) -> i32;

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
    pub unsafe fn sce_usb_cam_set_contrast(contrast: i32) -> i32;

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
    pub unsafe fn sce_usb_cam_set_sharpness(sharpness: i32) -> i32;

    #[psp(0xD4876173)]
    /// Sets the image effect mode
    ///
    /// # Parameters
    ///
    /// - `effectmode`: The effect mode, one of `CamEffectMode`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_set_image_effect_mode(effectmode: CamEffectMode) -> i32;

    #[psp(0x1D686870)]
    /// Sets the exposure level
    ///
    /// # Parameters
    ///
    /// - `ev`: The exposure level, one of `CamEvLevel`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_set_ev_level(ev: CamEvLevel) -> i32;

    #[psp(0x951BEDF5)]
    /// Sets the reverse mode
    ///
    /// # Parameters
    ///
    /// - `reverseflags`: The reverse flags, zero or more of `CamReverseFlags`
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_set_reverse_mode(reverseflags: CamReverseFlags) -> i32;

    #[psp(0xC484901F)]
    /// Sets the zoom.
    ///
    /// # Parameters
    ///
    /// - `zoom`: The zoom level starting by 10. (10 = 1X, 11 = 1.1X, etc)
    ///
    /// @returns 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_set_zoom(zoom: i32) -> i32;

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
    pub unsafe fn sce_usb_cam_get_saturation(saturation: *mut i32) -> i32;

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
    pub unsafe fn sce_usb_cam_get_brightness(brightness: *mut i32) -> i32;

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
    pub unsafe fn sce_usb_cam_get_contrast(contrast: *mut i32) -> i32;

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
    pub unsafe fn sce_usb_cam_get_sharpness(sharpness: *mut i32) -> i32;

    #[psp(0x994471E0)]
    /// Gets the current image efect mode
    ///
    /// # Parameters
    ///
    /// - `effectmode`: pointer to a variable that receives the current effect mode
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_get_image_effect_mode(
        effectmode: *mut CamEffectMode,
    ) -> i32;

    #[psp(0x2BCD50C0)]
    /// Gets the current exposure level.
    ///
    /// # Parameters
    ///
    /// - `ev`: pointer to a variable that receives the current exposure level
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_get_ev_level(ev: *mut CamEvLevel) -> i32;

    #[psp(0xD5279339)]
    /// Gets the current reverse mode.
    ///
    /// # Parameters
    ///
    /// - `reverseflags`: pointer to a variable that receives the current reverse mode flags
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_usb_cam_get_reverse_mode(
        reverseflags: *mut CamReverseFlags,
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
    pub unsafe fn sce_usb_cam_get_zoom(zoom: *mut i32) -> i32;

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
    pub unsafe fn sce_usb_cam_auto_image_reverse_sw(on: i32) -> i32;

    #[psp(0x11A1F128)]
    /// Gets the state of the autoreversal of the image.
    ///
    /// # Return Value
    ///
    /// 1 if it is set to automatic, 0 otherwise
    pub unsafe fn sce_usb_cam_get_auto_image_reverse_state() -> i32;

    #[psp(0x4C34F553)]
    /// Gets the direction of the camera lens
    ///
    /// # Return Value
    ///
    /// 1 if the camera is "looking to you", 0 if the camera
    /// is "looking to the other side".
    pub unsafe fn sce_usb_cam_get_lens_direction() -> i32;
}
