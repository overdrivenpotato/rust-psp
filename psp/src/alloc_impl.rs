use alloc::alloc::{Layout, GlobalAlloc};
use core::{ptr, mem};
use crate::sys::{self, SceUid, SceSysMemPartitionId, SceSysMemBlockTypes};

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

        let id = *ptr
            .sub(align_padding as usize)
            .cast::<SceUid>().offset(-1);

        // TODO: Error handling.
        sys::sceKernelFreePartitionMemory(id);
    }
}

#[global_allocator]
static ALLOC: SystemAlloc = SystemAlloc;

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn aeh(_: Layout) -> ! { loop {} }

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern fn memset(ptr: *mut u8, value: u32, num: usize) -> *mut u8 {
    crate::sys::sceKernelMemset(ptr, value as u8, num)
}

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern fn memcpy(dst: *mut u8, src: *const u8, num: usize) -> *mut u8 {
    if num < 12_000 {
        crate::sys::sceKernelMemcpy(dst, src, num)
    } else {
        crate::sys::sceDmacMemcpy(dst, src, num); 
        dst
    }
}


#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern fn memcmp(ptr1: *mut u8, ptr2: *mut u8, num: isize) -> i32 {
    for i in 0..num {
        let diff = ptr1.offset(i) as i32 - ptr2.offset(i) as i32;

        if diff != 0 {
            return diff;
        }
    }

    0
}

#[no_mangle]
#[cfg(not(feature = "stub-only"))]
unsafe extern fn memmove(dst: *mut u8, src: *mut u8, num: isize) -> *mut u8 {
    if dst < src {
        let mut size = num as usize;
        let mut dst32 = dst as *mut u32;
        let mut src32 = src as *mut u32;
        while size > 3 {
            *dst32 = *src32;
            dst32 = dst32.add(1);
            src32 = src32.add(1);
            size = size.saturating_sub(4);
        }
        let mut dst_new = dst32 as *mut u8;
        let mut src_new = src32 as *mut u8;
        while size > 0 {
            *dst_new = *src_new;
            dst_new = dst_new.add(1);
            src_new = src_new.add(1);
            size = size.saturating_sub(1);
        }
    } else {
        let mut size = num as u32;
        let mut dst32 = (dst as u32 + size -1) as *mut u32;
        let mut src32 = (src as u32 + size -1) as *mut u32;
        while size > 3 {
            *dst32 = *src32;
            dst32 = dst32.sub(1);
            src32 = src32.sub(1);
            size = size.saturating_sub(4);
        }
        let mut dst_new = dst32 as *mut u8;
        let mut src_new = src32 as *mut u8;
        while size > 0 {
            *dst_new = *src_new;
            dst_new = dst_new.sub(1);
            src_new = src_new.sub(1);
            size = size.saturating_sub(1);
        }
    }
    dst
}
