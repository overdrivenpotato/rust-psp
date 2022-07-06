use core::sync::atomic::AtomicPtr;
use core::{alloc, mem, ptr};

use crate::sys::TexturePixelFormat;
use crate::sys::{sceGeEdramGetAddr, sceGeEdramGetSize};

pub type VramAllocator = SimpleVramAllocator;

#[derive(Copy, Clone, Debug)]
pub struct VramAllocatorInUseError;

impl core::fmt::Display for VramAllocatorInUseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ownership of `VramAllocator` already taken")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VramAllocatorInUseError {}

mod vram_allocator_singleton {
    use super::{
        total_vram_size, vram_base_ptr, SimpleVramAllocator, VramAllocError,
        VramAllocatorInUseError, VramBox,
    };
    use core::{
        alloc,
        ptr::{self, NonNull},
        sync::atomic::{self, AtomicBool, AtomicPtr, AtomicUsize},
    };

    /// Indicates VRAM ownership is free for anyone to acquire
    static VRAM_OWNERSHIP: AtomicBool = AtomicBool::new(true);

    impl SimpleVramAllocator {
        /// # Safety
        ///
        /// No other code should assume it can borrow or access VRAM.
        /// Allocator assumes ownership over VRAM (or at least
        /// over memory allocated by it).
        pub unsafe fn take() -> Result<Self, VramAllocatorInUseError> {
            let direct_base = vram_base_ptr();
            VRAM_OWNERSHIP
                .fetch_update(atomic::Ordering::Acquire, atomic::Ordering::Relaxed, |t| {
                    (!t).then(|| false)
                })
                .map_err(|_| VramAllocatorInUseError)?;

            // fresh new allocator, there should be no active allocations
            // to vram at this point
            Ok(SimpleVramAllocator {
                vram_direct_base: direct_base,
                cursor: direct_base,
                vram_size: total_vram_size(),
            })
        }

        /// Frees all previously allocated VRAM.
        pub fn reset(&mut self) {
            *self.cursor.get_mut() = self.vram_direct_base.get();
        }

        // TODO: handle alignment
        /// Allocates `size` bytes TODO
        ///
        /// The returned VRAM chunk has the same lifetime as the
        /// `SimpleVramAllocator` borrow (i.e. `&self`) that allocated it.
        pub fn alloc_box<T>(&self, value: T) -> Result<VramBox<'_, T>, VramAllocError> {
            // SAFETY: Creating a (unique) mutable VRAM byte slice, see
            //         [`VramAllocator::take`] safety section.
            Ok(VramBox {
                inner: unsafe {
                    core::slice::from_raw_parts_mut(vram_base_ptr().add(old_offset), size)
                },
            })
        }

        pub fn alloc_boxed_slice<I>(
            &self,
            iter: I,
        ) -> Result<VramBox<'_, [I::Item]>, VramAllocError>
        where
            I: Iterator + ExactSizeIterator,
        {
            self.alloc(alloc::Layout::new::<I::Item>());
        }

        /// # Safety
        ///
        /// Returned memory blocks point to valid memory and retain
        /// their validity until the allocator is dropped or
        /// [`SimpleVramAllocator::free_all`] is called.
        pub fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<u8>, VramAllocError> {
            // Atomically bump a cursor, no order required
            let mut ptr = NonNull::dangling();

            // ZSTs get dangling pointers
            if layout.size() == 0 {
                return Ok(ptr);
            }

            let old = self
                .cursor
                .fetch_update(
                    atomic::Ordering::Relaxed,
                    atomic::Ordering::Relaxed,
                    |mut old| {
                        let offset = old.align_offset(layout.align());

                        // SAFETY: `cursor` and `vram_direct_base` fields
                        //         come from the same place pointer
                        let spare_capacity = unsafe {
                            self.vram_direct_base
                                .as_ptr()
                                .add(self.vram_size)
                                .offset_from(old) as usize
                        };

                        offset
                            <= (spare_capacity as usize)
                                .checked_sub(layout.size())?
                                .then(|| {
                                    // SAFETY: performed in-bounds check above
                                    ptr = unsafe { old.add(offset) };
                                    ptr.add(layout.size())
                                })
                    },
                )
                .map_err(|_| VramAllocError)?;

            Ok(ptr)
        }
    }

    impl Drop for SimpleVramAllocator {
        fn drop(&mut self) {
            // all mem chunks at this point are no longer used, releasing
            // vram ownership
            VRAM_OWNERSHIP.store(true, atomic::Ordering::Release);
        }
    }
}

// A dead-simple VRAM bump allocator.
// There could be only one instance of this type
// WARNING: should be instantiated only within [`vram_allocator_singleton`]
//          private module
pub struct SimpleVramAllocator {
    vram_direct_base: ptr::NonNull<u8>,
    cursor: AtomicPtr<u8>,
    vram_size: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct VramAllocError;

impl core::fmt::Display for VramAllocError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Total VRAM size exceeded")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VramAllocError {}

impl SimpleVramAllocator {
    pub fn alloc_texture_pixels(
        &self,
        width: u32,
        height: u32,
        psm: TexturePixelFormat,
    ) -> Result<VramBox<'_>, VramAllocError> {
        let size = get_memory_size(width, height, psm);
        self.alloc(size)
    }

    /// Total capacity of `SimpleVramAllocator`, equals to total VRAM
    fn capacity(&self) -> usize {
        total_vram_size()
    }
}

pub struct VramBox<'a, T: ?Sized> {
    inner: &'a mut mem::ManuallyDrop<T>,
}

impl<T: ?Sized> Drop for VramBox<'_, T> {
    fn drop(&mut self) {
        // SAFETY: `VramBox` assumes ownership over the inner value.
        //         Dropped value isn't accessable to anyone after, because
        //         it's in the `Drop::drop` implementation.
        unsafe { mem::ManuallyDrop::drop(self.inner) }
    }
}

impl<'a, T: ?Sized> VramBox<'a, T> {
    fn into_inner(boxed: VramBox<'a, T>) -> T {
        // SAFETY: `VramBox` assumes ownership over the inner value.
        //         `boxed` isn't accessable to anyone after, because
        //         function call owns it and then forgets it to avoid a
        //         double free.
        unsafe { mem::ManuallyDrop::take(boxed.inner) };
        mem::forget(boxed);
    }
}

impl<T: ?Sized> core::ops::Deref for VramBox<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<T: ?Sized> core::ops::DerefMut for VramBox<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

// See GE_EDRAM_ADDRESS in src/sys/gu.rs for that offset being used.
//
/// Converion trait for vram pointers.
///
/// **Direct** VRAM addresses start at 0x4000000, as returned by
/// sceGeEdramGetAddr. You can work with direct VRAM like it's regular
/// memory.
///
/// On the other hand, **zero-based** VRAM adresses start at 0x0 instead.
/// These pointers are used with `sys::sceGu*` functions, but outside of
/// that you cannot do anything until you convert them back to a direct
/// VRAM pointer.
///
/// # Non-volatile access to direct VRAM
///
/// Writes to and reads from VRAM are not required to be volatile.
///
/// Volatile reads and writes are guaranteed to not be reordered,
/// combined, or eliminated. This stands from requirement for memory
/// mapped io, as there can be side effects before individual reads or
/// after individual writes.
///
/// Provenance of VRAM's direct pointer comes from `sceGeEdramGetAddr`,
/// which is actually a stub function generated via `global_asm!` macro,
/// and because of this the compiler cannot infer pointer proviance any
/// further. It is assumed that the proviance of direct VRAM pointers
/// may be accessable to any unreachable to the compiler code. This
/// means the compiler is obligated to actually perform write before
/// any call to undefined (or stub) function.
pub trait VramConvPtr {
    fn vram_direct_into_zero_based(self) -> Self;

    fn vram_zero_based_into_direct(self) -> Self;
}

impl<T> VramConvPtr for *mut T {
    fn vram_direct_into_zero_based(self) -> Self {
        self.cast::<u8>()
            .wrapping_sub(vram_base_ptr() as usize)
            .cast::<T>()
    }

    fn vram_zero_based_into_direct(self) -> Self {
        self.cast::<u8>()
            .wrapping_add(vram_base_ptr() as usize)
            .cast::<T>()
    }
}

impl<T> VramConvPtr for *const T {
    fn vram_direct_into_zero_based(self) -> Self {
        self.cast::<u8>()
            .wrapping_sub(vram_base_ptr() as usize)
            .cast::<T>()
    }

    fn vram_zero_based_into_direct(self) -> Self {
        self.cast::<u8>()
            .wrapping_add(vram_base_ptr() as usize)
            .cast::<T>()
    }
}

/// A direct VRAM pointer
fn vram_base_ptr() -> ptr::NonNull<u8> {
    // We assume the returned pointer is not null, panic if we are wrong.
    // If you would like to eliminate (probably unused) code for this panic for this panic but this
    ptr::NonNull::new(unsafe { sceGeEdramGetAddr() })
        .expect("`sceGeEdramGetAddr` returned a null pointer")
}

fn total_vram_size() -> usize {
    unsafe { sceGeEdramGetSize() as usize }
}

// TODO: Add checks for width and height values, or mark as unsafe
fn get_memory_size(width: u32, height: u32, psm: TexturePixelFormat) -> usize {
    (match psm {
        // Want to eliminate modulo? Solve the todo!
        TexturePixelFormat::PsmT4 => (width * height) / 2 + (width * height) % 2,
        TexturePixelFormat::PsmT8 => width * height,

        TexturePixelFormat::Psm5650
        | TexturePixelFormat::Psm5551
        | TexturePixelFormat::Psm4444
        | TexturePixelFormat::PsmT16 => 2 * width * height,

        TexturePixelFormat::Psm8888 | TexturePixelFormat::PsmT32 => 4 * width * height,

        _ => unimplemented!(),
    }) as usize
}
