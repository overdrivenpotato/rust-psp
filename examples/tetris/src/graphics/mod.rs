use core::ptr;
use alloc::string::ToString;

use psp::sys::{
    self, DisplayPixelFormat, GuContextType, GuSyncMode, GuSyncBehavior,
    GuState, TexturePixelFormat, TextureEffect, TextureColorComponent,
    ClearBuffer, ScePspFVector3, VertexType, MipmapLevel, GuPrimitive,
    BlendOp, BlendFactor, MatrixMode, AlphaFunc, GuTexWrapMode,
};

use psp::Align16;
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};

use self::sprite::Vertex;

pub mod sprite;

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct Align4<T>(pub T);

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

/// Setup the GU Library with all of the configuration we need
///
/// # Parameters
///
/// - `allocator`: A reference to a `SimpleVramAllocator`.
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
    sys::sceGuTexWrap(GuTexWrapMode::Repeat, GuTexWrapMode::Repeat);

    sys::sceGuEnable(GuState::Blend);
    sys::sceGuBlendFunc(BlendOp::Add, BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha, 0, 0);
    sys::sceGuAlphaFunc(AlphaFunc::Greater, 0, 0xff);

    sys::sceGumMatrixMode(MatrixMode::View);
    sys::sceGumLoadIdentity();

    sys::sceGumMatrixMode(MatrixMode::Projection);
    sys::sceGumLoadIdentity();
    sys::sceGumOrtho(0.0,480.0,272.0,0.0,-30.0,30.0);

    sys::sceDisplayWaitVblankStart();
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
    sys::sceGuDisplay(true);
}

/// Clear the screen a particular colour.
///
/// # Parameters
///
/// - `color`: The colour to clear with, in big-endian ABGR, little endian RGBA.
pub unsafe fn clear_color(color: u32) {
    sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    sys::sceGuClearColor(color);
    sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::FAST_CLEAR_BIT);
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
}

/// Draw vertices to the screen.
///
/// # Parameters
///
/// - `vertices`: Reference to buffer of 4-byte aligned vertices. The buffer must be 
/// 16-byte aligned .
/// - `texture`: Reference to buffer of texture. The buffer must be 16-byte aligned.
/// - `texture_width`: Width of texture, must be power of 2.
/// - `texture_height`: Height of texture, must be power of 2.
/// - `scale_x`: Horizontal scale factor.
/// - `scale_y`: Vertical scale factor.
pub unsafe fn draw_vertices(
    vertices: &[Align4<Vertex>],
    texture: &[u8],
    texture_width: u32,
    texture_height: u32,
    scale_x: f32,
    scale_y: f32,
) {
    sys::sceGuStart(GuContextType::Direct, LIST.0.as_mut_ptr() as *mut _);
    
    sys::sceGumMatrixMode(MatrixMode::Model);
    sys::sceGumLoadIdentity();
    sys::sceGumScale(&ScePspFVector3 { x: scale_x, y: scale_y, z: 1.0 });

    // setup texture
    sys::sceGuTexImage(MipmapLevel::None, texture_width as i32, texture_height as i32, texture_width as i32, (*texture).as_ptr() as _); 
    sys::sceGuTexScale(1.0/texture_width as f32, 1.0/texture_height as f32);

    sys::sceKernelDcacheWritebackInvalidateAll();

    // draw
    sys::sceGumDrawArray(
        GuPrimitive::Sprites,
        VertexType::TEXTURE_32BITF | VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
        (*vertices).len() as i32,
        ptr::null_mut(),
        (*vertices).as_ptr() as _,
    );	
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
}

/// Draws text at a given point on the screen in a given colour.
///
/// # Parameters
/// 
/// - `x`: horizontal position
/// - `y`: vertical position
/// - `color`: Colour of text, in big-endian ABGR, little-endian RGBA.
/// - `text`: ASCII text as an &str.
pub unsafe fn draw_text_at(x: i32, y: i32, color: u32, text: &str) {
    sys::sceGuDebugPrint(x, y, color, (text.to_string() + "\0").as_bytes().as_ptr());
    sys::sceGuDebugFlush();
}

/// Finishes drawing by waiting for VBlank and swapping the Draw and Display buffer 
/// pointers.
pub unsafe fn finish_frame() {
    sys::sceDisplayWaitVblankStart();
    sys::sceGuSwapBuffers();
}
