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

use crate::vram_alloc::{VramMemChunk, self};

pub struct PspDisplay {
    draw_buf: VramMemChunk, 
    disp_buf: VramMemChunk,
    pub size: Size,
}

static mut LIST: crate::Align16<[u32; 0x40000]> = crate::Align16([0; 0x40000]);

impl PspDisplay {
    pub fn new() -> Self {
        unsafe {
            let size = Size::new(480, 272);

            sys::sceDisplaySetMode(sys::DisplayMode::Lcd, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);

            let mut allocator = vram_alloc::get_vram_allocator().unwrap();
            let disp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm8888);
            let draw = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm8888);
            sys::sceGuInit();
            sys::sceGuStart(
                sys::GuContextType::Direct,
                &mut LIST as *mut _ as *mut c_void,
            );
            sys::sceGuDrawBuffer(sys::DisplayPixelFormat::Psm8888, draw.as_mut_ptr_from_zero() as *mut c_void, BUF_WIDTH as i32);
            sys::sceGuDrawBufferList(sys::DisplayPixelFormat::Psm8888, draw.as_mut_ptr_from_zero() as *mut c_void, BUF_WIDTH as i32);
            sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, disp.as_mut_ptr_from_zero() as *mut c_void, BUF_WIDTH as i32);
            sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuEnable(sys::GuState::ScissorTest);

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceGuDisplay(true);

            Self { draw_buf: draw, disp_buf: disp, size }
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
            sys::sceGuCopyImage(
                sys::DisplayPixelFormat::Psm8888,
                0,
                0,
                480,
                272,
                512,
                self.draw_buf.as_mut_ptr_direct_to_vram() as *mut c_void,
                0,
                0,
                512,
                self.disp_buf.as_mut_ptr_direct_to_vram() as *mut c_void
            );
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        }
    }
}

impl Drop for PspDisplay {
    fn drop(&mut self) {
        unsafe {
            sys::sceGuTerm()
        }
    }
}

impl DrawTarget<Rgb888> for PspDisplay {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;
        if let Some(index) = self.point_to_index(point) {
            unsafe {
                *(self.draw_buf.as_mut_ptr_direct_to_vram() as *mut u32).add(index) = 0xFF << 24 | rgb_to_bgr(RawU24::from(color).into_inner());
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
                    *(self.draw_buf.as_mut_ptr_direct_to_vram() as *mut u32).add(index) =0xFF << 24 |  rgb_to_bgr(RawU24::from(color).into_inner());
                }
            }
        }

        Ok(())
    }

        fn clear(&mut self, color: Rgb888) -> Result<(), Self::Error> {
        unsafe {

            sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut _ as *mut _);
            sys::sceGuClearColor(rgb_to_bgr(RawU24::from(color).into_inner()));
            //sys::sceGuClearDepth(0);
            sys::sceGuClear(sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::FAST_CLEAR_BIT /*| sys::ClearBuffer::DEPTH_BUFFER_BIT*/);
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
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
            .map(|p| 0xFF << 24 | rgb_to_bgr(RawU24::from(p.1).into_inner())) {
                for (j, byte) in color.to_le_bytes().iter().enumerate() {
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
                self.draw_buf.as_mut_ptr_direct_to_vram() as *mut c_void
            );

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        }
        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

#[inline]
fn rgb_to_bgr(rgb: u32) -> u32 {
    core::intrinsics::bswap(rgb << 8 | rgb >> 24)
}
