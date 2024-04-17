use crate::sys::{self, SceUid};
use core::ffi::c_void;

pub const OUTPUT_FILENAME: &str = "psp_output_file.log";
pub const OUTPUT_FIFO: &str = "psp_output_pipe.fifo";

pub const STARTING_TOKEN: &str = "STARTING_TESTS";
pub const SUCCESS_TOKEN: &str = "FINAL_SUCCESS";
pub const FAILURE_TOKEN: &str = "FINAL_FAILURE";

use alloc::format;
use alloc::vec::Vec;
use core::fmt::Arguments;

pub struct TestRunner<'a> {
    mode: TestRunnerMode,
    failure: bool,
    failures: Vec<&'a str>,
}

enum TestRunnerMode {
    Fifo(SceUid),
    File(SceUid),
    Dprintln,
}

impl<'a> TestRunner<'a> {
    pub fn new_fifo_runner() -> Self {
        let fd = get_test_output_pipe();
        Self {
            mode: TestRunnerMode::Fifo(fd),
            failure: false,
            failures: Vec::new(),
        }
    }

    pub fn new_file_runner() -> Self {
        let fd = get_test_output_file();
        Self {
            mode: TestRunnerMode::File(fd),
            failure: false,
            failures: Vec::new(),
        }
    }

    pub fn new_dprintln_runner() -> Self {
        Self {
            mode: TestRunnerMode::Dprintln,
            failure: false,
            failures: Vec::new(),
        }
    }

    pub fn run<F: Fn(&mut TestRunner)>(&mut self, f: F) {
        f(self)
    }

    pub fn start_run(&self) {
        self.write_args(format_args!("\n\n{}\n", STARTING_TOKEN));
    }

    pub fn finish_run(self) {
        if self.failure {
            self.write_args(format_args!("Failing tests: {:?}\n", self.failures));
            self.write_args(format_args!("{}\n", FAILURE_TOKEN));
        } else {
            self.write_args(format_args!("{}\n", SUCCESS_TOKEN));
        }
        self.quit();
    }

    pub fn check_fns_do_not_panic(&self, tests: &[(&str, &dyn Fn())]) {
        for (testcase_name, f) in tests {
            f();
            self.pass(testcase_name, "");
        }
    }

    pub fn check<T>(&mut self, testcase_name: &'a str, l: T, r: T)
    where
        T: core::fmt::Debug + PartialEq,
    {
        if l == r {
            self.pass(testcase_name, &format!("{:?} == {:?}", l, r));
        } else {
            self.fail(testcase_name, &format!("{:?} != {:?}", l, r));
        }
    }

    pub fn check_true(&mut self, testcase_name: &'a str, pred: bool) {
        if pred {
            self.pass(testcase_name, "True.");
        } else {
            self.fail(testcase_name, "False!");
        }
    }

    pub fn check_large_collection<T>(&mut self, testcase_name: &'a str, l: &[T], r: &[T])
    where
        T: core::fmt::Debug + PartialEq + Eq,
    {
        if l.iter().eq(r.iter()) {
            self.pass(testcase_name, "Equal!");
        } else {
            if l.len() != r.len() {
                self.dbg(
                    testcase_name,
                    &format!("Lengths differ! {} != {}", l.len(), r.len()),
                );
            }

            for (i, (li, ri)) in l.iter().zip(r.iter()).enumerate() {
                if li != ri {
                    self.dbg(
                        testcase_name,
                        &format!("Differ on item {}: {:?} != {:?}", i, li, ri),
                    );
                    break;
                }
            }

            self.fail(testcase_name, "Collections were not equal!");
        }
    }

    pub fn check_silent<T>(&mut self, testcase_name: &'a str, l: T, r: T)
    where
        T: core::fmt::Debug + PartialEq,
    {
        if l == r {
            self.pass(testcase_name, "Equal.");
        } else {
            self.fail(testcase_name, "Not equal!");
        }
    }

    pub fn check_list<T>(&mut self, val_pairs: &[(&'a str, T, T)])
    where
        T: core::fmt::Debug + PartialEq,
    {
        for (testcase_name, l, r) in val_pairs {
            self.check(testcase_name, l, r)
        }
    }

    pub fn _check_return_values<T>(&mut self, val_pairs: &[(&'a str, &dyn Fn() -> T, T)])
    where
        T: core::fmt::Debug + PartialEq + Eq + Clone,
    {
        self.check_list(
            &val_pairs
                .iter()
                .map(|(testcase_name, f, v)| (*testcase_name, f(), v.clone()))
                .collect::<Vec<(&str, T, T)>>(),
        )
    }

    pub fn pass(&self, testcase_name: &str, msg: &str) {
        self.write_args(format_args!("[PASS]: ({}) {}\n", testcase_name, msg));
    }

    pub fn dbg(&self, testcase_name: &str, msg: &str) {
        self.write_args(format_args!("[NOTE]: ({}) {}\n", testcase_name, msg));
    }

    pub fn fail(&mut self, testcase_name: &'a str, msg: &str) {
        self.failure = true;
        self.failures.push(testcase_name);
        self.write_args(format_args!("[FAIL]: ({}) {}\n", testcase_name, msg));
    }

    pub fn write_args(&self, args: Arguments) {
        match self.mode {
            TestRunnerMode::File(fd) | TestRunnerMode::Fifo(fd) => {
                write_to_psp_output_fd(fd, &format!("{}", args));
            }
            TestRunnerMode::Dprintln => {
                crate::dprintln!("{}", args);
            }
        }
    }

    fn quit(self) {
        match self.mode {
            TestRunnerMode::File(fd) | TestRunnerMode::Fifo(fd) => {
                close_psp_file(fd);
                quit_game();
            }
            TestRunnerMode::Dprintln => loop {
                core::hint::spin_loop()
            },
        }
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
                You must create it yourself with `mkfifo`.",
                fd.0
            );
        }
        fd
    }
}

fn get_test_output_file() -> SceUid {
    unsafe {
        let fd = sys::sceIoOpen(
            psp_filename(OUTPUT_FILENAME),
            sys::IoOpenFlags::TRUNC | sys::IoOpenFlags::CREAT | sys::IoOpenFlags::RD_WR,
            0o777,
        );
        if fd.0 < 0 {
            panic!("Unable to open file \"{}\" for output!", OUTPUT_FILENAME);
        }
        fd
    }
}

fn psp_filename(filename: &str) -> *const u8 {
    format!("host0:/{}\0", filename).as_bytes().as_ptr()
}

fn write_to_psp_output_fd(fd: SceUid, msg: &str) {
    unsafe {
        sys::sceIoWrite(fd, msg.as_bytes().as_ptr() as *const c_void, msg.len());
    }
}

fn close_psp_file(fd: SceUid) {
    unsafe {
        sys::sceIoClose(fd);
    }
}

fn quit_game() {
    unsafe {
        sys::sceKernelExitGame();
    }
}
