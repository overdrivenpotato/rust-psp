#![no_std]
#![no_main]

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    
    unsafe{
        psp::sys::power::sce_power_set_clock_frequency(333, 333, 166);    
        
        psp::dprint!("Hello PSP from rust!\n");
        psp::dprint!("CPU Clock Speed: {}\n",psp::sys::power::sce_power_get_cpu_clock_frequency_int());
        psp::dprint!("BUS Clock Speed: {}\n",psp::sys::power::sce_power_get_bus_clock_frequency_int());
        psp::dprint!("Battery Level: {}%\n",psp::sys::power::sce_power_get_battery_life_percent());
    }
}
