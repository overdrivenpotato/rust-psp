//! Interop between the `psp` crate and the 2D `embedded-graphics` crate.

use crate::sys;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, BUF_WIDTH};
use core::ffi::c_void;
use core::convert::TryFrom;
use embedded_graphics::{
    drawable::Pixel,
    geometry::{Size, Dimensions},
    pixelcolor::Rgb888,
    pixelcolor::raw::{RawU24, RawData},
    geometry::Point,
    DrawTarget,
    image::{Image, ImageDimensions, IntoPixelIter},
};
use alloc::alloc::{alloc, Layout};

pub struct PspDisplay {
    buf: *mut u32,
    pub size: Size,
}

#[repr(C, align(4))]
struct Vertex {
    u: f32,
    v: f32,
    x: f32,
    y: f32,
    z: f32,
}

static mut LIST: crate::Align16<[u32; 0x40000]> = crate::Align16([0; 0x40000]);
static VERTICES: crate::Align16<[Vertex; 2]> = crate::Align16([
    Vertex { u: 0.0, v: 0.0, x: 0.0, y: 0.0, z: 0.0},
    Vertex { u: 0.9375, v: 0.53125 , x: SCREEN_WIDTH as f32, y: SCREEN_HEIGHT as f32, z: 1.0},
]);
static mut VRAM: *mut u32 = 0x4000_0000 as *mut u32;

impl PspDisplay {
    pub fn new() -> Self {
        unsafe {
            let size = Size::new(480, 272);

            sys::sceDisplaySetMode(sys::DisplayMode::Lcd, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);

            // Cache-through address
            VRAM = (0x4000_0000u32 | sys::sceGeEdramGetAddr() as u32) as *mut u32;

            sys::sceDisplaySetFrameBuf(
                VRAM as *const u8,
                BUF_WIDTH as usize,
                sys::DisplayPixelFormat::Psm8888,
                sys::DisplaySetBufSync::NextFrame,
            );


            let fbp0 = get_static_vram_buffer(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm8888);
            let fbp1 = get_static_vram_buffer(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm8888);
            let zbp = get_static_vram_buffer(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm4444);
            let buf = (0x4400_0000 as *mut u8).add(get_static_vram_buffer(512, 512, sys::TexturePixelFormat::Psm8888) as usize) as *mut u32;

            sys::sceGumLoadIdentity();
            sys::sceGuInit();

            sys::sceGuStart(
                sys::GuContextType::Direct,
                &mut LIST as *mut _ as *mut c_void,
            );
            sys::sceGuDrawBuffer(sys::DisplayPixelFormat::Psm8888, fbp0, BUF_WIDTH as i32);
            sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1, BUF_WIDTH as i32);
            sys::sceGuDepthBuffer(zbp, BUF_WIDTH as i32);
            sys::sceGuOffset(2048 - (SCREEN_WIDTH/2), 2048 - (SCREEN_HEIGHT/2));
            sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuDepthRange(65535, 0);
            sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuEnable(sys::GuState::ScissorTest);

            sys::sceGuEnable(sys::GuState::Texture2D);
            sys::sceGuEnable(sys::GuState::ClipPlanes);

            // setup matrices
            sys::sceGumMatrixMode(sys::MatrixMode::Projection);
            sys::sceGumLoadIdentity();
            sys::sceGumOrtho(0.0, 480.0, 272.0, 0.0, -30.0, 30.0);

            sys::sceGumMatrixMode(sys::MatrixMode::View);
            sys::sceGumLoadIdentity();

            sys::sceGumMatrixMode(sys::MatrixMode::Model);
            sys::sceGumLoadIdentity();

            sys::sceGuTexMode(sys::TexturePixelFormat::Psm8888, 0, 0, 0);

            sys::sceGuTexFunc(sys::TextureEffect::Replace, sys::TextureColorComponent::Rgb);
            sys::sceGuTexFilter(sys::TextureFilter::Linear, sys::TextureFilter::Linear);
            sys::sceGuTexWrap(sys::GuTexWrapMode::Clamp, sys::GuTexWrapMode::Clamp);

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceGuDisplay(true);

            Self { buf, size }
        }
    }

    #[inline]
    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < BUF_WIDTH && y < self.size.height {
                return Some((x + y * BUF_WIDTH) as usize);
            }
        }

        None
    }

    pub fn flush(&mut self) {
        unsafe {
            sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut _ as *mut _);

            sys::sceGuTexImage(sys::MipmapLevel::None, 512, 512, 512, self.buf as *const c_void);

            // draw buffer
            sys::sceGumDrawArray(
                sys::GuPrimitive::Sprites,
                sys::VertexType::TEXTURE_32BITF | sys::VertexType::VERTEX_32BITF | sys::VertexType::TRANSFORM_3D,
                2,
                core::ptr::null_mut(),
                &VERTICES as *const crate::Align16<_> as *const _
            );

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceGuSwapBuffers();
        }
    }

    pub fn destroy(self) {
        unsafe {
            sys::sceGuTerm();
        }
    }
}


impl DrawTarget<Rgb888> for PspDisplay {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;
        if let Some(index) = self.point_to_index(point) {
            unsafe {
                *self.buf.add(index) = rgba_to_bgra(RawU24::from(color).into_inner());
            }
        }
        Ok(())
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Rgb888>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                unsafe {
                    *self.buf.add(index) = rgba_to_bgra(RawU24::from(color).into_inner());
                }
            }
        }

        Ok(())
    }

    fn clear(&mut self, color: Rgb888) -> Result<(), Self::Error> {
        unsafe {

            sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
            sys::sceGuDrawBufferList(sys::DisplayPixelFormat::Psm8888, self.buf as *mut c_void, 512);
            sys::sceGuClearColor(rgba_to_bgra(RawU24::from(color).into_inner()));
            sys::sceGuClearDepth(0);
            sys::sceGuClear(sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::DEPTH_BUFFER_BIT);
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceGuSwapBuffers();
        }
        Ok(())
    }

     fn draw_image<'a, 'b, I>(&mut self, item: &'a Image<'b, I, Rgb888>) -> Result<(), Self::Error>
    where
        &'b I: IntoPixelIter<Rgb888>,
        I: ImageDimensions,
    {
        let dx = item.top_left().x as i32;
        let dy = item.top_left().y as i32;
        let width = item.size().width as i32;
        let height = item.size().height as i32;

        let padded_width = (width + 3) & !3; 
        let diff = (padded_width - width) as usize;

        let image_data_ptr = unsafe {
            alloc(Layout::from_size_align((padded_width*height*4) as usize, 16).unwrap()) 
        };
        let mut i = 0;
        for color in item.into_iter()
            .map(|p| rgba_to_bgra(RawU24::from(p.1).into_inner())) {
                for (j, byte) in color.to_ne_bytes().iter().enumerate() {
                    unsafe {
                        *image_data_ptr.add(i*4+j) = *byte;
                    }
                }
                i += 1;
                if (i % padded_width as usize) == 0 {
                    i += diff;
                }
        }

        unsafe {
            sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut _ as *mut _);
            sys::sceGuCopyImage(
                sys::DisplayPixelFormat::Psm8888,
                0,
                0,
                width,
                height,
                padded_width,
                image_data_ptr as *const c_void,
                dx,
                dy,
                512,
                self.buf as *mut c_void
            );

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceGuSwapBuffers();
        }
        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}


#[inline]
fn rgba_to_bgra(rgba: u32) -> u32 {
    // 0xAABBGGRR -> 0xAARRGGBB
    core::intrinsics::bswap(rgba << 8 | rgba >> 24)
}

fn get_memory_size(width: u32, height: u32, psm: sys::TexturePixelFormat) -> u32 {
    match psm {
        sys::TexturePixelFormat::PsmT4 => (width * height) >> 1,
        sys::TexturePixelFormat::PsmT8 => width * height,

        sys::TexturePixelFormat::Psm5650
        | sys::TexturePixelFormat::Psm5551
        | sys::TexturePixelFormat::Psm4444
        | sys::TexturePixelFormat::PsmT16 => {
            2 * width * height
        }

        sys::TexturePixelFormat::Psm8888 | sys::TexturePixelFormat::PsmT32 => 4 * width * height,

        _ => unimplemented!(),
    }
}

unsafe fn get_static_vram_buffer(width: u32, height: u32, psm: sys::TexturePixelFormat) -> *mut c_void {
    static mut STATIC_OFFSET: u32 = 0;

    let mem_size = get_memory_size(width, height, psm);
    let result = STATIC_OFFSET as *mut _;

    STATIC_OFFSET += mem_size as u32;

    result
}
