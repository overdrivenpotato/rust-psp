//! A basic graphics example that only clears the screen.

#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::{self, GuState, TexturePixelFormat, DisplayPixelFormat};

psp::module!("sample_gu_background", 1, 1);

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        let fbp0 = get_static_vram_buffer(512, 272, TexturePixelFormat::Psm8888);
        let fbp1 = get_static_vram_buffer(512, 272, TexturePixelFormat::Psm8888);
        let zbp = get_static_vram_buffer(512, 272, TexturePixelFormat::Psm4444);

        sys::sceGuInit();
        sys::sceGuStart(
            sys::Context::Direct,
            &mut LIST as *mut _ as *mut c_void,
        );
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0, 512);
        sys::sceGuDispBuffer(480, 272, fbp1, 512);
        sys::sceGuDepthBuffer(zbp, 512);
        sys::sceGuOffset(2048 - (480/2), 2048 - (272/2));
        sys::sceGuViewport(2048, 2048, 480, 272);
        sys::sceGuDepthRange(65535, 0);
        sys::sceGuScissor(0, 0, 480, 272);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuFinish();
        sys::sceGuSync(sys::SyncMode::Finish, sys::SyncBehavior::Wait);
        psp::sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);

        loop {
            sys::sceGuStart(
                sys::Context::Direct,
                &mut LIST as *mut _ as *mut c_void
            );
            sys::sceGuClearColor(0xff554433);
            sys::sceGuClearDepth(0);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT |
                sys::ClearBuffer::DEPTH_BUFFER_BIT
            );
            sys::sceGuFinish();
            sys::sceGuSync(sys::SyncMode::Finish, sys::SyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }
}

fn get_memory_size(width: i32, height: i32, psm: TexturePixelFormat) -> i32 {
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

unsafe fn get_static_vram_buffer(width: i32, height: i32, psm: TexturePixelFormat) -> *mut c_void {
    static mut STATIC_OFFSET: i32 = 0;

    let mem_size = get_memory_size(width, height, psm);
    let result = STATIC_OFFSET as *mut _;

    STATIC_OFFSET += mem_size;

    result
}
