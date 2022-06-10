//! This example only demonstrates functionality regarding the WLAN chip. It is
//! not a networking example. You might want to look into `sceNet*` functions
//! for actual network access.

#![no_std]
#![no_main]

psp::module!("sample_wlan", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        let wlan_power = psp::sys::sceWlanDevIsPowerOn() == 1;
        let wlan_switch = psp::sys::sceWlanGetSwitchState() == 1;

        let mut buf = [0; 8];
        psp::sys::sceWlanGetEtherAddr(&mut buf[0]);

        psp::dprintln!(
            "WLAN switch enabled: {}, WLAN active: {}, \
            MAC address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            wlan_power, wlan_switch,
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5],
        );
    }
}
