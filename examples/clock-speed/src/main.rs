#![no_std]
#![no_main]

psp::module!("sample_clock_speed", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        let cpu = psp::sys::scePowerGetCpuClockFrequency();
        let bus = psp::sys::scePowerGetBusClockFrequency();

        psp::dprintln!("PSP is operating at {}/{}MHz", cpu, bus);
        psp::dprintln!("Setting clock speed to maximum...");

        psp::sys::scePowerSetClockFrequency(333, 333, 166);

        let cpu = psp::sys::scePowerGetCpuClockFrequency();
        let bus = psp::sys::scePowerGetBusClockFrequency();

        psp::dprintln!("PSP is now operating at {}/{}MHz", cpu, bus);
    }
}
