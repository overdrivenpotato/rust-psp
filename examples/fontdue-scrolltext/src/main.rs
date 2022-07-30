#![no_std]
#![no_main]

mod graphics;

extern crate alloc;

use alloc::vec::Vec;
use core::f32::consts::TAU;
use core::slice;

use psp::math::sinf;
use psp::sys::TexturePixelFormat;
use psp::vram_alloc::get_vram_allocator;

use graphics::sprite::{Sprite, Vertex};
use graphics::Align4;

use fontdue;

psp::module!("fontdue-scroller", 1, 0);

const FONT: &[u8] = include_bytes!("../assets/Codystar-Regular.ttf") as &[u8];
const TEXT: &'static str = "Rust-PSP";
const BG_COLOR: u32 = 0xff00_0000;
const FONT_COLOR: u32 = 0xff00_ffff;
const LEN: usize = TEXT.len();
const BUF_WIDTH: usize = 64;
const BUF_HEIGHT: usize = 64;

fn psp_main() {
    psp::enable_home_button();

    // Set up buffers
    let mut allocator = get_vram_allocator().unwrap();
    graphics::setup(&mut allocator);
    let texture_buffer = allocator.alloc_texture_pixels(
        (LEN * BUF_WIDTH) as u32,
        BUF_HEIGHT as u32,
        TexturePixelFormat::Psm8888,
    );
    let texture_buffer = unsafe {
        slice::from_raw_parts_mut(
            texture_buffer.as_mut_ptr_direct_to_vram() as *mut u32,
            LEN as usize * BUF_WIDTH * BUF_HEIGHT,
        )
    };
    let vertex_buffer = allocator.alloc_sized::<Vertex>(LEN as u32 * 2);
    let vertex_buffer = unsafe {
        slice::from_raw_parts_mut(
            vertex_buffer.as_mut_ptr_direct_to_vram() as *mut Align4<Vertex>,
            LEN * 2,
        )
    };

    // Load font
    let settings = fontdue::FontSettings {
        scale: 40.0,
        ..fontdue::FontSettings::default()
    };
    let font = fontdue::Font::from_bytes(FONT, settings).unwrap();

    // Get appropriate x positions for every char in the string
    let mut layout = fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYUp);
    layout.reset(&fontdue::layout::LayoutSettings::default());
    layout.append(
        &[font.clone()],
        &fontdue::layout::TextStyle::new(TEXT, 60.0, 0),
    );
    let x_positions = layout
        .glyphs()
        .iter()
        .map(|glyph| glyph.x as i32)
        .collect::<Vec<i32>>();

    // Get character bitmaps, padded to nearest multiple of 4 pixel width, skipping whitespace
    let sprites: Vec<Option<(u32, Sprite)>> = TEXT
        .chars()
        .enumerate()
        .map(|(i, letter)| {
            if !letter.is_whitespace() {
                let (metrics, bitmap) = font.rasterize(letter, 60.0);
                let padded_width = (metrics.width + 3) & !3;
                let diff = padded_width - metrics.width;

                let mut j = 0;
                for (k, alpha) in bitmap.iter().enumerate() {
                    if k % metrics.width == 0 {
                        j += diff;
                    }
                    texture_buffer[j + i * BUF_WIDTH * BUF_HEIGHT] =
                        0x00ff_ffff | (*alpha as u32) << 24;
                    j += 1;
                }

                let sprite = Sprite::new(
                    FONT_COLOR,
                    0,
                    0,
                    metrics.width as u32,
                    metrics.height as u32,
                );
                Some((padded_width as u32, sprite))
            } else {
                None
            }
        })
        .collect();

    // Draw chars in a sine wave scroller every frame
    let mut val = 80.0;
    loop {
        graphics::clear_color(BG_COLOR);
        let mut j = 0;
        for i in 0..LEN {
            if let Some(mut sprite) = sprites[i] {
                sprite.1.set_pos(
                    x_positions[i] + val as i32,
                    100 + (20.0 * unsafe { sinf(j as f32 + val / TAU) }) as i32,
                );
                vertex_buffer[i * 2..i * 2 + 2].copy_from_slice(&sprite.1.as_vertices());
                graphics::draw_vertices(
                    &vertex_buffer[i * 2..i * 2 + 2],
                    &texture_buffer[i * BUF_WIDTH * BUF_HEIGHT..(i + 1) * BUF_WIDTH * BUF_HEIGHT],
                    sprite.0,
                    BUF_WIDTH as u32,
                    BUF_HEIGHT as u32,
                    1.0,
                    1.0,
                );
            }
            j += 1;
        }
        graphics::finish_frame();
        val -= 1.0;
    }
}
