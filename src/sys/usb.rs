use core::ffi::c_void;

pub const USBBUS_DRIVERNAME: &str = "USBBusDriver";

sys_lib! {
    #![name = "sceUsb"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0xAE5DE6AF)]
    pub unsafe fn sce_usb_start(driver_name: *const u8, size: u32, args: *const c_void);
}


