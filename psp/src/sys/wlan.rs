psp_extern! {
    #![name = "sceWlanDrv"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x93440B11)]
    /// Determine if the wlan device is currently powered on
    ///
    /// # Return Value
    ///
    /// 0 if off, 1 if on
    pub fn sceWlanDevIsPowerOn() -> i32;

    #[psp(0xD7763699)]
    /// Determine the state of the Wlan power switch
    ///
    /// # Return Value
    ///
    /// 0 if off, 1 if on
    pub fn sceWlanGetSwitchState() -> i32;

    #[psp(0x0C622081)]
    /// Get the Ethernet Address of the wlan controller
    ///
    /// # Parameters
    ///
    /// - `ether_addr`: pointer to an output buffer of u8 (NOTE: it only writes
    ///   to 6 bytes, but requests 8 so pass it 8 bytes just in case)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceWlanGetEtherAddr(ether_addr: *mut u8) -> i32;
}

psp_extern! {
    #![name = "sceWlanDrv_lib"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x482CAE9A)]
    /// Attach to the wlan device
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceWlanDevAttach() -> i32;

    #[psp(0xC9A8CAB7)]
    /// Detach from the wlan device
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceWlanDevDetach() -> i32;
}
