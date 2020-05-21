sys_lib! {
    #![name = "sceVideocodec"]
    #![flags = 0x4001]
    #![version = (0x00, 0x11)]

    #[psp(0xC01EC829)]
    pub unsafe fn sce_videocodec_open(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0x2D31F5B1)]
    pub unsafe fn sce_videocodec_get_edram(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0x17099F0A)]
    pub unsafe fn sce_videocodec_init(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0xDBA273FA)]
    pub unsafe fn sce_videocodec_decode(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;

    #[psp(0x4F160BF4)]
    pub unsafe fn sce_videocodec_release_edram(_buffer: *mut u32)
        -> i32;

}
