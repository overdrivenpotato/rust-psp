#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::kernel::SceUid;

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let fd = init_tests();
    test_hello(&fd);
    test_panic(&fd);
    end_tests(fd);
}

fn init_tests() -> SceUid {
    let buf = b"host0:/psp-ci-test.test\0";
    unsafe {
        let fd = psp::sys::io::sce_io_open(
            buf as *const u8, psp::sys::io::OpenFlags::CREAT |
            psp::sys::io::OpenFlags::RD_WR, 0o777
        );
        return fd
    }
}

fn end_tests(fd: SceUid) {
    unsafe { psp::sys::io::sce_io_close(fd); }
}

fn test_hello(fd: &SceUid) {
    unsafe {
        psp::sys::io::sce_io_write(*fd, b"Hello CI\n" as *const u8 as *const c_void, 9);
    }
}

fn test_panic(fd: &SceUid) {
    let result = psp::panic::catch_unwind(|| {
        panic!("panic test");
    });
    if result.is_err() {
        unsafe {
            psp::sys::io::sce_io_write(
                *fd, 
                b"Panics work\n" as *const u8 as *const c_void, 12
            );
        }
    }
}
