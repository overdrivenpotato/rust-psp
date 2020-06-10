#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::SceUid;

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let fd = init_tests();
    test_hello(&fd);
    psp::dprintln!("hello");
    test_screenshot();
    test_panic(&fd);
    end_tests(fd);
}

fn init_tests() -> SceUid {
    let buf = b"host0:/psp-ci-test.test\0";
    unsafe {
        let fd = psp::sys::sceIoOpen(
            buf as *const u8,
            psp::sys::IoOpenFlags::CREAT |
            psp::sys::IoOpenFlags::RD_WR, 0o777
        );
        return fd
    }
}

fn end_tests(fd: SceUid) {
    unsafe { psp::sys::sceIoClose(fd); }
}

fn test_hello(fd: &SceUid) {
    unsafe {
        psp::sys::sceIoWrite(*fd, b"Hello CI\n" as *const u8 as *const c_void, 9);
    }
}

fn test_panic(fd: &SceUid) {
    let result = psp::panic::catch_unwind(|| {
        panic!("panic test");
    });
    if result.is_err() {
        unsafe {
            psp::sys::sceIoWrite(
                *fd,
                b"Panics work\n" as *const u8 as *const c_void, 12
            );
        }
    }
}

fn test_screenshot() {
    let screenshot = psp::screenshot_bmp();

    unsafe {
        let fd = psp::sys::sceIoOpen(
            b"host0:/psp-ci-test.bmp\0" as *const u8,
            psp::sys::IoOpenFlags::CREAT |
            psp::sys::IoOpenFlags::RD_WR, 0o777
        );
        psp::sys::sceIoWrite(
            fd,
            &screenshot as *const _ as *const c_void,
            screenshot.len(),
        );
        psp::sys::sceIoClose(fd);
    }
}
