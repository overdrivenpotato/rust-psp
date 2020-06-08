use core::ffi::c_void;

psp_extern! {
    #![name = "sceNand_driver"]
    #![flags = 0x0001]
    #![version = (0x00, 0x00)]

    #[psp(0x84EE5D76)]
    pub unsafe fn sce_nand_set_write_protect(protect_flag: i32) -> i32;

    #[psp(0xAE4438C7)]
    pub unsafe fn sce_nand_lock(write_flag: i32) -> i32;

    #[psp(0x41FFA822)]
    pub unsafe fn sce_nand_unlock();

    #[psp(0xE41A11DE)]
    pub unsafe fn sce_nand_read_status() -> i32;

    #[psp(0x7AF7B77A)]
    pub unsafe fn sce_nand_reset(flag: i32) -> i32;

    #[psp(0xFCDF7610)]
    pub unsafe fn sce_nand_read_id(buf: *mut c_void, size: usize) -> i32;

    #[psp(0x89BDCA08)]
    pub unsafe fn sce_nand_read_pages(
        ppn: u32,
        buf: *mut c_void,
        buf2: *mut c_void,
        count: u32,
    ) -> i32;

    #[psp(0xCE9843E6)]
    pub unsafe fn sce_nand_get_page_size() -> i32;

    #[psp(0xB07C41D4)]
    pub unsafe fn sce_nand_get_pages_per_block() -> i32;

    #[psp(0xC1376222)]
    pub unsafe fn sce_nand_get_total_blocks() -> i32;

    #[psp(0xC32EA051)]
    pub unsafe fn sce_nand_read_block_with_retry(
        ppn: u32,
        buf: *mut c_void,
        buf2: *mut c_void,
    ) -> i32;

    #[psp(0x01F09203)]
    pub unsafe fn sce_nand_is_bad_block(ppn: u32) -> i32;
}
