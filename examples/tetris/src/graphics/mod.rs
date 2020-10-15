use core::ptr;
use alloc::string::ToString;

use psp::sys::{
    self, DisplayPixelFormat, GuContextType, GuSyncMode, GuSyncBehavior,
    GuState, TexturePixelFormat, TextureEffect, TextureColorComponent,
    ClearBuffer, ScePspFVector3, VertexType, MipmapLevel, GuPrimitive
};

use psp::Align16;
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};

use crate::BLOCK_SIZE;
use self::sprite::Vertex;

pub mod sprite;

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct Align4<T>(pub T);

pub static BLOCK: [u8;BLOCK_SIZE as usize*BLOCK_SIZE as usize*4] = 
    *include_bytes!("../../assets/block.bin");

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

pub unsafe fn setup(allocator: &mut psp::vram_alloc::SimpleVramAllocator) {
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
    sys::sceGuTexWrap(sys::GuTexWrapMode::Repeat, sys::GuTexWrapMode::Repeat);

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

pub unsafe fn clear_color(color: u32) {
    sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    sys::sceGuClearColor(color);
    sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::FAST_CLEAR_BIT);
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
}

pub unsafe fn draw_vertices(vertices: &Align16<alloc::boxed::Box<[Align4<Vertex>]>>, texture: &Align16<&mut [u8]>, length: usize) {
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

pub unsafe fn draw_text_at(x: i32, y: i32, color: u32, text: &str) {
    sys::sceGuDebugPrint(x, y, color, (text.to_string() + "\0").as_bytes().as_ptr());
    sys::sceGuDebugFlush();
}

pub unsafe fn finish_frame() {
    sys::sceDisplayWaitVblankStart();
    sys::sceGuSwapBuffers();
}
