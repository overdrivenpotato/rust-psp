#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]

use psp_paint_mode::{
    convert_analog_to_delta_with_sensitivity_deadzone, draw_debug_textbox, DrawObject,
};

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

use psp::embedded_graphics::Framebuffer;
use psp::sys::{CtrlButtons, CtrlMode, SceCtrlData};
use psp::{SCREEN_HEIGHT, SCREEN_WIDTH};

psp::module!("Paint Mode Example", 0, 1);

fn psp_main() {
    psp::enable_home_button();

    let disp = &mut Framebuffer::new();
    let mut cur_size = 1;
    let mut draw_obj = DrawObject::new_circle(get_midpoint(), cur_size);
    let mut cur_location = draw_obj.center();

    let mut i = 0;

    disp.clear(Rgb888::BLACK).unwrap();

    unsafe {
        psp::sys::sceCtrlSetSamplingCycle(0);
        psp::sys::sceCtrlSetSamplingMode(CtrlMode::Analog);
    };

    let pad_data = &mut SceCtrlData::default();
    loop {
        unsafe {
            // Read button/analog input
            psp::sys::sceCtrlReadBufferPositive(pad_data, 1);
        }

        if pad_data.buttons.contains(CtrlButtons::START) {
            // Wipe the screen
            disp.clear(Rgb888::BLACK).unwrap();
        }

        if pad_data.buttons.contains(CtrlButtons::RTRIGGER) {
            draw_obj.grow();
        }

        if pad_data.buttons.contains(CtrlButtons::LTRIGGER) {
            draw_obj.shrink();
        }

        if pad_data.buttons.contains(CtrlButtons::CIRCLE) {
            draw_obj = DrawObject::new_circle(cur_location, cur_size);
        }

        if pad_data.buttons.contains(CtrlButtons::TRIANGLE) {
            draw_obj = DrawObject::new_triangle(cur_location, cur_size);
        }

        if pad_data.buttons.contains(CtrlButtons::SQUARE) {
            draw_obj = DrawObject::new_rectangle(cur_location, cur_size);
        }

        if pad_data.buttons.contains(CtrlButtons::CROSS) {
            draw_obj = DrawObject::new_x(cur_location, cur_size);
        }

        let delta_x_pixels = convert_analog_to_delta_with_sensitivity_deadzone(pad_data.lx);
        let delta_y_pixels = convert_analog_to_delta_with_sensitivity_deadzone(pad_data.ly);
        draw_obj.move_by(
            delta_x_pixels,
            delta_y_pixels,
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32,
        );
        draw_obj.draw(disp);

        if i < 10 {
            i += 1;
        } else {
            draw_debug_textbox(disp, pad_data);
            i = 0;
        }

        cur_location = draw_obj.center();
        cur_size = draw_obj.size();
    }
}

fn get_midpoint() -> Point {
    Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2)
}
