//! PSP OS System API
//!
//! The names of functions and types beginning with `sce` or `Sce` were found by
//! reverse engineering various PSP games and OS versions.
//!
//! - `sceXYZ`: Sony API
//!     - `sceKernelXYZ`: Interface to the PSP OS kernel
//!     - `sceCtrlXYZ`: Button control API
//!     - `sceDisplayXYZ`: Display API
//!     - `sceGeXYZ`: Interface to the graphics chip (Graphics Engine)
//!     - `sceUsb`: USB API
//!         - `sceUsbCam`: USB camera
//!     - `scePower`: Power API
//!     - `sceWlan`: Wireless network API
//!     - `sceRtc`: Real time clock API
//!     - `sceIo`: File I/O API
//!     - `sceAudio`: Audio API
//!     - `sceAtrac`: Sony ATRAC3 Codec API
//!     - `sceJpeg`: JPEG decoding API
//!     - `sceUmd`: UMD Drive API
//!     - `sceMpeg`: MPEG codec API
//!     - `sceHprm`: Headphone Remote API (headphone accessory with controls)
//!     - `sceGu`: Graphics API (Similar to OpenGL)
//!     - `sceGum`: Matrix utility functions
//!     - `sceMp3`: MP3 decoder API
//!     - `sceRegistry`: PSP OS Registry API
//!     - `sceOpenPSID`: Console identification API (unique to every console)
//!     - `sceUtility`: Various utilities such as msg dialogs and savedata

#![allow(clippy::missing_safety_doc)]

use core::{mem, ptr};

#[macro_use]
mod macros;

mod ctrl;
pub use ctrl::*;

mod display;
pub use display::*;

mod ge;
pub use ge::*;

mod kernel;
pub use kernel::*;

mod usb;
pub use usb::*;

mod power;
pub use power::*;

mod wlan;
pub use wlan::*;

mod rtc;
pub use rtc::*;

mod io;
pub use io::*;

mod audio;
pub use audio::*;

mod atrac;
pub use atrac::*;

mod jpeg;
pub use jpeg::*;

mod umd;
pub use umd::*;

mod mpeg;
pub use mpeg::*;

mod hprm;
pub use hprm::*;

mod gu;
pub use gu::*;

mod gum;
pub use gum::*;

mod types;
pub use types::*;

mod mp3;
pub use mp3::*;

mod registry;
pub use registry::*;

mod openpsid;
pub use openpsid::*;

mod utility;
pub use utility::*;

mod net;
pub use net::*;

mod font;
pub use font::*;

mod psmf;
pub use psmf::*;

// These are not found (likely because this was tested in user mode on a PSP-2000).
// pub mod sircs;
// pub mod codec;
// TODO: Add kernel module support to this crate.
// pub mod nand;

pub mod vfpu_context;

use core::ffi::c_void;

// http://uofw.github.io/uofw/structSceStubLibraryEntryTable.html
#[repr(C)]
pub struct SceStubLibraryEntry {
    pub name: *const u8,
    pub version: [u8; 2],
    pub flags: u16,
    pub len: u8,
    pub v_stub_count: u8,
    pub stub_count: u16,
    pub nid_table: *const u32,
    pub stub_table: *const c_void,
}

unsafe impl Sync for SceStubLibraryEntry {}

#[repr(u16)]
pub enum ModuleInfoAttr {
    User = 0,
    NoStop = 0x0001,
    SingleLoad = 0x0002,
    SingleStart = 0x0004,
    Kernel = 0x1000,
}

#[repr(C, packed)]
pub struct SceModuleInfo {
    // TODO: Change this type to `ModuleInfoAttr`. This is a breaking change.
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
    #[doc(hidden)]
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
    pub entry_table: *const SceLibraryEntryTable,
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

#[repr(C)]
pub struct SceLibraryEntryTable {
    pub module_start_nid: u32,
    pub module_info_nid: u32,
    pub module_start: unsafe extern "C" fn(usize, *mut c_void) -> isize,
    pub module_info: *const SceModuleInfo,
}

unsafe impl Sync for SceLibraryEntryTable {}

/// Event which has occurred in the memory stick ejection callback, passed in
/// `arg2`.
pub enum MsCbEvent {
    Inserted = 1,
    Ejected = 2,
}

/// Returns whether a memory stick is current inserted
///
/// # Return Value
///
/// 1 if memory stick inserted, 0 if not or if < 0 on error
#[inline(always)]
#[allow(non_snake_case)]
pub unsafe fn MScmIsMediumInserted() -> i32 {
    let mut status: i32 = 0;

    let ret = io::sceIoDevctl(
        b"mscmhc0:\0" as _,
        0x02025806,
        ptr::null_mut(),
        0,
        &mut status as *mut _ as _,
        mem::size_of::<i32>() as i32,
    );

    if ret < 0 {
        ret
    } else if status != 1 {
        0
    } else {
        1
    }
}

/// Registers a memory stick ejection callback.
///
/// See `MsCbEvent`.
///
/// # Parameters
///
/// - `cbid`: The uid of an allocated callback
///
/// # Return Value
///
/// 0 on success, < 0 on error
#[inline(always)]
#[allow(non_snake_case)]
pub unsafe fn MScmRegisterMSInsertEjectCallback(mut cbid: SceUid) -> i32 {
    sceIoDevctl(
        b"fatms0:\0" as _,
        0x02415821,
        &mut cbid as *mut _ as _,
        mem::size_of::<SceUid>() as i32,
        ptr::null_mut(),
        0,
    )
}

/// Unregister a memory stick ejection callback
///
/// # Parameters
///
/// - `cbid`: The uid of an allocated callback
///
/// # Return Value
///
/// 0 on success, < 0 on error
#[inline(always)]
#[allow(non_snake_case)]
pub unsafe fn MScmUnregisterMSInsertEjectCallback(mut cbid: SceUid) -> i32 {
    sceIoDevctl(
        b"fatms0:\0" as _,
        0x02415822,
        &mut cbid as *mut _ as _,
        mem::size_of::<SceUid>() as i32,
        ptr::null_mut(),
        0,
    )
}
