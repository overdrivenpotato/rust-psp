#![feature(restricted_std)]
#![no_main]
use std::string::String;

psp::module!("rust_std_hello_world", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let people = vec!["sajattack", "overdrivenpotato", "iridescence"];
    for person in people {
        let x = format!("Hello, {}! I'm coming to you live from the standard library!\n", person);
        psp::dprint!("{}", x);
    }


}
