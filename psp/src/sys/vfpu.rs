use super::types::{
    ScePspFVector3, ScePspIVector3,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Set(
    vector3: *mut ScePspFVector3,
    x: f32,
    y: f32,
    z: f32,
) -> *mut ScePspFVector3 {
    (*vector3).x = x;
    (*vector3).y = y;
    (*vector3).z = z; 
    vector3
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Copy(
    dst: *mut ScePspFVector3,
    src: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    *dst = *src;
    dst 
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3PositiveZero(
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    (*vector3).x = 0.0;
    (*vector3).y = 0.0;
    (*vector3).z = 0.0;
    vector3 
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3NegativeZero(
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    (*vector3).x  = f32::from_bits(0x8000_0000); 
    (*vector3).y  = f32::from_bits(0x8000_0000);
    (*vector3).z  = f32::from_bits(0x8000_0000);
    vector3
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Ceil(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2id_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Trunc(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2iz_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Round(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2in_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Floor(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2iu_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3FromIVector(
    dst: *mut ScePspFVector3,
    src: *mut ScePspIVector3,
) -> *mut ScePspFVector3 {
    (*dst).z = (*src).z as f32;
    (*dst).y = (*src).y as f32;
    (*dst).x = (*src).x as f32;
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Add(
    result: *mut ScePspFVector3,
    left_addend: *mut ScePspFVector3,
    right_addend: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        lv_s S010, 0(a2);
        lv_s S011, 4(a2);
        lv_s S012, 8(a2);
        vadd_t C000, C000, C010;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(left_addend), "{6}"(right_addend) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Sub(
    result: *mut ScePspFVector3,
    minuend: *mut ScePspFVector3,
    subtrahend: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        lv_s S010, 0(a2);
        lv_s S011, 4(a2);
        lv_s S012, 8(a2);
        vsub_t C000, C000, C010;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(minuend), "{6}"(subtrahend) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Mul(
    result: *mut ScePspFVector3,
    multiplicand: *mut ScePspFVector3,
    multiplier: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        lv_s S010, 0(a2);
        lv_s S011, 4(a2);
        lv_s S012, 8(a2);
        vmul_t C000, C000, C010;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(multiplicand), "{6}"(multiplier) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Div(
    result: *mut ScePspFVector3,
    dividend: *mut ScePspFVector3,
    divisor: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S010, 0(a1);
        lv_s S011, 4(a1);
        lv_s S012, 8(a1);
        lv_s S020, 0(a2);
        lv_s S021, 4(a2);
        lv_s S022, 8(a2);
        vdiv_t C000, C010, C020;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(dividend), "{6}"(divisor) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Neg(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    (*result).z = 0.0 - (*vector3).z;
    (*result).y = 0.0 - (*vector3).y;
    (*result).x = 0.0 - (*vector3).x;
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Abs(
    result: *mut ScePspFVector3,
    input: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    llvm_asm! (
        "abs.s $$f3, $$f1;
        abs.s $$f2, $$f4;
        abs.s $$f5, $$f6;"
        : "={f3}"((*result).x), "={f2}"((*result).y), "={f5}"((*result).z) : "{f1}"((*input).x), "{f4}"((*input).y), "{f6}"((*input).z) : "memory" : "volatile"
    );
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Lerp(
    result: *mut ScePspFVector3,
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
    arg3: f32,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S030;
        lv_s S010, 0(a1);
        lv_s S011, 4(a1);
        lv_s S012, 8(a1);
        lv_s S020, 0(a2);
        lv_s S021, 4(a2);
        lv_s S022, 8(a2);
        vsub_t C000, C020, C010;
        vscl_t C000, C000, S030;
        vadd_t C000, C010, C000;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2), "f"(arg3) : "8","memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Hermite(
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
    arg3: *mut ScePspFVector3,
    arg4: *mut ScePspFVector3,
    arg5: *mut ScePspFVector3,
    arg6: f32,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        .mips "mfc1 $$t1, $0";
        lv_s S000,0(a1);
	lv_s S001,4(a1);
	lv_s S002,8(a1);
	lv_s S010,0(a2);
	lv_s S011,4(a2);
	lv_s S012,8(a2);
	lv_s S020,0(t0);
	lv_s S021,4(t0);
	lv_s S022,8(t0);
	lv_s S030,0(a3);
	lv_s S031,4(a3);
	lv_s S032,8(a3);
	lv_s S202,0(t1);
	vone_s S203;
	vmul_s S201,S202,S202;
	vpfxs [2],[1],[1],[-2];
	vmov_q C100,C100;
	vpfxs [-3],[-2],[-1],[3];
	vmov_q C110,C110;
	vmul_s S200,S201,S202;
	vpfxs [0],[1],[0],[0];
	vmov_q C120,C120;
	vpfxs [1],[0],[0],[0];
	vmov_q C130,C130;
	vtfm4_q C210,E100,C200;
	vtfm4_q C220,E000,C210;
	sv_s S220,0(a0);
	sv_s S221,4(a0);
	sv_s S222,8(a0);
        : : "{4}"(arg1), "{5}"(arg2), "{6}"(arg3), "{7}"(arg4), "{8}"(arg5), "f"(arg6) : "9", "memory" : "volatile"
    }
    arg1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Scale(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
    scale: f32,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        .mips "mfc1 $$t0, $0";
        mtv t0, S010;
        lv_s S000,0(a1);
	lv_s S001,4(a1);
	lv_s S002,8(a1);
	vscl_t C000,C000,S010;
	sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
	: : "{4}"(result),"{5}"(vector3),"f"(scale) : "8", "memory": "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Clamp(
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
    arg3: f32,
    arg4: f32,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        .mips "mfc1 $$t0,$0";
        .mips "mfc1 $$t1,$1";
        mtv t0,S010;
        mtv t1,S011;
        lv_s S000, 0(a1);
        lv_s S001,4(a1);
        lv_s S002,8(a1);
        vpfxt [X], [X], [X], [W];
        vmax_t C000,C000,C010;
        vpfxt [Y], [Y], [Y], [W];
        vmin_t C000,C000,C010;
        sv_s S000, 0(a0);
        sv_s S001,4(a0);
        sv_s S002,8(a0);
        : : "{4}"(arg1), "{5}"(arg2), "f"(arg3), "f"(arg4) : "8","9","memory" : "volatile"
    }
    arg1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Max(
    result: *mut ScePspFVector3,
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S001,4(a1);
	lv_s S002,8(a1);
	lv_s S010,0(a2);
	lv_s S011,4(a2);
	lv_s S012,8(a2);
	vmax_t C000,C000,C010;
	sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Min(
    result: *mut ScePspFVector3,
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S001,4(a1);
	lv_s S002,8(a1);
	lv_s S010,0(a2);
	lv_s S011,4(a2);
	lv_s S012,8(a2);
	vmin_t C000,C000,C010;
	sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3InnerProduct(
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_s S000, 0(a0);
        lv_s S001, 4(a0);
        lv_s S002, 8(a0);
        lv_s S010, 0(a1);
        lv_s S011, 4(a1);
        lv_s S012, 8(a1);
        vdot_t S000, C000, C010;
        mfv a0, S000;
        .mips "mtc1 $$a0, $0";
        : "=f"(out) : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3OuterProduct(
    result: *mut ScePspFVector3,
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S010,4(a1);
	lv_s S011,8(a1);
	lv_s S012,0(a2);
	lv_s S020,0(a2);
	lv_s S021,4(a2);
	lv_s S022,8(a2);
	vcrs_t C000,C010,C020;
	sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
        : : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Funnel(
    arg1: *mut ScePspFVector3,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_s S000,0(a0);
	lv_s S001,4(a0);
	lv_s S002,8(a0);
	vfad_t S000,C000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(arg1) : "8","memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Average(
    arg1: *mut ScePspFVector3,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_s S000,0(a0);
	lv_s S001,4(a0);
	lv_s S002,8(a0);
	vavg_t S000,C000;
        mfv a0, S000;
        .mips "mtc1 $$a0, $0";
        : "=f"(out) : "{4}"(arg1) : "memory" : "volatile"
    }
    out
}
    
//#[allow(non_snake_case)]
//#[no_mangle]
//pub unsafe extern "C" fn sceVfpuVector3IsEqual(
    //arg1: *mut ScePspFVector2,
    //arg2: *mut ScePspFVector2,
//) -> i32 {
    //vfpu_asm! {
        //lv_s S000, 0(a0);
        //lv_s S001, 4(a0);
        //lv_s S002, 8(a0);
        //lv_s S010, 0(a1);
        //lv_s S011, 4(a1);
        //lv_s S012, 8(a1);
        //.mips "li v0, 0";
        //vcmp_t EQ, C000, C010
        //// bvtl ret (CC[5])
        //.mips "li v0, 1";
    //}
    //return 0// ret
//}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3IsZero(
    vector3: *mut ScePspFVector3
) -> bool {
    ((*vector3).x.to_bits() | (*vector3).y.to_bits() | (*vector3).z.to_bits()) & 0x7fff_ffff == 0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3SignFloat(
    result: *mut ScePspFVector3,
    input: *mut ScePspFVector3
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000,0(a1);
	lv_s S001,4(a1);
	lv_s S002,8(a1);
	vsgn_t C000,C000;
	sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
        : : "{4}"(result), "{5}"(input) : "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3SignInt(
    result: *mut ScePspIVector3,
    input: *mut ScePspFVector3
) -> *mut ScePspIVector3 {
    vfpu_asm! {
        lv_s S000,0(a1);
	lv_s S001,4(a1);
	lv_s S002,8(a1);
	vsgn_t C000,C000;
	vf2iz_t C000,C000,0;
	sv_s S000,0(a0);
	sv_s S001,4(a0);
	sv_s S002,8(a0);
        : : "{4}"(result), "{5}"(input) : "memory" : "volatile"
    }
    result
}

//#[allow(non_snake_case)]
//#[no_mangle]
//pub unsafe extern "C" fn sceVfpuVector3Normalize(
    //result: *mut ScePspFVector3,
    //input: *mut ScePspFVector3
//) -> *mut ScePspFVector3 {
    //vfpu_asm! {
	//lv_s S000,0(a1);
	//lv_s S001,4(a1);
	//lv_s S002,8(a1);
	//vdot_t S010,C000,C000;
	//vzero_s S011;
	//vcmp_s EZ,S010,S010;
	//vrsq_s S010,S010;
	//vcmovt_s S010,S011,CC[0];
	//vpfxd [-1:1,-1:1,-1:1,M];
	//vscl_t C000,C000,S010;
	//sv_s S000,0(a0);
	//sv_s S001,4(a0);
	//sv_s S002,8(a0);
        //: : "{4}"(result), "{5}"(input) : "memory" : "volatile"
    //}
    //result
//}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Length(
    arg: *mut ScePspFVector3,
) -> f32 {
    let out: f32;
    vfpu_asm! {
        lv_s S000, 0(a0);
        lv_s S001, 4(a0);
        lv_s S002, 8(a0);
        vdot_t S000,C000,C000;
        vsqrt_s S000,S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(arg) : "8","memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Distance(
    arg1: *mut ScePspFVector3,
    arg2: *mut ScePspFVector3,
) -> f32 {
    let out: f32;
    vfpu_asm! {
	lv_s S000,0(a0);
	lv_s S001,4(a0);
	lv_s S002,8(a0);
	lv_s S010,0(a1);
	lv_s S011,4(a1);
	lv_s S012,8(a1);
	vsub_t C000,C000,C010;
	vdot_t S000,C000,C000;
	vsqrt_s S000,S000;
        mfv t0, S000;
        .mips "mtc1 $$t0, $0";
        : "=f"(out) : "{4}"(arg1),"{5}"(arg2) : "8","memory" : "volatile"
    }
    out
}

//sceVfpuVector3FaceForward

//#[allow(non_snake_case)]
//#[no_mangle]
//pub unsafe extern "C" fn sceVfpuVector3Reflect(
    //result: *mut ScePspFVector3,
    //arg1: *mut ScePspFVector3,
    //arg2: *mut ScePspFVector3,
//) -> *mut ScePspFVector3 {
    //vfpu_asm! {
        //lv_s S010,0(a1);
	//lv_s S011,4(a1);
	//lv_s S012,8(a1);
	//lv_s S020,0(a2);
	//lv_s S021,4(a2);
	//lv_s S022,8(a2);
	//vdot_t S031,C010,C020;
	//vfim_s S030,[-2.000000];
	//vmul_s S032,S030,S031;
	//vscl_t C020,C020,S032;
	//vadd_t C000,C010,C020;
	//vdot_t S033,C000,C000;
	//vcmp_s EZ,S033,S033;
	//vrsq_s S033,S033;
	//vpfxs [0],[Y],[Z],[W];
	//vcmovt_s S033,S033,CC0;
	//vpfxd [-1:1],[-1:1],[-1:1],[M];
	//vscl_t C000,C000,S033;
	//sv_s S000,0(a0);
	//sv_s S001,4(a0);
	//sv_s S002,8(a0);
        //: : "{4}"(result), "{5}"(arg1), "{6}"(arg2) : "memory" : "volatile"
    //}
    //result
//}

// sceVfpuVector3Refract
