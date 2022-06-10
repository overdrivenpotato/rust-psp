// TODO: cosf vs cosf32? which makes intrinsics::cosf32 work?
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn cosf32(rad: f32) -> f32 {
    let out;

    vfpu_asm!(
        "mtv {1}, S000",
        "vcst.s S001, VFPU_2_PI",
        "vmul.s S000, S000, S001",
        "vcos.s S000, S000",
        "mfv {tmp}, S000",
        "mtc1 {tmp}, {0}",
        "nop",
        out(freg) out,
        in(reg) rad,
        tmp = out(reg) _, 
        options(nostack),
    );

    out
}
