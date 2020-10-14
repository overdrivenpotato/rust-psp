#![no_std]
#![no_main]

#![allow(incomplete_features)]
#![feature(const_generics, const_fn)]

extern crate alloc;

mod sprite;
mod tetromino;
mod gameboard;

use core::ptr;

use psp::sys::{
    self, DisplayPixelFormat, GuContextType, GuSyncMode, GuSyncBehavior,
    GuState, TexturePixelFormat, TextureEffect, TextureColorComponent,
    ClearBuffer, ScePspFVector3, VertexType, MipmapLevel, GuPrimitive
};

use psp::Align16;
use psp::vram_alloc::get_vram_allocator;
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};
use psp::benchmark;

psp::module!("tetris", 1, 1);

pub const BLOCK_SIZE: u32 = 16;

// The image data *must* be aligned to a 16 byte boundary and 
// width / height must be a power of 2
pub static BLOCK: [u8;BLOCK_SIZE as usize*BLOCK_SIZE as usize*4] = 
    *include_bytes!("../assets/block.bin");

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct Align4<T>(pub T);

fn psp_main() {
    unsafe {
        let mut allocator = get_vram_allocator().unwrap();
        setup(&mut allocator);
        let mut gameboard = gameboard::Gameboard::new(15, 1);

        let vertex_buffer = allocator.alloc_sized::<sprite::Vertex>(400);
        let vertex_buffer = alloc::boxed::Box::from_raw(core::slice::from_raw_parts_mut(vertex_buffer.as_mut_ptr_direct_to_vram() as *mut Align4<sprite::Vertex>, 400));
        let mut vertex_buffer = Align16(vertex_buffer);
        let texture_buffer = allocator.alloc_texture_pixels(16, 16, TexturePixelFormat::Psm8888);
        let texture_buffer = Align16(core::slice::from_raw_parts_mut(texture_buffer.as_mut_ptr_direct_to_vram(), texture_buffer.len() as usize));
        texture_buffer.0.copy_from_slice(&BLOCK);

        loop {
            //let mut buffer_pos = 0;
            let dur = benchmark(||{
                clear_color(0xff554433);
                for y in 0..20 {
                    gameboard.fill_row(
                        y, 
                        Some(0xffffff00),
                    ).unwrap();
                }
                vertex_buffer.0.copy_from_slice(&gameboard.as_vertices());
                //for y in 0..5 {
                    //for x in 0..10 {
                        //let mut i = tetromino::Tetromino::new_i();
                        //i.set_pos(15+x,y*4+2); 
                        //vertex_buffer.0[buffer_pos..buffer_pos+8]
                            //.copy_from_slice(&i.as_vertices());
                        //buffer_pos += 8;
                    //}
                //};
                //let mut o = tetromino::Tetromino::new_o();
                //let mut i = tetromino::Tetromino::new_i();
                //let mut s = tetromino::Tetromino::new_s();
                //let mut z = tetromino::Tetromino::new_z();
                //let mut l = tetromino::Tetromino::new_l();
                //let mut j = tetromino::Tetromino::new_j();
                //let mut t = tetromino::Tetromino::new_t();
                //o.set_pos(1, 1);
                //i.set_pos(4, 1);
                //s.set_pos(7, 1);
                //z.set_pos(11, 1);
                //l.set_pos(3, 6);
                //j.set_pos(5, 6);
                //t.set_pos(9, 6);
                //t.rotate_cw();
                //vertex_buffer.0[buffer_pos..buffer_pos+8]
                    //.copy_from_slice(&o.as_vertices());
                //buffer_pos += 8;
                //vertex_buffer.0[buffer_pos..buffer_pos+8]
                    //.copy_from_slice(&i.as_vertices());
                //buffer_pos += 8;
                //vertex_buffer.0[buffer_pos..buffer_pos+8]
                    //.copy_from_slice(&s.as_vertices());
                //buffer_pos += 8;
                //vertex_buffer.0[buffer_pos..buffer_pos+8]
                    //.copy_from_slice(&z.as_vertices());
                //buffer_pos += 8;
                //vertex_buffer.0[buffer_pos..buffer_pos+8]
                    //.copy_from_slice(&l.as_vertices());
                //buffer_pos += 8;
                //vertex_buffer.0[buffer_pos..buffer_pos+8]
                    //.copy_from_slice(&j.as_vertices());
                //buffer_pos += 8;
                //vertex_buffer.0[buffer_pos..buffer_pos+8]
                    //.copy_from_slice(&t.as_vertices());
                //buffer_pos += 8;
                draw_vertices(&vertex_buffer, &texture_buffer, 400);
                finish_frame();
            }, 1);
            let fps_string = alloc::format!("{}\n", 1.0 / (dur.as_micros() as f32 / 1_000_000.0));
            sys::sceIoWrite(sys::SceUid(1), fps_string.as_str().as_bytes().as_ptr() as _, fps_string.len());
        }
    }
}

unsafe fn setup(allocator: &mut psp::vram_alloc::SimpleVramAllocator) {
    psp::enable_home_button();

    let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
    let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();

    sys::sceGumLoadIdentity();
    sys::sceGuInit();

    sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
    sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1 as _, BUF_WIDTH as i32);
    sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
    sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sys::sceGuEnable(GuState::ScissorTest);
    sys::sceGuEnable(GuState::Texture2D);

    sys::sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
    sys::sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgb);
    sys::sceGuTexWrap(sys::GuTexWrapMode::Clamp, sys::GuTexWrapMode::Clamp);

    sys::sceGumMatrixMode(sys::MatrixMode::View);
    sys::sceGumLoadIdentity();

    sys::sceGumMatrixMode(sys::MatrixMode::Projection);
    sys::sceGumLoadIdentity();
    sys::sceGumOrtho(0.0,480.0,272.0,0.0,-30.0,30.0);

    psp::sys::sceDisplayWaitVblankStart();
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
    sys::sceGuDisplay(true);
}

unsafe fn clear_color(color: u32) {
    sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    sys::sceGuClearColor(color);
    sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::FAST_CLEAR_BIT);
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
}

unsafe fn draw_vertices(vertices: &Align16<alloc::boxed::Box<[Align4<sprite::Vertex>]>>, texture: &Align16<&mut [u8]>, length: usize) {
    sys::sceGuStart(GuContextType::Direct, LIST.0.as_mut_ptr() as *mut _);
    
    sys::sceGumMatrixMode(sys::MatrixMode::Model);
    sys::sceGumLoadIdentity();
    sys::sceGumScale(&ScePspFVector3 { x: 0.75, y: 0.75, z: 1.0 });

    // setup texture
    sys::sceGuTexImage(MipmapLevel::None, BLOCK_SIZE as i32, BLOCK_SIZE as i32, BLOCK_SIZE as i32, (*texture.0).as_ptr() as _); 
    sys::sceGuTexScale(1.0/BLOCK_SIZE as f32, 1.0/BLOCK_SIZE as f32);

    sys::sceKernelDcacheWritebackInvalidateAll();

    // draw
    sys::sceGumDrawArray(
        GuPrimitive::Sprites,
        VertexType::TEXTURE_32BITF | VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
        length as i32,
        ptr::null_mut(),
        (*vertices).0.as_ptr() as *const _
    );	
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
}

unsafe fn finish_frame() {
    //sys::sceDisplayWaitVblankStart();
    sys::sceGuSwapBuffers();
}
