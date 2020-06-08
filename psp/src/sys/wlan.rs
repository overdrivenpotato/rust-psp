psp_extern! {
    #![name = "sceWlanDrv"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x93440B11)]
    /// Determine if the WLAN device is currently powered on
    ///
    /// # Return Value
    ///
    /// Returns 0 if off, 1 if on
    pub fn sceWlanDevIsPowerOn() -> i32;

    #[psp(0xD7763699)]
    /// Determine the state of the WLAN power switch
    ///
    /// # Return Value
    ///
    /// Return 0 if off, 1 if on
    pub fn sceWlanGetSwitchState() -> i32;

    #[psp(0x0C622081)]
    /// Get the ethernet address of the wlan controller
    ///
    /// # Parameters
    ///
    /// - `ether_addr`: Pointer to a buffer to write the output into (must be at
    ///    least 8 bytes)
    ///
    /// # Return Value
    ///
    /// Return 0 on success, < 0 on error
    pub fn sceWlanGetEtherAddr(ether_addr: *mut u8) -> i32;
}

psp_extern! {
    #![name = "sceWlanDrv_lib"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x482CAE9A)]
    /// Attach to the WLAN device
    ///
    /// # Return Value
    ///
    /// Returns 0 on success, < 0 on error
    pub fn sceWlanDevAttach() -> i32;

    #[psp(0xC9A8CAB7)]
    /// Detach to the WLAN device
    ///
    /// # Return Value
    ///
    /// Returns 0 on success, < 0 on error
    pub fn sceWlanDevDetach() -> i32;
}
