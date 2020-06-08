#![no_std]
#![no_main]

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    
    unsafe{
        psp::sys::power::scePowerSetClockFrequency(333, 333, 166);    
        
        psp::dprintln!("Hello PSP from rust!");
        psp::dprintln!("CPU Clock Speed: {}",psp::sys::power::scePowerGetCpuClockFrequencyInt());
        psp::dprintln!("BUS Clock Speed: {}",psp::sys::power::scePowerGetBusClockFrequencyInt());
        psp::dprintln!("Battery Level: {}%",psp::sys::power::scePowerGetBatteryLifePercent());
    }
}
