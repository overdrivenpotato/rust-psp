use crate::sys::{self, SceUid};
use core::ffi::c_void;

pub const OUTPUT_FILENAME: &str = "psp_output_file.log";
pub const OUTPUT_FIFO: &str = "psp_output_pipe.fifo";

pub const STARTING_TOKEN: &str = "\n\nSTARTING_TESTS\n";
pub const SUCCESS_TOKEN: &str = "FINAL_SUCCESS\n";
pub const FAILURE_TOKEN: &str = "FINAL_FAILURE\n";

use alloc::format;
use alloc::vec::Vec;

pub struct TestRunner {
    _mode: TestRunnerMode,
    fd: SceUid,
    failure: bool,
}

enum TestRunnerMode {
    FIFO,
    FILE,
}

impl TestRunner {
    pub fn new_fifo_runner() -> Self {
        let fd = get_test_output_pipe();
        Self {
            fd,
            _mode: TestRunnerMode::FIFO,
            failure: false,
        }
    }

    pub fn new_file_runner() -> Self {
        let fd = get_test_output_file();
        Self {
            fd,
            _mode: TestRunnerMode::FILE,
            failure: false,
        }
    }

    pub fn start(&self) {
        self.write(STARTING_TOKEN);
    }

    pub fn finish(self) {
        if self.failure {
            self.write(FAILURE_TOKEN);
        } else {
            self.write(SUCCESS_TOKEN);
        }
        self.quit();
    }

    pub fn check_fns_do_not_panic(&mut self, tests: &[&dyn Fn()]) {
        for test in tests {
            test()
        }
    }

    pub fn check_value_equality<T>(&mut self, val_pairs: &[(T, T)])
    where
        T: core::fmt::Debug + PartialEq,
    {
        for (l, r) in val_pairs {
            if l == r {
                self.write(&format!("PASS: {:?} == {:?}\n", l, r));
            } else {
                self.write(&format!("FAIL: {:?} == {:?}\n", l, r));
                self.failure = true;
            }
        }
    }

    pub fn _check_return_values<T>(&mut self, val_pairs: &[(&dyn Fn() -> T, T)])
    where
        T: core::fmt::Debug + PartialEq + Eq + Clone,
    {
        self.check_value_equality(
            &val_pairs
                .iter()
                .map(|(f, v)| (f(), v.clone()))
                .collect::<Vec<(T, T)>>(),
        )
    }

    fn write(&self, msg: &str) {
        write_to_psp_output_fd(self.fd, msg);
    }

    fn quit(self) {
        close_psp_file_and_quit_game(self.fd);
    }
}

fn get_test_output_pipe() -> SceUid {
    unsafe {
        let fd = sys::sceIoOpen(
            psp_filename(OUTPUT_FIFO),
            sys::IoOpenFlags::APPEND | sys::IoOpenFlags::WR_ONLY,
            0o777,
        );
        if fd.0 < 0 {
            panic!(
                "Unable to open pipe \"{}\" for output! \
                You must create it yourself with `mkfifo`."
            );
        }
        return fd;
    }
}

fn get_test_output_file() -> SceUid {
    unsafe {
        let fd = sys::sceIoOpen(
            psp_filename(OUTPUT_FILENAME),
            sys::IoOpenFlags::CREAT | sys::IoOpenFlags::RD_WR,
            0o777,
        );
        if fd.0 < 0 {
            panic!("Unable to open file \"{}\" for output!", OUTPUT_FILENAME);
        }
        return fd;
    }
}

fn psp_filename(filename: &str) -> *const u8 {
    format!("host0:/{}\0", filename).as_bytes().as_ptr()
}

fn write_to_psp_output_fd(fd: SceUid, msg: &str) {
    unsafe {
        sys::sceIoWrite(
            fd,
            msg.as_bytes().as_ptr() as *const u8 as *const c_void,
            msg.len(),
        );
    }
}

fn close_psp_file_and_quit_game(fd: SceUid) {
    unsafe {
        sys::sceIoClose(fd);
        sys::sceKernelExitGame();
    }
}
