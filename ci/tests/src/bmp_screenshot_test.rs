use alloc::vec::Vec;
use core::ffi::c_void;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, Triangle};

use psp::embedded_graphics::Framebuffer;
use psp::test_runner::TestRunner;

const BLANK_SCREENSHOT: &[u8] = include_bytes!("../assets/blank_screenshot.bmp");
const EG_TRIANGLE_SCREENSHOT: &[u8] = include_bytes!("../assets/embedded_graphics_triangle.bmp");

pub fn test_main(test_runner: &mut TestRunner) {
    test_runner.check_large_collection("blank_screenshot", BLANK_SCREENSHOT, &blank_screenshot());

    test_runner.check_large_collection(
        "embedded_graphics_triangle",
        EG_TRIANGLE_SCREENSHOT,
        &eg_triangle_screenshot(),
    );
}

// NOTE: This does not clear the screen, so running it
// after other tests will most likely fail until that is added.
fn blank_screenshot() -> Vec<u8> {
    psp::screenshot_bmp()
}

fn eg_triangle_screenshot() -> Vec<u8> {
    let mut disp = Framebuffer::new();

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::BLACK)
        .build();
    let black_backdrop = Rectangle::new(Point::new(0, 0), Size::new(160, 80)).into_styled(style);
    black_backdrop.draw(&mut disp).unwrap();
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

    psp::screenshot_bmp()
}

// Useful for generating bmp files for comparison.
fn _write_bmp_helper(screenshot: &[u8]) {
    unsafe {
        let fd = psp::sys::sceIoOpen(
            b"host0:/TEST_OUTPUT.bmp\0" as *const u8,
            psp::sys::IoOpenFlags::CREAT | psp::sys::IoOpenFlags::RD_WR,
            0o777,
        );
        psp::sys::sceIoWrite(
            fd,
            screenshot as *const _ as *const c_void,
            screenshot.len(),
        );
        psp::sys::sceIoClose(fd);
    }
}
