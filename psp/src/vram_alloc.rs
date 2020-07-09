use crate::sys::TexturePixelFormat;
use crate::sys::sceGeEdramGetSize;
use core::mem::size_of;

pub struct VramMemChunk {
    start: u32,
    len: u32,
}

impl VramMemChunk {
    fn new(start: u32, len: u32) -> Self {
        Self { start, len }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn len(&self) -> u32 {
        self.len
    }
}

// A dead-simple VRAM bump allocator.
pub struct SimpleVramAllocator {
   offset: u32,
}

impl SimpleVramAllocator {
    pub fn new() -> Self {
        Self { offset: 0 }
    }

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

    fn total_mem(&self) -> u32 {
        unsafe {
            sceGeEdramGetSize()
        }
    }
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
