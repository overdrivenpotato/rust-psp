use core::ffi::c_void;

#[repr(C, packed)]
pub struct BmpHeader {
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
    fn to_bytes(self) -> [u8; 54] {
        unsafe {
            core::mem::transmute::<BmpHeader, [u8;54]>(self)
        }
    }
}

#[inline]
fn extract_bits(value: u32, offset: u32, size: usize) -> u32 {
    (value >> offset) & ((1 << size) - 1)
}


fn rgab8888_to_bgra8888(dst: *mut u32, src: *const u32, num: usize) {
    for i in 0..num as isize {
        unsafe {
            let c = *src.offset(i);
            let r = extract_bits(c,  0, 8);
            let g = extract_bits(c,  8, 8);
            let b = extract_bits(c, 16, 8);
            let a = extract_bits(c, 24, 8);
            *dst.offset(i) = (b << 0) | (g << 8) | (r << 16) | (a << 24);
        }
    }
}

pub fn raw_screenshot() -> [u8; 512*272*4] {
    let mut screenshot_buffer = [0u8; 512*272*4];
    let mut buffer_width: usize = 0;
    let mut pixel_format = crate::sys::display::DisplayPixelFormat::_565;
    let mut top_addr: u32 = 0;


    unsafe {
        crate::sys::display::sce_display_get_frame_buf(
            &mut top_addr as *mut _ as *mut *mut c_void,  
            &mut buffer_width as *mut usize,
            &mut pixel_format as *mut crate::sys::display::DisplayPixelFormat,
            crate::sys::display::DisplaySetBufSync::Immediate
        );
    }

    if top_addr & 0x80000000 != 0 {
        top_addr |= 0xA0000000;
    } else {
        top_addr |= 0x40000000;
    }

    let mut vram_row: *mut u32;
    let mut row_buf = [0u32; 512*4];
    let row_bytes: u32 = match pixel_format {
        crate::sys::display::DisplayPixelFormat::_8888 => (4 * buffer_width) as u32,
        _ => (2 * buffer_width) as u32,
    };
    

    for y in 0..272 {
        vram_row = (top_addr + row_bytes * (271-y)) as *mut u32;
        match pixel_format {
            crate::sys::display::DisplayPixelFormat::_8888 => {
                rgab8888_to_bgra8888(&mut row_buf as *mut _ as *mut u32, vram_row, 512);
            },
            _ => {todo!("Support more pixel formats");}
        }
        unsafe {
            core::ptr::copy(
                core::mem::transmute::<*mut u32, *mut u8>(vram_row),
                (&mut screenshot_buffer as *mut _ as  *mut u8).offset(y as isize*512*4),
                512*4
                );
        }
    }
    screenshot_buffer
}

pub fn screenshot() -> [u8; 54+512*272*4] {
    let bmp_header = BmpHeader {
        file_type: *b"BM",
        file_size: 54+512*272*4,
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

    let mut screenshot_buffer = [0u8; 54+512*272*4];
    screenshot_buffer[0..54].copy_from_slice(&bmp_header.to_bytes());
    screenshot_buffer[54..].copy_from_slice(&raw_screenshot());
    screenshot_buffer
}
