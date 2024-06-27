psp_extern! {
    #![name = "sceDmac"]
    #![flags = 0x4001]
    #![version = (0x00, 0x11)]

    #[psp(0x617F3FE6)]
    pub fn sceDmacMemcpy(dst: *mut u32, src: *const u32, size: usize) -> i32;
}
