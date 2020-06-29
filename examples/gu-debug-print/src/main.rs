//! A basic example of sceGuDebugPrint functionality
//! Prints "Hello World" in red at 100, 100

#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::{self, GuState, TexturePixelFormat, DisplayPixelFormat};
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};

psp::module!("sample_gu_debug", 1, 1);

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        let fbp0 = get_static_vram_buffer(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888);
        let fbp1 = get_static_vram_buffer(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888);
        let zbp = get_static_vram_buffer(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444);

        sys::sceGuInit();
        sys::sceGuStart(
            sys::GuContextType::Direct,
            &mut LIST as *mut _ as *mut c_void,
        );
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0, BUF_WIDTH as i32);
        sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1, BUF_WIDTH as i32);
        sys::sceGuDepthBuffer(zbp, BUF_WIDTH as i32);
        sys::sceGuOffset(2048 - (SCREEN_WIDTH/2), 2048 - (SCREEN_HEIGHT/2));
        sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuDepthRange(65535, 0);
        sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);


        sys::sceGuStart(
            sys::GuContextType::Direct,
            &mut LIST as *mut _ as *mut c_void,
        );

        sys::sceGuDebugPrint(100, 100, 0xff0000ff, b"Hello World\0" as *const u8);
        sys::sceGuDebugFlush();

        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);
        sys::sceGuSwapBuffers();
    }
}

fn get_memory_size(width: u32, height: u32, psm: TexturePixelFormat) -> u32 {
    match psm {
        TexturePixelFormat::PsmT4 => (width * height) >> 1,
        TexturePixelFormat::PsmT8 => width * height,

        TexturePixelFormat::Psm5650
        | TexturePixelFormat::Psm5551
        | TexturePixelFormat::Psm4444
        | TexturePixelFormat::PsmT16 => {
            2 * width * height
        }

        TexturePixelFormat::Psm8888 | TexturePixelFormat::PsmT32 => 4 * width * height,

        _ => unimplemented!(),
    }
}

unsafe fn get_static_vram_buffer(width: u32, height: u32, psm: TexturePixelFormat) -> *mut c_void {
    static mut STATIC_OFFSET: u32 = 0;

    let mem_size = get_memory_size(width, height, psm);
    let result = STATIC_OFFSET as *mut _;

    STATIC_OFFSET += mem_size as u32;

    result
}
