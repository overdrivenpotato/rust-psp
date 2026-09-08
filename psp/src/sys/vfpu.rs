use super::types::{
    ScePspFMatrix2,
    ScePspFVector2,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Unit(
    result: *mut ScePspFMatrix2
) -> *mut ScePspFMatrix2 {
    vfpu_asm! {
        vmidt_p E000;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S010, 8(a0);
        sv_s S011, 12(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Zero(
    result: *mut ScePspFMatrix2
) -> *mut ScePspFMatrix2 {
    sceVfpuVector2PositiveZero(result as _);
    sceVfpuVector2PositiveZero((result as *mut ScePspFVector2).offset(1));
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Copy(
    dst: *mut ScePspFMatrix2,
    src: *mut ScePspFMatrix2
) -> *mut ScePspFMatrix2 {
    *dst = *src;
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Transform(
    result: *mut ScePspFVector2,
    matrix2: *mut ScePspFMatrix2,
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    vfpu_asm! {
        lv_s S100, 0(a1);
        lv_s S101, 4(a1);
        lv_s S110, 8(a1);
        lv_s S111, 12(a1);
        lv_s S200, 0(a2);
        lv_s S201, 4(a2);
        vtfm2_p C000, E100, C200;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        : : "{4}"(result), "{5}"(matrix2), "{6}"(vector2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Mul(
    result: *mut ScePspFMatrix2,
    arg1: *mut ScePspFMatrix2,
    arg2: *mut ScePspFMatrix2,
) -> *mut ScePspFMatrix2 {
    vfpu_asm! {
        lv_s S100, 0(a1);
        lv_s S101, 4(a1);
        lv_s S110, 8(a1);
        lv_s S111, 12(a1);
        lv_s S200, 0(a2);
        lv_s S201, 4(a2);
        lv_s S210, 8(a2);
        lv_s S211, 12(a2);
        vmmul_p E000, E200, E100;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S010, 8(a0);
        sv_s S011, 12(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Scale(
    dst: *mut ScePspFMatrix2,
    src: *mut ScePspFMatrix2,
    scale: f32,
) -> *mut ScePspFMatrix2 {
    sceVfpuVector2Scale(dst as _, src as _, scale);
    sceVfpuVector2Scale((dst as *mut ScePspFVector2).offset(1), (src as *mut ScePspFVector2).offset(1), scale);
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Transpose(
    result: *mut ScePspFMatrix2,
    input: *mut ScePspFMatrix2,
) -> *mut ScePspFMatrix2 {
    (*result).x.y = (*input).y.x;
    (*result).x.x = (*input).x.x;
    (*result).y.x = (*input).x.y;
    (*result).y.y = (*input).y.y;
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2RotZ(
    result: *mut ScePspFMatrix2,
    matrix2: *mut ScePspFMatrix2,
    rotz_radians: f32,
) -> *mut ScePspFMatrix2 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        lv_s S100, t0;
        vcst_s S101,VFPU_2_PI;
        vmul_s S100,S100,S101;
        vrot_p C000,S100,[C,S];
        vrot_p C010,S100,[-S,C];
        : : "f"(rotz_radians) : "8", "memory" : "volatile"
    } 
    if !matrix2.is_null() {
        vfpu_asm! {
            lv_s S020,0(a1);
            lv_s S021,4(a1);
            lv_s S030,8(a1);
            lv_s S031,12(a1);
            vmmul_p E200,E100,E000;
            : : "{5}"(matrix2) : "memory" : "volatile"
        }
    } else {
        vfpu_asm! { vmmov_p E200,E000; : : : : "volatile" }
    }
    vfpu_asm! {
        sv_s S200,0(a0);
        sv_s S201,4(a0);
        sv_s S210,8(a0);
        sv_s S211,12(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2IsUnit(
    matrix2: *mut ScePspFMatrix2,
) -> bool {
    (*matrix2).x.x == 1.0 && (*matrix2).y.y == 1.0 
        && (*matrix2).x.y == 0.0 &&  (*matrix2).y.x == 0.0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Trace(
    matrix2: *mut ScePspFMatrix2,
) -> f32 {
    (*matrix2).x.x + (*matrix2).y.y
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Determinant(
    matrix2: *mut ScePspFMatrix2,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_s S000,0(a0);
        lv_s S001,4(a0);
        lv_s S010,8(a0);
        lv_s S011,12(a0);
        vdet_p S000, C000, C010;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(matrix2) : "8","memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix2Adjoint(
    result: *mut ScePspFMatrix2,
    matrix2: *mut ScePspFMatrix2,
) -> *mut ScePspFMatrix2 {
    (*result).x.x = (*matrix2).y.y;
    (*result).x.y = -(*matrix2).y.x;
    (*result).y.x = -(*matrix2).x.y;
    (*result).y.y = (*matrix2).x.x;
    result
}

