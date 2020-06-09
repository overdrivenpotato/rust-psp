#![no_std]
#![no_main]

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    unsafe{
        psp::dprintln!("Hello PSP from rust!");
        psp::dprintln!("POWER ON: {}", psp::sys::sceWlanDevIsPowerOn());
        psp::dprintln!("SWITCH ON: {}", psp::sys::sceWlanGetSwitchState());

        let mut ether_addr: [u8; 8] = [0; 8];
        psp::sys::sceWlanGetEtherAddr(ether_addr.as_mut_ptr());
        psp::dprintln!(
            "ETHER ADDR: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            ether_addr[0],
            ether_addr[1],
            ether_addr[2],
            ether_addr[3],
            ether_addr[4],
            ether_addr[5],
        );
    }
}
