//! A basic graphics example that only clears the screen.

#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::{self, GuState, TexturePixelFormat, DisplayPixelFormat};
use psp::vram_alloc::get_vram_allocator;
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};

psp::module!("sample_gu_background", 1, 1);

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

fn psp_main() {
    psp::enable_home_button();

    let mut allocator = get_vram_allocator().unwrap();
    let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
    let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
    let zbp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444).as_mut_ptr_from_zero();

    unsafe {

        sys::sceGuInit();
        sys::sceGuStart(
            sys::GuContextType::Direct,
            &mut LIST as *mut _ as *mut c_void,
        );
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
        sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1 as _, BUF_WIDTH as i32);
        sys::sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);
        sys::sceGuOffset(2048 - (SCREEN_WIDTH/2), 2048 - (SCREEN_HEIGHT/2));
        sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuDepthRange(65535, 0);
        sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);

        loop {
            sys::sceGuStart(
                sys::GuContextType::Direct,
                &mut LIST as *mut _ as *mut c_void
            );
            sys::sceGuClearColor(0xff554433);
            sys::sceGuClearDepth(0);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT |
                sys::ClearBuffer::DEPTH_BUFFER_BIT
            );
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }
}
