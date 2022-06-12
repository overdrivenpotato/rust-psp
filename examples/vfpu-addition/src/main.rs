#![recursion_limit = "256"]
#![no_std]
#![feature(asm_experimental_arch)]
#![no_main]

psp::module!("vfpu_test", 1, 1);

fn vfpu_add(a: i32, b: i32) -> i32 {
    let ret_val;

    unsafe {
        psp::vfpu_asm! (
            // Convert `a` to float
            "mtc1 {a}, {ftmp}",
            "nop",
            "cvt.s.w {ftmp}, {ftmp}",
            "mfc1 {a}, {ftmp}",
            "nop",

            // Convert `b` to float
            "mtc1 {b}, {ftmp}",
            "nop",
            "cvt.s.w {ftmp}, {ftmp}",
            "mfc1 {b}, {ftmp}",
            "nop",

            // Perform addition
            "mtv {a}, S000",
            "mtv {b}, S001",
            "vadd.s S000, S000, S001",
            "mfv {ret}, S000",

            // Convert result to `i32`
            "mtc1 {ret}, {ftmp}",
            "nop",
            "cvt.w.s {ftmp}, {ftmp}",
            "mfc1 {ret}, {ftmp}",
            "nop",

            ftmp = out(freg) _,
            a = inout(reg) a => _,
            b = inout(reg) b => _,
            ret = out(reg) ret_val,
            options(nostack, nomem),
        );
    }

    ret_val
}

fn psp_main() {
    psp::enable_home_button();
    psp::dprintln!("Testing VFPU...");
    psp::dprintln!("VFPU 123 + 4 = {}", vfpu_add(123, 4));
}
