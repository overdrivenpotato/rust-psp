#![no_main]
use std::string::String;

psp::module!("rust_std_hello_world", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let x = String::from("Hello, PSP!");
    psp::dprint!("{}", x);
}
