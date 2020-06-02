#![no_std]
#![no_main]

use psp::Align16;
use core::{ptr, ffi::c_void};
use psp::sys::{
    gum::{self, FVector3},
    display::DisplayPixelFormat,
    gu::{
        self, Context, SyncMode, SyncBehavior, Primitive, TextureFilter,
        TextureEffect, TextureColorComponent, FrontFaceDirection, ShadingModel,
        State, TexturePixelFormat, DepthFunc, VertexType, ClearBuffer,
        MipmapLevel,
    },
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

    gum::sce_gum_load_identity();

    gu::sce_gu_init();

    gu::sce_gu_start(Context::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
    gu::sce_gu_draw_buffer(DisplayPixelFormat::Psm8888, fbp0, BUF_WIDTH);
    gu::sce_gu_disp_buffer(SCR_WIDTH, SCR_HEIGHT, fbp1, BUF_WIDTH);
    gu::sce_gu_depth_buffer(zbp, BUF_WIDTH);
    gu::sce_gu_offset(2048 - (SCR_WIDTH as u32 / 2), 2048 - (SCR_HEIGHT as u32 / 2));
    gu::sce_gu_viewport(2048, 2048, SCR_WIDTH, SCR_HEIGHT);
    gu::sce_gu_depth_range(65535, 0);
    gu::sce_gu_scissor(0, 0, SCR_WIDTH, SCR_HEIGHT);
    gu::sce_gu_enable(State::ScissorTest);
    gu::sce_gu_depth_func(DepthFunc::GreaterOrEqual);
    gu::sce_gu_enable(State::DepthTest);
    gu::sce_gu_front_face(FrontFaceDirection::Clockwise);
    gu::sce_gu_shade_model(ShadingModel::Smooth);
    gu::sce_gu_enable(State::CullFace);
    gu::sce_gu_enable(State::Texture2D);
    gu::sce_gu_enable(State::ClipPlanes);
    gu::sce_gu_finish();
    gu::sce_gu_sync(SyncMode::Finish, SyncBehavior::Wait);

    psp::sys::display::sce_display_wait_vblank_start();

    gu::sce_gu_display(true);

    // run sample

    let mut val = 0.0;

    loop {
        gu::sce_gu_start(Context::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);

        // clear screen
        gu::sce_gu_clear_color(0xff554433);
        gu::sce_gu_clear_depth(0);
        gu::sce_gu_clear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);

        // setup matrices for cube

        gum::sce_gum_matrix_mode(gum::Mode::Projection);
        gum::sce_gum_load_identity();
        gum::sce_gum_perspective(75.0, 16.0 / 9.0, 0.5, 1000.0);

        gum::sce_gum_matrix_mode(gum::Mode::View);
        gum::sce_gum_load_identity();

        gum::sce_gum_matrix_mode(gum::Mode::Model);
        gum::sce_gum_load_identity();

        {
            let pos = FVector3 { x: 0.0, y: 0.0, z: -2.5 };
            let rot = FVector3 {
                x: val * 0.79 * (gu::PI / 180.0),
                y: val * 0.98 * (gu::PI / 180.0),
                z: val * 1.32 * (gu::PI / 180.0),
            };

            gum::sce_gum_translate(&pos);
            gum::sce_gum_rotate_xyz(&rot);
        }

        // setup texture

        gu::sce_gu_tex_mode(TexturePixelFormat::Psm8888, 0, 0, 0);
        gu::sce_gu_tex_image(MipmapLevel::None, 128, 128, 128, &FERRIS as *const _ as *const _);
        gu::sce_gu_tex_func(TextureEffect::Replace, TextureColorComponent::Rgb);
        gu::sce_gu_tex_filter(TextureFilter::Linear, TextureFilter::Linear);
        gu::sce_gu_tex_scale(1.0, 1.0);
        gu::sce_gu_tex_offset(0.0, 0.0);

        // draw cube

        gum::sce_gum_draw_array(
            Primitive::Triangles,
            VertexType::TEXTURE_32BITF | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
            12 * 3,
            ptr::null_mut(),
            &VERTICES as *const Align16<_> as *const _,
        );

        gu::sce_gu_finish();
        gu::sce_gu_sync(SyncMode::Finish, SyncBehavior::Wait);

        psp::sys::display::sce_display_wait_vblank_start();
        gu::sce_gu_swap_buffers();

        val += 1.0;
    }

    // gu::sce_gu_term();
    // psp::sys::kernel::sce_kernel_exit_game();
}
