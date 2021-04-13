use crate::sys::TexturePixelFormat;
use crate::sys::{sceGeEdramGetAddr, sceGeEdramGetSize};
use core::mem::size_of;
use core::ptr::null_mut;

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

pub struct VramMemChunk {
    start: u32,
    len: u32,
}

impl VramMemChunk {
    fn new(start: u32, len: u32) -> Self {
        Self { start, len }
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
// TODO: pin?
#[derive(Debug)]
pub struct SimpleVramAllocator {
    offset: u32,
}

impl SimpleVramAllocator {
    const fn new() -> Self {
        Self { offset: 0 }
    }

    // TODO: return a Result instead of panicking
    pub fn alloc(&mut self, size: u32) -> VramMemChunk {
        // Align to 16 bytes
        let offset = (self.offset + 15) & !15;
        self.offset = offset + size;

        if self.offset > self.total_mem() {
            panic!("Total VRAM size exceeded!");
        }

        VramMemChunk::new(offset, size)
    }

    pub fn alloc_sized<T: Sized>(&mut self, count: u32) -> VramMemChunk {
        let size = size_of::<T>() as u32;
        self.alloc(count * size)
    }

    pub fn alloc_texture_pixels(
        &mut self,
        width: u32,
        height: u32,
        psm: TexturePixelFormat,
    ) -> VramMemChunk {
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
