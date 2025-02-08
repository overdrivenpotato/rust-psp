#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Set(
    vector4: *mut ScePspFVector4,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) -> *mut ScePspFVector4 {
    (*vector4).x = x;
    (*vector4).y = y;
    (*vector4).z = z; 
    (*vector4).w = w;
    vector4
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4SetXYZ(
    vector4: *mut ScePspFVector4,
    x: f32,
    y: f32,
    z: f32,
) -> *mut ScePspFVector4 {
    (*vector4).x = x;
    (*vector4).y = y;
    (*vector4).z = z; 
    vector4
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Copy(
    dst: *mut ScePspFVector4,
    src: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    *dst = *src;
    dst 
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4PositiveZero(
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    (*vector4).x = 0.0;
    (*vector4).y = 0.0;
    (*vector4).z = 0.0;
    (*vector4).w = 0.0;
    vector4 
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4NegativeZero(
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    (*vector4).x = f32::from_bits(0x8000_0000);
    (*vector4).y = f32::from_bits(0x8000_0000);
    (*vector4).z = f32::from_bits(0x8000_0000);
    (*vector4).w = f32::from_bits(0x8000_0000);
    vector4 
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Ceil(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        vf2id_q C000, C000, 0;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Trunc(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        vf2iz_q C000, C000, 0;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Round(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        vf2in_q C000, C000, 0;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Floor(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        vf2iu_q C000, C000, 0;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4FromIVector(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspIVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        vi2f_q C000, C000, 0;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Add(
    result: *mut ScePspFVector4,
    left_addend: *mut ScePspFVector4,
    right_addend: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vadd_q C000, C010, C020;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(left_addend), "{6}"(right_addend) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4AddXYZ(
    result: *mut ScePspFVector4,
    left_addend: *mut ScePspFVector4,
    right_addend: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        lv_q C010, 0(a2);
        vadd_t C000, C000, C010;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(left_addend), "{6}"(right_addend) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Sub(
    result: *mut ScePspFVector4,
    minuend: *mut ScePspFVector4,
    subtrahend: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vsub_q C000, C010, C020;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(minuend), "{6}"(subtrahend) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4SubXYZ(
    result: *mut ScePspFVector4,
    minuend: *mut ScePspFVector4,
    subtrahend: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        lv_q C010, 0(a2);
        vsub_t C000, C000, C010;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(minuend), "{6}"(subtrahend) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Mul(
    result: *mut ScePspFVector4,
    multiplicand: *mut ScePspFVector4,
    multiplier: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vmul_q C000, C010, C020;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(multiplicand), "{6}"(multiplier) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4MulXYZ(
    result: *mut ScePspFVector4,
    multiplicand: *mut ScePspFVector4,
    multiplier: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        lv_q C010, 0(a2);
        vmul_t C000, C000, C010;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(multiplicand), "{6}"(multiplier) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Div(
    result: *mut ScePspFVector4,
    dividend: *mut ScePspFVector4,
    divisor: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vdiv_q C000, C010, C020;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(dividend), "{6}"(divisor) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4DivXYZ(
    result: *mut ScePspFVector4,
    dividend: *mut ScePspFVector4,
    divisor: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vdiv_t C000, C010, C020;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(dividend), "{6}"(divisor) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Neg(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    (*result).w = 0.0 - (*vector4).w;
    (*result).z = 0.0 - (*vector4).z;
    (*result).y = 0.0 - (*vector4).y;
    (*result).x = 0.0 - (*vector4).x;
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Abs(
    result: *mut ScePspFVector4,
    input: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    llvm_asm! (
        "abs.s $$f3, $$f1;
        abs.s $$f2, $$f4;
        abs.s $$f5, $$f6;
        abs.s $$f7, $$f8;"
        : "={f3}"((*result).x), "={f2}"((*result).y), "={f5}"((*result).z), "={f7}"((*result).w) : "{f1}"((*input).x), "{f4}"((*input).y), "{f6}"((*input).z), "{f8}"((*input).w) : "memory" : "volatile"
    );
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Lerp(
    result: *mut ScePspFVector4,
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
    arg3: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S030;
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vsub_q C000, C020, C010;
        vscl_q C000, C000, S030;
        vadd_q C010, C010, C000;
        sv_q C010, 0(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2), "f"(arg3) : "8","memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4LerpXYZ(
    result: *mut ScePspFVector4,
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
    arg3: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S030;
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vsub_t C000, C020, C010;
        vscl_t C000, C000, S030;
        vadd_t C010, C010, C000;
        sv_q C010, 0(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2), "f"(arg3) : "8","memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Hermite(
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
    arg3: *mut ScePspFVector4,
    arg4: *mut ScePspFVector4,
    arg5: *mut ScePspFVector4,
    arg6: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t1, $0";
        lv_q	C000,0(a1);
        lv_q	C010,0(a2);
        lv_q	C020,0(t0);
        lv_q	C030,0(a3);
        lv_s	S202,0(t1);
        vone_s	S203;
        vmul_s	S201,S202,S202;
        vpfxs	[2],[1],[1],[-2];
        vmov_q	C100,C100;
        vpfxs	[-3],[-2],[-1],[3];
        vmov_q	C110,C110;
        vmul_s	S200,S201,S202;
        vpfxs	[0],[1],[0],[0];
        vmov_q	C120,C120;
        vpfxs	[1],[0],[0],[0];
        vmov_q	C130,C130;
        vtfm4_q	C210,E100,C200;
        vtfm4_q	C220,E000,C210;
        : : "{4}"(arg1), "{5}"(arg2), "{6}"(arg3), "{7}"(arg4), "{8}"(arg5), "f"(arg6) : "9", "memory" : "volatile"
    }
    arg1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4HermiteXYZ(
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
    arg3: *mut ScePspFVector4,
    arg4: *mut ScePspFVector4,
    arg5: *mut ScePspFVector4,
    arg6: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t1, $0";
        lv_q	C000,0(a1);
        lv_q	C010,0(a2);
        lv_q	C020,0(t0);
        lv_q	C030,0(a3);
        lv_s	S202,0(t1);
        vone_s	S203;
        vmul_s	S201,S202,S202;
        vpfxs	[2],[1],[1],[-2];
        vmov_q	C100,C100;
        vpfxs	[-3],[-2],[-1],[3];
        vmov_q	C110,C110;
        vmul_s	S200,S201,S202;
        vpfxs	[0],[1],[0],[0];
        vmov_q	C120,C120;
        vpfxs	[1],[0],[0],[0];
        vmov_q	C130,C130;
        vtfm4_q	C210,E100,C200;
        vtfm4_q	C220,E000,C210;
        vmov_s S223, S003;
        sv_q C220,0(a0);
        : : "{4}"(arg1), "{5}"(arg2), "{6}"(arg3), "{7}"(arg4), "{8}"(arg5), "f"(arg6) : "9", "memory" : "volatile"
    }
    arg1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Scale(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspFVector4,
    scale: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S010;
        lv_q C000,0(a1);
	vscl_q C000,C000,S010;
	sv_q C000,0(a0);
	: : "{4}"(result),"{5}"(vector4),"f"(scale) : "8", "memory": "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4ScaleXYZ(
    result: *mut ScePspFVector4,
    vector4: *mut ScePspFVector4,
    scale: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S010;
        lv_q C000,0(a1);
	vscl_t C000,C000,S010;
	sv_q C000,0(a0);
	: : "{4}"(result),"{5}"(vector4),"f"(scale) : "8", "memory": "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Clamp(
    result: *mut ScePspFVector4,
    input: *mut ScePspFVector4,
    min: f32,
    max: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        .mips "mfc1 $$t1,$1";
        mtv t0,S010;
        mtv t1,S011;
        lv_q C000, 0(a1);
        vpfxt [X], [X], [X], [X];
        vmax_q C000,C000,C010;
        vpfxt [Y], [Y], [Y], [Y];
        vmin_q C000,C000,C010;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(input), "f"(min), "f"(max) : "8","9","memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4ClampXYZ(
    result: *mut ScePspFVector4,
    input: *mut ScePspFVector4,
    min: f32,
    max: f32,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        .mips "mfc1 $$t1,$1";
        mtv t0,S010;
        mtv t1,S011;
        lv_q C000, 0(a1);
        vpfxt [X], [X], [X], [X];
        vmax_t C000,C000,C010;
        vpfxt [Y], [Y], [Y], [Y];
        vmin_t C000,C000,C010;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(input), "f"(min), "f"(max) : "8","9","memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Max(
    result: *mut ScePspFVector4,
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        lv_q C010, 0(a2);
        vmax_q C000, C000, C010;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Min(
    result: *mut ScePspFVector4,
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vmin_q C000, C010, C020;
        sv_q C000, 0(a0); 
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4InnerProduct(
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_q C010, 0(a0);
        lv_q C020, 0(a1);
        vdot_q S000, C010, C020;
        mfv a0, S000;
        .mips "mtc1 $$a0, $0";
        : "=f"(out) : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4InnerProductXYZ(
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_q C010, 0(a0);
        lv_q C020, 0(a1);
        vdot_t S000, C010, C020;
        mfv a0, S000;
        .mips "mtc1 $$a0, $0";
        : "=f"(out) : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4OuterProductXYZ(
    result: *mut ScePspFVector4,
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vzero_s S003;
        vcrs_t C000, C010, C020;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Funnel(
    arg1: *mut ScePspFVector4,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_q C000,0(a0);
	vfad_q S000,C000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(arg1) : "8","memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4Average(
    arg1: *mut ScePspFVector4,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_q C000,0(a0);
	vavg_q S000,C000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(arg1) : "8","memory" : "volatile"
    }
    out
}

// sceVfpuVector4IsEqual

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4IsZero(
    vector4: *mut ScePspFVector4
) -> bool {
    ((*vector4).x.to_bits() | (*vector4).y.to_bits() | (*vector4).z.to_bits() | (*vector4).w.to_bits()) & 0x7fff_ffff == 0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4SignFloat(
    result: *mut ScePspFVector4,
    input: *mut ScePspFVector4
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000,0(a1);
	vsgn_q C000,C000;
	sv_q C000,0(a0);
        : : "{4}"(result), "{5}"(input) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4SignInt(
    result: *mut ScePspIVector4,
    input: *mut ScePspFVector4
) -> *mut ScePspIVector4 {
    vfpu_asm! {
        lv_q C000,0(a1);
	vsgn_q C000,C000;
	vf2iz_q C000,C000,0;
	sv_q C000,0(a0);
        : : "{4}"(result), "{5}"(input) : "memory" : "volatile"
    }
    result
}

// sceVfpuVector4Normalize
// sceVfpuVector4NormalizeXYZ

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4NormalizePhase(
    result: *mut ScePspFVector4,
    input: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C000,0(a1);
	vcst_q C010,VFPU_PI;
        vadd_q	C020,C000,C010;
        vcst_s	S010,VFPU_2_PI;
        vscl_q	C020,C020,S010;
        vf2id_q	C020,C020,0;
        vi2f_q	C020,C020,0;
        vcst_s	S011,VFPU_2PI;
        vscl_q	C020,C020,S011;
        vsub_q	C000,C000,C020;
	sv_q C000,0(a0);
        : : "{4}"(result), "{5}"(input) : "memory" : "volatile"
    }
    result

}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4LengthXYZ(
    arg1: *mut ScePspFVector4,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_q C000,0(a0);
	vdot_t S010,C000,C000;
	vsqrt_s S010,S010;
        mfv t0, S010;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(arg1) : "8","memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector4DistanceXYZ(
    arg1: *mut ScePspFVector4,
    arg2: *mut ScePspFVector4,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_q C000,0(a0);
        lv_q C010,0(a1);
        vsub_t C000,C000,C010;
	vdot_t S010,C000,C000;
	vsqrt_s S010,S010;
        mfv t0, S010;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(arg1), "{5}"(arg2) : "8","memory" : "volatile"
    }
    out
}

//sceVfpuVector4FaceForwardXYZ
//sceVfpuVector4ReflectXYZ
//sceVfpuVector4RefractXYZ


