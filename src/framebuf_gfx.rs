use crate::sys::{display, ge};
use core::convert::TryInto;
use embedded_graphics::{
    drawable::Pixel,
    geometry::Size,
    pixelcolor::{Rgb888, RgbColor},
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
            display::sce_display_set_frame_buf(
                vram_base as *const u8,
                512,
                display::DisplayPixelFormat::_8888,
                display::DisplaySetBufSync::NextFrame,
            );
            Framebuffer { vram_base }
        }
    }
}

impl DrawTarget<Rgb888> for Framebuffer {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let Pixel(coord, color) = pixel;

        if let Ok((x @ 0..=480u32, y @ 0..=272u32)) = coord.try_into() {
            unsafe {
                let ptr = (self.vram_base as *mut u32)
                    .offset(x as isize)
                    .offset((y * 512) as isize);

                *ptr = (color.r() as u32)
                    | ((color.g() as u32) << 8)
                    | ((color.b() as u32) << 16);
            }
        }

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(480, 272)
    }
}
