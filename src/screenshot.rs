use core::{ptr, ffi::c_void};
use crate::sys::display::{self, DisplayPixelFormat};

const SCREEN_WIDTH: usize = 480;
const SCREEN_HEIGHT: usize = 272;
// RGBA
const BYTES_PER_PIXEL: usize = 4;

const NUM_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

#[repr(C, packed)]
struct BmpHeader {
    pub file_type: [u8; 2],
    pub file_size: u32,
    pub reserved_1: u16,
    pub reserved_2: u16,
    pub image_data_start: u32,
    pub dib_header_size: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub color_planes: u16,
    pub bpp: u16,
    pub compression: u32,
    pub image_data_len: u32,
    pub print_resolution_x: u32,
    pub print_resolution_y: u32,
    pub palette_color_count: u32,
    pub important_colors: u32,
}

impl BmpHeader {
    const BYTES: usize = core::mem::size_of::<Self>();

    fn to_bytes(self) -> [u8; Self::BYTES] {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

/// Take a screenshot, returning a raw RGBA array.
pub fn screenshot() -> [u32; NUM_PIXELS] {
    let mut screenshot_buffer = [0; NUM_PIXELS];
    let mut buffer_width: usize = 0;
    let mut pixel_format = DisplayPixelFormat::_565;
    let mut top_addr: *mut c_void = ptr::null_mut();

    unsafe {
        display::sce_display_get_frame_buf(
            &mut top_addr,
            &mut buffer_width,
            &mut pixel_format,
            display::DisplaySetBufSync::Immediate,
        );
    }

    // http://uofw.github.io/upspd/docs/hardware/PSPTEK.htm#memmap

    // If this is a kernel address...
    if top_addr as u32 & 0x80000000 != 0 {
        // Set the kernel cache-through bit.
        top_addr = (top_addr as u32 | 0xA0000000) as _;
    } else {
        // Else set the regular cache-through bit.
        top_addr = (top_addr as u32 | 0x40000000) as _;
    }

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            match pixel_format {
                display::DisplayPixelFormat::_8888 => {
                    let rgba = unsafe {
                        *(top_addr as *mut u32).add(x + y * buffer_width)
                    };

                    // Reversed for little-endian based copying.
                    let abgr = (rgba >> 24)
                        | ((rgba >> 16) & 0xff)
                        | ((rgba >> 8) & 0xff)
                        | ((rgba >> 0) & 0xff);

                    screenshot_buffer[x + y * SCREEN_HEIGHT] = abgr;
                }

                _ => unimplemented!("unimplemented pixel format"),
            }
        }
    }

    screenshot_buffer
}

/// Take a screenshot, returning a valid bitmap file.
pub fn screenshot_bmp() -> [u8; BmpHeader::BYTES + NUM_PIXELS * BYTES_PER_PIXEL] {
    let bmp_header = BmpHeader {
        file_type: *b"BM",
        file_size: 54 + 512 * 272 * 4,
        reserved_1: 0,
        reserved_2: 0,
        image_data_start: 54,
        dib_header_size: 40,
        image_width: 512,
        image_height: 272,
        color_planes: 1,
        bpp: 32,
        compression: 0,
        image_data_len: 512*272*4,
        print_resolution_x: 2835, // 72 DPI,
        print_resolution_y: 2835, // 72 DPI,
        palette_color_count: 0,
        important_colors: 0
    };

    let mut screenshot_buffer = [0; BmpHeader::BYTES + NUM_PIXELS * BYTES_PER_PIXEL];
    screenshot_buffer[0..54].copy_from_slice(&bmp_header.to_bytes());

    let payload = screenshot();

    unsafe {
        core::ptr::copy_nonoverlapping(
            &payload[0] as *const _ as _,
            &mut screenshot_buffer[54] as *mut u8,
            NUM_PIXELS * BYTES_PER_PIXEL,
        );
    }

    screenshot_buffer
}
