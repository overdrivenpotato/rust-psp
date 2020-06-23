#![no_std]
#![no_main]

use core::time::Duration;

use psp::embedded_graphics::PspDisplay;
use psp::sys;

extern crate alloc;

use lvgl;
use lvgl::{UI, Color, State, Widget, Part, Animation};
use lvgl::style::Style;
use lvgl::widgets::{Calendar, Keyboard, Msgbox, Bar};

psp::module!("sample_lvgl", 1, 1);

#[no_mangle]
fn psp_main() {
    psp::enable_home_button();
    let mut disp = PspDisplay::new();

    let mut ui = UI::init().unwrap();

    // Implement and register your display:
    let display_driver = lvgl::DisplayDriver::new(&mut disp);
    ui.disp_drv_register(display_driver);

    // Create screen and widgets
    let mut screen = ui.scr_act().unwrap();

    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style).unwrap();

    let mut calendar = Calendar::new(&mut screen).unwrap();
    calendar.set_size(200, 250).unwrap();

    let mut keyboard = Keyboard::new(&mut screen).unwrap(); 
    keyboard.set_size(250, 100).unwrap();
    keyboard.set_pos(205, 150).unwrap();

    let mut msgbox = Msgbox::new(&mut screen).unwrap();
    msgbox.set_size(200, 50).unwrap();
    msgbox.set_pos(205, 0).unwrap();
    msgbox.set_text("Hello lvgl-rs fans").unwrap();

    let mut bar = Bar::new(&mut screen).unwrap();
    bar.set_size(250, 20).unwrap();
    bar.set_pos(205, 100).unwrap();
    bar.set_range(0, 20).unwrap();
    //bar.set_anim_time(50000).unwrap();
    //bar.set_value(1000, Animation::ON).unwrap();

    ui.task_handler();
    disp.flush();

    let mut loop_start: u64 = 0;
    let mut loop_end: u64 = 0;
    let mut loop_millis: u64;
    let loops_per_sec = unsafe { sys::sceRtcGetTickResolution() };
    let mut loops = 0;

    loop {
        unsafe {
            sys::sceRtcGetCurrentTick(&mut loop_start as *mut u64);
            bar.set_value(loops, Animation::ON).unwrap();
            sys::sceDisplayWaitVblankStart();
            ui.task_handler();
            disp.flush();
            sys::sceRtcGetCurrentTick(&mut loop_end as *mut u64);
            loop_millis = (((loop_end - loop_start) as f64 / loops_per_sec as f64) * 1_000.0) as u64; 
            ui.tick_inc(Duration::from_millis(loop_millis));
            loops += 1;
        }
    }
}

