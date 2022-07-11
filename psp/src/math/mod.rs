#[no_mangle]
pub unsafe extern "C" fn fminf(x: f32, y: f32) -> f32 {
    let out: f32;
    if x.is_nan() && !y.is_nan() {
        out = y;
    } else if y.is_nan() && !x.is_nan() {
        out = x;
    } else if x.is_nan() && y.is_nan() {
        out = core::f32::NAN;
    } else {
        vfpu_asm! (
            "mfc1 {tmp1}, {x}",
            "mfc1 {tmp2}, {y}",
            "mtv {tmp1}, S000",
            "mtv {tmp2}, S001",
            "vmin.s S000, S000, S001",
            "mfv {tmp1}, S000",
            "mtc1 {tmp1}, {out}",
            "nop",
            x = in(freg) x,
            y = in(freg) y,
            tmp1 = out(reg) _,
            tmp2 = out(reg) _,
            out = out(freg) out,
            options(nostack, nomem),
        );
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn fmaxf(x: f32, y: f32) -> f32 {
    let out: f32;
    if x.is_nan() && !y.is_nan() {
        out = y;
    } else if y.is_nan() && !x.is_nan() {
        out = x;
    } else if x.is_nan() && y.is_nan() {
        out = core::f32::NAN;
    } else {
        vfpu_asm! (
            "mfc1 {tmp1}, {x}",
            "mfc1 {tmp2}, {y}",
            "mtv {tmp1}, S000",
            "mtv {tmp2}, S001",
            "vmax.s S000, S000, S001",
            "mfv {tmp1}, S000",
            "mtc1 {tmp1}, {out}",
            "nop",
            x = in(freg) x,
            y = in(freg) y,
            tmp1 = out(reg) _,
            tmp2 = out(reg) _,
            out = out(freg) out,
            options(nostack, nomem),
        );
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn cosf(scalar: f32) -> f32 {
    let out: f32;
    vfpu_asm! (
        "mfc1 {tmp}, {scalar}",
        "mtv {tmp}, S000",
        "nop",
        "vcst.s S001, VFPU_2_PI",
        "vmul.s S000, S000, S001",
        "vcos.s S000, S000",
        "mfv {tmp}, S000",
        "mtc1 {tmp}, {scalar}",
        "nop",
        scalar = inlateout(freg) scalar => out,
        tmp = out(reg) _,
        options(nostack, nomem),
    );
    out
}

#[no_mangle]
pub unsafe extern "C" fn sinf(scalar: f32) -> f32 {
    let out: f32;
    vfpu_asm! (
        "mfc1 {tmp}, {scalar}",
        "mtv {tmp}, S000",
        "nop",
        "vcst.s S001, VFPU_2_PI",
        "vmul.s S000, S000, S001",
        "vsin.s S000, S000",
        "mfv {tmp}, S000",
        "mtc1 {tmp}, {scalar}",
        "nop",
        scalar = inlateout(freg) scalar => out,
        tmp = out(reg) _,
        options(nostack, nomem),
    );
    out
}
// borrowed from https://github.com/samcrow/cmsis_dsp.rs/blob/master/src/libm_c.rs
macro_rules! forward {
    // One argument, argument and return types are the same
    { $( $name:ident($value_type:ty) ,)+ } => {
        $(
            #[no_mangle]
            pub extern "C" fn $name(value: $value_type) -> $value_type {
                libm::$name(value)
            }
        )+
    };
    // Two arguments, argument and return types may be different
    { $( $name:ident($arg1_type:ty, $arg2_type:ty) -> $return_type:ty ,)+ }
    => {
        $(
            #[no_mangle]
            pub extern "C" fn $name(arg1: $arg1_type, arg2: $arg2_type) -> $return_type {
                libm::$name(arg1, arg2)
            }
        )+
    };
    // Three arguments, argument and return types may be different
    { $( $name:ident($arg1_type:ty, $arg2_type:ty, $arg3_type:ty) -> $return_type:ty ,)+ }
    => {
        $(
            #[no_mangle]
            pub extern "C" fn $name(arg1: $arg1_type, arg2: $arg2_type, arg3: $arg3_type) -> $return_type {
                libm::$name(arg1, arg2, arg3)
            }
        )+
    };
}

// One-argument functions
forward! {
    fabsf(f32),
    fabs(f64),
    expf(f32),
    exp(f64),
    exp2(f64),
    exp2f(f32),
    expm1(f64),
    expm1f(f32),
    log(f64),
    logf(f32),
    log10(f64),
    log10f(f32),
    log2(f64),
    log2f(f32),
    log1p(f64),
    log1pf(f32),
    sqrtf(f32),
    sqrt(f64),
    cbrtf(f32),
    cbrt(f64),
    sin(f64),
    cos(f64),
    tan(f64),
    tanf(f32),
    asin(f64),
    asinf(f32),
    acos(f64),
    acosf(f32),
    atan(f64),
    atanf(f32),
    sinh(f64),
    sinhf(f32),
    cosh(f64),
    coshf(f32),
    tanh(f64),
    tanhf(f32),
    asinh(f64),
    asinhf(f32),
    acosh(f64),
    acoshf(f32),
    atanh(f64),
    atanhf(f32),
    erf(f64),
    erff(f32),
    erfc(f64),
    erfcf(f32),
    tgamma(f64),
    tgammaf(f32),
    lgamma(f64),
    lgammaf(f32),
    ceil(f64),
    ceilf(f32),
    floor(f64),
    floorf(f32),
    trunc(f64),
    round(f64),
    roundf(f32),
}

// Two-argument functions
forward! {
    fmod(f64, f64) -> f64,
    fmodf(f32, f32) -> f32,
    remainder(f64, f64) -> f64,
    remainderf(f32, f32) -> f32,
    fmax(f64, f64) -> f64,
    fmin(f64, f64) -> f64,
    fdim(f64, f64) -> f64,
    fdimf(f32, f32) -> f32,
    pow(f64, f64) -> f64,
    powf(f32, f32) -> f32,
    hypot(f64, f64) -> f64,
    hypotf(f32, f32) -> f32,
    atan2(f64, f64) -> f64,
    atan2f(f32, f32) -> f32,
}

// Three-argument functions
forward! {
    fma(f64, f64, f64) -> f64,
    fmaf(f32, f32, f32) -> f32,
}
