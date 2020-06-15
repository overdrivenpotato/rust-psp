#![no_std]
#![no_main]

use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{circle::Circle, rectangle::Rectangle, triangle::Triangle};
use embedded_graphics::style::PrimitiveStyleBuilder;
use embedded_graphics::style::TextStyleBuilder;
use tinybmp::Bmp;

use psp::embedded_graphics::PspDisplay;

psp::module!("sample_emb_gfx", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let mut disp = PspDisplay::new();

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::BLACK)
        .build();
    let black_backdrop = Rectangle::new(Point::new(0, 0), Point::new(480, 272)).into_styled(style);
    black_backdrop.draw(&mut disp).unwrap();

    // draw ferris
    let bmp = Bmp::from_slice(include_bytes!("../assets/ferris.bmp")).unwrap();
    let image: Image<Bmp, _> = Image::new(&bmp, Point::zero());
    image.draw(&mut disp).unwrap();

    Triangle::new(
        Point::new(8, 66 + 16),
        Point::new(8 + 16, 66 + 16),
        Point::new(8 + 8, 66),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .build(),
    )
    .draw(&mut disp)
    .unwrap();

    Rectangle::new(Point::new(36, 66), Point::new(36 + 16, 66 + 16))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::GREEN)
                .stroke_width(1)
                .build(),
        )
        .draw(&mut disp)
        .unwrap();

    Circle::new(Point::new(72, 66 + 8), 8)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::BLUE)
                .stroke_width(1)
                .build(),
        )
        .draw(&mut disp)
        .unwrap();

    let rust = Rgb888::new(0xff, 0x07, 0x00);
    Text::new("Hello Rust!", Point::new(0, 86))
        .into_styled(TextStyleBuilder::new(Font6x8).text_color(rust).build())
        .draw(&mut disp)
        .unwrap();
    loop {
        unsafe {
            psp::sys::sceDisplayWaitVblankStart();
            disp.flush();
        }
    }
    //disp.destroy();
}
