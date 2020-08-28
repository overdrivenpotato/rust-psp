#![feature(restricted_std)]
#![no_main]

use core::{ptr, ffi::c_void};

use psp::{sys, vram_alloc::get_vram_allocator, BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, 
    sys::{
        TexturePixelFormat, DisplayPixelFormat, GuPrimitive, VertexType, GuState,
        sceAudioChReserve, sceAudioOutput, sceAudioGetChannelRestLen, 
        sceAudioSetChannelDataLen,
    }
};

use wavefront_obj::obj;
use wavefront_obj::ParseError;

psp::module!("std_obj_draw", 1, 1);

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

static AUDIO_CLIP: [u8; 672320] = *include_bytes!("../assets/crabrave.pcm"); 

const MAX_VOL: i32 = 0x8000;
const MAX_SAMPLES: usize = 65472;
const CHANNEL: i32 = 0;


#[no_mangle]
fn psp_main() -> Result<(), ParseError> {
    psp::enable_home_button();
    let mut vertices: Vec<wavefront_obj::obj::Vertex> = Vec::new();
    match obj::parse("host0:/assets/teapot.obj") {
        Err(e) => {/* psp::dprintln!("{:?}\n", e); panic!();*/ },
        Ok(teapot) => vertices = teapot.objects[0].clone().vertices,
    }

    let mut allocator = get_vram_allocator().unwrap();
    let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
    let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
    let zbp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444).as_mut_ptr_from_zero();

    psp::enable_home_button();
    unsafe { 
        sceAudioChReserve(CHANNEL, MAX_SAMPLES as i32, psp::sys::AudioFormat::Stereo) 
    };
    let mut start_pos: usize = 0;
    let mut restlen = 0;

    unsafe {

        sys::sceGuInit();
        sys::sceGuStart(
            sys::GuContextType::Direct,
            &mut LIST as *mut _ as *mut c_void,
        );
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
        sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1 as _, BUF_WIDTH as i32);
        sys::sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);
        sys::sceGuOffset(2048 - (SCREEN_WIDTH/2), 2048 - (SCREEN_HEIGHT/2));
        sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuDepthRange(65535, 0);
        sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);
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


            sys::sceGuStart(
                sys::GuContextType::Direct,
                &mut LIST as *mut _ as *mut c_void
            );
            sys::sceGuClearColor(0xff554433);
            sys::sceGuClearDepth(0);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT |
                sys::ClearBuffer::DEPTH_BUFFER_BIT
            );

            //sys::sceGumMatrixMode(sys::MatrixMode::Projection);
            //sys::sceGumLoadIdentity();
            //sys::sceGumPerspective(75.0, 16.0 / 9.0, 0.5, 1000.0);

            //sys::sceGumMatrixMode(sys::MatrixMode::View);
            //sys::sceGumLoadIdentity();

            //sys::sceGumMatrixMode(sys::MatrixMode::Model);
            //sys::sceGumLoadIdentity();


            //sys::sceGumDrawArray(
                //GuPrimitive::Triangles,
                //VertexType::TEXTURE_32BITF | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
                //vertices.len() as i32,
                //ptr::null_mut(),
                //vertices.as_ptr() as *const _,
            //);


            //let pos = sys::ScePspFVector3 { x: 0.0, y: 0.0, z: -2.5 };
            
            //sys::sceGumTranslate(&pos);

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }
    //Ok(())
}
