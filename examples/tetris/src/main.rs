#![no_std]
#![no_main]

#![allow(incomplete_features)]
#![feature(const_generics, const_fn)]

extern crate alloc;

mod sprite;
mod tetromino;

use core::ptr;

use psp::sys::{
    self, DisplayPixelFormat, GuContextType, GuSyncMode, GuSyncBehavior,
    GuState, TexturePixelFormat, DepthFunc, TextureEffect, TextureColorComponent,
    TextureFilter, ClearBuffer, ScePspFVector3, VertexType, MipmapLevel, GuPrimitive
};

use psp::Align16;
use psp::vram_alloc::get_vram_allocator;
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};
use psp::benchmark;

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
        loop {
            let mut vertex_buffer = Align16([sprite::Vertex::default(); 400]);
            let mut buffer_pos = 0;
            let dur = benchmark(||{
            clear_color(0xff554433);
            for y in 0..5 {
                for x in 0..10 {
                    let mut i = tetromino::Tetromino::new_i();
                    i.set_pos(15+x,y*4+2); 
                    i.as_sprites()
                    .iter()
                    .for_each(|sprite| {
                        let vertices = sprite.as_vertices();
                        vertex_buffer.0[buffer_pos] = vertices[0];
                        vertex_buffer.0[buffer_pos+1] = vertices[1];
                        buffer_pos += 2;
                    });
                }
            }
            draw_sprite_vertices(vertex_buffer, buffer_pos);
            }, 1);
            let fps_string = alloc::format!("{}\n", 1.0 / (dur.as_micros() as f32 / 1_000_000.0));
            sys::sceIoWrite(sys::SceUid(1), fps_string.as_str().as_bytes().as_ptr() as _, fps_string.len());
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

unsafe fn draw_sprite_vertices(vertices: Align16<[sprite::Vertex; 400]>, length: usize) {
    unsafe {
            sys::sceGuStart(GuContextType::Direct, LIST.0.as_mut_ptr() as *mut _);
            
            sys::sceGumMatrixMode(sys::MatrixMode::Model);
            sys::sceGumLoadIdentity();
            sys::sceGumScale(&ScePspFVector3 { x: 0.75, y: 0.75, z: 1.0 });
            //sys::sceGumTranslate(&ScePspFVector3 { x: self.x as f32, y: self.y as f32, z: 0.0});
            // setup texture
            sys::sceGuTexImage(MipmapLevel::None, BLOCK_SIZE as i32, BLOCK_SIZE as i32, BLOCK_SIZE as i32, BLOCK.0.as_ptr() as *const _); 
            sys::sceGuTexScale(1.0/BLOCK_SIZE as f32, 1.0/BLOCK_SIZE as f32);

            sys::sceKernelDcacheWritebackInvalidateAll();

            // draw sprite
            sys::sceGumDrawArray(
                GuPrimitive::Sprites,
                VertexType::TEXTURE_32BITF | VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
                length as i32,
                ptr::null_mut(),
                &vertices as *const Align16<_> as *const _,
            );	
            sys::sceGuFinish();
            sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

        }
}

unsafe fn finish_frame() {
    //sys::sceDisplayWaitVblankStart();
    sys::sceGuSwapBuffers();
    sys::sceGuDisplay(true);
}
