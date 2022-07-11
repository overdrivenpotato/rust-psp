//! PGF Font Library

use crate::sys::kernel::SceUid;
use core::ffi::c_void;

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum SceFontFamilyCode {
    Default,
    SansSerif,
    Serif,
    Rounded,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum SceFontStyleCode {
    Default,
    Regular,
    Italic,
    Narrow,
    NarrowItalic,
    Bold,
    BoldItalic,
    Black,
    BlackItalic,
    L = 101,
    M,
    /// DemiBold
    DB,
    B,
    EB,
    UB,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum SceFontLanguageCode {
    Default,
    Japanese,
    Latin,
    Korean,
    Chinese,
    Cjk,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SceFontPixelFormatCode {
    /// 2 pixels packed in 1 byte (natural order)
    Format4,
    /// 2 pixels packed in 1 byte (reversed order)
    Format4Rev,
    /// 1 pixel in 1 byte
    Format8,
    /// 1 pixel in 3 bytes (RGB)
    Format24,
    /// 1 pixel in 4 bytes (RGBA)
    Format32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SceFontErrorCode {
    Success = 0,
    OutOfMemory = 0x80460001,
    InvalidLibId = 0x80460002,
    InvalidParameter = 0x80460003,
    NoFile = 0x80460004,
    HandlerOpenFailed = 0x80460005,
    HandlerCloseFailed = 0x80460006,
    HandlerReadFailed = 0x80460007,
    HandlerSeekFailed = 0x80460008,
    TooManyOpenFonts = 0x80460009,
    InvalidFontData = 0x8046000A,
    InconsistentData = 0x8046000B,
    Expired = 0x8046000C,
    Registry = 0x8046000D,
    NoSupport = 0x8046000E,
    Unknown = 0x8046FFFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceFontStyle {
    pub font_h: f32,
    pub font_v: f32,
    pub font_h_res: f32,
    pub font_v_res: f32,
    pub font_weight: f32,
    pub font_family: SceFontFamilyCode,
    pub font_style: SceFontStyleCode,
    // ???
    pub font_style_sub: u16,
    pub font_language: SceFontLanguageCode,
    pub font_region: u16,
    pub font_country: u16,
    pub font_name: [u8; 64],
    pub font_file_name: [u8; 64],
    pub font_attributes: u32,
    pub font_expire: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontGlyphImage {
    pub pixel_format: SceFontPixelFormatCode,
    pub x_pos_64: i32,
    pub y_pos_64: i32,
    pub buf_width: u16,
    pub buf_height: u16,
    pub bytes_per_line: u16,
    pub pad: u16,
    pub buffer_ptr: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct SceFontCharInfo {
    pub bitmap_width: u32,
    pub bitmap_height: u32,
    pub bitmap_left: u32,
    pub bitmap_top: u32,
    // Glyph metrics (in 26.6 signed fixed-point).
    pub sfp26_width: u32,
    pub sfp26_height: u32,
    pub sfp26_ascender: i32,
    pub sfp26_descender: i32,
    pub sfp26_bearing_hx: i32,
    pub sfp26_bearing_hy: i32,
    pub sfp26_bearing_vx: i32,
    pub sfp26_bearing_vy: i32,
    pub sfp26_advance_h: i32,
    pub sfp26_advance_v: i32,
    pub shadow_flags: i16,
    pub shadow_id: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceFontInfo {
    // Glyph metrics (in 26.6 signed fixed-point).
    pub max_glyph_width_i: i32,
    pub max_glyph_height_i: i32,
    pub max_glyph_ascender_i: i32,
    pub max_glyph_descender_i: i32,
    pub max_glyph_left_x_i: i32,
    pub max_glyph_base_y_i: i32,
    pub max_glyph_center_x_i: i32,
    pub max_glyph_top_y_i: i32,
    pub max_glyph_advance_x_i: i32,
    pub max_glyph_advance_y_i: i32,

    // Glyph metrics (replicated as float).
    pub max_glyph_width_f: f32,
    pub max_glyph_height_f: f32,
    pub max_glyph_ascender_f: f32,
    pub max_glyph_descender_f: f32,
    pub max_glyph_left_x_f: f32,
    pub max_glyph_base_y_f: f32,
    pub max_glyph_center_x_f: f32,
    pub max_glyph_top_y_f: f32,
    pub max_glyph_advance_x_f: f32,
    pub max_glyph_advance_y_f: f32,

    // Bitmap dimensions
    pub max_glyph_width: i16,
    pub max_glyph_height: i16,
    pub num_glyphs: i32,
    /// Number of elements in the font's shadow charmap.
    pub shadow_map_length: i32,

    /// Font style (used by font comparison functions).
    pub font_style: SceFontStyle,
    pub bpp: u8,
    pub pad: [u8; 3],
}

type UnknownFn = extern "C" fn();

/// The library works with only num_fonts, alloc_func, and free_func set to
/// non-null values so long as you don't load a font from a file.
/// Function signatures reversed from 11 Eyes Crossover.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceFontNewLibParams {
    pub user_data_addr: u32,
    pub num_fonts: u32,
    pub cache_data: u32,
    /// Returns pointer to allocated memory
    pub alloc_func: Option<extern "C" fn(unk_ptr: *mut c_void, amount: usize) -> *mut c_void>,
    pub free_func: Option<extern "C" fn(unk_ptr: *mut c_void, ptr: *mut c_void)>,
    /// Returns fd of opened file
    pub open_func: Option<
        extern "C" fn(
            unk_ptr: *mut c_void,
            filename: *const u8,
            error_code: &mut SceFontErrorCode,
        ) -> SceUid,
    >,
    /// Returns an SceFontErrorCode
    pub close_func: Option<extern "C" fn(unk_ptr: *mut c_void, fd: SceUid) -> SceFontErrorCode>,
    /// Returns number of "type"s read (ie bytes_read / type_size)
    pub read_func:
        Option<extern "C" fn(unk_ptr: *mut c_void, data: *mut c_void, type_size: u32) -> u32>,
    /// Returns an SceFontErrorCode
    pub seek_func:
        Option<extern "C" fn(unk_ptr: *mut c_void, fd: SceUid, offset: i32) -> SceFontErrorCode>,
    /// Unknown, pass None
    pub error_func: Option<UnknownFn>,
    /// Unknown, pass None
    pub io_finish_func: Option<UnknownFn>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct SceFontImageRect {
    pub width: i16,
    pub height: i16,
}

psp_extern! {
    #![name = "sceLibFont"]
    #![flags = 0x0009]
    #![version = (0x00, 0x11)]

    #[psp(0x67F17ED7)]
    pub fn sceFontNewLib(param: &SceFontNewLibParams, error_code: &mut SceFontErrorCode) -> u32;

    #[psp(0x574B6FBC)]
    pub fn sceFontDoneLib(handle: u32) -> i32;

    #[psp(0xA834319D)]
    pub fn sceFontOpen(handle: u32, index: u32, mode: u32, error_code: &mut SceFontErrorCode) -> u32;

    #[psp(0xBB8E7FE6)]
    pub fn sceFontOpenUserMemory(handle: u32, font_data: *const u8, font_length: i32, error_code: &mut SceFontErrorCode) -> u32;

    #[psp(0x57FCB733)]
    pub fn sceFontOpenUserFile(handle: u32, file_name: *const u8, mode: u32, error_code: &mut SceFontErrorCode) -> u32;

    #[psp(0x3AEA8CB6)]
    pub fn sceFontClose(handle: u32) -> i32;

    #[psp(0x099EF33C)]
    pub fn sceFontFindOptimumFont(handle: u32, font_style: &SceFontStyle, error_code: &mut SceFontErrorCode) -> i32;

    #[psp(0x681E61A7)]
    pub fn sceFontFindFont(handle: u32, font_style: &SceFontStyle, error_code: &mut SceFontErrorCode) -> i32;

    #[psp(0x0DA7535E)]
    pub fn sceFontGetFontInfo(handle: u32, font_info: &mut SceFontInfo) -> i32;

    #[psp(0x5333322D)]
    pub fn sceFontGetFontInfoByIndexNumber(handle: u32, font_style: &mut SceFontStyle, index: u32) -> i32;

    #[psp(0xDCC80C2F)]
    pub fn sceFontGetCharInfo(handle: u32, char_code: u32, char_info: &mut SceFontCharInfo) -> i32;

    #[psp(0xAA3DE7B5)]
    pub fn sceFontGetShadowInfo(handle: u32, char_code: u32, char_info: &mut SceFontCharInfo) -> i32;

    #[psp(0x5C3E4A9E)]
    pub fn sceFontGetCharImageRect(handle: u32, char_code: u32, char_rect: &mut SceFontImageRect) -> i32;

    #[psp(0x48B06520)]
    pub fn sceFontGetShadowImageRect(handle: u32, char_code: u32, char_rect: &mut SceFontImageRect) -> i32;

    #[psp(0x980F4895)]
    pub fn sceFontGetCharGlyphImage(handle: u32, char_code: u32, glyph: &mut SceFontGlyphImage) -> i32;

    #[psp(0xCA1E6945)]
    pub fn sceFontGetCharGlyphImage_Clip(handle: u32, char_code: u32, glyph: &mut SceFontGlyphImage, clip_x_pos: i32, clip_y_pos: i32) -> i32;

    #[psp(0xEE232411)]
    pub fn sceFontSetAltCharacterCode(handle: u32, char_code: u32) -> i32;

    #[psp(0x02D7F94B)]
    pub fn sceFontFlush(handle: u32) -> i32;

    #[psp(0xBC75D85B)]
    pub fn sceFontGetFontList(handle: u32, font_style: *mut SceFontStyle, num_fonts: i32) -> i32;

    #[psp(0x27F6E642)]
    pub fn sceFontGetNumFontList(handle: u32, error_code: &mut SceFontErrorCode) -> i32;

    #[psp(0x48293280)]
    pub fn sceFontSetResolution(handle: u32, h_res: f32, v_res: f32) -> i32;

    #[psp(0x74B21701)]
    pub fn sceFontPixelToPointH(handle: u32, pixels_h: f32, error_code: &mut SceFontErrorCode) -> f32;

    #[psp(0xF8F0752E)]
    pub fn sceFontPixelToPointV(handle: u32, pixels_v: f32, error_code: &mut SceFontErrorCode) -> f32;

    #[psp(0x472694CD)]
    pub fn sceFontPointToPixelH(handle: u32, point_h: f32, error_code: &mut SceFontErrorCode) -> f32;

    #[psp(0x3C4B7E82)]
    pub fn sceFontPointToPixelV(handle: u32, point_v: f32, error_code: &mut SceFontErrorCode) -> f32;

    #[psp(0x2F67356A)]
    pub fn sceFontCalcMemorySize() -> i32;

    #[psp(0x568BE516)]
    pub fn sceFontGetShadowGlyphImage(handle: u32, char_code: u32, glyph: &mut SceFontGlyphImage) -> i32;

    #[psp(0x5DCF6858)]
    pub fn sceFontGetShadowGlyphImage_Clip(handle: u32, char_code: u32, glyph: &mut SceFontGlyphImage, clip_x_pos: i32, clip_y_pos: i32) -> i32;
}
