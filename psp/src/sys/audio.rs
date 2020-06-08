use core::ffi::c_void;

pub const VOLUME_MAX: u32 = 0x8000;
pub const CHANNEL_MAX: u32 = 8;
pub const NEXT_CHANNEL: i32 = -1;
pub const SAMPLE_MIN: u32 = 64;
pub const SAMPLE_MAX: u32 = 65472;

#[repr(u32)]
pub enum Format {
    /// Channel set to stereo output
    Stereo = 0,
    /// Channel set to mono output
    Mono = 0x10,
}

#[repr(C)]
pub struct InputParams {
    /// Unknown. Pass 0
    pub unknown1: i32,
    pub gain: i32,
    /// Unknown. Pass 0
    pub unknown2: i32,
    /// Unknown. Pass 0
    pub unknown3: i32,
    /// Unknown. Pass 0
    pub unknown4: i32,
    /// Unknown. Pass 0
    pub unknown5: i32,

}

#[repr(i32)]
pub enum OutputFrequency {
    Khz48 = 48000,
    Khz44_1 = 44100,
    Khz32 = 32000,
    Khz24 = 24000,
    Khz22_05 = 22050,
    Khz16 = 16000,
    Khz12 = 12000,
    Khz11_025 = 11025,
    Khz8 = 8000,
}

#[repr(i32)]
pub enum InputFrequency {
    Khz44_1 = 44100,
    Khz22_05 = 22050,
    Khz11_025 = 11025,
}

/// Make the given sample count a multiple of 64.
pub const fn sample_align(sample_count:i32) -> i32 {
    (sample_count + 63) & !63
}

psp_extern! {
    #![name = "sceAudio"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x5EC81C55)]
    /// Allocate and initialize a hardware output channel.
    ///
    /// # Parameters
    ///
    /// `channel` - Use a value between 0-7 to reserve a specific channel.
    /// Pass NEXT_CHANNEL to get the first available channel.
    /// `sample_count` - The number of samples that can be output on the channel per output
    /// call. It must be a value between SAMPLE_MIN and SAMPLE_MAX, and it must be 
    /// aligned to 64 bytes. Use sample_align to align it.
    /// `format` - The output format to use for the channel. One of Format.
    ///
    /// # Return value
    ///
    /// The channel number on success, or <0 on error.
    pub fn sce_audio_ch_reserve(channel: i32, sample_count: i32, format: i32) -> i32;
    #[psp(0x6FC46853)]
    /// Release a hardware output channel.
    ///
    /// # Parameters
    ///
    /// `channel` - The channel to release.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_ch_release(channel: i32) -> i32;

    #[psp(0x8C1009B2)]
    /// Output audio to the specified channel.
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// `vol` - The volume.
    /// `buf` - Pointer to PCM data to output
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_output(channel: i32, vol: i32, buf: *mut c_void) -> i32;

    #[psp(0x136CAF51)]
    /// Output audio to the specified channel (blocking)
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// `vol` - The volume.
    /// `buf` - Pointer to PCM data to output
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.

    pub fn sce_audio_output_blocking(channel: i32, vol: i32, buf: *mut c_void) -> i32;

    #[psp(0xE2D56B2D)]
    /// Output panned audio to the specified channel.
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// `left_vol` - The left volume.
    /// `right_vol` - The right volume.
    /// `buf` Pointer to PCM data to output
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_output_panned(channel: i32, left_vol: i32, right_vol: i32, buf: *mut c_void) -> i32;

    #[psp(0x13F592BC)]
    /// Output panned audio to the specified channel (blocking)
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// `left_vol` - The left volume.
    /// `right_vol` - The right volume.
    /// `buf` Pointer to PCM data to output
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.

    pub fn sce_audio_output_panned_blocking(channel: i32, left_vol: i32, right_vol: i32, buf: *mut c_void) -> i32;

    #[psp(0xE9D97901)]
    /// Get count of uplayed samples remaining
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// 
    /// # Return value
    ///
    /// Number of samples to be played, <0 on error.
    pub fn sce_audio_get_channel_rest_len(channel: i32) -> i32;

    #[psp(0xB011922F)]
    /// Get count of uplayed samples remaining
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// 
    /// # Return value
    ///
    /// Number of samples to be played, <0 on error.
    pub fn sce_audio_get_channel_rest_length(channel: i32) -> i32;

    #[psp(0xCB2E439E)]
    /// Change the output sample count, after it's already been reserved
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// `sample_count` - The number of samples to output in one output call.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_set_channel_data_len(channel: i32, sample_count: i32) -> i32;

    #[psp(0x95FD0C2D)]
    /// Change the format of a channel
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// `format` - One of Format.
    ///
    /// # Return value
    ///
    /// 0 on success, < 0 on error.
    pub fn sce_audio_change_channel_config(channel: i32, format: i32) -> i32;

    #[psp(0xB7E1D8E7)]
    /// Change the volume of a channel
    ///
    /// # Parameters
    ///
    /// `channel` - The channel number.
    /// `left_vol` - The left volume.
    /// `right_vol` - The right volume.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_change_channel_volume(channel:i32, left_vol: i32, right_vol:i32) -> i32;

    #[psp(0x01562BA3)]
    /// Reserve the audio output and set the sample count
    /// 
    /// # Parameters
    ///
    /// `sample_count` - The number of samples to output in one output call (min 17, max 4111).
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_output2_reserve(sample_count: i32) -> i32;

    #[psp(0x43196845)]
    /// Release the audio output
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_output2_release() -> i32;

    #[psp(0x63F2889C)]
    /// Change the output sample count, after it's already been reserved
    ///
    /// # Parameters
    /// 
    /// `sample_count` - The number of samples to output in one output call (min 17, max 4111)
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_output2_change_length(sample_count: i32) -> i32;

    #[psp(0x2D53F36E)]
    /// Output audio (blocking)
    ///
    /// # Parameters
    ///
    /// `vol` - The volume.
    /// ` buf` - Pointer to PCM data.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_output2_output_blocking(vol: i32, buf: *mut c_void) -> i32;

    #[psp(0x647CEF33)]
    /// Get count of unplayed samples remaining
    ///
    /// # Return value
    ///
    /// Number of samples to be played, < 0 on error.
    pub fn sce_audio_output2_get_rest_sample() -> i32;

    #[psp(0x38553111)]
    /// Reserve the audio output
    ///
    /// # Parameters
    ///
    /// `sample_count` - The number of samples to output in one output call (min 17, max 4111).
    /// `freq` - One of OutputFrequency.
    /// `channels` - Number of channels. Pass 2 (stereo).
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_src_ch_reserve(sample_count: i32, freq: OutputFrequency, channels: i32) -> i32;

    #[psp(0x5C37C0AE)]
    /// Release the audio output
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_src_ch_release() -> i32;

    #[psp(0xE0727056)]
    /// Output audio (blocking)
    /// 
    /// # Parameters
    ///
    /// `vol` - The volume.
    /// `buf` - Pointer to PCM data.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_src_output_blocking(vol: i32, buf: *mut c_void) -> i32;

    #[psp(0x7DE61688)]
    /// Init audio input
    ///
    /// # Parameters
    ///
    /// `unknown1` - Unknown. Pass 0.
    /// `gain` - Gain.
    /// `unknown2` - Unknown. Pass 0.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_input_init(unknown1: i32, gain: i32, unknown2: i32) -> i32;

    #[psp(0xE926D3FB)]
    /// Init audio input (with extra arguments)
    ///
    /// # Parameters
    ///
    /// `params` - A pointer to an InputParams struct.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_input_init_ex(params: *mut InputParams) -> i32;

    #[psp(0x086E5895)]
    /// Perform audio input (blocking)
    ///
    /// # Parameters
    /// 
    /// `sample_count` - Number of samples.
    /// `freq` - One of InputFrequency.
    /// `buf` - Pointer to where the audio data will be stored.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_input_blocking(sample_count: i32, freq: InputFrequency, buf: *mut c_void);

    #[psp(0x6D4BEC68)]
    /// Perform audio input
    ///
    /// # Parameters
    /// 
    /// `sample_count` - Number of samples.
    /// `freq` - One of InputFrequency.
    /// `buf` - Pointer to where the audio data will be stored.
    ///
    /// # Return value
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_input(sample_count: i32, freq: InputFrequency, buf: *mut c_void);

    #[psp(0xA708C6A6)]
    /// Get the number of samples that were acquired
    ///
    /// # Return value
    ///
    /// Number of samples acquired, <0 on error.
    pub fn sce_audio_get_input_length() -> i32;

    #[psp(0x87B2E651)]
    /// Wait for non-blocking audio input to complete
    ///
    /// # Return value 
    ///
    /// 0 on success, <0 on error.
    pub fn sce_audio_wait_input_end() -> i32;

    #[psp(0xA633048E)]
    /// Poll for non-blocking audio input status
    ///
    /// # Return value
    ///
    /// 0 if input has completed, 1 if not completed, <0 on error.
    pub fn sce_audio_poll_input_end() -> i32;
}
