//! A basic example of sceGuDebugPrint functionality
//! Prints "Hello World" in red at 100, 100

#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::{self, get_static_vram_buffer, TexturePixelFormat, DisplayPixelFormat};
use psp::{BUF_WIDTH, SCREEN_HEIGHT};

psp::module!("sample_gu_debug", 1, 1);

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        let fbp0 = get_static_vram_buffer(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888);
        sys::sceGuInit();
        sys::sceGuStart(
            sys::GuContextType::Direct,
            &mut LIST as *mut _ as *mut c_void,
        );
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0, BUF_WIDTH as i32);
        sys::sceGuDebugPrint(100, 100, 0xff0000ff, b"Hello World\0" as *const u8);
        sys::sceGuDebugFlush();

        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);
    }
}
