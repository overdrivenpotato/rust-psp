psp_extern! {
    #![name = "sceVideocodec"]
    #![flags = 0x4001]
    #![version = (0x00, 0x11)]

    #[psp(0xC01EC829)]
    pub unsafe fn sceVideocodecOpen(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0x2D31F5B1)]
    pub unsafe fn sceVideocodecGetEDRAM(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0x17099F0A)]
    pub unsafe fn sceVideocodecInit(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0xDBA273FA)]
    pub unsafe fn sceVideocodecDecode(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0x4F160BF4)]
    pub unsafe fn sceVideocodecReleaseEDRAM(buffer: *mut u32) -> i32;
}
