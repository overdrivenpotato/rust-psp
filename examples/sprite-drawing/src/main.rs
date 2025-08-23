#![no_std]
#![no_main]

use core::{ffi::c_void, ptr};
use psp::{sys::*, vram_alloc::*, *};

// Width and height must be the same and be a power of 2.
const IMAGE_SIZE: usize = 128;
static FERRIS: Align16<[u8; IMAGE_SIZE * IMAGE_SIZE * 4 as usize]> =
    Align16(*include_bytes!("../ferris.bin"));
static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

#[repr(C, align(4))]
#[derive(Default, Clone, Copy)]
pub struct Vertex {
    u: f32,
    v: f32,
    color: u32,
    x: f32,
    y: f32,
    z: f32,
}

psp::module!("sprite-example", 1, 1);

fn psp_main() {
    unsafe { psp_innermain() }
}

#[allow(unreachable_code)]
unsafe fn psp_innermain() {
    psp::enable_home_button();

    let allocator = get_vram_allocator().unwrap();
    let fbp0 = allocator
        .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
        .as_mut_ptr_from_zero();
    let fbp1 = allocator
        .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
        .as_mut_ptr_from_zero();
    let zbp = allocator
        .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444)
        .as_mut_ptr_from_zero();

    sceGumLoadIdentity();
    sceGuInit();

    sceGuStart(GuContextType::Direct, &mut LIST as *mut _ as *mut c_void);
    sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
    sceGuDispBuffer(
        SCREEN_WIDTH as i32,
        SCREEN_HEIGHT as i32,
        fbp1 as _,
        BUF_WIDTH as i32,
    );
    sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);

    sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
    sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sceGuDepthRange(65535, 0);
    sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sceGuEnable(GuState::ScissorTest);

    sceGuEnable(GuState::Texture2D);
    sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
    sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgb);
    sceGuAlphaFunc(AlphaFunc::Greater, 0, 0xff);

    sceGuFinish();
    sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);

    sceDisplayWaitVblankStart();
    sceGuDisplay(true);

    sceGumMatrixMode(MatrixMode::Projection);
    sceGumLoadIdentity();
    sceGumOrtho(0.0, 480.0, 272.0, 0.0, -10.0, 10.0);

    sceGumMatrixMode(MatrixMode::View);
    sceGumLoadIdentity();

    sceGumMatrixMode(MatrixMode::Model);
    sceGumLoadIdentity();

    let vertices: Align16<[Vertex; 2]> = Align16([
        Vertex {
            u: 0.0,
            v: 0.0,
            color: 0xFFFFFFFF,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vertex {
            u: IMAGE_SIZE as f32,
            v: IMAGE_SIZE as f32,
            color: 0xFFFFFFFF,
            x: IMAGE_SIZE as f32,
            y: IMAGE_SIZE as f32,
            z: 0.0,
        },
    ]);

    loop {
        sceGuStart(GuContextType::Direct, &mut LIST as *mut _ as *mut c_void);

        // Clear Screen
        sceGuClearColor(0x00000000);
        sceGuClearDepth(0);
        sceGuClear(
            ClearBuffer::COLOR_BUFFER_BIT
                | ClearBuffer::DEPTH_BUFFER_BIT
                | ClearBuffer::STENCIL_BUFFER_BIT,
        );

        // Draw Sprite
        sceGumMatrixMode(sys::MatrixMode::Model);
        sceGumLoadIdentity();
        sceGumTranslate(&ScePspFVector3 {
            x: 240.0 - (IMAGE_SIZE as f32 / 2.0),
            y: 136.0 - (IMAGE_SIZE as f32 / 2.0),
            z: 0.0,
        });
        sceGuTexImage(
            MipmapLevel::None,
            IMAGE_SIZE as i32,
            IMAGE_SIZE as i32,
            IMAGE_SIZE as i32,
            &FERRIS as *const Align16<_> as *const _,
        );
        sceGuTexFilter(TextureFilter::Linear, TextureFilter::Linear);
        sceGuTexScale(1.0 / IMAGE_SIZE as f32, 1.0 / IMAGE_SIZE as f32);
        sceGuTexOffset(0.0, 0.0);
        sceGumDrawArray(
            GuPrimitive::Sprites,
            VertexType::TEXTURE_32BITF
                | VertexType::COLOR_8888
                | VertexType::VERTEX_32BITF
                | VertexType::TRANSFORM_3D,
            2,
            ptr::null_mut(),
            &vertices as *const Align16<_> as *const _,
        );

        // End frame
        sceGuFinish();
        sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
        sceDisplayWaitVblankStart();
        sceGuSwapBuffers();
    }

    sceGuTerm();
    sceKernelExitGame();
}
