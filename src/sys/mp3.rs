use core::ffi::c_void;

/// A structure used for initializing a handle in `sce_mp3_reserve_mp3_handle`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InitArg {
    /// Stream start position
    pub mp3_stream_start: u32,
    /// Unknown - set to 0
    pub unk1: u32,
    /// Stream end position
    pub mp3_stream_end: u32,
    /// Unknown - set to 0
    pub unk2: u32,
    /// Pointer to a buffer to contain raw mp3 stream data (+1472 bytes workspace)
    pub mp3_buf: *mut c_void,
    /// Size of mp3Buf buffer (must be >= 8192)
    pub mp3_buf_size: i32,
    /// Pointer to decoded pcm samples buffer
    pub pcm_buf: *mut c_void,
    /// Size of pcmBuf buffer (must be >= 9216)
    pub pcm_buf_size: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Handle(pub i32);

sys_lib! {
    #![name = "sceMp3"]
    #![flags = 0x0009]
    #![version = (0x00, 0x11)]

    #[psp(0x07EC321A)]
    /// # Parameters
    ///
    /// - `args`: Pointer to `InitArg` structure
    ///
    /// # Return Value
    ///
    /// Raw MP3 handle on success, < 0 on error. Construct a `Handle` instance
    /// from this value to use the other functions in this module.
    // TODO: Investigate adding `Result` support to `sys_lib!`.
    pub unsafe fn sce_mp3_reserve_mp3_handle(args: *mut InitArg) -> i32;

    #[psp(0xF5478233)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub unsafe fn sce_mp3_release_mp3_handle(handle: Handle) -> i32;

    #[psp(0x35750070)]
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub unsafe fn sce_mp3_init_resource() -> i32;

    #[psp(0x3C2FA058)]
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub unsafe fn sce_mp3_term_resource() -> i32;

    #[psp(0x44E07129)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub unsafe fn sce_mp3_init(handle: Handle) -> i32;

    #[psp(0xD021C0FB)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    /// - `dst`: Pointer to destination pcm samples buffer
    ///
    /// # Return Value
    ///
    /// number of bytes in decoded pcm buffer, < 0 on error.
    pub unsafe fn sce_mp3_decode(handle: Handle, dst: *mut *mut i16) -> i32;

    #[psp(0xA703FE0F)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    /// - `dst`: Pointer to stream data buffer
    /// - `towrite`: Space remaining in stream data buffer
    /// - `srcpos`: Position in source stream to start reading from
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub unsafe fn sce_mp3_get_info_to_add_stream_data(
        handle: Handle,
        dst: *mut *mut u8,
        towrite: *mut i32,
        srcpos: *mut i32,
    ) -> i32;

    #[psp(0x0DB149F4)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    /// - `size`: number of bytes added to the stream data buffer
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub unsafe fn sce_mp3_notify_add_stream_data(handle: Handle, size: i32) -> i32;

    #[psp(0xD0A56296)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// 1 if more stream data is needed, < 0 on error.
    pub unsafe fn sce_mp3_check_stream_data_needed(handle: Handle) -> i32;

    #[psp(0x3CEF484F)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    /// - `loop`: Number of loops
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub unsafe fn sce_mp3_set_loop_num(handle: Handle, loop_: i32) -> i32;

    #[psp(0xD8F54A51)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of loops
    pub unsafe fn sce_mp3_get_loop_num(handle: Handle) -> i32;

    #[psp(0x354D27EA)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of decoded samples
    pub unsafe fn sce_mp3_get_sum_decoded_sample(handle: Handle) -> i32;

    #[psp(0x87C263D1)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of max samples to output
    pub unsafe fn sce_mp3_get_max_output_sample(handle: Handle) -> i32;

    #[psp(0x8F450998)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Sampling rate of the mp3
    pub unsafe fn sce_mp3_get_sampling_rate(handle: Handle) -> i32;

    #[psp(0x87677E40)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Bitrate of the mp3
    pub unsafe fn sce_mp3_get_bit_rate(handle: Handle) -> i32;

    #[psp(0x7F696782)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of channels of the mp3
    pub unsafe fn sce_mp3_get_mp3_channel_num(handle: Handle) -> i32;

    #[psp(0x2A368661)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub unsafe fn sce_mp3_reset_play_position(handle: Handle) -> i32;
}
