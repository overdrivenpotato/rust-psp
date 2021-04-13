#[no_mangle]
pub unsafe extern "C" fn sceVfpuMemcpy(
    dst: *mut u8,
    src: *const u8,
    size: usize,
) -> *mut u8 {
    if size == 0 {
        return dst
    }

    let mut size = size;
    let mut dst8 = dst;
    let mut src8 = src;

    if ((src8 as u32)&0xF) == 0 //Both src and dst are 16byte aligned
    {
        while size > 63 {
            vfpu_asm! {
                lv_q C000, 0(a1);
                lv_q C010, 16(a1);
                lv_q C020, 32(a1);
                lv_q C030, 48(a1);
                sv_q C000, 0(a0);
                sv_q C010, 16(a0);
                sv_q C020, 32(a0);
                sv_q C030, 48(a0);
                : : "{4}"(dst8), "{5}"(src8), "{6}"(size) : "memory" : "volatile"
            };
            dst8 = dst8.add(64);
            src8 = src8.add(64);
            size = size.saturating_sub(64);
        }

        while size > 15 {
            vfpu_asm! {
                lv_q C000, 0(a1);
                sv_q C000, 0(a0);
                : : "{4}"(dst8), "{5}"(src8), "{6}"(size) : "memory" : "volatile"
            }
            dst8 = dst8.add(16);
            src8 = src8.add(16);
            size = size.saturating_sub(16);
        }

        let mut dst32 = dst8 as *mut u32;
        let mut src32 = src8 as *const u32;

        while size > 3 {
            *dst32 = *src32;
            dst32 = dst32.add(1);
            src32 = src32.add(1);
            size = size.saturating_sub(4);
        }

        while size > 0 {
            *dst8 = *src8;
            dst8 = dst8.add(1);
            src8 = src8.add(1);
            size = size.saturating_sub(1);
        }
        dst
    } else {
         panic!("Unaligned vfpu memcpy");
    }
}
