#![no_std]
#![no_main]

use lvgl;
use lvgl::style::Style;
use lvgl::widgets::{Label, LabelAlign};
use lvgl::{Align, Color, Part, State, Widget, UI};
use lvgl_sys;
use core::time::Duration;
use psp::embedded_graphics::Framebuffer;

psp::module!("sample_lvgl", 1, 1);

#[no_mangle]
fn psp_main() {
    psp::enable_home_button();
    let mut disp = Framebuffer::new();

    let mut ui = UI::init().unwrap();

    // Implement and register your display:
    let display_driver = lvgl::DisplayDriver::new(&mut disp);
    ui.disp_drv_register(display_driver);

    // Create screen and widgets
    let mut screen = ui.scr_act().unwrap();

    let font_roboto_28 = unsafe { &lvgl_sys::lv_theme_get_font_normal() };
    let font_noto_sans_numeric_28 = unsafe { &noto_sans_numeric_80 };

    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style).unwrap();

    let mut time = Label::new(&mut screen).unwrap();
    let mut style_time = Style::default();
    //style_time.set_text_font(font_noto_sans_numeric_28);
    style_time.set_text_color(State::DEFAULT, Color::from_rgb((255, 255, 255)));
    time.add_style(Part::Main, style_time).unwrap();
    time.set_align(&mut screen, Align::Center, 0, 0).unwrap();
    time.set_text("20:46").unwrap();
    time.set_width(240).unwrap();
    time.set_height(240).unwrap();

    let mut bt = Label::new(&mut screen).unwrap();
    bt.set_width(50).unwrap();
    bt.set_height(80).unwrap();
    bt.set_recolor(true).unwrap();
    bt.set_text("#5794f2 \u{F293}#").unwrap();
    bt.set_label_align(LabelAlign::Left).unwrap();
    bt.set_align(&mut screen, Align::InTopLeft, 0, 0).unwrap();

    let mut power = Label::new(&mut screen).unwrap();
    power.set_recolor(true).unwrap();
    power.set_width(80).unwrap();
    power.set_height(20).unwrap();
    power.set_text("#fade2a 20%#").unwrap();
    power.set_label_align(LabelAlign::Right).unwrap();
    power.set_align(&mut screen, Align::InTopRight, 0, 0).unwrap();

    loop {
        ui.tick_inc(Duration::from_micros(16667));
        ui.task_handler();
        unsafe {
            psp::sys::sceDisplayWaitVblankStart();
        }
    }
}

extern "C" {
    pub static mut noto_sans_numeric_80: lvgl_sys::lv_font_t;
}
