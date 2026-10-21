use super::types::{
    ScePspFVector3,
    ScePspFMatrix3,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Unit(
    result: *mut ScePspFMatrix3
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        vmidt_t E000;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        sv_s S010, 12(a0);
        sv_s S011, 16(a0);
        sv_s S012, 20(a0);
        sv_s S020, 24(a0);
        sv_s S021, 28(a0);
        sv_s S022, 32(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Zero(
    result: *mut ScePspFMatrix3
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        vmzero_t E000;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        sv_s S010, 12(a0);
        sv_s S011, 16(a0);
        sv_s S012, 20(a0);
        sv_s S020, 24(a0);
        sv_s S021, 28(a0);
        sv_s S022, 32(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Copy(
    dst: *mut ScePspFMatrix3,
    src: *mut ScePspFMatrix3
) -> *mut ScePspFMatrix3 {
    *dst = *src;
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Transform(
    result: *mut ScePspFVector3,
    matrix3: *mut ScePspFMatrix3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S100, 0(a1);
        lv_s S101, 4(a1);
        lv_s S102, 8(a1);
        lv_s S110, 12(a1);
        lv_s S111, 16(a1);
        lv_s S112, 20(a1);
        lv_s S120, 24(a1);
        lv_s S121, 28(a1);
        lv_s S122, 32(a1);
        lv_s S200, 0(a2);
        lv_s S201, 4(a2);
        lv_s S202, 8(a2);
        vtfm3_t C000, E100, C200;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(matrix3), "{6}"(vector3) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Mul(
    result: *mut ScePspFMatrix3,
    multiplicand: *mut ScePspFMatrix3,
    multiplier: *mut ScePspFMatrix3,
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        lv_s S100, 0(a1);
        lv_s S101, 4(a1);
        lv_s S102, 8(a1);
        lv_s S110, 12(a1);
        lv_s S111, 16(a1);
        lv_s S112, 20(a1);
        lv_s S120, 24(a1);
        lv_s S121, 28(a1);
        lv_s S122, 32(a1);
        lv_s S200, 0(a2);
        lv_s S201, 4(a2);
        lv_s S202, 8(a2);
        lv_s S210, 12(a2);
        lv_s S211, 16(a2);
        lv_s S212, 20(a2);
        lv_s S220, 24(a2);
        lv_s S221, 28(a2);
        lv_s S222, 32(a2);
        vmmul_t E000, E200, E100;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        sv_s S010, 12(a0);
        sv_s S011, 16(a0);
        sv_s S012, 20(a0);
        sv_s S020, 24(a0);
        sv_s S021, 28(a0);
        sv_s S022, 32(a0);
        : : "{4}"(result), "{5}"(multiplicand), "{6}"(multiplier) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Scale(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
    scale: f32, 
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S200;
        lv_s S100, 0(a1);
        lv_s S101, 4(a1);
        lv_s S102, 8(a1);
        lv_s S110, 12(a1);
        lv_s S111, 16(a1);
        lv_s S112, 20(a1);
        lv_s S120, 24(a1);
        lv_s S121, 28(a1);
        lv_s S122, 32(a1);
        vmscl_t E000, E100, S200;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        sv_s S010, 12(a0);
        sv_s S011, 16(a0);
        sv_s S012, 20(a0);
        sv_s S020, 24(a0);
        sv_s S021, 28(a0);
        sv_s S022, 32(a0);
        : : "{4}"(result), "{5}"(matrix3), "f"(scale) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Transpose(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
) -> *mut ScePspFMatrix3 {
    (*result).x.x = (*matrix3).x.x;
    (*result).x.y = (*matrix3).y.x;
    (*result).x.z = (*matrix3).z.x;
    (*result).y.x = (*matrix3).x.y;
    (*result).y.y = (*matrix3).y.y;
    (*result).y.z = (*matrix3).z.y;
    (*result).z.x = (*matrix3).x.z;
    (*result).z.y = (*matrix3).y.z;
    (*result).z.z = (*matrix3).z.z;
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3RotZ(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
    rotz_radians: f32,
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        lv_s S100, t0;
        vcst_s S101,VFPU_2_PI;
        vmul_s S100,S100,S101;
        vrot_t C000,S100,[C,S,0];
        vrot_t C010,S100,[-S,C,0];
        vidt_q C020;
        : : "f"(rotz_radians) : "8", "memory" : "volatile"
    } 
    if !matrix3.is_null() {
        vfpu_asm! {
            lv_s S100,0(a1);
            lv_s S101,4(a1);
            lv_s S102,8(a1);
            lv_s S110,12(a1);
            lv_s S111,16(a1);
            lv_s S112,20(a1);
            lv_s S120,24(a1);
            lv_s S121,28(a1);
            lv_s S122,32(a1);
            vmmul_p E200,E100,E000;
            : : "{5}"(matrix3) : "memory" : "volatile"
        }
    } else {
        vfpu_asm! { vmmov_p E200,E000; : : : : "volatile" }
    }
    vfpu_asm! {
        sv_s S200,0(a0);
        sv_s S201,4(a0);
        sv_s S202,8(a0);
        sv_s S210,12(a0);
        sv_s S211,16(a0);
        sv_s S212,20(a0);
        sv_s S220,24(a0);
        sv_s S221,28(a0);
        sv_s S222,32(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3RotY(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
    roty_radians: f32,
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        lv_s S100, t0;
        vcst_s S101,VFPU_2_PI;
        vmul_s S100,S100,S101;
        vrot_t C000,S100,[C,0,-S];
        vrot_t C010,S100,[S,0,C];
        vidt_q C020;
        : : "f"(roty_radians) : "8", "memory" : "volatile"
    } 
    if !matrix3.is_null() {
        vfpu_asm! {
            lv_s S100,0(a1);
            lv_s S101,4(a1);
            lv_s S102,8(a1);
            lv_s S110,12(a1);
            lv_s S111,16(a1);
            lv_s S112,20(a1);
            lv_s S120,24(a1);
            lv_s S121,28(a1);
            lv_s S122,32(a1);
            vmmul_p E200,E100,E000;
            : : "{5}"(matrix3) : "memory" : "volatile"
        }
    } else {
        vfpu_asm! { vmmov_p E200,E000; : : : : "volatile" }
    }
    vfpu_asm! {
        sv_s S200,0(a0);
        sv_s S201,4(a0);
        sv_s S202,8(a0);
        sv_s S210,12(a0);
        sv_s S211,16(a0);
        sv_s S212,20(a0);
        sv_s S220,24(a0);
        sv_s S221,28(a0);
        sv_s S222,32(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3RotX(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
    rotx_radians: f32,
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        lv_s S100, t0;
        vcst_s S101,VFPU_2_PI;
        vmul_s S100,S100,S101;
        vrot_t C000,S100,[0,C,S];
        vrot_t C010,S100,[0,-S,C];
        vidt_q C020;
        : : "f"(rotx_radians) : "8", "memory" : "volatile"
    } 
    if !matrix3.is_null() {
        vfpu_asm! {
            lv_s S100,0(a1);
            lv_s S101,4(a1);
            lv_s S102,8(a1);
            lv_s S110,12(a1);
            lv_s S111,16(a1);
            lv_s S112,20(a1);
            lv_s S120,24(a1);
            lv_s S121,28(a1);
            lv_s S122,32(a1);
            vmmul_p E200,E100,E000;
            : : "{5}"(matrix3) : "memory" : "volatile"
        }
    } else {
        vfpu_asm! { vmmov_p E200,E000; : : : : "volatile" }
    }
    vfpu_asm! {
        sv_s S200,0(a0);
        sv_s S201,4(a0);
        sv_s S202,8(a0);
        sv_s S210,12(a0);
        sv_s S211,16(a0);
        sv_s S212,20(a0);
        sv_s S220,24(a0);
        sv_s S221,28(a0);
        sv_s S222,32(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Rot(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
    rot_radians: *mut ScePspFVector3,
) -> *mut ScePspFMatrix3 {
    sceVfpuMatrix3RotZ(result, matrix3, (*rot_radians).z);
    sceVfpuMatrix3RotY(result, matrix3, (*rot_radians).y);
    sceVfpuMatrix3RotX(result, matrix3, (*rot_radians).x);
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3IsUnit(
    matrix3: *mut ScePspFMatrix3,
) -> bool {
    (*matrix3).x.x == 1.0 && (*matrix3).y.y == 1.0 && (*matrix3).z.z == 1.0 
        && (*matrix3).x.y == 0.0 &&  (*matrix3).x.z == 0.0
        && (*matrix3).y.x == 0.0 &&  (*matrix3).y.z == 0.0
        && (*matrix3).z.x == 0.0 &&  (*matrix3).z.y == 0.0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Trace(
    matrix3: *mut ScePspFMatrix3,
) -> f32 {
    (*matrix3).x.x + (*matrix3).y.y + (*matrix3).z.z
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Determinant(
    matrix3: *mut ScePspFMatrix3,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_s S100, 0(a0);
        lv_s S101, 4(a0);
        lv_s S102, 8(a0);
        lv_s S110, 12(a0);
        lv_s S111, 16(a0);
        lv_s S112, 20(a0);
        lv_s S120, 24(a0);
        lv_s S121, 28(a0);
        lv_s S122, 32(a0);
        vpfxs [Y],[Z],[X],[W];
        vpfxt [Z],[X],[Y],[W];
        vmul_t C000, C110, C120;
        vpfxs [Z],[X],[Y],[W];
        vpfxt [Y],[Z],[X],[W];
        vmul_t C010,C100,C110;
        vdot_t S000,C100,C000;
        vdot_t S010,C010,C120;
        vsub_s S000,S000,S010;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(matrix3) : "8","memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Adjoint(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        lv_s S100, 0(a1);
        lv_s S101, 4(a1);
        lv_s S102, 8(a1);
        lv_s S110, 12(a1);
        lv_s S111, 16(a1);
        lv_s S112, 20(a1);
        lv_s S120, 24(a1);
        lv_s S121, 28(a1);
        lv_s S122, 32(a1);
        vpfxt [0],[Z],[-Y],[W];
        vdot_t S000,R101,R102;
	vpfxt [-Z],[0],[X],[W];
	vdot_t S001,R101,R102;
	vpfxt [Y],[-X],[0],[W];
	vdot_t S002,R101,R102;
	vpfxt [0],[-Z],[Y],[W];
	vdot_t S010,R100,R102;
	vpfxt [Z],[0],[-X],[W];
	vdot_t S011,R100,R102;
	vpfxt [-Y],[X],[0],[W];
	vdot_t S012,R100,R102;
	vpfxt [0],[Z],[-Y],[W];
	vdot_t S020,R100,R101;
	vpfxt [-Z],[0],[X],[W];
	vdot_t S021,R100,R101;
	vpfxt [Y],[-X],[0],[W];
	vdot_t S022,R100,R101;
	sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
	sv_s S010,12(a0);
	sv_s S011,16(a0);
	sv_s S012,20(a0);
	sv_s S020,24(a0);
	sv_s S021,28(a0);
	sv_s S022,32(a0);
	: : "{4}"(result), "{5}"(matrix3) : "memory" : "volatile"
    }
    result
}

// sceVfpuMatrix3Inverse

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix3Normalize(
    result: *mut ScePspFMatrix3,
    matrix3: *mut ScePspFMatrix3,
) -> *mut ScePspFMatrix3 {
    vfpu_asm! {
        lv_s S100, 0(a1);
        lv_s S101, 4(a1);
        lv_s S102, 8(a1);
        lv_s S110, 12(a1);
        lv_s S111, 16(a1);
        lv_s S112, 20(a1);
        lv_s S120, 24(a1);
        lv_s S121, 28(a1);
        lv_s S122, 32(a1);
        vcrs_t C020, C100, C110;
        vcrs_t C000, C110, C020;
        vdot_t S100, C000, C000;
        vdot_t S101, C010, C010;
        vdot_t S102, C020, C020;
        vrsq_t C110, C100;
        vscl_t C000, C000, S110;
        vscl_t C010, C010, S111;
        vscl_t C020, C020, S112;
        sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
	sv_s S010,12(a0);
	sv_s S011,16(a0);
	sv_s S012,20(a0);
	sv_s S020,24(a0);
	sv_s S021,28(a0);
	sv_s S022,32(a0);
	: : "{4}"(result), "{5}"(matrix3) : "memory" : "volatile"
    }
    result
}
