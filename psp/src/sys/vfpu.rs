use super::types::{
    ScePspFVector4,
    ScePspFMatrix4,
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Unit(
    result: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        vmidt_q E000;
        sv_q C000, 0(a0);
        sv_q C010, 16(a0);
        sv_q C020, 32(a0);
        sv_q C030, 48(a0);
	: : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Zero(
    result: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        vmzero_q E000;
        sv_q C000, 0(a0);
        sv_q C010, 16(a0);
        sv_q C020, 32(a0);
        sv_q C030, 48(a0);
	: : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Copy(
    dst: *mut ScePspFMatrix4,
    src: *mut ScePspFMatrix4
) -> *mut ScePspFMatrix4 {
    *dst = *src;
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4SetTransfer(
    dst: *mut ScePspFMatrix4,
    src: *mut ScePspFMatrix4
) -> *mut ScePspFMatrix4 {
    (*dst).w = (*src).x;
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4GetTransfer(
    dst: *mut ScePspFMatrix4,
    src: *mut ScePspFMatrix4
) -> *mut ScePspFMatrix4 {
    (*dst).x = (*src).w;
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Transform(
    result: *mut ScePspFVector4,
    matrix4: *mut ScePspFMatrix4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C100,0(a1);
        lv_q C110,16(a1);
        lv_q C120,32(a1);
        lv_q C130,48(a1);
        lv_q C200,0(a2);
        vtfm4_q C000, E100, C200;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(matrix4), "{6}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4TransformXYZ(
    result: *mut ScePspFVector4,
    matrix4: *mut ScePspFMatrix4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C100,0(a1);
        lv_q C110,16(a1);
        lv_q C120,32(a1);
        lv_q C200,0(a2);
        vmov_s S003,S203;
        vtfm3_t C000, E100, C200;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(matrix4), "{6}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4HomogenousTransform(
    result: *mut ScePspFVector4,
    matrix4: *mut ScePspFMatrix4,
    vector4: *mut ScePspFVector4,
) -> *mut ScePspFVector4 {
    vfpu_asm! {
        lv_q C100,0(a1);
        lv_q C110,16(a1);
        lv_q C120,32(a1);
        lv_q C130,48(a1);
        lv_q C200,0(a2);
        vhtfm4_q C000, E100, C200;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(matrix4), "{6}"(vector4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Mul(
    result: *mut ScePspFMatrix4,
    multiplicand: *mut ScePspFMatrix4,
    multiplier: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        lv_q C100,0(a1);
	lv_q C110,16(a1);
	lv_q C120,32(a1);
	lv_q C130,48(a1);
	lv_q C200,0(a2);
	lv_q C210,16(a2);
	lv_q C220,32(a2);
	lv_q C230,48(a2);
        vmmul_q E000, E200, E100;
	sv_q C000,0(a0);
	sv_q C010,16(a0);
	sv_q C020,32(a0);
	sv_q C030,48(a0);
        : : "{4}"(result), "{5}"(multiplicand), "{6}"(multiplier) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Scale(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
    scale: f32, 
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S200;
	lv_q C100,0(a1);
	lv_q C110,16(a1);
	lv_q C120,32(a1);
	lv_q C130,48(a1);
        vmscl_q E000, E100, S200;
	sv_q C000,0(a0);
	sv_q C010,16(a0);
	sv_q C020,32(a0);
	sv_q C030,48(a0);
        : : "{4}"(result), "{5}"(matrix4), "f"(scale) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Transpose(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        lv_q C000, 0(a1);
        lv_q C010, 16(a1);
        lv_q C020, 32(a1);
        lv_q C030, 48(a1);
        sv_q R000, 0(a0);
        sv_q R001, 16(a0);
        sv_q R002, 32(a0);
        sv_q R003, 48(a0);
        : : "{4}"(result), "{5}"(matrix4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Inverse(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        lv_q C100, 0(a1);
        lv_q C110, 16(a1);
        lv_q C120, 32(a1);
        lv_q C000, 48(a1);
        vzero_t C130;
        vtfm3_t C010,M100,C000;
        sv_q R100, 0(a0);
        sv_q R101, 16(a0);
        vneg_t C000,C010;
        sv_q R102, 32(a0);
        sv_q C000, 48(a0);
        : : "{4}"(result), "{5}"(matrix4) : "memory" : "volatile"
    }
    result
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Transfer(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        lv_q C000,0(a1);
	lv_q C010,16(a1);
	lv_q C020,32(a1);
	lv_q C030,48(a1);
	lv_q C100,0(a2);
	vadd_t C100,C100,C030;
	sv_q C000,0(a0);
	sv_q C010,16(a0);
	sv_q C020,32(a0);
	sv_q C100,48(a0);
        : : "{4}"(result), "{5}"(matrix4) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4RotZ(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
    rotz_radians: f32,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        lv_s S100, t0;
        vcst_s S101,VFPU_2_PI;
        vmul_s S100,S100,S101;
        vrot_q C000,S100,[C,S,0,0];
        vrot_q C010,S100,[-S,C,0,0];
        vidt_q C020;
        vidt_q C030;
        : : "f"(rotz_radians) : "8", "memory" : "volatile"
    } 
    if !matrix4.is_null() {
        vfpu_asm! {
            lv_q C100,0(a1);
            lv_q C110,16(a1);
            lv_q C120,32(a1);
            lv_q C130,48(a1);
            vmmul_q E200,E100,E000;
            : : "{5}"(matrix4) : "memory" : "volatile"
        }
    } else {
        vfpu_asm! { vmmov_p E200,E000; : : : : "volatile" }
    }
    vfpu_asm! {
        sv_q C200,0(a0);
        sv_q C210,16(a0);
        sv_q C220,32(a0);
        sv_q C230,48(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4RotY(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
    roty_radians: f32,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        lv_s S100, t0;
        vcst_s S101,VFPU_2_PI;
        vmul_s S100,S100,S101;
        vrot_q C000,S100,[C,0,-S,0];
        vidt_q C010;
        vrot_q C020,S100,[S,0,C,0];
        vidt_q C030;
        : : "f"(roty_radians) : "8", "memory" : "volatile"
    } 
    if !matrix4.is_null() {
        vfpu_asm! {
            lv_q C100,0(a1);
            lv_q C110,16(a1);
            lv_q C120,32(a1);
            lv_q C130,48(a1);
            vmmul_q E200,E100,E000;
            : : "{5}"(matrix4) : "memory" : "volatile"
        }
    } else {
        vfpu_asm! { vmmov_p E200,E000; : : : : "volatile" }
    }
    vfpu_asm! {
        sv_q C200,0(a0);
        sv_q C210,16(a0);
        sv_q C220,32(a0);
        sv_q C230,48(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4RotX(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
    rotx_radians: f32,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        lv_s S100, t0;
        vcst_s S101,VFPU_2_PI;
        vmul_s S100,S100,S101;
        vidt_q C000;
        vrot_q C010,S100,[0,C,S,0];
        vrot_q C020,S100,[0,-S,C,0];
        vidt_q C030;
        : : "f"(rotx_radians) : "8", "memory" : "volatile"
    } 
    if !matrix4.is_null() {
        vfpu_asm! {
            lv_q C100,0(a1);
            lv_q C110,16(a1);
            lv_q C120,32(a1);
            lv_q C130,48(a1);
            vmmul_q E200,E100,E000;
            : : "{5}"(matrix4) : "memory" : "volatile"
        }
    } else {
        vfpu_asm! { vmmov_p E200,E000; : : : : "volatile" }
    }
    vfpu_asm! {
        sv_q C200,0(a0);
        sv_q C210,16(a0);
        sv_q C220,32(a0);
        sv_q C230,48(a0);
        : : "{4}"(result) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Rot(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
    rot_radians: *mut ScePspFVector3,
) -> *mut ScePspFMatrix4 {
    sceVfpuMatrix4RotZ(result, matrix4, (*rot_radians).z);
    sceVfpuMatrix4RotY(result, matrix4, (*rot_radians).y);
    sceVfpuMatrix4RotX(result, matrix4, (*rot_radians).x);
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4IsUnit(
    matrix4: *mut ScePspFMatrix4,
) -> bool {
    (*matrix4).x.x == 1.0 && (*matrix4).y.y == 1.0 
        && (*matrix4).z.z == 1.0 && (*matrix4).w.w == 1.0 
        && (*matrix4).x.y == 0.0 &&  (*matrix4).x.z == 0.0 && (*matrix4).x.w == 0.0
        && (*matrix4).y.x == 0.0 &&  (*matrix4).y.z == 0.0 && (*matrix4).y.w == 0.0
        && (*matrix4).z.x == 0.0 &&  (*matrix4).z.y == 0.0 && (*matrix4).z.w == 0.0
        && (*matrix4).w.x == 0.0 &&  (*matrix4).w.y == 0.0 && (*matrix4).w.z == 0.0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Trace(
    matrix4: *mut ScePspFMatrix4,
) -> f32 {
    (*matrix4).x.x + (*matrix4).y.y + (*matrix4).z.z + (*matrix4).w.w
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Determinant(
    matrix4: *mut ScePspFMatrix4,
) -> f32 {
    let out: f32;
    vfpu_asm! {
	lv_q C000,0(a0);
	lv_q C010,16(a0);
	lv_q C020,32(a0);
	lv_q C030,48(a0);
	vpfxs [X],[Z],[Y],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C100,C011,C021;
	vpfxs [X],[Z],[Y],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C110,C001,C021;
	vpfxs [X],[Z],[Y],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C120,C001,C011;
	vpfxs [X],[Z],[Y],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C130,C001,C011;
	vpfxs [Z],[Y],[X],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C200,C011,C021;
	vpfxs [Z],[Y],[X],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C210,C001,C021;
	vpfxs [Z],[Y],[X],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C220,C001,C011;
	vpfxs [Z],[Y],[X],[W];
	vpfxt [Y],[X],[Z],[W];
	vmul_t C230,C001,C011;
	vpfxt [Z],[Y],[X],[W];
	vdot_t S100,C100,C031;
	vpfxt [Z],[Y],[X],[W];
	vdot_t S110,C110,C031;
	vpfxt [Z],[Y],[X],[W];
	vdot_t S120,C120,C031;
	vpfxt [Z],[Y],[X],[W];
	vdot_t S130,C130,C021;
	vpfxt [X],[Z],[Y],[W];
	vdot_t S200,C200,C031;
	vpfxt [X],[Z],[Y],[W];
	vdot_t S210,C210,C031;
	vpfxt [X],[Z],[Y],[W];
	vdot_t S220,C220,C031;
	vpfxt [X],[Z],[Y],[W];
	vdot_t S230,C230,C021;
	vsub_q R100,R100,R200;
	vmul_q R101,R000,R100;
	vpfxs [X],[-Y],[Z],[-W];
	vfad_q S000,R101;
	sv_s S000,0(t0);
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(matrix4) : "8","memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4Adjoint(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
	lv_q C000,0(a1);
	lv_q C010,16(a1);
	lv_q C020,32(a1);
	lv_q C030,48(a1);
	vpfxs [0],[Z],[Y],[Y];
	vpfxt [0],[W],[W],[Z];
	vmul_q C100,C020,C030;
	vpfxs [X],[0],[W],[Z];
	vpfxt [Y],[0],[X],[X];
	vmul_q C110,C020,C030;
	vpfxs [0],[Z],[Y],[Y];
	vpfxt [0],[W],[W],[Z];
	vmul_q C120,C030,C020;
	vpfxs [X],[0],[W],[Z];
	vpfxt [Y],[0],[X],[X];
	vmul_q C130,C030,C020;
	vsub_q C100,C100,C120;
	vsub_q C110,C110,C130;
	vmov_p R122,R100;
	vmov_p R123,C110;
	vpfxs [X],[-Y],[Z],[W];
	vmov_t R110,C101;
	vpfxs [X],[-Y],[Z],[W];
	vmov_p R121,C112;
	vtfm4_q R200,E100,C010;
	vtfm4_q R201,E100,C000;
	vpfxs [0],[Z],[Y],[Y];
	vpfxt [0],[W],[W],[Z];
	vmul_q C100,C000,C010;
	vpfxs [X],[0],[W],[Z];
	vpfxt [Y],[0],[X],[X];
	vmul_q C110,C000,C010;
	vpfxs [0],[Z],[Y],[Y];
	vpfxt [0],[W],[W],[Z];
	vmul_q C120,C010,C000;
	vpfxs [X],[0],[W],[Z];
	vpfxt [Y],[0],[X],[X];
	vmul_q C130,C010,C000;
	vsub_q C100,C100,C120;
	vsub_q C110,C110,C130;
	vmov_p R122,R100;
	vmov_p R123,C110;
	vpfxs [X],[-Y],[Z],[W];
	vmov_t R110,C101;
	vpfxs [X],[-Y],[Z],[W];
	vmov_p R121,C112;
	vtfm4_q R202,E100,C030;
	vtfm4_q R203,E100,C020;
	vpfxd [M],[],[M],[];
	vneg_q C200,C200;
	vpfxd [],[M],[],[M];
	vneg_q C210,C210;
	vpfxd [M],[],[M],[];
	vneg_q C220,C220;
	vpfxd [],[M],[],[M];
	vneg_q C230,C230;
	sv_q C200,0(a0);
	sv_q C210,16(a0);
	sv_q C220,32(a0);
	sv_q C230,48(a0);
	: : "{4}"(result), "{5}"(matrix4) : "memory" : "volatile"
    }
    result
}

// sceVfpuMatrix4Inverse2

//#[allow(non_snake_case)]
//#[no_mangle]
//pub unsafe extern "C" fn sceVfpuMatrix4DropShadow(
    //result: *mut ScePspFMatrix4,
    //arg1: *mut ScePspFVector4,
    //arg2: *mut ScePspFVector4,
//) -> *mut ScePspFMatrix4 {
    //vfpu_asm! {
        //lv_q C100,0(a1);
	//lv_q C200,0(a2);
	//vdot_t S210,C200,C200;
	//vzero_s S211;
	//vcmp_s EZ,S210,S210;
	//vrsq_s S210,S210;
	//vcmovt_s S210,S211,CC0;
	//vpfxd [-1:1],[-1:1],[-1:1],[-1:1];
	//vscl_q C200,C200,S210;
	//vdot_q S110,C100,C200;
	//vneg_q C210,C200;
	//vscl_q C000,C100,S210;
	//vscl_q C010,C100,S211;
	//vscl_q C020,C100,S212;
	//vscl_q C030,C100,S213;
	//vadd_s S000,S000,S110;
	//vadd_s S011,S011,S110;
	//vadd_s S022,S022,S110;
	//vadd_s S033,S033,S110;
	//sv_q C000,0(a0);
	//sv_q C010,16(a0);
	//sv_q C020,32(a0);
	//sv_q C030,48(a0);
	//: : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    //}
    //result
//}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuMatrix4NormalizeXYZ(
    result: *mut ScePspFMatrix4,
    matrix4: *mut ScePspFMatrix4,
) -> *mut ScePspFMatrix4 {
    vfpu_asm! {
        lv_q C100,0(a1);
	lv_q C010,16(a1);
	lv_q C120,32(a1);
	lv_q C130,48(a1);
	vzero_t R003;
	vcrs_t C020,C100,C010;
	vcrs_t C000,C010,C020;
	vdot_t S100,C000,C000;
	vdot_t S101,C010,C010;
	vdot_t S102,C020,C020;
	vrsq_t C110,C100;
	vscl_t C000,C000,S110;
	vscl_t C010,C010,S111;
	vscl_t C020,C020,S112;
	sv_q C000,0(a0);
	sv_q C010,16(a0);
	sv_q C020,32(a0);
	sv_q C130,48(a0);
	: : "{4}"(result), "{5}"(matrix4) : "memory" : "volatile"
    }
    result
}


