#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarAbs(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vabs_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8" : "volatile" 
    }
    out
}
 
// sceVfpuScalarAcos
// sceVfpuScalarAsin
// sceVfpuScalarAtan
// sceVfpuScalarAtan2
 
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarCos(
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

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarExp(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vcst_s S001, VFPU_LOG2E;
        vmul_s S000, S000, S001;
        vexp2_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarFloor(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vf2id_s S000, S000;
        vi2f_s S000, S000, 0;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarLog(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vlog2_s S000, S000;
        vcst_s S001, VFPU_LOG2E; 
        vdiv_s S000,S000,S001;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarLog2(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vlog2_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarLog10(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vlog2_s S000, S000;
        vcst_s S001, VFPU_LOG2TEN; 
        vdiv_s S000,S000,S001;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarMax(
    arg1: f32,
    arg2: f32,
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
        : "=f"(out) : "f"(arg1), "f"(arg2) : "8","9": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarMin(
    arg1: f32,
    arg2: f32,
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
        : "=f"(out) : "f"(arg1), "f"(arg2) : "8","9": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarPow(
    arg1: f32,
    arg2: f32,
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        .mips "mfc1 $$t1, $2";
        mtv t0, S000;
        mtv t1, S001;
        vlog2_s S000, S000;
        vmul_s S000, S000, S001;
        vexp2_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(arg1), "f"(arg2) : "8","9": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarPow2(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vexp2_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarRound(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vf2in_s S000, S000, 0;
        vi2f_s S000, S000, 0;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarRsqrt(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vrsq_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarSin(
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

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarSqrt(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vsqrt_s S000, S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarTan(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vcst_s S001, VFPU_2_PI;
        vmul_s S000, S000, S001;
        vrot_p C002, S000, [C,S];
        vdiv_s S000, S003, S002;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuScalarTrunc(
    scalar: f32
) -> f32 {
    let out: f32;
    vfpu_asm! { 
        .mips "mfc1 $$t0, $1";
        mtv t0, S000;
        vf2iz_s S000, S000;
        vi2f_s S000, S000, 0;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0"; 
        : "=f"(out) : "f"(scalar) : "8": "volatile" 
    }
    out
}
