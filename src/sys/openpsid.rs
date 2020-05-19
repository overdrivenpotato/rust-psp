#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpenPSID {
    pub data: [u8; 16usize],
}

sys_lib! {
    #![name = "sceOpenPSID"]
    #![flags = 0x4001]
    #![version = (0x00, 0x11)]

    #[psp(0xC69BEBCE)]
    pub unsafe fn sce_open_psidget_open_psid(openpsid: *mut OpenPSID) -> i32;
}
