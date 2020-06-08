#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpenPSID {
    pub data: [u8; 16usize],
}

psp_extern! {
    #![name = "sceOpenPSID"]
    #![flags = 0x4001]
    #![version = (0x00, 0x11)]

    #[psp(0xC69BEBCE)]
    pub fn sceOpenPSIDGetOpenPSID(openpsid: *mut OpenPSID) -> i32;
}
