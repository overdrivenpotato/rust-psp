use crate::sys::{self, SceSysMemBlockTypes, SceSysMemPartitionId, SceUid};
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};

/// An allocator that hooks directly into the PSP OS memory allocator.
struct SystemAlloc;

unsafe impl GlobalAlloc for SystemAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size()
            // We need to store the memory block ID.
            + mem::size_of::<SceUid>()

            // We also store padding bytes, in case the block returned from the
            // system is not aligned. The count of padding bytes is also stored
            // here, in the last byte.
            + layout.align();

        // crate::debug::print_num(size);

        let id = sys::sceKernelAllocPartitionMemory(
            SceSysMemPartitionId::SceKernelPrimaryUserPartition,
            &b"block\0"[0],
            SceSysMemBlockTypes::Low,
            size as u32,
            ptr::null_mut(),
        );

        // TODO: Error handling.
        let mut ptr: *mut u8 = sys::sceKernelGetBlockHeadAddr(id).cast();
        *ptr.cast() = id;

        ptr = ptr.add(mem::size_of::<SceUid>());

        // We must add at least one, to store this value.
        let align_padding = 1 + ptr.add(1).align_offset(layout.align());
        *ptr.add(align_padding - 1) = align_padding as u8;
        ptr.add(align_padding)
    }

    #[inline(never)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let align_padding = *ptr.sub(1);

        let id = *ptr.sub(align_padding as usize).cast::<SceUid>().offset(-1);

        // TODO: Error handling.
        sys::sceKernelFreePartitionMemory(id);
    }
}

#[global_allocator]
static ALLOC: SystemAlloc = SystemAlloc;

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn aeh(_: Layout) -> ! {
    loop {
        core::hint::spin_loop()
    }
}

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern "C" fn memset(ptr: *mut u8, value: u32, num: usize) -> *mut u8 {
    let mut i = 0;

    while i < num {
        *((ptr as usize + i) as *mut u8) = value as u8;
        i += 1;
    }

    ptr
}

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, num: isize) -> *mut u8 {
    let mut i = 0;

    while i < num {
        *((dst as isize + i) as *mut u8) = *((src as isize + i) as *mut u8);
        i += 1;
    }

    dst
}

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern "C" fn memcmp(ptr1: *mut u8, ptr2: *mut u8, num: usize) -> i32 {
    let mut i = 0;

    while i < num {
        let val1 = *((ptr1 as usize + i) as *mut u8);
        let val2 = *((ptr2 as usize + i) as *mut u8);
        let diff = val1 as i32 - val2 as i32;

        if diff != 0 {
            return diff;
        }

        i += 1;
    }

    0
}

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern "C" fn memmove(dst: *mut u8, src: *mut u8, num: isize) -> *mut u8 {
    if dst < src {
        let mut i = 0;

        while i < num {
            *((dst as isize + i) as *mut u8) = *((src as isize + i) as *mut u8);
            i += 1;
        }
    } else {
        let mut i = num - 1;

        while i >= 0 {
            *((dst as isize + i) as *mut u8) = *((src as isize + i) as *mut u8);
            i -= 1;
        }
    }

    dst
}

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern "C" fn strlen(s: *mut u8) -> usize {
    let mut len = 0;

    while *s.add(len) != 0 {
        len += 1;
    }

    len
}
