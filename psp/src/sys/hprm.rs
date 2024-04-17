//! Headphone Remote

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct HprmKey: u32 {
        const PLAY_PAUSE  = 0x1;
        const FORWARD     = 0x4;
        const BACK        = 0x8;
        const VOL_UP      = 0x10;
        const VOL_DOWN    = 0x20;
        const HOLD        = 0x80;
    }
}

psp_extern! {
    #![name = "sceHprm"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x1910B327)]
    /// Peek at the current being pressed on the remote.
    ///
    /// # Parameters
    ///
    /// - `key`: Pointer to receive the key bitmap, should be an instance of ::Key
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sceHprmPeekCurrentKey(key: *mut HprmKey) -> i32;

    #[psp(0x2BCEC83E)]
    /// Peek at the current latch data.
    ///
    /// # Parameters
    ///
    /// - `latch`: Pointer a to a 4 dword array to contain the latch data.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceHprmPeekLatch(latch: *mut [u32;4]) -> i32;

    #[psp(0x40D2F9F0)]
    /// Read the current latch data.
    ///
    /// # Parameters
    ///
    /// - `latch`: Pointer a to a 4 dword array to contain the latch data.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceHprmReadLatch(latch: *mut [u32;4]) -> i32;

    #[psp(0x7E69EDA4)]
    /// Determines whether the headphones are plugged in.
    ///
    /// # Return Value
    ///
    /// 1 if the headphones are plugged in, else 0.
    pub fn sceHprmIsHeadphoneExist() -> i32;

    #[psp(0x208DB1BD)]
    /// Determines whether the remote is plugged in.
    ///
    /// # Return Value
    ///
    /// 1 if the remote is plugged in, else 0.
    pub fn sceHprmIsRemoteExist() -> i32;

    #[psp(0x219C58F1)]
    /// Determines whether the microphone is plugged in.
    ///
    /// # Return Value
    ///
    /// 1 if the microphone is plugged in, else 0.
    pub fn sceHprmIsMicrophoneExist() -> i32;
}
