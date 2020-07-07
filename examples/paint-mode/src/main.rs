#![no_std]
#![no_main]
#![feature(slice_fill)]
#![feature(clamp)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]

use psp_paint_mode::{
    convert_analog_to_delta_with_sensitivity_deadzone, draw_debug_textbox, get_background,
    DrawObject,
};

use embedded_graphics::prelude::*;

use psp::embedded_graphics::Framebuffer;
use psp::sys::{CtrlButtons, CtrlMode, SceCtrlData};
use psp::{SCREEN_HEIGHT, SCREEN_WIDTH};

psp::module!("Paint Mode Example", 0, 1);

fn psp_main() {
    psp::enable_home_button();

    let disp = &mut Framebuffer::new();
    let background = get_background();
    let mut cur_size = 1;
    let mut draw_obj = DrawObject::new_circle(get_midpoint(), cur_size);
    let mut cur_location = draw_obj.center();

    let mut i = 0;

    background.draw(disp).unwrap();

    unsafe {
        psp::sys::sceCtrlSetSamplingCycle(0);
        psp::sys::sceCtrlSetSamplingMode(CtrlMode::Analaog);
    };

    let pad_data = &mut get_empty_ctrl_data();
    loop {
        unsafe {
            // Read button/analog input
            psp::sys::sceCtrlReadBufferPositive(pad_data, 1);
        }

        if pad_data.buttons.contains(CtrlButtons::START) {
            // Wipe the screen
            background.draw(disp).unwrap();
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

        move_draw_obj(&mut draw_obj, pad_data);
        cur_location = draw_obj.center();
        cur_size = draw_obj.size();

        if i < 10 {
            i += 1;
        } else {
            draw_debug_textbox(disp, pad_data);
            i = 0;
        }

        draw_obj.draw(disp);
    }
}

fn get_empty_ctrl_data() -> SceCtrlData {
    SceCtrlData {
        timestamp: Default::default(),
        buttons: CtrlButtons::empty(),
        lx: Default::default(),
        ly: Default::default(),
        rsrv: Default::default(),
    }
}

fn get_midpoint() -> Point {
    Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2)
}

fn move_draw_obj(draw_obj: &mut DrawObject, pad_data: &SceCtrlData) {
    let delta_x = convert_analog_to_delta_with_sensitivity_deadzone(pad_data.lx);
    let delta_y = convert_analog_to_delta_with_sensitivity_deadzone(pad_data.ly);

    let existing_center = draw_obj.center();
    let requested_delta = Point::new(delta_x, delta_y);
    let mut target_location: Point = existing_center + requested_delta;
    target_location.x = target_location.x.clamp(0, SCREEN_WIDTH as i32);
    target_location.y = target_location.y.clamp(0, SCREEN_HEIGHT as i32);
    let actual_delta = target_location - existing_center;
    draw_obj.translate_mut(actual_delta);
}
