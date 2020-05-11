#![no_std]
#![no_main]

use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    rectangle::Rectangle,
    triangle::Triangle,
    circle::Circle,
};
use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::style::PrimitiveStyleBuilder;

use psp::framebuf_gfx;

psp::module!("emb_gfx", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let mut disp = framebuf_gfx::Framebuffer::new();

    let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build();
    let black_backdrop = Rectangle::new(Point::new(0, 0), Point::new(160, 80)).into_styled(style);
    black_backdrop.draw(&mut disp).unwrap();
    
    // draw ferris
    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86, 64);
    let image: Image<_, Rgb565> = Image::new(&image_raw, Point::new(0, 0));
    image.draw(&mut disp).unwrap();

    
    Triangle::new(
        Point::new(8, 66 + 16),
        Point::new(8 + 16, 66 + 16),
        Point::new(8 + 8, 66),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(1)
            .build(),
    )
    .draw(&mut disp)
    .unwrap();

    Rectangle::new(Point::new(36, 66), Point::new(36 + 16, 66 + 16))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::GREEN)
                .stroke_width(1)
                .build(),
        )
        .draw(&mut disp)
        .unwrap();

    Circle::new(Point::new(72, 66 + 8), 8)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::BLUE)
                .stroke_width(1)
                .build(),
        )
        .draw(&mut disp)
        .unwrap();

    let rust = Rgb565::new(0xff, 0x07, 0x00);
    Text::new("Hello Rust!", Point::new(0, 86))
        .into_styled(TextStyleBuilder::new(Font6x8).text_color(rust).build())
        .draw(&mut disp)
        .unwrap();
}
