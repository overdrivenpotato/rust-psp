#![no_std]
#![no_main]

use lvgl;
use lvgl::{Object, UI};
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
    let mut screen = ui.scr_act();

    let font_roboto_28 = unsafe { &lvgl_sys::lv_font_roboto_28 };
    let font_noto_sans_numeric_28 = unsafe { &noto_sans_numeric_80 };

    let mut screen_style = lvgl::Style::new();
    screen_style.set_body_main_color(lvgl::Color::from_rgb((0, 0, 0)));
    screen_style.set_body_grad_color(lvgl::Color::from_rgb((0, 0, 0)));
    screen_style.set_body_radius(0);
    screen.set_style(screen_style);

    let mut time = lvgl::Label::new(&mut screen);
    let mut style_time = lvgl::Style::new();
    style_time.set_text_font(font_noto_sans_numeric_28);
    style_time.set_text_color(lvgl::Color::from_rgb((255, 255, 255)));
    time.set_style(style_time);
    time.set_align(&mut screen, lvgl::Align::InLeftMid, 20, 0);
    time.set_text("20:46");
    time.set_width(240);
    time.set_height(240);

    let mut bt = lvgl::Label::new(&mut screen);
    let mut style_bt = lvgl::Style::new();
    style_bt.set_text_font(font_roboto_28);
    let style_power = style_bt.clone();
    bt.set_style(style_bt);
    bt.set_width(50);
    bt.set_height(80);
    bt.set_recolor(true);
    bt.set_text("#5794f2 \u{F293}#");
    bt.set_label_align(lvgl::LabelAlign::Left);
    bt.set_align(&mut screen, lvgl::Align::InTopLeft, 0, 0);

    let mut power = lvgl::Label::new(&mut screen);
    power.set_style(style_power);
    power.set_recolor(true);
    power.set_width(80);
    power.set_height(20);
    power.set_text("#fade2a 20%#");
    power.set_label_align(lvgl::LabelAlign::Right);
    power.set_align(&mut screen, lvgl::Align::InTopRight, 0, 0);

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

// work around a bug in released version of lvgl
// hack the planet
#[no_mangle]
fn strcmp(cs: *const u8, ct: *const u8) -> i32
{
    let mut c1: u8;
    let mut c2: u8;

    loop {
        unsafe {
            c1 = *cs.add(1);
            c2 = *ct.add(1);
        }
        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }
        if c1 == 0 {
            break;
        }
    }
    return 0;
}
