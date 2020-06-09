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

// These are not found (likely because this was tested in user mode on a PSP-2000).
// pub mod sircs;
// pub mod video;
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
