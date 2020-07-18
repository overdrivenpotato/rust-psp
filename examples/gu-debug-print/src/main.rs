//! A basic example of sceGuDebugPrint functionality
//! Prints "Hello World" in red at 100, 100

#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::{self, TexturePixelFormat, DisplayPixelFormat};
use psp::vram_alloc::get_vram_allocator;
use psp::{BUF_WIDTH, SCREEN_HEIGHT};

psp::module!("sample_gu_debug", 1, 1);

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

fn psp_main() {
    psp::enable_home_button();

    let mut allocator = get_vram_allocator().unwrap();
    let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();

    unsafe {
        sys::sceGuInit();
        sys::sceGuStart(
            sys::GuContextType::Direct,
            &mut LIST as *mut _ as *mut c_void,
        );
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
        sys::sceGuDebugPrint(100, 100, 0xff0000ff, b"Hello World\0" as *const u8);
        sys::sceGuDebugFlush();

        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);
    }
}
