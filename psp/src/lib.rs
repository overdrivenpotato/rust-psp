#![feature(
    alloc_error_handler,
    llvm_asm,
    global_asm,
    untagged_unions,
    core_intrinsics,
    const_loop,
    const_if_match,
    const_generics,
    restricted_std,
    c_variadic,
    lang_items,
)]

// For unwinding support
#![feature(std_internals, panic_info_message, panic_internals, unwind_attributes)]
#![cfg_attr(not(feature = "stub-only"), feature(panic_unwind))]

// For the `const_generics` feature.
#![allow(incomplete_features)]

#[macro_use] extern crate paste;
#[cfg(not(feature = "stub-only"))] extern crate alloc;
#[cfg(not(feature = "stub-only"))] extern crate panic_unwind;

#[macro_use]
#[doc(hidden)]
#[cfg(not(feature = "stub-only"))]
pub mod debug;

#[macro_use] mod vfpu;
mod eabi;
pub mod math;
pub mod sys;
pub mod test_runner;
pub mod vram_alloc;

#[cfg(not(feature = "stub-only"))] mod alloc_impl;
#[cfg(not(feature = "stub-only"))] pub mod panic;

#[cfg(not(feature = "stub-only"))] mod screenshot;
#[cfg(not(feature = "stub-only"))] pub use screenshot::*;

#[cfg(not(feature = "stub-only"))] mod benchmark;
#[cfg(not(feature = "stub-only"))] pub use benchmark::*;

#[cfg(not(feature = "stub-only"))] mod constants;
#[cfg(not(feature = "stub-only"))] pub use constants::*;

//#[cfg(feature = "stub-only")]
//#[panic_handler]
//fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[cfg(feature="embedded-graphics")]
pub mod embedded_graphics;

#[repr(align(16))]
pub struct Align16<T>(pub T);

#[cfg(all(target_os = "psp", not(feature = "stub-only")))]
global_asm!(
    r#"
        .section .lib.ent.top, "a", @progbits
        .align 2
        .word 0
    .global __lib_ent_top
    __lib_ent_top:
        .section .lib.ent.btm, "a", @progbits
        .align 2
    .global __lib_ent_bottom
    __lib_ent_bottom:
        .word 0

        .section .lib.stub.top, "a", @progbits
        .align 2
        .word 0
    .global __lib_stub_top
    __lib_stub_top:
        .section .lib.stub.btm, "a", @progbits
        .align 2
    .global __lib_stub_bottom
    __lib_stub_bottom:
        .word 0
    "#
);

/// Declare a PSP module.
///
/// You must also define a `fn psp_main() { ... }` function in conjunction with
/// this macro.
#[macro_export]
macro_rules! module {
    ($name:expr, $version_major:expr, $version_minor: expr) => {
        #[doc(hidden)]
        mod __psp_module {
            #[no_mangle]
            #[link_section = ".rodata.sceModuleInfo"]
            #[used]
            static MODULE_INFO: $crate::Align16<$crate::sys::SceModuleInfo> = $crate::Align16(
                $crate::sys::SceModuleInfo {
                    mod_attribute: 0,
                    mod_version: [$version_major, $version_minor],
                    mod_name: $crate::sys::SceModuleInfo::name($name),
                    terminal: 0,
                    gp_value: unsafe { &_gp },
                    stub_top: unsafe { &__lib_stub_top },
                    stub_end: unsafe { &__lib_stub_bottom },
                    ent_top: unsafe { &__lib_ent_top },
                    ent_end: unsafe { &__lib_ent_bottom },
                }
            );

            extern {
                static _gp: u8;
                static __lib_ent_bottom: u8;
                static __lib_ent_top: u8;
                static __lib_stub_bottom: u8;
                static __lib_stub_top: u8;
            }

            #[no_mangle]
            #[link_section = ".lib.ent"]
            #[used]
            static LIB_ENT: $crate::sys::SceLibraryEntry = $crate::sys::SceLibraryEntry {
                // TODO: Fix this?
                name: core::ptr::null(),
                version: ($version_major, $version_minor),
                attribute: $crate::sys::SceLibAttr::SCE_LIB_IS_SYSLIB,
                entry_len: 4,
                var_count: 1,
                func_count: 1,
                entry_table: &LIB_ENT_TABLE,
            };

            #[no_mangle]
            #[link_section = ".rodata.sceResident"]
            #[used]
            static LIB_ENT_TABLE: $crate::sys::SceLibraryEntryTable = $crate::sys::SceLibraryEntryTable {
                module_start_nid: 0xd632acdb, // module_start
                module_info_nid: 0xf01d73a7, // SceModuleInfo
                module_start: module_start,
                module_info: &MODULE_INFO.0,
            };

            #[no_mangle]
            extern "C" fn module_start(_argc: isize, _argv: *const *const u8) -> isize {
                use $crate::sys::ThreadAttributes;
                use core::ffi::c_void;

                unsafe {
                    extern fn main_thread(_argc: usize, _argv: *mut c_void) -> i32 {
                        // TODO: Maybe print any error to debug screen?
                        let _ = std::panic::catch_unwind(|| {
                            super::psp_main();
                        });

                        0
                    }

                    let id = $crate::sys::sceKernelCreateThread(
                        &b"main_thread\0"[0],
                        main_thread,
                        // default priority of 32.
                        32,
                        // 256kb stack
                        256 * 1024,
                        ThreadAttributes::USER,
                        core::ptr::null_mut(),
                    );

                    $crate::sys::sceKernelStartThread(id, 0, core::ptr::null_mut());
                }

                0
            }
        }
    }
}

/// Enable the home button.
///
/// This API does not have destructor support yet. You can manually setup an
/// exit callback if you need this, see the source code of this function.
pub fn enable_home_button() {
    use core::{ptr, ffi::c_void};
    use sys::ThreadAttributes;

    unsafe {
        unsafe extern fn exit_thread(_args: usize, _argp: *mut c_void) -> i32 {
            unsafe extern fn exit_callback(_arg1: i32, _arg2: i32, _arg: *mut c_void) -> i32 {
                sys::sceKernelExitGame();
                0
            }

            let id = sys::sceKernelCreateCallback(
                &b"exit_callback\0"[0],
                exit_callback,
                ptr::null_mut(),
            );

            sys::sceKernelRegisterExitCallback(id);
            sys::sceKernelSleepThreadCB();

            0
        }

        // Enable the home button.
        let id = sys::sceKernelCreateThread(
            &b"exit_thread\0"[0],
            exit_thread,
            32,
            0x1000,
            ThreadAttributes::empty(),
            ptr::null_mut(),
        );

        sys::sceKernelStartThread(id, 0, ptr::null_mut());
    }
}
