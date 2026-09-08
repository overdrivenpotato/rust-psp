#![no_std]
#![no_main]

extern crate alloc;
use alloc::alloc::Layout;
use alloc::format;
use core::time::Duration;
use core::ffi::c_void;
use psp::sys::SceUid;

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    // Enable the VFPU
    //unsafe {
        //use psp::sys::{self, ThreadAttributes};
        //sys::sceKernelChangeCurrentThreadAttr(0, ThreadAttributes::VFPU);
    //}

    let iters: [usize; 11] = [16, 8, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let sizes: [usize; 11] = [32,64,512,1024,2048,16348,32768,65536,131072,524288,1048576];

    let mut cpu_dur: Duration;
    let mut kernel_dur: Duration;
    let mut dmac_dur: Duration;
    let mut vfpu_dur: Duration;

    let fd = unsafe { psp::sys::sceIoOpen(b"host0:/results.txt\0".as_ptr(), psp::sys::IoOpenFlags::CREAT | psp::sys::IoOpenFlags::RD_WR, 0o777) };

    for i in 0..11 {
        let size = sizes[i];
        let iterations = iters[i];
        let src = unsafe { alloc::alloc::alloc(Layout::from_size_align_unchecked(size, 16)) };
        let dst = unsafe { alloc::alloc::alloc(Layout::from_size_align_unchecked(size, 16)) };

        let src = unsafe { core::mem::transmute::<*mut u8, *mut u32>(src) };
        let dst = unsafe { core::mem::transmute::<*mut u8, *mut u32>(dst) };

        unsafe { psp::sys::sceKernelMemset(src, 0xAA, size) };

        let src = unsafe { core::mem::transmute::<*mut u32, *mut u8>(src) };
        let dst = unsafe { core::mem::transmute::<*mut u32, *mut u8>(dst) };


        cpu_dur = psp::benchmark(|| {
            for _ in 0..iterations {
                unsafe { memcpy(dst, src as *const u8, size); }
            }
        }, 10);
        assert_eq!(unsafe { *dst }, 0xAA);


        let src = unsafe { core::mem::transmute::<*mut u8, *mut u32>(src) };
        let dst = unsafe { core::mem::transmute::<*mut u8, *mut u32>(dst) };

        unsafe { psp::sys::sceKernelMemset(src, 0x00, size) };

        unsafe { psp::sys::sceKernelMemset(src, 0xAA, size) };
        kernel_dur = psp::benchmark(|| {
            for _ in 0..iterations {
                unsafe { psp::sys::sceKernelMemcpy(dst, src, size); }
            }
        }, 10);
        assert_eq!(unsafe { *dst }, 0xAA);
        unsafe { psp::sys::sceKernelMemset(src, 0x00, size) };

        unsafe { psp::sys::sceKernelMemset(src, 0xAA, size) };
        dmac_dur = psp::benchmark(|| {
            for _ in 0..iterations {
                unsafe { psp::sys::sceDmacMemcpy(dst, src, size); }
            }
        }, 10);
        assert_eq!(unsafe { *dst }, 0xAA);
        unsafe { psp::sys::sceKernelMemset(src, 0x00, size) };

        vfpu_dur = Duration::new(0, 0);

        //unsafe { psp::sys::sceKernelMemset(src, 0xAA, size) };
        //vfpu_dur = psp::benchmark(|| {
            //for _ in 0..iterations {
                //unsafe { psp::sys::sceVfpuMemcpy(dst, src as *const u8, size); }
            //}
        //}, 10);
        //assert_eq!(unsafe { *dst }, 0xAA);
        //unsafe { psp::sys::sceKernelMemset(src, 0x00, size) };

        let src = unsafe { core::mem::transmute::<*mut u32, *mut u8>(src) };
        let dst = unsafe { core::mem::transmute::<*mut u32, *mut u8>(dst) };

        unsafe { alloc::alloc::dealloc(src, Layout::from_size_align_unchecked(size, 16)); }
        unsafe { alloc::alloc::dealloc(dst, Layout::from_size_align_unchecked(size, 16)); }

        let output = format!(
        "size: {} bytes
iterations: {} 
cpu: {} microseconds
kernel: {} microseconds
dmac: {} microseconds
vfpu: {} microseconds\n\n",
        size, iterations, cpu_dur.as_micros(), 
        kernel_dur.as_micros(), dmac_dur.as_micros(), 
        vfpu_dur.as_micros()
        );
        write_to_fd(fd, output);
    }
    unsafe { psp::sys::sceIoClose(fd) };
}

fn write_to_fd(fd: SceUid, msg: alloc::string::String) {

    unsafe {
        psp::sys::sceIoWrite(
            fd,
            msg.as_str().as_bytes().as_ptr() as *const u8 as *const c_void,
            msg.len()
        ) 
    };
}

extern "C" {
    fn memcpy(dst: *mut u8, src: *const u8, num: usize) -> *mut u8;
}
