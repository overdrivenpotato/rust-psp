use core::ffi::c_void;

psp_extern! {
    #![name = "sceNand_driver"]
    #![flags = 0x0001]
    #![version = (0x00, 0x00)]

    #[psp(0x84EE5D76)]
    pub fn sceNandSetWriteProtect(protect_flag: i32) -> i32;

    #[psp(0xAE4438C7)]
    pub fn sceNandLock(write_flag: i32) -> i32;

    #[psp(0x41FFA822)]
    pub fn sceNandUnlock();

    #[psp(0xE41A11DE)]
    pub fn sceNandReadStatus() -> i32;

    #[psp(0x7AF7B77A)]
    pub fn sceNandReset(flag: i32) -> i32;

    #[psp(0xFCDF7610)]
    pub fn sceNandReadId(buf: *mut c_void, size: usize) -> i32;

    #[psp(0x89BDCA08)]
    pub fn sceNandReadPages(
        ppn: u32,
        buf: *mut c_void,
        buf2: *mut c_void,
        count: u32,
    ) -> i32;

    #[psp(0xCE9843E6)]
    pub fn sceNandGetPageSize() -> i32;

    #[psp(0xB07C41D4)]
    pub fn sceNandGetPagesPerBlock() -> i32;

    #[psp(0xC1376222)]
    pub fn sceNandGetTotalBlocks() -> i32;

    #[psp(0xC32EA051)]
    pub fn sceNandReadBlockWithRetry(
        ppn: u32,
        buf: *mut c_void,
        buf2: *mut c_void,
    ) -> i32;

    #[psp(0x01F09203)]
    pub fn sceNandIsBadBlock(ppn: u32) -> i32;
}
