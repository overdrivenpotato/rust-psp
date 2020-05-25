#![no_std]
#![no_main]

use core::ffi::c_void;

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        let buf = b"host0:/psp-ci-test.test\0";
        let fd = psp::sys::io::sce_io_open(buf as *const u8, psp::sys::io::OpenFlags::CREAT | psp::sys::io::OpenFlags::RD_WR, 0o777);
        psp::sys::io::sce_io_write(fd, b"Hello CI" as *const u8 as *const c_void, 8);
        psp::sys::io::sce_io_close(fd);
    }
}
