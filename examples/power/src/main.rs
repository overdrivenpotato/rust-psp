#![no_std]
#![no_main]

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    
    unsafe{
        psp::sys::power::sce_power_set_clock_frequency(333, 333, 166);    
        
        psp::dprintln!("Hello PSP from rust!");
        psp::dprintln!("CPU Clock Speed: {}",psp::sys::power::sce_power_get_cpu_clock_frequency_int());
        psp::dprintln!("BUS Clock Speed: {}",psp::sys::power::sce_power_get_bus_clock_frequency_int());
        psp::dprintln!("Battery Level: {}%",psp::sys::power::sce_power_get_battery_life_percent());
    }
}
