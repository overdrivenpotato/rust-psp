use core::mem::MaybeUninit;

macro_rules! trig_func {
    ( $($name:ident),* ) => {
        $( item! {
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn [< $name f32 >](rad: f32) -> f32 {
                let mut out = MaybeUninit::uninit();

                $crate::vfpu_asm!(
                    .mips "mfc1 $$t0, $1";
                    mtv t0, S000;
                    vcst_s S001, VFPU_2_PI;
                    vmul_s S000, S000, S001;
                    [< v $name _s>] S000, S000;
                    mfv v0, S000;
                    .mips "mtc1 $$v0, $1";

                    : "={v0}"(out.as_mut_ptr()) : "{t0}"(rad) : "memory" : "volatile"
                );

                out.assume_init()
            }
        })*
    };
}

trig_func! {cos, sin}
