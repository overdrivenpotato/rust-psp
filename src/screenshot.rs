use core::ffi::c_void;

static BMP_HEADER: [u8;54] = [
    0x42, 0x4D, 0x36, 0x80, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00,
    0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x10, 0x01,
    0x00, 0x00, 0x01, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x80,
    0x08, 0x00, 0x12, 0x0B, 0x00, 0x00, 0x12, 0x0B, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

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
    let mut screenshot_buffer = [0u8; 54+512*272*4];
    screenshot_buffer[0..54].copy_from_slice(&BMP_HEADER);
    screenshot_buffer[54..].copy_from_slice(&raw_screenshot());
    screenshot_buffer
}
