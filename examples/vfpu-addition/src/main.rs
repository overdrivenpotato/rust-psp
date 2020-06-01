#![no_std]
#![no_main]
#![feature(llvm_asm)]

use psp::sys::kernel::{self, ThreadAttributes};

psp::module!("vfpu_test", 1, 1);

fn vfpu_add(a: i32, b: i32) -> i32 {
    let out;

    unsafe {
        psp::vfpu_asm! (
            mtv a0, S000;
            mtv a1, S001;
            vadd_s S000, S000, S001;
            mfv v0, S000;

            : "={2}"(out)
            : "{4}"(a), "{5}"(b)
        );
    }

    out
}

fn psp_main() {
    psp::enable_home_button();
    psp::dprintln!("Testing VFPU...");
    unsafe {
        kernel::sce_kernel_change_current_thread_attr(0, ThreadAttributes::VFPU);
    }
    psp::dprintln!("VFPU 123 + 4 = {}", vfpu_add(123, 4));
}
