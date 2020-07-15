use crate::sys::TexturePixelFormat;
use crate::sys::{sceGeEdramGetAddr, sceGeEdramGetSize};
use core::mem::size_of;

type VramAllocator = SimpleVramAllocator;

static mut VRAM_ALLOCATOR: VramAllocatorSingleton = VramAllocatorSingleton {
    alloc: Some(VramAllocator::new()),
};

pub fn get_vram_allocator() -> Option<VramAllocator> {
    unsafe { VRAM_ALLOCATOR.get_vram_alloc() }
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

    pub fn as_mut_ptr(&self) -> *mut u8 {
        unsafe { vram_start_addr().offset(self.start as isize) }
    }

    pub fn len(&self) -> u32 {
        self.len
    }
}

// A dead-simple VRAM bump allocator.
// TODO: ensure 16-bit alignment?
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
        let old_offset = self.offset;
        self.offset += size;

        if self.offset > self.total_mem() {
            panic!("Total VRAM size exceeded!");
        }

        VramMemChunk::new(old_offset, size)
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
    pub fn move_to_vram<T: Sized>(&mut self, obj: T) -> &mut T {
        unsafe {
            let chunk = self.alloc_sized::<T>(1);
            let ptr = chunk.as_mut_ptr() as *mut T;
            ptr.write(obj);
            ptr.as_mut().unwrap()
        }
    }

    fn total_mem(&self) -> u32 {
        total_vram_size()
    }
}

fn total_vram_size() -> u32 {
    unsafe { sceGeEdramGetSize() }
}

fn vram_start_addr() -> *mut u8 {
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
