#![feature(
    alloc_error_handler,
    llvm_asm,
    global_asm,
    naked_functions,
    untagged_unions,
    core_intrinsics,
    const_in_array_repeat_expressions,
    const_loop,
    const_if_match,
    const_generics,
    c_variadic,
    start,
)]

// For unwinding support
#![feature(std_internals, panic_info_message, panic_internals, unwind_attributes, panic_unwind)]

// For the `const_generics` feature.
#![allow(incomplete_features)]

#![no_std]

#[macro_use] extern crate paste;
extern crate alloc;
extern crate panic_unwind;

#[macro_use] pub mod debug;
mod eabi;
mod alloc_impl;
pub mod panic;
pub mod sys;
pub mod vfpu;

#[cfg(feature="emb-gfx")]
pub mod framebuf_gfx;

#[repr(align(16))]
pub struct Align16<T>(pub T);

#[repr(C, packed)]
pub struct SceModuleInfo {
    pub mod_attribute: u16,
    pub mod_version: [u8; 2],
    pub mod_name: [u8; 27],
    pub terminal: u8,
    pub gp_value: *const u8,
    pub ent_top: *const u8,
    pub ent_end: *const u8,
    pub stub_top: *const u8,
    pub stub_end: *const u8,
}

unsafe impl Sync for SceModuleInfo {}

impl SceModuleInfo {
    pub const fn name(s: &str) -> [u8; 27] {
        let bytes = s.as_bytes();
        let mut result = [0; 27];

        let mut i = 0;
        while i < bytes.len() {
            result[i] = bytes[i];

            i += 1;
        }

        result
    }
}

#[repr(C, packed)]
pub struct SceLibraryEntry {
    pub name: *const u8,
    pub version: (u8, u8),
    pub attribute: SceLibAttr,
    pub entry_len: u8,
    pub var_count: u8,
    pub func_count: u16,
    pub entry_table: *const LibraryEntryTable,
}

unsafe impl Sync for SceLibraryEntry {}

bitflags::bitflags! {
    // https://github.com/uofw/uofw/blob/f099b78dc0937df4e7346e2e417b63f471f8a3af/include/loadcore.h#L152
    pub struct SceLibAttr: u16 {
        const SCE_LIB_NO_SPECIAL_ATTR = 0;
        const SCE_LIB_AUTO_EXPORT = 0x1;
        const SCE_LIB_WEAK_EXPORT = 0x2;
        const SCE_LIB_NOLINK_EXPORT = 0x4;
        const SCE_LIB_WEAK_IMPORT = 0x8;
        const SCE_LIB_SYSCALL_EXPORT = 0x4000;
        const SCE_LIB_IS_SYSLIB = 0x8000;
    }
}

pub struct LibraryEntryTable {
    pub module_start_nid: u32,
    pub module_info_nid: u32,
    pub module_start: unsafe extern "C" fn(isize, *const *const u8) -> isize,
    pub module_info: *const SceModuleInfo,
}

unsafe impl Sync for LibraryEntryTable {}

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

#[macro_export]
macro_rules! module {
    ($name:expr, $version_major:expr, $version_minor: expr) => {
        #[doc(hidden)]
        mod __psp_module {
            #[no_mangle]
            #[link_section = ".rodata.sceModuleInfo"]
            static MODULE_INFO: $crate::Align16<$crate::SceModuleInfo> = $crate::Align16(
                $crate::SceModuleInfo {
                    mod_attribute: 0,
                    mod_version: [$version_major, $version_minor],
                    mod_name: $crate::SceModuleInfo::name($name),
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
            static LIB_ENT: $crate::SceLibraryEntry = $crate::SceLibraryEntry {
                // TODO: Fix this?
                name: core::ptr::null(),
                version: ($version_major, $version_minor),
                attribute: $crate::SceLibAttr::SCE_LIB_IS_SYSLIB,
                entry_len: 4,
                var_count: 1,
                func_count: 1,
                entry_table: &LIB_ENT_TABLE,
            };

            #[no_mangle]
            #[link_section = ".rodata.sceResident"]
            static LIB_ENT_TABLE: $crate::LibraryEntryTable = $crate::LibraryEntryTable {
                module_start_nid: 0xd632acdb, // module_start
                module_info_nid: 0xf01d73a7, // SceModuleInfo
                module_start: module_start,
                module_info: &MODULE_INFO.0,
            };

            #[no_mangle]
            extern "C" fn module_start(_argc: isize, _argv: *const *const u8) -> isize {
                use $crate::sys::kernel::ThreadAttributes;
                use core::ffi::c_void;

                unsafe {
                    extern fn main_thread(_argc: usize, _argv: *mut c_void) -> i32 {
                        // TODO: Maybe print any error to debug screen?
                        let _ = $crate::panic::catch_unwind(|| {
                            super::psp_main();
                        });

                        0
                    }

                    let id = $crate::sys::kernel::sce_kernel_create_thread(
                        &b"main_thread\0"[0],
                        main_thread,
                        // default priority of 32.
                        32,
                        // 256kb stack
                        256 * 1024,
                        ThreadAttributes::USER,
                        core::ptr::null_mut(),
                    );

                    $crate::sys::kernel::sce_kernel_start_thread(id, 0, core::ptr::null_mut());
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
    use sys::kernel::ThreadAttributes;

    unsafe {
        unsafe extern fn exit_thread(_args: usize, _argp: *mut c_void) -> i32 {
            unsafe extern fn exit_callback(_arg1: i32, _arg2: i32, _arg: *mut c_void) -> i32 {
                sys::kernel::sce_kernel_exit_game();
                0
            }

            let id = sys::kernel::sce_kernel_create_callback(
                &b"exit_callback\0"[0],
                exit_callback,
                ptr::null_mut(),
            );

            sys::kernel::sce_kernel_register_exit_callback(id);
            sys::kernel::sce_kernel_sleep_thread_cb();

            0
        }

        // Enable the home button.
        let id = sys::kernel::sce_kernel_create_thread(
            &b"exit_thread\0"[0],
            exit_thread,
            32,
            0x1000,
            ThreadAttributes::empty(),
            ptr::null_mut(),
        );

        sys::kernel::sce_kernel_start_thread(id, 0, ptr::null_mut());
    }
}
