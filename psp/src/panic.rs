//! Panic support for the PSP.

// Most of the code here is lifted from `rustc/src/libstd/panicking.rs`. It has
// been adapted to run on the PSP.

#[cfg(not(feature = "std"))]
use crate::sys;

#[cfg(feature = "std")]
use core::{any::Any, mem::ManuallyDrop};
#[cfg(not(feature = "std"))]
use core::{
    any::Any,
    mem::{self, ManuallyDrop},
    panic::{Location, PanicInfo, PanicMessage, PanicPayload as BoxMeUp},
};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String};

#[link(name = "unwind", kind = "static")]
extern "C" {}

#[cfg(not(feature = "std"))]
fn print_and_die(s: String) -> ! {
    dprintln!("{}", s);

    unsafe {
        sys::sceKernelExitDeleteThread(1);
        core::intrinsics::unreachable()
    }
}

#[cfg(not(feature = "std"))]
#[panic_handler]
#[inline(never)]
fn panic(info: &PanicInfo) -> ! {
    panic_impl(info)
}

#[inline(always)]
#[cfg_attr(not(target_os = "psp"), allow(unused))]
#[cfg(not(feature = "std"))]
fn panic_impl(info: &PanicInfo) -> ! {
    use core::fmt;

    struct PanicPayload<'a> {
        message: PanicMessage<'a>,
        location: &'a Location<'a>,
        string: Option<String>,
    }

    impl<'a> PanicPayload<'a> {
        fn new(info: &'a PanicInfo<'a>) -> PanicPayload<'a> {
            let message = info.message();
            let location = info.location().unwrap();
            PanicPayload {
                message,
                location,
                string: None,
            }
        }

        fn fill(&mut self) -> &mut String {
            let s = alloc::format!(
                "panicked at {}:{}:{}: {}",
                self.location.file(),
                self.location.line(),
                self.location.column(),
                self.message.as_str().unwrap_or_default()
            );

            self.string.get_or_insert_with(|| s)
        }
    }

    unsafe impl<'a> BoxMeUp for PanicPayload<'a> {
        fn take_box(&mut self) -> *mut (dyn Any + Send) {
            let contents = mem::take(self.fill());
            Box::into_raw(Box::new(contents))
        }

        fn get(&mut self) -> &(dyn Any + Send) {
            self.fill()
        }
    }

    impl fmt::Display for PanicPayload<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some(s) = &self.string {
                s.fmt(f)
            } else {
                Err(fmt::Error)
            }
        }
    }

    rust_panic_with_hook(&mut PanicPayload::new(&info));
}

/// Central point for dispatching panics.
///
/// Executes the primary logic for a panic, including checking for recursive
/// panics, panic hooks, and finally dispatching to the panic runtime to either
/// abort or unwind.
#[cfg(not(feature = "std"))]
fn rust_panic_with_hook(payload: &mut dyn BoxMeUp) -> ! {
    let panics = update_panic_count(1);

    fn die_nested() -> ! {
        print_and_die("thread panicked while processing panic. aborting.".into());
    }

    payload.get(); // populate the payload's string
    dprintln!("{}", payload);

    if panics > 1 {
        // If a thread panics while it's already unwinding then we
        // have limited options. Currently our preference is to
        // just abort. In the future we may consider resuming
        // unwinding or otherwise exiting the thread cleanly.
        die_nested();
    }

    rust_panic(payload)
}

fn update_panic_count(amt: isize) -> usize {
    // TODO: Make this thread local
    static mut PANIC_COUNT: usize = 0;

    unsafe {
        PANIC_COUNT = (PANIC_COUNT as isize + amt) as usize;
        PANIC_COUNT
    }
}

#[allow(improper_ctypes)]
extern "C" {
    fn __rust_panic_cleanup(payload: *mut u8) -> *mut (dyn Any + Send + 'static);
}

#[allow(improper_ctypes)]
extern "C-unwind" {
    fn __rust_start_panic(payload: usize) -> u32;
}

#[inline(never)]
#[no_mangle]
#[cfg(not(feature = "std"))]
fn rust_panic(msg: &mut dyn BoxMeUp) -> ! {
    let code = unsafe {
        let obj = msg;
        panic_unwind::__rust_start_panic(obj as _)
    };

    print_and_die(alloc::format!("failed to initiate panic, error {}", code))
}

#[cfg(not(test))]
#[no_mangle]
#[cfg(not(feature = "std"))]
extern "C" fn __rust_drop_panic() -> ! {
    print_and_die("Rust panics must be rethrown".into());
}

/// Invoke a closure, capturing the cause of an unwinding panic if one occurs.
#[inline(never)]
pub fn catch_unwind<R, F: FnOnce() -> R>(f: F) -> Result<R, Box<dyn Any + Send>> {
    // This whole function is directly lifted out of rustc. See comments there
    // for an explanation of how this actually works.

    union Data<F, R> {
        f: ManuallyDrop<F>,
        r: ManuallyDrop<R>,
        p: ManuallyDrop<Box<dyn Any + Send>>,
    }

    let mut data = Data {
        f: ManuallyDrop::new(f),
    };

    let data_ptr = &mut data as *mut _ as *mut u8;

    return unsafe {
        if core::intrinsics::catch_unwind(do_call::<F, R>, data_ptr, do_catch::<F, R>) == 0 {
            Ok(ManuallyDrop::into_inner(data.r))
        } else {
            Err(ManuallyDrop::into_inner(data.p))
        }
    };

    #[cold]
    unsafe fn cleanup(payload: *mut u8) -> Box<dyn Any + Send + 'static> {
        let obj = Box::from_raw(__rust_panic_cleanup(payload));
        update_panic_count(-1);
        obj
    }

    #[inline]
    fn do_call<F: FnOnce() -> R, R>(data: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let f = ManuallyDrop::take(&mut data.f);
            data.r = ManuallyDrop::new(f());
        }
    }

    #[inline]
    fn do_catch<F: FnOnce() -> R, R>(data: *mut u8, payload: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let data = &mut (*data);
            let obj = cleanup(payload);
            data.p = ManuallyDrop::new(obj);
        }
    }
}

// TODO: EH personality was moved from the panic_unwind crate to std in
// https://github.com/rust-lang/rust/pull/92845. This no-op implementation
// should be replaced with the version from std when using no_std.
#[cfg(not(feature = "std"))]
#[lang = "eh_personality"]
unsafe extern "C" fn rust_eh_personality() {}

/// These symbols and functions should not actually be used. `libunwind`,
/// however, requires them to be present so that it can link.
// TODO: Patch these out of libunwind instead.
#[cfg(all(target_os = "psp", not(feature = "stub-only")))]
mod libunwind_shims {
    #[no_mangle]
    unsafe extern "C" fn fprintf(_stream: *const u8, _format: *const u8, ...) -> isize {
        -1
    }

    #[no_mangle]
    unsafe extern "C" fn fflush(_stream: *const u8) -> i32 {
        -1
    }

    #[no_mangle]
    #[allow(deprecated)]
    unsafe extern "C" fn abort() {
        loop {
            core::arch::asm!("");
        }
    }

    #[no_mangle]
    unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
        use alloc::alloc::{alloc, Layout};

        let size = size + 4;

        let data = alloc(Layout::from_size_align_unchecked(size, 4));
        *(data as *mut usize) = size;

        data.offset(4)
    }

    #[no_mangle]
    unsafe extern "C" fn free(data: *mut u8) {
        use alloc::alloc::{dealloc, Layout};

        let base = data.sub(4);
        let size = *(base as *mut usize);

        dealloc(base, Layout::from_size_align_unchecked(size, 4));
    }

    #[no_mangle]
    unsafe extern "C" fn getenv(_name: *const u8) -> *const u8 {
        core::ptr::null()
    }

    #[no_mangle]
    unsafe extern "C" fn __assert_func(_: *const u8, _: i32, _: *const u8, _: *const u8) {}

    #[no_mangle]
    static _impure_ptr: [usize; 0] = [];
}
