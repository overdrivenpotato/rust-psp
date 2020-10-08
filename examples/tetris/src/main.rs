#![no_std]
#![no_main]

#![allow(incomplete_features)]
#![feature(const_generics, const_fn)]

extern crate alloc;

mod sprite;
mod tetromino;

use psp::sys::{
    self, DisplayPixelFormat, GuContextType, GuSyncMode, GuSyncBehavior,
    GuState, TexturePixelFormat, DepthFunc, TextureEffect, TextureColorComponent,
    TextureFilter, ClearBuffer, 
};

use psp::Align16;
use psp::vram_alloc::get_vram_allocator;
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};

psp::module!("tetris", 1, 1);

pub const BLOCK_SIZE: u32 = 16;

// The image data *must* be aligned to a 16 byte boundary and 
// width / height must be a power of 2
pub static BLOCK: Align16<[u8;BLOCK_SIZE as usize*BLOCK_SIZE as usize*4]> = 
    Align16(*include_bytes!("../assets/block.bin"));

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);


fn psp_main() {
    unsafe {
        setup();
        let mut o = tetromino::Tetromino::new_o();
        let mut i = tetromino::Tetromino::new_i();
        let mut s = tetromino::Tetromino::new_s();
        let mut z = tetromino::Tetromino::new_z();
        let mut l = tetromino::Tetromino::new_l();
        let mut j = tetromino::Tetromino::new_j();
        let mut t = tetromino::Tetromino::new_t();
        o.set_pos(1, 1);
        i.set_pos(4, 0);
        s.set_pos(6, 0);
        z.set_pos(10, 0);
        l.set_pos(3, 5);
        j.set_pos(5, 5);
        t.set_pos(9, 5);
        loop {
            clear_color(0xff554433);
            o.draw(&mut LIST);
            i.draw(&mut LIST);
            s.draw(&mut LIST);
            z.draw(&mut LIST);
            l.draw(&mut LIST);
            j.draw(&mut LIST);
            t.draw(&mut LIST);
            finish_frame();
        }
    }
}

unsafe fn setup() {
    psp::enable_home_button();

    let mut allocator = get_vram_allocator().unwrap();
    let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
    let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
    let zbp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444).as_mut_ptr_from_zero();

    sys::sceGumLoadIdentity();
    sys::sceGuInit();

    sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
    sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1 as _, BUF_WIDTH as i32);
    sys::sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);
    sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
    sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sys::sceGuDepthRange(65535, 0);
    sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sys::sceGuEnable(GuState::ScissorTest);
    sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
    sys::sceGuEnable(GuState::DepthTest);
    sys::sceGuEnable(GuState::Texture2D);

    sys::sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
    sys::sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgb);
    sys::sceGuTexWrap(sys::GuTexWrapMode::Clamp, sys::GuTexWrapMode::Clamp);
    sys::sceGuTexFilter(TextureFilter::Nearest, TextureFilter::Nearest);

    sys::sceGumMatrixMode(sys::MatrixMode::View);
    sys::sceGumLoadIdentity();

    sys::sceGumMatrixMode(sys::MatrixMode::Projection);
    sys::sceGumLoadIdentity();
    sys::sceGumOrtho(0.0,480.0,272.0,0.0,-30.0,30.0);

    psp::sys::sceDisplayWaitVblankStart();
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
}

unsafe fn clear_color(color: u32) {
    sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    sys::sceGuClearColor(color);
    sys::sceGuClearDepth(0);
    sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

}

unsafe fn finish_frame() {
    sys::sceDisplayWaitVblankStart();
    sys::sceGuSwapBuffers();
    sys::sceGuDisplay(true);
}
