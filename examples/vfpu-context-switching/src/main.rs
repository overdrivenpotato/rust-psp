#![no_std]
#![no_main]
#![feature(llvm_asm)]

use psp::sys::vfpu_context::{Context, MatrixSet};

psp::module!("vfpu_context_test", 1, 1);

#[no_mangle]
#[inline(never)]
extern fn psp_main() {
    psp::dprintln!("Testing VFPU context switcher...");

    let mut context = Context::new();

    unsafe {
        context.prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);
        psp::vfpu_asm! {
            vmzero_q M000;

            viim_s S000, 1;
            viim_s S001, 2;
            viim_s S002, 3;
            viim_s S003, 4;

            vmmov_q M300, M000;

            : : : : "volatile"
        }

        // Clobber M300 and M000
        context.prepare(MatrixSet::empty(), MatrixSet::VMAT0 | MatrixSet::VMAT3);
        psp::vfpu_asm! {
            vmzero_q M000;
            vmzero_q M300;

            : : : : "volatile"
        }

        // Read M300 back from the context, and clobber M000.
        context.prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);
        let mut out: i32;
        psp::vfpu_asm! {
            vmmov_q M000, M300;

            mfv t0, S000;
            .mips "mtc1 $$t0, $$f0";
            .mips "cvt.w.s $$f0, $$f0";
            .mips "mfc1 $$t0, $$f0";
            .mips "addu $0, $$zero, $$t0";

            mfv t0, S001;
            .mips "mtc1 $$t0, $$f0";
            .mips "cvt.w.s $$f0, $$f0";
            .mips "mfc1 $$t0, $$f0";
            .mips "addu $0, $0, $$t0";

            mfv t0, S002;
            .mips "mtc1 $$t0, $$f0";
            .mips "cvt.w.s $$f0, $$f0";
            .mips "mfc1 $$t0, $$f0";
            .mips "addu $0, $0, $$t0";

            mfv t0, S003;
            .mips "mtc1 $$t0, $$f0";
            .mips "cvt.w.s $$f0, $$f0";
            .mips "mfc1 $$t0, $$f0";
            .mips "addu $0, $0, $$t0";

            : "=r"(out) : : "t0", "f0" : "volatile"
        }

        psp::dprintln!("1 + 2 + 3 + 4 = {}", out);
    }
}
