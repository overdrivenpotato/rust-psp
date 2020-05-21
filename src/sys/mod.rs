#[macro_use]
mod macros;

pub mod ctrl;
pub mod display;
pub mod ge;
pub mod kernel;
pub mod usb;
pub mod power;
pub mod wlan;
pub mod rtc;
pub mod io;
pub mod audio;
pub mod atrac;
pub mod jpeg;
pub mod umd;
pub mod mpeg;
pub mod hprm;
pub mod gu;

// These fail with a bus error when being loaded (tested in user mode on a PSP-2000).
// TODO: Investigate and fix this
// pub mod mp3;
// pub mod registry;

// These are not found (likely because this was tested in user mode).
// TODO: Add kernel module support to this crate.
// pub mod openpsid;
// pub mod sircs;
// pub mod video;
// pub mod nand;

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
