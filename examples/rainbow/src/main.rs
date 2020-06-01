#![no_std]
#![no_main]

use psp;
use psp::sys::display;
use psp::sys::ge;

psp::module!("sample_module", 1, 1);

const BUFFER_WIDTH: usize = 512;
const DISPLAY_HEIGHT: usize = 272;
const DISPLAY_WIDTH: usize = 480;
static mut VRAM: *mut u32 = 0x4000_0000 as *mut u32;

fn psp_main() {
    psp::enable_home_button();
    unsafe {
        display::sce_display_set_mode(display::DisplayMode::Lcd, DISPLAY_WIDTH, DISPLAY_HEIGHT);
        VRAM = (0x4000_0000u32 | ge::sce_ge_edram_get_addr() as u32) as *mut u32;
        display::sce_display_set_frame_buf(VRAM as *const u8, BUFFER_WIDTH, display::DisplayPixelFormat::_8888, display::DisplaySetBufSync::NextFrame);
        let time = psp::benchmark(|| {
            //display::sce_display_wait_vblank_start();
            for pos in 0..255  {
                let color = wheel(pos);

                for i in 0..(BUFFER_WIDTH * DISPLAY_HEIGHT) {
                    *VRAM.add(i) = color;
                }
            }
        }, 10);
        psp::dprintln!("{:?}", time);
    }
}

fn wheel(mut pos: u8) -> u32 {
    pos = 255 - pos;
    if pos < 85 {
        u32::from_be_bytes([255 - pos * 3, 0, pos * 3, 255])
    } else if pos < 170 {
        pos -= 85;
        u32::from_be_bytes([0, pos * 3, 255 - pos * 3, 255])
    } else {
        pos -= 170;
        u32::from_be_bytes([pos * 3, 255 - pos * 3, 0, 255])
    }
}
