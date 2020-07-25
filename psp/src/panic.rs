//! Panic support for the PSP.

// Most of the code here is lifted from `rustc/src/libstd/panicking.rs`. It has
// been adapted to run on the PSP.

use crate::sys;
use core::{mem::{self, ManuallyDrop}, any::Any, panic::{PanicInfo, BoxMeUp, Location}};
use core::fmt;
use alloc::{boxed::Box, string::{String, ToString}};

#[link(name = "unwind", kind = "static")]
extern {}

fn print_and_die(s: String) -> ! {
    dprintln!("{}", s);

    unsafe {
        sys::sceKernelExitDeleteThread(1);
        core::intrinsics::unreachable()
    }
}

//#[panic_handler]
//#[inline(never)]
//fn panic(info: &PanicInfo) -> ! {
//    panic_impl(info)
//}

#[inline(always)]
#[cfg_attr(not(target_os = "psp"), allow(unused))]
fn panic_impl(info: &PanicInfo) -> ! {
    struct PanicPayload<'a> {
        inner: &'a fmt::Arguments<'a>,
        string: Option<String>,
    }

    impl<'a> PanicPayload<'a> {
        fn new(inner: &'a fmt::Arguments<'a>) -> PanicPayload<'a> {
            PanicPayload { inner, string: None }
        }

        fn fill(&mut self) -> &mut String {
            use fmt::Write;
            let inner = self.inner;
            self.string.get_or_insert_with(|| {
                let mut s = String::new();
                drop(s.write_fmt(*inner));
                s
            })
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

    let loc = info.location().unwrap();
    let msg = info.message().unwrap();
    rust_panic_with_hook(&mut PanicPayload::new(msg), info.message(), loc);
}

/// Central point for dispatching panics.
///
/// Executes the primary logic for a panic, including checking for recursive
/// panics, panic hooks, and finally dispatching to the panic runtime to either
/// abort or unwind.
fn rust_panic_with_hook(
    payload: &mut dyn BoxMeUp,
    message: Option<&fmt::Arguments<'_>>,
    location: &Location<'_>,
) -> ! {
    let panics = update_panic_count(1);

    fn die_nested() -> ! {
        print_and_die("thread panicked while processing panic. aborting.".into());
    }

    let mut info = PanicInfo::internal_constructor(message, location);
    info.set_payload(payload.get());

    dprintln!("{}", info.to_string());

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
    #[unwind(allowed)]
    fn __rust_start_panic(payload: usize) -> u32;
}

#[inline(never)]
#[no_mangle]
fn rust_panic(mut msg: &mut dyn BoxMeUp) -> ! {
    let code = unsafe {
        let obj = &mut msg as *mut &mut dyn BoxMeUp;
        panic_unwind::__rust_start_panic(obj as usize)
    };

    print_and_die(alloc::format!("failed to initiate panic, error {}", code))
}

#[cfg(not(test))]
#[no_mangle]
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

    let mut data = Data { f: ManuallyDrop::new(f) };

    let data_ptr = &mut data as *mut _ as *mut u8;

    return unsafe {
        if core::intrinsics::r#try(do_call::<F, R>, data_ptr, do_catch::<F, R>) == 0 {
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
    unsafe extern "C" fn abort() {
        loop { llvm_asm!("" :::: "volatile"); }
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
