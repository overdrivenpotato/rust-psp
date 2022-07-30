#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

use psp::sys::vfpu_context::{Context, MatrixSet};

psp::module!("vfpu_context_test", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    psp::dprintln!("Testing VFPU context switcher...");

    let mut context = Context::new();

    unsafe {
        context.prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);
        psp::vfpu_asm! {
            "vmzero.q M000",

            "viim.s S000, 1",
            "viim.s S001, 2",
            "viim.s S002, 3",
            "viim.s S003, 4",

            "vmmov.q M300, M000",

            options(nomem, nostack),
        }

        // Clobber M300 and M000
        context.prepare(MatrixSet::empty(), MatrixSet::VMAT0 | MatrixSet::VMAT3);
        psp::vfpu_asm! {
            "vmzero.q M000",
            "vmzero.q M300",
            options(nomem, nostack),
        }

        // Read M300 back from the context, and clobber M000.
        context.prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);
        let mut out: i32;
        psp::vfpu_asm! {
            "vmmov.q M000, M300",

            "mfv {tmp}, S000",
            "mtc1 {tmp}, {ftmp}",
            "nop",
            "cvt.w.s {ftmp}, {ftmp}",
            "mfc1 {tmp}, {ftmp}",
            "nop",
            "addu {out}, $0, {tmp}",

            "mfv t0, S001",
            "mtc1 $8, {ftmp}",
            "nop",
            "cvt.w.s {ftmp}, {ftmp}",
            "mfc1 $8, {ftmp}",
            "nop",
            "addu {out}, {out}, $8",

            "mfv t0, S002",
            "mtc1 $8, {ftmp}",
            "nop",
            "cvt.w.s {ftmp}, {ftmp}",
            "mfc1 $8, {ftmp}",
            "nop",
            "addu {out}, {out}, $8",

            "mfv t0, S003",
            "mtc1 $8, {ftmp}",
            "nop",
            "cvt.w.s {ftmp}, {ftmp}",
            "mfc1 $8, {ftmp}",
            "nop",
            "addu {out}, {out}, $8",

            out = out(reg) out,
            tmp = out(reg) _,
            ftmp = out(freg) _,
            options(nostack, nomem),
        }

        psp::dprintln!("1 + 2 + 3 + 4 = {}", out);
    }
}
