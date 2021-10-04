use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{rectangle::Rectangle, PrimitiveStyle, PrimitiveStyleBuilder, Styled},
};

use psp::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn get_background() -> Styled<Rectangle, PrimitiveStyle<Rgb888>> {
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::BLACK)
        .build();
    Rectangle::new(Point::new(0, 0), Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)).into_styled(style)
}
