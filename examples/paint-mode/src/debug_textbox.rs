use core::str;
use numtoa::NumToA;

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::rectangle::Rectangle,
    style::{PrimitiveStyle, PrimitiveStyleBuilder, Styled, TextStyle},
};

use psp::embedded_graphics::Framebuffer;
use psp::sys::SceCtrlData;
use psp::{SCREEN_HEIGHT, SCREEN_WIDTH};
pub fn get_textbox<'a>() -> Styled<Text<'a>, TextStyle<Rgb888, Font6x8>> {
    Text::new("", get_textbox_top_left()).into_styled(TextStyle::new(Font6x8, Rgb888::WHITE))
}

fn get_textbox_top_left() -> Point {
    Point::new(SCREEN_WIDTH as i32 - 42, SCREEN_HEIGHT as i32 - 8)
}

fn get_textbox_wipe_rect() -> Styled<Rectangle, PrimitiveStyle<Rgb888>> {
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::BLACK)
        .build();
    Rectangle::new(
        get_textbox_top_left(),
        Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32),
    )
    .into_styled(style)
}

pub fn draw_debug_textbox(disp: &mut Framebuffer, pad_data: &SceCtrlData) {
    // Create a str holding our analog pad X and Y values
    let mut holder = [' ' as u8; 7];
    holder[3] = ':' as u8;
    pad_data.lx.numtoa(10, &mut holder[..3]);
    pad_data.ly.numtoa(10, &mut holder[4..]);

    let pad_debug_data_str = unsafe {
        // We can be extremely sure that our array holds nothing
        // but ASCII values, so we can safely skip UTF-8 checks.
        str::from_utf8_unchecked(&holder)
    };

    // Instantiate our textboxes
    let textbox_wipe = get_textbox_wipe_rect();
    let mut textbox = get_textbox();
    textbox.primitive.text = pad_debug_data_str;

    // Actually clear and redraw the textbox on screen
    textbox_wipe.draw(disp).unwrap();
    textbox.draw(disp).unwrap();
}
