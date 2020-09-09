#![no_std]
#![no_main]

use core::f32::consts::PI;

use psp::{
    math::{cosf32, sinf32},
    sys::{
        self, ClearBuffer, DepthFunc, DisplayPixelFormat, FrontFaceDirection, GuContextType,
        GuPrimitive, GuState, GuSyncBehavior, GuSyncMode, LightComponent, LightType::Pointlight,
        MatrixMode, ScePspFVector3, ShadingModel, TexturePixelFormat, VertexType,

        sceAudioChReserve, sceAudioOutput, sceAudioGetChannelRestLen, 
        sceAudioSetChannelDataLen, 
    },
    vram_alloc::get_vram_allocator,
    Align16, BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

#[repr(C, align(4))]
pub struct NPVertex {
    nx: f32,
    ny: f32,
    nz: f32,
    px: f32,
    py: f32,
    pz: f32,
}

static CRAB_VERTICES: Align16<[NPVertex; 27684]> = include!("../assets/crab.in");

psp::module!("crab-rave", 1, 1);

const LIGHT_DISTANCE: f32 = 5.0;

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

static AUDIO_CLIP: [u8; 672320] = *include_bytes!("../assets/crabrave.pcm"); 

const MAX_VOL: i32 = 0x8000;
const MAX_SAMPLES: usize = 65472;
const CHANNEL: i32 = 0;

fn psp_main() {
    psp::enable_home_button();

    let np_vertex_format: VertexType = VertexType::NORMAL_32BITF | VertexType::VERTEX_32BITF;

    unsafe { 
        sceAudioChReserve(CHANNEL, MAX_SAMPLES as i32, psp::sys::AudioFormat::Stereo) 
    };
    let mut start_pos: usize = 0;
    let mut restlen = 0;

    unsafe {
        sys::sceKernelChangeCurrentThreadAttr(0, sys::ThreadAttributes::VFPU);
        sys::sceKernelDcacheWritebackAll();

        let mut allocator = get_vram_allocator().unwrap();
        let fbp0 = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
            .as_mut_ptr_from_zero();
        let fbp1 = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
            .as_mut_ptr_from_zero();
        let zbp = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444)
            .as_mut_ptr_from_zero();

        sys::sceGuInit();
        sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut _ as _);
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
        sys::sceGuDispBuffer(
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32,
            fbp1 as _,
            BUF_WIDTH as i32,
        );
        sys::sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);
        sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
        sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuDepthRange(0xc350, 0x2710);
        sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
        sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
        sys::sceGuShadeModel(ShadingModel::Smooth);
        sys::sceGuEnable(GuState::DepthTest);
        sys::sceGuEnable(GuState::CullFace);
        sys::sceGuEnable(GuState::ClipPlanes);
        sys::sceGuEnable(GuState::Lighting);
        sys::sceGuEnable(GuState::Light0);
        sys::sceGuEnable(GuState::Light1);
        sys::sceGuEnable(GuState::Light2);
        sys::sceGuEnable(GuState::Light3);
        sys::sceGuFinish();
        sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);

        let mut val = 0.0;

        loop {
            if (start_pos+MAX_SAMPLES*4) < AUDIO_CLIP.len() {
                if restlen == 0 {
                        sceAudioOutput(
                            CHANNEL,
                            MAX_VOL,
                            AUDIO_CLIP.as_ptr().add(start_pos) as *mut _
                        );
                    start_pos += MAX_SAMPLES*4;
                }
            } else {
                let remainder: i32 = (((AUDIO_CLIP.len() % (MAX_SAMPLES*4)/4)+63) & !63) as i32;
                if restlen == 0 {
                    sceAudioSetChannelDataLen(CHANNEL, remainder);
                    sceAudioOutput(
                        CHANNEL,
                        MAX_VOL,
                        AUDIO_CLIP.as_ptr().add(start_pos) as *mut _
                    );
                    start_pos += (remainder*4) as usize;
                }
                if start_pos >= AUDIO_CLIP.len() {
                    start_pos = 0;
                    sceAudioSetChannelDataLen(CHANNEL, MAX_SAMPLES as i32);
                }
            }

            restlen = sceAudioGetChannelRestLen(CHANNEL);

            sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut _ as _);
            sys::sceGuClearColor(0x554433);
            sys::sceGuClearDepth(0);
            sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);

            for i in 0..4 {
                let pos = ScePspFVector3 {
                    x: cosf32(i as f32 * (PI / 2.0) + val as f32 * (PI / 180.0)) * LIGHT_DISTANCE,
                    y: 0.0,
                    z: (sinf32(i as f32 * (PI / 2.0) + val as f32 * (PI / 180.0)) * LIGHT_DISTANCE),
                };
                sys::sceGuLight(
                    i,
                    Pointlight,
                    LightComponent::DIFFUSE | LightComponent::SPECULAR,
                    &pos,
                );
                sys::sceGuLightColor(i, LightComponent::DIFFUSE, 0xff00_ffff);
                sys::sceGuLightColor(i, LightComponent::SPECULAR, 0xffff_ffff);
                sys::sceGuLightAtt(i, 0.0, 1.0, 0.0);
            }

            sys::sceGuSpecular(12.0);
            sys::sceGuAmbient(0x0022_2222);

            sys::sceGumMatrixMode(MatrixMode::Projection);
            sys::sceGumLoadIdentity();
            sys::sceGumPerspective(75.0, 16.0 / 9.0, 1.0, 1000.0);

            sys::sceGumMatrixMode(MatrixMode::View);
            let pos = ScePspFVector3 {
                x: 0.0,
                y: 0.0,
                z: -9.0,
            };
            sys::sceGumLoadIdentity();
            sys::sceGumTranslate(&pos);

            sys::sceGumMatrixMode(MatrixMode::Model);
            let pos = ScePspFVector3 {
                x: 0.0,
                y: -1.5,
                z: 0.0,
            };
            sys::sceGumLoadIdentity();
            sys::sceGumTranslate(&pos);

            sys::sceGuColor(0xff7777);

            sys::sceGumMatrixMode(MatrixMode::Model);
            let rot = ScePspFVector3 {
                x: val * 0.79 * (PI / 180.0),
                y: val * 0.98 * (PI / 180.0),
                z: val * 1.32 * (PI / 180.0),
            };
            sys::sceGumLoadIdentity();
            sys::sceGumRotateXYZ(&rot);

            sys::sceGuColor(0xff0000ff);
            sys::sceGumDrawArray(
                GuPrimitive::Triangles,
                np_vertex_format | VertexType::TRANSFORM_3D,
                CRAB_VERTICES.0.len() as i32,
                core::ptr::null(), 
                &CRAB_VERTICES.0 as *const _ as _,
            );
            sys::sceGuFinish();
            sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();

            val += 1.0;
        }
    }
}
