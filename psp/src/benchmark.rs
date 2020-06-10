/// Execute `f` `iterations` times and return average duration per iteration
pub fn benchmark<F: FnMut()>(mut f: F, iterations: usize) -> core::time::Duration {
    let mut loop_start: u64 = 0;
    let mut loop_end: u64 = 0;
    let avg_micros: u64;

    unsafe {
        crate::sys::sceRtcGetCurrentTick(&mut loop_start as *mut u64);

        for _ in 0..iterations {
            f();
        }

        crate::sys::sceRtcGetCurrentTick(&mut loop_end as *mut u64);
        let avg_iter_ticks = (loop_end - loop_start) / iterations as u64;
        let ticks_per_sec = crate::sys::sceRtcGetTickResolution();
        avg_micros = ((avg_iter_ticks as f64 / ticks_per_sec as f64) * 1_000_000.0) as u64;
    }

    core::time::Duration::from_micros(avg_micros)
}
