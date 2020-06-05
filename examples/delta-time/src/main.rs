#![no_std]
#![no_main]

psp::module!("sample_delta_time", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    unsafe{
        let resolution: u32 = psp::sys::rtc::sce_rtc_get_tick_resolution();

        let mut start_time: u64 = 0;
        psp::sys::rtc::sce_rtc_get_current_tick(&mut start_time as *mut u64 );

        psp::dprintln!("Hello PSP from rust!");


        let mut last_time: u64 = 0;
        psp::sys::rtc::sce_rtc_get_current_tick(&mut last_time as *mut u64 );

        psp::dprintln!("Delta Time: {}", ((last_time - start_time) as f32) / (resolution as f32));
    }
}
