use crate::eabi::i5;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Atrac3BufferInfo {
    pub puc_write_position_first_buf: *mut u8,
    pub ui_writable_byte_first_buf: u32,
    pub ui_min_write_byte_first_buf: u32,
    pub ui_read_position_first_buf: u32,
    pub puc_write_position_second_buf: *mut u8,
    pub ui_writable_byte_second_buf: u32,
    pub ui_min_write_byte_second_buf: u32,
    pub ui_read_position_second_buf: u32,
}

psp_extern! {
    #![name = "sceAtrac3plus"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0x780F88D1)]
    pub fn sceAtracGetAtracID(ui_codec_type: u32) -> i32;

    #[psp(0x7A20E7AF)]
    /// Creates a new Atrac ID from the specified data
    ///
    /// # Parameters
    ///
    /// - `buf`: the buffer holding the atrac3 data, including the RIFF/WAVE header.
    /// - `bufsize`: the size of the buffer pointed by buf
    ///
    /// # Return Value
    ///
    /// the new atrac ID, or < 0 on error
    pub fn sceAtracSetDataAndGetID(
        buf: *mut c_void,
        bufsize: usize,
    ) -> i32;

    #[psp(0x6A8C3CD5, i5)]
    /// Decode a frame of data.
    ///
    /// # Parameters
    ///
    /// - `atrac_id`: the atrac ID
    /// - `out_samples`: pointer to a buffer that receives the decoded data of the current frame
    /// - `out_n`: pointer to a integer that receives the number of audio samples of the decoded frame
    /// - `out_end`: pointer to a integer that receives a boolean value indicating if the decoded frame is the last one
    /// - `out_remain_frame`: pointer to a integer that receives either -1 if all at3 data is already on memory,
    ///  or the remaining (not decoded yet) frames at memory if not all at3 data is on memory
    ///
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    pub fn sceAtracDecodeData(
        atrac_id: i32,
        out_samples: *mut u16,
        out_n: *mut i32,
        out_end: *mut i32,
        out_remain_frame: *mut i32,
    ) -> i32;

    #[psp(0x9AE849A7)]
    /// Gets the remaining (not decoded) number of frames
    ///
    /// # Parameters
    ///
    /// - `atrac_id`: the atrac ID
    /// - `out_remain_frame`: pointer to a integer that receives either -1 if all at3 data is already on memory,
    ///  or the remaining (not decoded yet) frames at memory if not all at3 data is on memory
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    pub fn sceAtracGetRemainFrame(
        atrac_id: i32,
        out_remain_frame: *mut i32,
    ) -> i32;

    #[psp(0x5D268707)]
    /// # Parameters
    ///
    /// - `atrac_id`: the atrac ID
    /// - `write_pointer`: Pointer to where to read the atrac data
    /// - `available_bytes`: Number of bytes available at the writePointer location
    /// - `read_offset`: Offset where to seek into the atrac file before reading
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    pub fn sceAtracGetStreamDataInfo(
        atrac_id: i32,
        write_pointer: *mut *mut u8,
        available_bytes: *mut u32,
        read_offset: *mut u32,
    ) -> i32;

    #[psp(0x7DB31251)]
    /// # Parameters
    ///
    /// - `atrac_id`: the atrac ID
    /// - `bytes_to_add`: Number of bytes read into location given by sceAtracGetStreamDataInfo().
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    pub fn sceAtracAddStreamData(
        atrac_id: i32,
        bytes_to_add: u32,
    ) -> i32;

    #[psp(0xA554A158)]
    /// Gets the bitrate.
    ///
    /// # Parameters
    ///
    /// - `atrac_id`: the atracID
    /// - `out_bitrate`: pointer to a integer that receives the bitrate in kbps
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    pub fn sceAtracGetBitrate(
        atrac_id: i32,
        out_bitrate: *mut i32,
    ) -> i32;

    #[psp(0x868120B5)]
    /// Sets the number of loops for this atrac ID
    ///
    /// # Parameters
    ///
    /// - `atrac_id`: the atracID
    /// - `nloops`: the number of loops to set
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    pub fn sceAtracSetLoopNum(
        atrac_id: i32,
        nloops: i32,
    ) -> i32;

    #[psp(0x61EB33F5)]
    /// It releases an atrac ID
    ///
    /// # Parameters
    ///
    /// - `atrac_id`: the atrac ID to release
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sceAtracReleaseAtracID(atrac_id: i32) -> i32;

    #[psp(0x36FAABFB)]
    /// Gets the number of samples of the next frame to be decoded.
    ///
    /// # Parameters
    ///
    /// - `atrac_id`: the atrac ID
    /// - `out_n`: pointer to receives the number of samples of the next frame.
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    ///
    pub fn sceAtracGetNextSample(
        atrac_id: i32,
        out_n: *mut i32,
    ) -> i32;

    #[psp(0xD6A5F2F7)]
    /// Gets the maximum number of samples of the atrac3 stream.
    ///
    /// # Parameters
    ///
    /// - `atrac_id`: the atrac ID
    /// - `out_max`: pointer to a integer that receives the maximum number of samples.
    ///
    /// # Return Value
    ///
    /// < 0 on error, otherwise 0
    ///
    pub fn sceAtracGetMaxSample(
        atrac_id: i32,
        out_max: *mut i32,
    ) -> i32;

    #[psp(0xCA3CA3D2)]
    pub fn sceAtracGetBufferInfoForReseting(
        atrac_id: i32,
        ui_sample: u32,
        pbuffer_info: *mut Atrac3BufferInfo,
    ) -> i32;

    #[psp(0x31668BAA)]
    pub fn sceAtracGetChannel(
        atrac_id: i32,
        pui_channel: *mut u32,
    ) -> i32;

    #[psp(0xE88F759B)]
    pub fn sceAtracGetInternalErrorInfo(
        atrac_id: i32,
        pi_result: *mut i32,
    ) -> i32;

    #[psp(0xFAA4F89B)]
    pub fn sceAtracGetLoopStatus(
        atrac_id: i32,
        pi_loop_num: *mut i32,
        pui_loop_status: *mut u32,
    ) -> i32;

    #[psp(0xE23E3A35)]
    pub fn sceAtracGetNextDecodePosition(
        atrac_id: i32,
        pui_sample_position: *mut u32,
    ) -> i32;

    #[psp(0x83E85EA0)]
    pub fn sceAtracGetSecondBufferInfo(
        atrac_id: i32,
        pui_position: *mut u32,
        pui_data_byte: *mut u32,
    ) -> i32;

    #[psp(0xA2BBA8BE)]
    pub fn sceAtracGetSoundSample(
        atrac_id: i32,
        pi_end_sample: *mut i32,
        pi_loop_start_sample: *mut i32,
        pi_loop_end_sample: *mut i32,
    ) -> i32;

    #[psp(0x644E5607)]
    pub fn sceAtracResetPlayPosition(
        atrac_id: i32,
        ui_sample: u32,
        ui_write_byte_first_buf: u32,
        ui_write_byte_second_buf: u32,
    ) -> i32;

    #[psp(0x0E2A73AB)]
    pub fn sceAtracSetData(
        atrac_id: i32,
        puc_buffer_addr: *mut u8,
        ui_buffer_byte: u32,
    ) -> i32;

    #[psp(0x3F6E26B5)]
    pub fn sceAtracSetHalfwayBuffer(
        atrac_id: i32,
        puc_buffer_addr: *mut u8,
        ui_read_byte: u32,
        ui_buffer_byte: u32,
    ) -> i32;

    #[psp(0x0FAE370E)]
    pub fn sceAtracSetHalfwayBufferAndGetID(
        puc_buffer_addr: *mut u8,
        ui_read_byte: u32,
        ui_buffer_byte: u32,
    ) -> i32;

    #[psp(0x83BF7AFD)]
    pub fn sceAtracSetSecondBuffer(
        atrac_id: i32,
        puc_second_buffer_addr: *mut u8,
        ui_second_buffer_byte: u32,
    ) -> i32;
}
