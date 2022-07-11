use crate::sys::TexturePixelFormat;
use crate::sys::{sceGeEdramGetAddr, sceGeEdramGetSize};
use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU32, Ordering};

type VramAllocator = SimpleVramAllocator;

#[derive(Debug)]
pub struct VramAllocatorInUseError {}

static mut VRAM_ALLOCATOR: VramAllocatorSingleton = VramAllocatorSingleton {
    alloc: Some(VramAllocator::new()),
};

pub fn get_vram_allocator() -> Result<VramAllocator, VramAllocatorInUseError> {
    let opt_alloc = unsafe { VRAM_ALLOCATOR.get_vram_alloc() };
    opt_alloc.ok_or(VramAllocatorInUseError {})
}

pub struct VramAllocatorSingleton {
    alloc: Option<VramAllocator>,
}

impl VramAllocatorSingleton {
    pub fn get_vram_alloc(&mut self) -> Option<VramAllocator> {
        self.alloc.take()
    }
}

pub struct VramMemChunk<'a> {
    start: u32,
    len: u32,
    // Needed since VramMemChunk has a lifetime, but doesn't contain references
    vram: PhantomData<&'a mut ()>,
}

impl VramMemChunk<'_> {
    fn new(start: u32, len: u32) -> Self {
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

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// A dead-simple VRAM bump allocator.
// TODO: pin?
#[derive(Debug)]
pub struct SimpleVramAllocator {
    offset: AtomicU32,
}

impl SimpleVramAllocator {
    const fn new() -> Self {
        Self {
            offset: AtomicU32::new(0),
        }
    }

    /// Frees all previously allocated VRAM chunks.
    ///
    /// This resets the allocator's counter, but does not change the contents of
    /// VRAM. Since this method requires `&mut Self`, it cannot overlap with any
    /// previously allocated `VramMemChunk`s since they have the lifetime of the
    /// `&Self` that allocated them.
    pub fn free_all(&mut self) {
        self.offset.store(0, Ordering::Relaxed);
    }

    // TODO: return a Result instead of panicking
    /// Allocates `size` bytes of VRAM
    ///
    /// The returned VRAM chunk has the same lifetime as the
    /// `SimpleVramAllocator` borrow (i.e. `&self`) that allocated it.
    pub fn alloc(&self, size: u32) -> VramMemChunk<'_> {
        let old_offset = self.offset.load(Ordering::Relaxed);
        let new_offset = old_offset + size;
        self.offset.store(new_offset, Ordering::Relaxed);

        if new_offset > self.total_mem() {
            panic!("Total VRAM size exceeded!");
        }

        VramMemChunk::new(old_offset, size)
    }

    // TODO: ensure 16-bit alignment?
    pub fn alloc_sized<T: Sized>(&self, count: u32) -> VramMemChunk<'_> {
        let size = size_of::<T>() as u32;
        self.alloc(count * size)
    }

    pub fn alloc_texture_pixels(
        &self,
        width: u32,
        height: u32,
        psm: TexturePixelFormat,
    ) -> VramMemChunk<'_> {
        let size = get_memory_size(width, height, psm);
        self.alloc(size)
    }

    // TODO: write, or write_volatile?
    // TODO: result instead of unwrap?
    // TODO: Keep track of the allocated chunk
    // TODO: determine unsafety of this
    pub unsafe fn move_to_vram<T: Sized>(&mut self, obj: T) -> &mut T {
        let chunk = self.alloc_sized::<T>(1);
        let ptr = chunk.as_mut_ptr_direct_to_vram() as *mut T;
        ptr.write(obj);
        ptr.as_mut().unwrap()
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
