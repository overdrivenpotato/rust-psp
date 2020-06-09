#![no_std]
#![no_main]

use psp::Align16;
use core::{ptr, ffi::c_void};
use psp::sys::{
    self, ScePspFVector3, DisplayPixelFormat, Context, SyncMode, SyncBehavior,
    Primitive, TextureFilter, TextureEffect, TextureColorComponent,
    FrontFaceDirection, ShadingModel, GuState, TexturePixelFormat, DepthFunc,
    VertexType, ClearBuffer, MipmapLevel,
};

psp::module!("sample_cube", 1, 1);

// Both width and height, this is a square image.
const IMAGE_SIZE: usize = 128;

// The image data *must* be aligned to a 16 byte boundary.
static FERRIS: Align16<[u8; IMAGE_SIZE * IMAGE_SIZE * 4]> = Align16(*include_bytes!("../ferris.bin"));

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

#[repr(C, align(4))]
struct Vertex {
    u: f32,
    v: f32,
    x: f32,
    y: f32,
    z: f32,
}

static VERTICES: Align16<[Vertex; 12 * 3]> = Align16([
    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z:  1.0}, // 0
    Vertex { u: 1.0, v: 0.0, x: -1.0, y:  1.0, z:  1.0}, // 4
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z:  1.0}, // 5

    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z:  1.0}, // 0
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z:  1.0}, // 5
    Vertex { u: 0.0, v: 1.0, x:  1.0, y: -1.0, z:  1.0}, // 1

    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z: -1.0}, // 3
    Vertex { u: 1.0, v: 0.0, x:  1.0, y: -1.0, z: -1.0}, // 2
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z: -1.0}, // 6

    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z: -1.0}, // 3
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z: -1.0}, // 6
    Vertex { u: 0.0, v: 1.0, x: -1.0, y:  1.0, z: -1.0}, // 7

    Vertex { u: 0.0, v: 0.0, x:  1.0, y: -1.0, z: -1.0}, // 0
    Vertex { u: 1.0, v: 0.0, x:  1.0, y: -1.0, z:  1.0}, // 3
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z:  1.0}, // 7

    Vertex { u: 0.0, v: 0.0, x:  1.0, y: -1.0, z: -1.0}, // 0
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z:  1.0}, // 7
    Vertex { u: 0.0, v: 1.0, x:  1.0, y:  1.0, z: -1.0}, // 4

    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z: -1.0}, // 0
    Vertex { u: 1.0, v: 0.0, x: -1.0, y:  1.0, z: -1.0}, // 3
    Vertex { u: 1.0, v: 1.0, x: -1.0, y:  1.0, z:  1.0}, // 7

    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z: -1.0}, // 0
    Vertex { u: 1.0, v: 1.0, x: -1.0, y:  1.0, z:  1.0}, // 7
    Vertex { u: 0.0, v: 1.0, x: -1.0, y: -1.0, z:  1.0}, // 4

    Vertex { u: 0.0, v: 0.0, x: -1.0, y:  1.0, z: -1.0}, // 0
    Vertex { u: 1.0, v: 0.0, x:  1.0, y:  1.0, z: -1.0}, // 1
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z:  1.0}, // 2

    Vertex { u: 0.0, v: 0.0, x: -1.0, y:  1.0, z: -1.0}, // 0
    Vertex { u: 1.0, v: 1.0, x:  1.0, y:  1.0, z:  1.0}, // 2
    Vertex { u: 0.0, v: 1.0, x: -1.0, y:  1.0, z:  1.0}, // 3

    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z: -1.0}, // 4
    Vertex { u: 1.0, v: 0.0, x: -1.0, y: -1.0, z:  1.0}, // 7
    Vertex { u: 1.0, v: 1.0, x:  1.0, y: -1.0, z:  1.0}, // 6

    Vertex { u: 0.0, v: 0.0, x: -1.0, y: -1.0, z: -1.0}, // 4
    Vertex { u: 1.0, v: 1.0, x:  1.0, y: -1.0, z:  1.0}, // 6
    Vertex { u: 0.0, v: 1.0, x:  1.0, y: -1.0, z: -1.0}, // 5
]);

const BUF_WIDTH: i32 = 512;
const SCR_WIDTH: i32 = 480;
const SCR_HEIGHT: i32 = 272;

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

        _ => 0,
    }
}

unsafe fn get_static_vram_buffer(width: i32, height: i32, psm: TexturePixelFormat) -> *mut c_void {
    static mut STATIC_OFFSET: i32 = 0;

    let mem_size = get_memory_size(width, height, psm);
    let result = STATIC_OFFSET as *mut _;

    STATIC_OFFSET += mem_size;

    result
}

fn psp_main() {
    unsafe { psp_main_inner() }
}

unsafe fn psp_main_inner() {
    psp::enable_home_button();

    let fbp0 = get_static_vram_buffer(BUF_WIDTH, SCR_HEIGHT, TexturePixelFormat::Psm8888);
    let fbp1 = get_static_vram_buffer(BUF_WIDTH, SCR_HEIGHT, TexturePixelFormat::Psm8888);
    let zbp = get_static_vram_buffer(BUF_WIDTH, SCR_HEIGHT, TexturePixelFormat::Psm4444);

    sys::sceGumLoadIdentity();

    sys::sceGuInit();

    sys::sceGuStart(Context::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0, BUF_WIDTH);
    sys::sceGuDispBuffer(SCR_WIDTH, SCR_HEIGHT, fbp1, BUF_WIDTH);
    sys::sceGuDepthBuffer(zbp, BUF_WIDTH);
    sys::sceGuOffset(2048 - (SCR_WIDTH as u32 / 2), 2048 - (SCR_HEIGHT as u32 / 2));
    sys::sceGuViewport(2048, 2048, SCR_WIDTH, SCR_HEIGHT);
    sys::sceGuDepthRange(65535, 0);
    sys::sceGuScissor(0, 0, SCR_WIDTH, SCR_HEIGHT);
    sys::sceGuEnable(GuState::ScissorTest);
    sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
    sys::sceGuEnable(GuState::DepthTest);
    sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
    sys::sceGuShadeModel(ShadingModel::Smooth);
    sys::sceGuEnable(GuState::CullFace);
    sys::sceGuEnable(GuState::Texture2D);
    sys::sceGuEnable(GuState::ClipPlanes);
    sys::sceGuFinish();
    sys::sceGuSync(SyncMode::Finish, SyncBehavior::Wait);

    psp::sys::sceDisplayWaitVblankStart();

    sys::sceGuDisplay(true);

    // run sample

    let mut val = 0.0;

    loop {
        sys::sceGuStart(Context::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);

        // clear screen
        sys::sceGuClearColor(0xff554433);
        sys::sceGuClearDepth(0);
        sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);

        // setup matrices for cube

        sys::sceGumMatrixMode(sys::Mode::Projection);
        sys::sceGumLoadIdentity();
        sys::sceGumPerspective(75.0, 16.0 / 9.0, 0.5, 1000.0);

        sys::sceGumMatrixMode(sys::Mode::View);
        sys::sceGumLoadIdentity();

        sys::sceGumMatrixMode(sys::Mode::Model);
        sys::sceGumLoadIdentity();

        {
            let pos = ScePspFVector3 { x: 0.0, y: 0.0, z: -2.5 };
            let rot = ScePspFVector3 {
                x: val * 0.79 * (sys::PI / 180.0),
                y: val * 0.98 * (sys::PI / 180.0),
                z: val * 1.32 * (sys::PI / 180.0),
            };

            sys::sceGumTranslate(&pos);
            sys::sceGumRotateXYZ(&rot);
        }

        // setup texture

        sys::sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
        sys::sceGuTexImage(MipmapLevel::None, 128, 128, 128, &FERRIS as *const _ as *const _);
        sys::sceGuTexFunc(TextureEffect::Replace, TextureColorComponent::Rgb);
        sys::sceGuTexFilter(TextureFilter::Linear, TextureFilter::Linear);
        sys::sceGuTexScale(1.0, 1.0);
        sys::sceGuTexOffset(0.0, 0.0);

        // draw cube

        sys::sceGumDrawArray(
            Primitive::Triangles,
            VertexType::TEXTURE_32BITF | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
            12 * 3,
            ptr::null_mut(),
            &VERTICES as *const Align16<_> as *const _,
        );

        sys::sceGuFinish();
        sys::sceGuSync(SyncMode::Finish, SyncBehavior::Wait);

        sys::sceDisplayWaitVblankStart();
        sys::sceGuSwapBuffers();

        val += 1.0;
    }

    // sys::sceGuTerm();
    // psp::sys::sceKernelExitGame();
}
