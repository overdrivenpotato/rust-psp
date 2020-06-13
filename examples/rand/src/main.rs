#![no_std]
#![no_main]

use psp::rand::Mt19937;

psp::module!("sample_rand", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let mut mt19937 = Mt19937::new(1337).unwrap();
    loop {
        psp::dprintln!("{}", mt19937.random_u32());
    }
}

