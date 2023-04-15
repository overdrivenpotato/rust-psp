use core::ffi::c_void;

/// A structure used for initializing a handle in `sceMp3ReserveMp3Handle`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMp3InitArg {
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
    /// Size of the `mp3_buf` buffer (must be >= 8192)
    pub mp3_buf_size: i32,
    /// Pointer to output buffer where decoded PCM samples will be written.
    pub pcm_buf: *mut c_void,
    /// Size of `pcm_buf` buffer (must be >= 9216)
    pub pcm_buf_size: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Mp3Handle(pub i32);

psp_extern! {
    #![name = "sceMp3"]
    #![flags = 0x0009]
    #![version = (0x00, 0x11)]

    #[psp(0x07EC321A)]
    /// # Parameters
    ///
    /// - `args`: Pointer to `SceMp3InitArg` structure
    ///
    /// # Return Value
    ///
    /// Raw MP3 handle on success, < 0 on error. Construct a `Handle` instance
    /// from this value to use the other functions in this module.
    // TODO: Investigate adding `Result` support to `psp_extern!`.
    pub fn sceMp3ReserveMp3Handle(args: *mut SceMp3InitArg) -> i32;

    #[psp(0xF5478233)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub fn sceMp3ReleaseMp3Handle(handle: Mp3Handle) -> i32;

    #[psp(0x35750070)]
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub fn sceMp3InitResource() -> i32;

    #[psp(0x3C2FA058)]
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub fn sceMp3TermResource() -> i32;

    #[psp(0x44E07129)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub fn sceMp3Init(handle: Mp3Handle) -> i32;

    #[psp(0xD021C0FB)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    /// - `dst`: Pointer to destination pcm samples buffer
    ///
    /// # Return Value
    ///
    /// number of bytes in decoded pcm buffer, < 0 on error.
    pub fn sceMp3Decode(handle: Mp3Handle, dst: *mut *mut i16) -> i32;

    #[psp(0xA703FE0F)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    /// - `dst`: Pointer to stream data buffer
    /// - `to_write`: Space remaining in stream data buffer
    /// - `src_pos`: Position in source stream to start reading from
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub fn sceMp3GetInfoToAddStreamData(
        handle: Mp3Handle,
        dst: *mut *mut u8,
        to_write: *mut i32,
        src_pos: *mut i32,
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
    pub fn sceMp3NotifyAddStreamData(handle: Mp3Handle, size: i32) -> i32;

    #[psp(0xD0A56296)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// 1 if more stream data is needed, < 0 on error.
    pub fn sceMp3CheckStreamDataNeeded(handle: Mp3Handle) -> i32;

    #[psp(0x3CEF484F)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    /// - `loop_`: Number of loops
    ///
    /// # Return Value
    ///
    /// 0 if success, < 0 on error.
    pub fn sceMp3SetLoopNum(handle: Mp3Handle, loop_: i32) -> i32;

    #[psp(0xD8F54A51)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of loops
    pub fn sceMp3GetLoopNum(handle: Mp3Handle) -> i32;

    #[psp(0x354D27EA)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of decoded samples
    pub fn sceMp3GetSumDecodedSample(handle: Mp3Handle) -> i32;

    #[psp(0x87C263D1)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of max samples to output
    pub fn sceMp3GetMaxOutputSample(handle: Mp3Handle) -> i32;

    #[psp(0x8F450998)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Sampling rate of the mp3
    pub fn sceMp3GetSamplingRate(handle: Mp3Handle) -> i32;

    #[psp(0x87677E40)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Bitrate of the mp3
    pub fn sceMp3GetBitRate(handle: Mp3Handle) -> i32;

    #[psp(0x7F696782)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// Number of channels of the mp3
    pub fn sceMp3GetMp3ChannelNum(handle: Mp3Handle) -> i32;

    #[psp(0x2A368661)]
    /// # Parameters
    ///
    /// - `handle`: MP3 handle
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sceMp3ResetPlayPosition(handle: Mp3Handle) -> i32;
}
