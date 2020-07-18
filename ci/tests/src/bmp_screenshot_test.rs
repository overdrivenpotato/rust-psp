use core::ffi::c_void;
use psp::test_runner::TestRunner;

pub fn test_main(test_runner: &mut TestRunner) {
    let screenshot = psp::screenshot_bmp();

    unsafe {
        let fd = psp::sys::sceIoOpen(
            b"host0:/psp-ci-test.bmp\0" as *const u8,
            psp::sys::IoOpenFlags::CREAT | psp::sys::IoOpenFlags::RD_WR,
            0o777,
        );
        psp::sys::sceIoWrite(
            fd,
            &screenshot as *const _ as *const c_void,
            screenshot.len(),
        );
        psp::sys::sceIoClose(fd);
    }

    test_runner.pass(
        "bmp_screenshot_generated",
        "Successfully wrote out BMP screenshot.",
    );
}
