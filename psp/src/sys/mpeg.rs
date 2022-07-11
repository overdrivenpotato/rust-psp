use crate::{
    eabi::{i5, i6, i7},
    sys,
};
use core::ffi::c_void;

/// A data handle used for various functions.
///
/// This struct can be created with the `SceMpeg::null()` method, and initialized
/// via `sceMpegCreate`.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SceMpeg(*mut *mut c_void);

impl SceMpeg {
    /// Create a null handle, which needs to be initialized with `sceMpegCreate`.
    pub fn null() -> Self {
        Self(core::ptr::null_mut())
    }
}

/// Internal structure. Passed around but never created manually.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SceMpegStream(*mut c_void);

/// Ringbuffer callback.
pub type SceMpegRingbufferCb =
    Option<unsafe extern "C" fn(data: *mut c_void, num_packets: i32, param: *mut c_void) -> i32>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMpegRingbuffer {
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
    pub callback: SceMpegRingbufferCb,
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
pub struct SceMpegAu {
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
pub struct SceMpegAvcMode {
    /// Unknown, set to -1
    pub unk0: i32,
    /// Decode pixelformat
    pub pixel_format: sys::DisplayPixelFormat,
}

psp_extern! {
    #![name = "sceMpeg"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0x682A619B)]
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegInit() -> i32;

    #[psp(0x874624D6)]
    pub fn sceMpegFinish();

    #[psp(0xD7A29F46)]
    /// # Parameters
    ///
    /// - `packets`: number of packets in the ringbuffer
    ///
    /// # Return Value
    ///
    /// < 0 if error else ringbuffer data size.
    pub fn sceMpegRingbufferQueryMemSize(packets: i32) -> i32;

    #[psp(0x37295ED8, i6)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a `SceMpegRingbuffer` struct
    /// - `packets`: number of packets in the ringbuffer
    /// - `data`: pointer to allocated memory
    /// - `size`: size of allocated memory, shoud be `sceMpegRingbufferQueryMemSize`(iPackets)
    /// - `callback`: ringbuffer callback
    /// - `cb_param`: param passed to callback
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegRingbufferConstruct(
        ringbuffer: *mut SceMpegRingbuffer,
        packets: i32,
        data: *mut c_void,
        size: i32,
        callback: SceMpegRingbufferCb,
        cb_param: *mut c_void,
    ) -> i32;

    #[psp(0x13407F13)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a `SceMpegRingbuffer` struct
    pub fn sceMpegRingbufferDestruct(ringbuffer: *mut SceMpegRingbuffer);

    #[psp(0xB5F6DC87)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a `SceMpegRingbuffer` struct
    ///
    /// # Return Value
    ///
    /// < 0 if error else number of free packets in the ringbuffer.
    pub fn sceMpegRingbufferAvailableSize(ringbuffer: *mut SceMpegRingbuffer) -> i32;

    #[psp(0xB240A59E)]
    /// # Parameters
    ///
    /// - `ringbuffer`: pointer to a `SceMpegRingbuffer` struct
    /// - `num_packets`: num packets to put into the ringbuffer
    /// - `available`: free packets in the ringbuffer, should be `sceMpegRingbufferAvailableSize()`
    ///
    /// # Return Value
    ///
    /// < 0 if error else number of packets.
    pub fn sceMpegRingbufferPut(
        ringbuffer: *mut SceMpegRingbuffer,
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
    pub fn sceMpegQueryMemSize(unk: i32) -> i32;

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
    pub fn sceMpegCreate(
        handle: SceMpeg,
        data: *mut c_void,
        size: i32,
        ringbuffer: *mut SceMpegRingbuffer,
        frame_width: i32,
        unk1: i32,
        unk2: i32,
    ) -> i32;

    #[psp(0x606A4649)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    pub fn sceMpegDelete(handle: SceMpeg);

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
    pub fn sceMpegQueryStreamOffset(
        handle: SceMpeg,
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
    pub fn sceMpegQueryStreamSize(buffer: *mut c_void, size: *mut i32) -> i32;

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
    pub fn sceMpegRegistStream(
        handle: SceMpeg,
        stream_id: i32,
        unk: i32,
    ) -> SceMpegStream;

    #[psp(0x591A4AA2)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `stream`: pointer to stream
    pub fn sceMpegUnRegistStream(handle: SceMpeg, stream: SceMpegStream);

    #[psp(0x707B7629)]
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegFlushAllStream(handle: SceMpeg) -> i32;

    #[psp(0xA780CF7E)]
    /// # Return Value
    ///
    /// 0 if error else pointer to buffer.
    pub fn sceMpegMallocAvcEsBuf(handle: SceMpeg) -> *mut c_void;

    #[psp(0xCEB870B1)]
    pub fn sceMpegFreeAvcEsBuf(handle: SceMpeg, buf: *mut c_void);

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
    pub fn sceMpegQueryAtracEsSize(
        handle: SceMpeg,
        es_size: *mut i32,
        out_size: *mut i32,
    ) -> i32;

    #[psp(0x167AFD9E)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `es_buffer`: prevously allocated Es buffer
    /// - `au`: will contain pointer to `SceMpegAu`
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegInitAu(handle: SceMpeg, es_buffer: *mut c_void, au: *mut SceMpegAu) -> i32;

    #[psp(0xFE246728)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `stream`: associated stream
    /// - `au`: will contain pointer to `SceMpegAu`
    /// - `unk`: unknown
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegGetAvcAu(
        handle: SceMpeg,
        stream: SceMpegStream,
        au: *mut SceMpegAu,
        unk: *mut i32,
    ) -> i32;

    #[psp(0xA11C7026)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `mode`: pointer to `SceMpegAvcMode` struct defining the decode mode (pixelformat)
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegAvcDecodeMode(handle: SceMpeg, mode: *mut SceMpegAvcMode) -> i32;

    #[psp(0x0E3C2E9D, i5)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `au`: video `SceMpegAu`
    /// - `iframe_width`: output buffer width, set to 512 if writing to framebuffer
    /// - `buffer`: buffer that will contain the decoded frame
    /// - `init`: will be set to 0 on first call, then 1
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegAvcDecode(
        handle: SceMpeg,
        au: *mut SceMpegAu,
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
    pub fn sceMpegAvcDecodeStop(
        handle: SceMpeg,
        frame_width: i32,
        buffer: *mut c_void,
        status: *mut i32,
    ) -> i32;

    #[psp(0xE1CE83A7)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `stream`: associated stream
    /// - `au`: will contain pointer to `SceMpegAu`
    /// - `unk`: unknown
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegGetAtracAu(
        handle: SceMpeg,
        stream: SceMpegStream,
        au: *mut SceMpegAu,
        unk: *mut c_void,
    ) -> i32;

    #[psp(0x800C44DF)]
    /// # Parameters
    ///
    /// - `handle`: Instance handle
    /// - `au`: video `SceMpegAu`
    /// - `buffer`: buffer that will contain the decoded frame
    /// - `init`: set this to 1 on first call
    ///
    /// # Return Value
    ///
    /// 0 if success.
    pub fn sceMpegAtracDecode(
        handle: SceMpeg,
        au: *mut SceMpegAu,
        buffer: *mut c_void,
        init: i32,
    ) -> i32;
}

#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct SceMpegLLI {
    pub src: *mut c_void,
    pub dst: *mut c_void,
    pub next: *mut c_void,
    pub size: i32,
}

#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct SceMpegYCrCbBuffer {
    pub frame_buffer_height16: i32,
    pub frame_buffer_width16: i32,
    /// Set to 0.
    pub unknown: i32,
    /// Set to 1.
    pub unknown2: i32,

    // Pointer to YBuffer (in VME EDRAM?)
    pub y_buffer: *mut c_void,
    // Pointer to YBuffer + framebufferwidth*(frameheight/32)
    pub y_buffer2: *mut c_void,
    // Pointer to CrBuffer (in VME EDRAM?)
    pub cr_buffer: *mut c_void,
    // Pointer to CbBuffer (in VME EDRAM?)
    pub cb_buffer: *mut c_void,
    // Pointer to CrBuffer + (framebufferwidth/2)*(frameheight/64)
    pub cr_buffer2: *mut c_void,
    // Pointer to CbBuffer + (framebufferwidth/2)*(frameheight/64)
    pub cb_buffer2: *mut c_void,

    pub frame_height: i32,
    pub frame_width: i32,
    pub frame_buffer_width: i32,
    pub unknown3: [i32; 11usize],
}

psp_extern! {
    #![name = "sceMpegbase"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0xBE45C284)]
    pub fn sceMpegBaseYCrCbCopyVme(
        yuv_buffer: *mut c_void,
        buffer: *mut i32,
        type_: i32,
    ) -> i32;

    #[psp(0x492B5E4B)]
    pub fn sceMpegBaseCscInit(width: i32) -> i32;

    #[psp(0xCE8EB837)]
    pub fn sceMpegBaseCscVme(
        rgb_buffer: *mut c_void,
        rgb_buffer2: *mut c_void,
        width: i32,
        y_cr_cb_buffer: *mut SceMpegYCrCbBuffer,
    ) -> i32;

    #[psp(0xBEA18F91)]
    /// Unknown real function name.
    pub fn sceMpegbase_BEA18F91(lli: *mut SceMpegLLI) -> i32;
}
