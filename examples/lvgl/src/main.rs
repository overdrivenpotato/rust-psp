#![no_std]
#![no_main]

use core::time::Duration;
use psp::embedded_graphics::PspDisplay;

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
    bar.set_range(0, 100).unwrap();
    bar.set_anim_time(1000).unwrap();
    bar.set_value(100, Animation::ON).unwrap();

    loop {
        unsafe { psp::sys::sceDisplayWaitVblankStart(); }
        ui.tick_inc(Duration::from_millis(16));
        ui.task_handler();
        disp.flush();
    }
}
