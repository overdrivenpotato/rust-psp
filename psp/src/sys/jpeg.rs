use core::ffi::c_void;

psp_extern! {
    #![name = "sceJpeg"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0xAC9E70E6)]
    /// Inits the MJpeg library
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceJpegInitMJpeg() -> i32;

    #[psp(0x7D2F3D7F)]
    /// Finishes the MJpeg library
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceJpegFinishMJpeg() -> i32;

    #[psp(0x9D47469C)]
    /// Creates the decoder context.
    ///
    /// # Parameters
    ///
    /// - `width`: The width of the frame
    /// - `height`: The height of the frame
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceJpegCreateMJpeg(width: i32, height: i32) -> i32;

    #[psp(0x48B602B7)]
    /// Deletes the current decoder context.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceJpegDeleteMJpeg() -> i32;

    #[psp(0x04B93CEF)]
    /// Decodes a mjpeg frame.
    ///
    /// # Parameters
    ///
    /// - `jpeg_buf`: the buffer with the mjpeg frame
    /// - `size`: size of the buffer pointed by `jpeg_buf`
    /// - `rgba`: buffer where the decoded data in RGBA format will be stored.
    ///           It should have a size of (width * height * 4).
    /// - `unk`: Unknown, pass 0
    ///
    /// # Return Value
    ///
    /// (width * 65536) + height on success, < 0 on error
    pub fn sceJpegDecodeMJpeg(
        jpeg_buf: *mut u8,
        size: usize,
        rgba: *mut c_void,
        unk: u32,
    ) -> i32;
}
