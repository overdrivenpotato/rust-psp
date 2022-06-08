use crate::sys::TexturePixelFormat;
use crate::sys::{sceGeEdramGetAddr, sceGeEdramGetSize};
use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU32, Ordering};

type VramAllocator = SimpleVramAllocator;

#[derive(Copy, Clone, Debug)]
pub struct VramAllocatorInUseError;

impl core::fmt::Display for VramAllocatorInUseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ownership of `VramAllocator` already taken")
    }
}

mod vram_allocator_singleton {
    use super::{SimpleVramAllocator, VramAllocator, VramAllocatorInUseError};
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

    static VRAM_ALLOCATOR_IS_TAKEN: AtomicBool = AtomicBool::new(false);

    impl VramAllocator {
        pub fn take() -> Result<VramAllocator, VramAllocatorInUseError> {
            if !VRAM_ALLOCATOR_IS_TAKEN.swap(true, Ordering::Relaxed) {
                // new and empty, since old one droped and cannot have any references to it
                Ok(VramAllocator {
                    offset: AtomicU32::new(0),
                })
            } else {
                Err(VramAllocatorInUseError)
            }
        }
    }

    impl Drop for SimpleVramAllocator {
        fn drop(&mut self) {
            VRAM_ALLOCATOR_IS_TAKEN.store(false, Ordering::Relaxed)
        }
    }
}

pub struct VramMemChunk<'a> {
    start: u32,
    len: u32,
    // Needed since VramMemChunk has a lifetime, but doesn't contain references
    vram: PhantomData<&'a mut ()>,
}

impl VramMemChunk<'_> {
    fn new_(start: u32, len: u32) -> Self {
        Self {
            start,
            len,
            vram: PhantomData,
        }
    }

    pub fn as_mut_ptr_from_zero(&self) -> *mut u8 {
        unsafe { vram_start_addr_zero().add(self.start as usize) }
    }

    pub fn as_mut_ptr_direct_to_vram(&self) -> *mut u8 {
        unsafe { vram_start_addr_direct().add(self.start as usize) }
    }

    pub fn len(&self) -> u32 {
        self.len
    }
}

// A dead-simple VRAM bump allocator.
// There could be only one value of this type
// WARNING: should be instantiated only within [`vram_allocator_singleton`] private module
// TODO: remove Debug
#[derive(Debug)]
pub struct SimpleVramAllocator {
    offset: AtomicU32,
}

#[derive(Copy, Clone, Debug)]
pub struct VramAllocError;

impl core::fmt::Display for VramAllocError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Total VRAM size exceeded")
    }
}

impl SimpleVramAllocator {
    /// Frees all previously allocated VRAM chunks.
    ///
    /// This resets the allocator's counter, but does not change the contents of
    /// VRAM. Since this method requires `&mut Self` and there are no other
    /// `SimpleVramAllocator` values to be swapped with, it cannot overlap with any
    /// previously allocated `VramMemChunk`s since they have the lifetime of the
    /// `&Self` that allocated them.
    pub fn free_all(&mut self) {
        // Store is required to occure after all previous bumps and before any next bumps, so SeqCst
        self.offset.store(0, Ordering::SeqCst);
    }

    // TODO: handle alignment
    /// Allocates `size` bytes of VRAM
    ///
    /// The returned VRAM chunk has the same lifetime as the
    /// `SimpleVramAllocator` borrow (i.e. `&self`) that allocated it.
    pub fn alloc<'a>(&'a self, size: u32) -> Result<VramMemChunk<'a>, VramAllocError> {
        // Atomically bump offset, no ordering required
        let old_offset = self
            .offset
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
                old.checked_add(size).filter(|new| *new <= self.total_mem())
            })
            .map_err(|_| VramAllocError)?;

        Ok(VramMemChunk::new_(old_offset, size))
    }

    // TODO: ensure 16-bit alignment?
    pub fn alloc_sized<'a, T: Sized>(
        &'a self,
        count: u32,
    ) -> Result<VramMemChunk<'a>, VramAllocError> {
        let size = size_of::<T>() as u32;
        self.alloc(count * size)
    }

    pub fn alloc_texture_pixels<'a>(
        &'a self,
        width: u32,
        height: u32,
        psm: TexturePixelFormat,
    ) -> Result<VramMemChunk<'a>, VramAllocError> {
        let size = get_memory_size(width, height, psm);
        self.alloc(size)
    }

    // TODO: write, or write_volatile?
    // TODO: panic instead of result?
    // TODO: Keep track of the allocated chunk
    // TODO: determine unsafety of this
    pub unsafe fn move_to_vram<T: Sized>(&mut self, obj: T) -> Result<&mut T, VramAllocError> {
        let chunk = self.alloc_sized::<T>(1)?;
        let ptr = chunk.as_mut_ptr_direct_to_vram() as *mut T;
        ptr.write(obj);
        Ok(&mut *ptr)
    }

    fn total_mem(&self) -> u32 {
        total_vram_size()
    }
}

fn total_vram_size() -> u32 {
    unsafe { sceGeEdramGetSize() }
}

// NOTE: VRAM actually starts at 0x4000000, as returned by sceGeEdramGetAddr.
//       The Gu functions take that into account, and start their pointer
//       indices at 0. See GE_EDRAM_ADDRESS in gu.rs for that offset being used.
fn vram_start_addr_zero() -> *mut u8 {
    null_mut()
}

fn vram_start_addr_direct() -> *mut u8 {
    unsafe { sceGeEdramGetAddr() }
}

fn get_memory_size(width: u32, height: u32, psm: TexturePixelFormat) -> u32 {
    match psm {
        TexturePixelFormat::PsmT4 => (width * height) >> 1,
        TexturePixelFormat::PsmT8 => width * height,

        TexturePixelFormat::Psm5650
        | TexturePixelFormat::Psm5551
        | TexturePixelFormat::Psm4444
        | TexturePixelFormat::PsmT16 => 2 * width * height,

        TexturePixelFormat::Psm8888 | TexturePixelFormat::PsmT32 => 4 * width * height,

        _ => unimplemented!(),
    }
}
