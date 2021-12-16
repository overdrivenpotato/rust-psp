#[no_mangle]
pub unsafe extern "C" fn fminf(
    x: f32,
    y: f32,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        .mips "mfc1 $$t0, $1";
        .mips "mfc1 $$t1, $2";
        mtv t0, S000;
        mtv t1, S001;
        vmin_s S000, S000, S001;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "f"(x), "f"(y) : "8","9": "volatile"
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn fmaxf(
    x: f32,
    y: f32,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        .mips "mfc1 $$t0, $1";
        .mips "mfc1 $$t1, $2";
        mtv t0, S000;
        mtv t1, S001;
        vmax_s S000, S000, S001;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "f"(x), "f"(y) : "8","9": "volatile"
    }
    out
}

#[no_mangle]
pub extern "C" fn fmodf(x: f32, y: f32) -> f32 {
    libm::fmodf(x, y)
}

#[no_mangle]
pub unsafe extern "C" fn cosf(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! {
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vcst_s S001, VFPU_2_PI;
        vmul_s S000, S000, S001;
        vcos_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "f"(scalar) : "8": "volatile"
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn sinf(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! {
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vcst_s S001, VFPU_2_PI;
        vmul_s S000, S000, S001;
        vsin_s S000,S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "f"(scalar) : "8": "volatile"
    }
    out
}

