use core::convert::{From, TryInto};
use crate::sys::{display, ge};
use embedded_graphics::{
    drawable::Pixel,
    pixelcolor::{raw::RawU16, raw::RawData, Rgb565, Bgr565}, 
    geometry::Size,
    DrawTarget,
};

pub struct Framebuffer {
    vram_base: *mut u16,
}

impl Framebuffer {
    pub fn new() -> Self {
        unsafe {
        display::sce_display_set_mode(display::DisplayMode::Lcd, 480, 272);
        let vram_base = (0x4000_0000u32 | ge::sce_ge_edram_get_addr() as u32) as *mut u16;
        display::sce_display_set_frame_buf(vram_base as *const u8, 512, display::DisplayPixelFormat::_565, display::DisplaySetBufSync::NextFrame);
        Framebuffer{vram_base}
        }
    }
}

impl DrawTarget<Rgb565> for Framebuffer {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb565>) -> Result<(), Self::Error> {
        let Pixel(coord, color) = pixel;
        if let Ok((x @ 0..=480u32, y @ 0..=272u32)) = coord.try_into() {
            // I really don't know why I have to convert to Bgr here but fuck it
            unsafe {*(self.vram_base as *mut u16).offset(x as isize).offset((y * 512) as isize) = RawU16::from(Bgr565::from(color)).into_inner();}
        }

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(480, 272)
    }
}
