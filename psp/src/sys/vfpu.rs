#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, Default)]
pub struct VfpuColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorToRGB565(
    color: *mut VfpuColor
) -> u32 {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    if (*color).r > 0.0 {
        red = ((*color).r * 31.0) as u32 + 0x1f;
    }

    if (*color).g > 0.0 {
        green = ((*color).g * 63.0) as u32 + 0x3f;
    }

    if (*color).b > 0.0 {
        blue = ((*color).b * 31.0) as u32 + 0x1f;
    }
        
    (blue << 0xb | green << 5 | red) & 0xffff
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorSet(
    color: *mut VfpuColor,
    r: f32, 
    g: f32,
    b: f32,
    a: f32,
) -> *mut VfpuColor {
    (*color).r = r;
    (*color).g = g;
    (*color).b = b;
    (*color).a = a;
    color
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorSetRGB(
    color: *mut VfpuColor,
    r: f32,
    g: f32,
    b: f32,
) -> *mut VfpuColor {
    (*color).r = r;
    (*color).g = g;
    (*color).b = b;
    color
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorCopy(
    dst: *mut VfpuColor,
    src: *mut VfpuColor,
) -> *mut VfpuColor {
    *dst = *src;
    dst
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorZero(color: *mut VfpuColor) -> *mut VfpuColor {
    (*color).r = 0.0;
    (*color).g = 0.0;
    (*color).b = 0.0;
    (*color).a = 0.0;
    color
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorToRGBA8888(color: *mut VfpuColor) -> u32 {
    let out: u32;
    vfpu_asm! {
        lv_q C000, 0(a0);
        vsat0_q C000,C000;
	viim_s S010,255;
	vscl_q C000,C000,S010;
	vf2iz_q C000,C000,23;
	vi2uc_q S000,C000;
	mfv v0,S000;
	: "={2}"(out) : "{4}"(color) : "memory" : "volatile"
    }
    out
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorToRGBA4444(color: *mut VfpuColor) -> u32 {
    let red: u32;
    let green: u32;
    let blue: u32;
    let alpha: u32;

    vfpu_asm! {
        lv_q C000, 0(a0);
        vsat0_q C000,C000;
	viim_s S010, 15;
	vscl_q C000,C000,S010;
	vf2iz_q C000,C000,0;
        mfv v1,S000;
	mfv t0,S001;
	mfv t1,S002;
	mfv t2,S003;
	: "={8}"(blue), "={9}"(green), "={10}"(red), "={3}"(alpha): "{4}"(color) : "memory" : "volatile"
    }
    (alpha | ((blue & 0b1111) << 4) | ((green & 0b1111) << 8) | ((red & 0b1111) << 12)) & 0xffff
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorToRGBA5551(color: *mut VfpuColor, alpha_threshold: f32) -> u32 {
    let red: u32;
    let green: u32;
    let blue: u32;
    let alpha: u32;
    let alpha_f = (*color).a;

    vfpu_asm! {
        lv_q C000, 0(a0);
        vsat0_q C000,C000;
	viim_s S010, 31;
	vscl_q C000,C000,S010;
	vf2iz_q C000,C000,0;
        mfv v1,S000;
	mfv t0,S001;
	mfv t1,S002;
	: "={8}"(blue), "={9}"(green), "={3}"(red) : "{4}"(color) : "memory" : "volatile"
    }
    if alpha_f < alpha_threshold {
        alpha = 0;
    } else {
        alpha = 0x8000;
    }
    (alpha | ((blue & 0b11111) << 5) | ((green & 0b11111) << 10) | red) & 0xffff
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorFromRGBA8888(result: *mut VfpuColor, input: u32)
-> *mut VfpuColor {
    vfpu_asm! {
        mtv a1, S010;
        vuc2i_s S000, S010;
        vi2f_q C000, C000, 31;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(input): "memory": "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorFromRGBA4444(result: *mut VfpuColor, input: u32)
-> *mut VfpuColor {
    let input = input & 0xffff;
    let red = input & 0b1111;
    let green = (input >> 4) & 0b1111;
    let blue = (input >> 8) & 0b1111;
    let alpha = (input >> 12) & 0b1111;
    let two_thirds_f: u32 = 0x3d88_8889;
    vfpu_asm! {
        mtv at, S010;
        mtv t0, S000;
        mtv t1, S001;
        mtv t2, S002;
        mtv t3, S003;
        vi2f_q C000,C000,0;
        vscl_q C000,C000,S010;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{1}"(two_thirds_f), "{8}"(red),"{9}"(green),"{10}"(blue),"{11}"(alpha): "memory": "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorFromRGBA5551(result: *mut VfpuColor, input: u32, alpha: f32)
-> *mut VfpuColor {
    let input = input & 0xffff;
    let red = input & 0b11111;
    let green = (input >> 5) & 0b11111;
    let blue = (input >> 10) & 0b11111;
    let constant: u32 = 0x3d04_2108;
    vfpu_asm! {
        mtv at, S010;
        mtv t0, S000;
        mtv t1, S001;
        mtv t2, S002;
        mtv t3, S003;
        vi2f_q C000,C000,0;
        vscl_q C000,C000,S010;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{1}"(constant), "{8}"(red),"{9}"(green),"{10}"(blue): "memory": "volatile"
    }
    if input & 0x8000 == 0 {
        (*result).a = 0.0;
    } else {
        (*result).a = alpha;
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorFromRGB565(result: *mut VfpuColor, input: u32, alpha: f32) -> *mut VfpuColor {
    let two_thirds = f32::from_bits(0x3d88_8889);
    let unk_fconst = f32::from_bits(0x3d04_2108);
    (*result).a = alpha;
    (*result).b = ((input & 0xffff) >> 0xb) as f32 * two_thirds;
    (*result).g = (((input & 0xffff) << 0x19) >> 0x1a) as f32 * unk_fconst;
    (*result).r = (input & 0x1f) as f32 * (2.0/3.0) * two_thirds;
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorAdd(
    result: *mut VfpuColor, 
    left_addend: *mut VfpuColor,
    right_addend: *mut VfpuColor, 
) -> *mut VfpuColor {
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
pub unsafe extern "C" fn sceVfpuColorAddRGB(
    result: *mut VfpuColor, 
    left_addend: *mut VfpuColor,
    right_addend: *mut VfpuColor, 
) -> *mut VfpuColor {
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
pub unsafe extern "C" fn sceVfpuColorSub(
    result: *mut VfpuColor, 
    minuend: *mut VfpuColor,
    subtrahend: *mut VfpuColor, 
) -> *mut VfpuColor {
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
pub unsafe extern "C" fn sceVfpuColorSubRGB(
    result: *mut VfpuColor, 
    minuend: *mut VfpuColor,
    subtrahend: *mut VfpuColor, 
) -> *mut VfpuColor {
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
pub unsafe extern "C" fn sceVfpuColorMul(
    result: *mut VfpuColor, 
    multiplicand: *mut VfpuColor,
    multiplier: *mut VfpuColor, 
) -> *mut VfpuColor {
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
pub unsafe extern "C" fn sceVfpuColorMulRGB(
    result: *mut VfpuColor, 
    multiplicand: *mut VfpuColor,
    multiplier: *mut VfpuColor, 
) -> *mut VfpuColor {
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
pub unsafe extern "C" fn sceVfpuColorNeg(
    result: *mut VfpuColor,
    color: *mut VfpuColor
) -> *mut VfpuColor {
    vfpu_asm! {
        lv_q C000, 0(a1);
        vocp_q C000, C000;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color): "memory" : "volatile"

    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorNegRGB(
    result: *mut VfpuColor,
    color: *mut VfpuColor
) -> *mut VfpuColor {
    vfpu_asm! {
        lv_q C000, 0(a1);
        vocp_t C000, C000;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color): "memory" : "volatile"

    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorLerp(
    result: *mut VfpuColor,
    color1: *mut VfpuColor,
    color2: *mut VfpuColor,
    factor: f32,
) -> *mut VfpuColor {
    vfpu_asm! {
        .mips "mfc1 $$t0, $$f12";
        mtv t0, S030;
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vsub_q C000, C020, C010;
        vscl_q C000, C000, S030;
        vadd_q C010, C010, C000;
        sv_q C010, 0(a0);
        : : "{4}"(result), "{5}"(color1), "{6}"(color2), "{f12}"(factor): "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorLerpRGB(
    result: *mut VfpuColor,
    color1: *mut VfpuColor,
    color2: *mut VfpuColor,
    factor: f32,
) -> *mut VfpuColor {
    vfpu_asm! {
        .mips "mfc1 $$t0, $$f12";
        mtv t0, S030;
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vsub_t C000, C020, C010;
        vscl_t C000, C000, S030;
        vadd_t C010, C010, C000;
        sv_q C010, 0(a0);
        : : "{4}"(result), "{5}"(color1), "{6}"(color2), "{f12}"(factor): "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorScale(
    result: *mut VfpuColor,
    color: *mut VfpuColor,
    scale: f32,
) -> *mut VfpuColor {
    vfpu_asm! {
        .mips "mfc1 $$t0, $$f12";
        mtv t0, S020;
        lv_q C010, 0(a1);
        vscl_q C000, C010, S020;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color), "{f12}"(scale): "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorScaleRGB(
    result: *mut VfpuColor,
    color: *mut VfpuColor,
    scale: f32,
) -> *mut VfpuColor {
    vfpu_asm! {
        .mips "mfc1 $$t0, $$f12";
        mtv t0, S010;
        lv_q C000, 0(a1);
        vscl_t C000, C000, S010;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color), "{f12}"(scale): "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorClamp(
    result: *mut VfpuColor,
    color: *mut VfpuColor,
    min: f32,
    max: f32,
) -> *mut VfpuColor {
    vfpu_asm! {
        .mips "mfc1 $$t0, $$f12";
        .mips "mfc1 $$t1, $$f13";
        mtv t0, S010;
        mtv t1, S011;
        lv_q C000, 0(a1);
        vpfxt [X], [X], [X], [X];
        vmax_q C000, C000, C010;
        vpfxt [Y], [Y], [Y], [Y];
        vmin_q C000, C000, C010;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color), "{f12}"(min), "{f13}"(max): "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorClampRGB(
    result: *mut VfpuColor,
    color: *mut VfpuColor,
    min: f32,
    max: f32,
) -> *mut VfpuColor {
    vfpu_asm! {
        .mips "mfc1 $$t0, $$f12";
        .mips "mfc1 $$t1, $$f13";
        mtv t0, S010;
        mtv t1, S011;
        lv_q C000, 0(a1);
        vpfxt [X], [X], [X], [X];
        vmax_t C000, C000, C010;
        vpfxt [Y], [Y], [Y], [Y];
        vmin_t C000, C000, C010;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color), "{f12}"(min), "{f13}"(max): "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorMax(
    result: *mut VfpuColor,
    color1: *mut VfpuColor,
    color2: *mut VfpuColor,
) -> *mut VfpuColor {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vmax_q C000,C010,C020;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color1), "{6}"(color2): "memory" : "volatile"
    }
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorMin(
    result: *mut VfpuColor,
    color1: *mut VfpuColor,
    color2: *mut VfpuColor,
) -> *mut VfpuColor {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vmin_q C000,C010,C020;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(color1), "{6}"(color2): "memory" : "volatile"
    }
    result
}

// sceVfpuColorIsEqual

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorIsZero(
    color: *mut VfpuColor,
) -> bool {
    (*color).r == 0.0 && (*color).g == 0.0 && (*color).b == 0.0 && (*color).a == 0.0
}

// sceVfpuColorNormalize
// sceVfpuColorNormalizeRGB
