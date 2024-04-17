use crate::sys::{self, DisplayPixelFormat};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use core::{ffi::c_void, ptr};

// RGBA
const BYTES_PER_PIXEL: usize = 4;

const NUM_PIXELS: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[repr(C, packed)]
#[derive(Clone, Copy)]
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
        unsafe { core::mem::transmute(self) }
    }
}

fn rgba_to_bgra(rgba: u32) -> u32 {
    // 0xAABBGGRR -> 0xAARRGGBB

    core::intrinsics::bswap(rgba << 8 | rgba >> 24)
}

fn rgb565_to_bgra(rgb565: u16) -> u32 {
    let rgb565 = rgb565 as u32;

    // bbbb bggg gggr rrrr -> 0xffRRGGBB
    (((rgb565 & 0x1f) << 16) * 0x100 / 0x20)
        | (((rgb565 & 0x7e0) << 3) * 0x100 / 0x40)
        | (((rgb565 & 0xf800) >> 11) * 0x100 / 0x20)
        | 0xff00_0000
}

fn rgba5551_to_bgra(rgba5551: u16) -> u32 {
    let rgba5551 = rgba5551 as u32;

    // abbb bbgg gggr rrrr -> 0xAARRGGBB
    (((rgba5551 & 0x1f) << 16) * 0x100 / 0x20)
        | (((rgba5551 & 0x3e0) << 3) * 0x100 / 0x20)
        | (((rgba5551 & 0x7c00) >> 10) * 0x100 / 0x20)
        | (((rgba5551 & 0x8000) >> 15) * 0xff00_0000)
}

fn rgba4444_to_bgra(rgba4444: u16) -> u32 {
    let rgba4444 = rgba4444 as u32;

    // aaaa bbbb gggg rrrr -> 0xAARRGGBB
    (((rgba4444 & 0x000f) << 16) * 0x100 / 0x10)
        | (((rgba4444 & 0x00f0) << 4) * 0x100 / 0x10)
        | (((rgba4444 & 0x0f00) >> 8) * 0x100 / 0x10)
        | (((rgba4444 & 0xf000) << 12) * 0x100 / 0x10)
}

/// Take a screenshot, returning a raw ARGB (big-endian) array.
pub fn screenshot_argb_be() -> alloc::vec::Vec<u32> {
    let mut screenshot_buffer = alloc::vec![0; NUM_PIXELS];
    let mut buffer_width: usize = 0;
    let mut pixel_format = DisplayPixelFormat::Psm5650;
    let mut top_addr: *mut c_void = ptr::null_mut();

    unsafe {
        sys::sceDisplayGetFrameBuf(
            &mut top_addr,
            &mut buffer_width,
            &mut pixel_format,
            sys::DisplaySetBufSync::Immediate,
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
            // BGRA is reversed ARGB. We do this for little-endian based copying.
            let bgra = match pixel_format {
                sys::DisplayPixelFormat::Psm8888 => {
                    let rgba = unsafe {
                        *(top_addr as *mut u32).add(x as usize + y as usize * buffer_width)
                    };

                    rgba_to_bgra(rgba)
                }

                sys::DisplayPixelFormat::Psm5650 => {
                    let rgb565 = unsafe {
                        *(top_addr as *mut u16).add(x as usize + y as usize * buffer_width)
                    };

                    rgb565_to_bgra(rgb565)
                }

                sys::DisplayPixelFormat::Psm5551 => {
                    let rgba5551 = unsafe {
                        *(top_addr as *mut u16).add(x as usize + y as usize * buffer_width)
                    };

                    rgba5551_to_bgra(rgba5551)
                }

                sys::DisplayPixelFormat::Psm4444 => {
                    let rgba4444 = unsafe {
                        *(top_addr as *mut u16).add(x as usize + y as usize * buffer_width)
                    };

                    rgba4444_to_bgra(rgba4444)
                }
            };

            // Display buffer is flipped upside down.
            let y_inv = SCREEN_HEIGHT - y - 1;
            screenshot_buffer[x as usize + y_inv as usize * SCREEN_WIDTH as usize] = bgra;
        }
    }

    screenshot_buffer
}

/// Take a screenshot, returning a valid bitmap file.
pub fn screenshot_bmp() -> alloc::vec::Vec<u8> {
    let mut screenshot_buffer = alloc::vec![0; BmpHeader::BYTES + NUM_PIXELS * BYTES_PER_PIXEL];

    let payload = screenshot_argb_be();

    let bmp_header = BmpHeader {
        file_type: *b"BM",
        file_size: BmpHeader::BYTES as u32 + payload.len() as u32 * 4,
        reserved_1: 0,
        reserved_2: 0,
        image_data_start: BmpHeader::BYTES as u32,
        dib_header_size: 40,
        image_width: SCREEN_WIDTH,
        image_height: SCREEN_HEIGHT,
        color_planes: 1,
        bpp: 32,
        compression: 0,
        image_data_len: payload.len() as u32 * 4,
        print_resolution_x: 2835, // 72 DPI
        print_resolution_y: 2835, // 72 DPI
        palette_color_count: 0,
        important_colors: 0,
    };

    screenshot_buffer[0..BmpHeader::BYTES].copy_from_slice(&bmp_header.to_bytes());

    unsafe {
        core::ptr::copy_nonoverlapping(
            &payload[0] as *const _ as _,
            &mut screenshot_buffer[BmpHeader::BYTES] as *mut u8,
            NUM_PIXELS * BYTES_PER_PIXEL,
        );
    }

    screenshot_buffer
}
