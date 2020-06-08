use core::ffi::c_void;
use crate::eabi::{i5, i6, i7};

/// A data handle used for various functions.
///
/// This struct can be created with the `Handle::null()` method, and initialized
/// via `sce_mpeg_create`.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Handle(*mut *mut c_void);

impl Handle {
    /// Create a null handle, which needs to be initialized with `sce_mpeg_create`.
    pub fn null() -> Self {
        Self(core::ptr::null_mut())
    }
}

/// Internal structure. Passed around but never created manually.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Stream(*mut c_void);

/// Ringbuffer callback.
pub type RingbufferCb = Option<
    unsafe extern "C" fn(data: *mut c_void, num_packets: i32, param: *mut c_void) -> i32,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ringbuffer {
    /// Packets
    pub packets: i32,
    /// Unknown
    pub unk0: u32,
    /// Unknown
    pub unk1: u32,
    /// Unknown
    pub unk2: u32,
    /// Unknown
    pub unk3: u32,
    /// Pointer to data
    pub data: *mut c_void,
    /// Ringbuffer callback
    pub callback: RingbufferCb,
    /// Callback param
    pub cb_param: *mut c_void,
    /// Unknown
    pub unk4: u32,
    /// Unknown
    pub unk5: u32,
    /// Mpeg ID
    pub sce_mpeg: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Au {
    /// Presentation timestamp MSB
    pub pts_msb: u32,
    /// Presentation timestamp LSB
    pub pts: u32,
    /// Decode timestamp MSB
    pub dts_msb: u32,
    /// Decode timestamp LSB
    pub dts: u32,
    /// Es buffer handle
    pub es_buffer: u32,
    /// Au size
    pub au_size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AvcMode {
    /// Unknown, set to -1
    pub unk0: i32,
    /// Decode pixelformat
    pub pixel_format: i32,
}

psp_extern! {
    #![name = "sceMpeg"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0x682A619B)]
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_init() -> i32;

    #[psp(0x874624D6)]
    pub fn sce_mpeg_finish();

    #[psp(0xD7A29F46)]
    /// # Parameters
    ///
    /// - `packets`: number of packets in the ringbuffer
    ///
    /// # Return Value
    ///
    /// < 0 if error else ringbuffer data size.
    pub fn sce_mpeg_ringbuffer_query_mem_size(packets: i32) -> i32;

    #[psp(0x37295ED8, i6)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a sceMpegRingbuffer struct
    /// - `packets`: number of packets in the ringbuffer
    /// - `data`: pointer to allocated memory
    /// - `size`: size of allocated memory, shoud be `sce_mpeg_ringbuffer_query_mem_size`(iPackets)
    /// - `callback`: ringbuffer callback
    /// - `cb_param`: param passed to callback
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_ringbuffer_construct(
        ringbuffer: *mut Ringbuffer,
        packets: i32,
        data: *mut c_void,
        size: i32,
        callback: RingbufferCb,
        cb_param: *mut c_void,
    ) -> i32;

    #[psp(0x13407F13)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a sceMpegRingbuffer struct
    pub fn sce_mpeg_ringbuffer_destruct(ringbuffer: *mut Ringbuffer);

    #[psp(0xB5F6DC87)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a sceMpegRingbuffer struct
    ///
    /// # Return Value
    ///
    /// < 0 if error else number of free packets in the ringbuffer.
    pub fn sce_mpeg_ringbuffer_available_size(ringbuffer: *mut Ringbuffer) -> i32;

    #[psp(0xB240A59E)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a sceMpegRingbuffer struct
    /// - `num_packets`: num packets to put into the ringbuffer
    /// - `available`: free packets in the ringbuffer, should be sceMpegRingbufferAvailableSize()
    ///
    /// # Return Value
    ///
    /// < 0 if error else number of packets.
    pub fn sce_mpeg_ringbuffer_put(
        ringbuffer: *mut Ringbuffer,
        num_packets: i32,
        available: i32,
    ) -> i32;

    #[psp(0xC132E22F)]
    /// # Parameters
    ///
    /// - `unk`: Unknown, set to 0
    ///
    /// # Return Value
    ///
    /// < 0 if error else decoder data size.
    pub fn sce_mpeg_query_mem_size(unk: i32) -> i32;

    #[psp(0xD8C5F121, i7)]
    /// # Parameters
    ///
    /// - `mpeg`: will be filled
    /// - `data`: pointer to allocated memory of size = sceMpegQueryMemSize()
    /// - `size`: size of data, should be = sceMpegQueryMemSize()
    /// - `ringbuffer`: a ringbuffer
    /// - `frame_width`: display buffer width, set to 512 if writing to framebuffer
    /// - `unk1`: unknown, set to 0
    /// - `unk2`: unknown, set to 0
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_create(
        handle: Handle,
        data: *mut c_void,
        size: i32,
        ringbuffer: *mut Ringbuffer,
        frame_width: i32,
        unk1: i32,
        unk2: i32,
    ) -> i32;

    #[psp(0x606A4649)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    pub fn sce_mpeg_delete(handle: Handle);

    #[psp(0x21FF80E4)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `buffer`: pointer to file header
    /// - `offset`: will contain stream offset in bytes, usually 2048
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_query_stream_offset(
        handle: Handle,
        buffer: *mut c_void,
        offset: *mut i32,
    ) -> i32;

    #[psp(0x611E9E11)]
    /// # Parameters
    ///
    /// - `buffer`: pointer to file header
    /// - `size`: will contain stream size in bytes
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_query_stream_size(buffer: *mut c_void, size: *mut i32) -> i32;

    #[psp(0x42560F23)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `stream_id`: stream id, 0 for video, 1 for audio
    /// - `unk`: unknown, set to 0
    ///
    /// # Return Value
    ///
    /// 0 if error.
    pub fn sce_mpeg_regist_stream(
        handle: Handle,
        stream_id: i32,
        unk: i32,
    ) -> Stream;

    #[psp(0x591A4AA2)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `stream`: pointer to stream
    pub fn sce_mpeg_un_regist_stream(handle: Handle, stream: Stream);

    #[psp(0x707B7629)]
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_flush_all_stream(handle: Handle) -> i32;

    #[psp(0xA780CF7E)]
    /// # Return Value
    ///
    /// 0 if error else pointer to buffer.
    pub fn sce_mpeg_malloc_avc_es_buf(handle: Handle) -> *mut c_void;

    #[psp(0xCEB870B1)]
    pub fn sce_mpeg_free_avc_es_buf(handle: Handle, buf: *mut c_void);

    #[psp(0xF8DCB679)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `es_size`: will contain size of Es
    /// - `out_size`: will contain size of decoded data
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_query_atrac_es_size(
        handle: Handle,
        es_size: *mut i32,
        out_size: *mut i32,
    ) -> i32;

    #[psp(0x167AFD9E)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `es_buffer`: prevously allocated Es buffer
    /// - `au`: will contain pointer to Au
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_init_au(handle: Handle, es_buffer: *mut c_void, au: *mut Au) -> i32;

    #[psp(0xFE246728)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `stream`: associated stream
    /// - `au`: will contain pointer to Au
    /// - `unk`: unknown
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_get_avc_au(
        handle: Handle,
        stream: Stream,
        au: *mut Au,
        unk: *mut i32,
    ) -> i32;

    #[psp(0xA11C7026)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `mode`: pointer to AvcMode struct defining the decode mode (pixelformat)
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_avc_decode_mode(handle: Handle, mode: *mut AvcMode) -> i32;

    #[psp(0x0E3C2E9D, i5)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `au`: video Au
    /// - `iframe_width`: output buffer width, set to 512 if writing to framebuffer
    /// - `buffer`: buffer that will contain the decoded frame
    /// - `init`: will be set to 0 on first call, then 1
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_avc_decode(
        handle: Handle,
        au: *mut Au,
        iframe_width: i32,
        buffer: *mut c_void,
        init: *mut i32,
    ) -> i32;

    #[psp(0x740FCCD1)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `frame_width`: output buffer width, set to 512 if writing to framebuffer
    /// - `buffer`: buffer that will contain the decoded frame
    /// - `status`: frame number
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_avc_decode_stop(
        handle: Handle,
        frame_width: i32,
        buffer: *mut c_void,
        status: *mut i32,
    ) -> i32;

    #[psp(0xE1CE83A7)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `stream`: associated stream
    /// - `au`: will contain pointer to Au
    /// - `unk`: unknown
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_get_atrac_au(
        handle: Handle,
        stream: Stream,
        au: *mut Au,
        unk: *mut c_void,
    ) -> i32;

    #[psp(0x800C44DF)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `au`: video Au
    /// - `buffer`: buffer that will contain the decoded frame
    /// - `init`: set this to 1 on first call
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sce_mpeg_atrac_decode(
        handle: Handle,
        au: *mut Au,
        buffer: *mut c_void,
        init: i32,
    ) -> i32;
}
