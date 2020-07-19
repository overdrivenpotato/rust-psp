use crate::sys::SceUid;

bitflags::bitflags! {
    /// Power callback flags
    #[repr(transparent)]
    pub struct PowerInfo: u32 {
        /// Indicates the power switch it pushed, putting the unit into suspend mode
        const POWER_SWITCH = 0x80000000;

        /// Indicates the hold switch is on
        const HOLD_SWITCH = 0x40000000;

        /// Indicates the PSP has gone to standby (screen off)
        const STANDBY = 0x00080000;

        /// Indicates the resume process is complete. (Only triggered when another even happens)
        const RESUME_COMPLETE = 0x00040000;

        /// Indicates the unit is resuming from suspend mode.
        const RESUMING = 0x00020000;

        /// Indicates the unit is suspending - occurs due to inactivity
        const SUSPENDING = 0x00010000;

        /// Indicates the unit is plugged into an AC outlet
        const AC_POWER = 0x00001000;

        /// Indicates the battery charge level is low
        const BATTERY_LOW = 0x00000100;

        /// Indicates there is a battery present
        const BATTERY_EXIST = 0x00000080;

        /// Indicates the unit is relying on a battery for power (instead of AC adapter)
        const BATTERY_POWER = 0x0000007;
    }
}

/// Power tick flags
#[repr(u32)]
pub enum PowerTick {
    /// All
    All = 0,
    /// Suspend
    Suspend = 1,
    /// Display
    Display = 6,
}

/// Power Callback Function Definition
///
/// # Parameters
///
/// `unknown`: Unknown function, appears to cycle between 1,2 and 3
/// `power_info`: Combination of `PowerInfo` flags
pub type PowerCallback = extern fn (unknown: i32, power_info: PowerInfo);

psp_extern! {
    #![name = "scePower"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x04B7766E)]
    /// Register Power Callback Function
    ///
    /// # Parameters
    ///
    /// - `slot`: slot of the callback in the list, 0 to 15, pass -1 to get an
    ///   auto assignment.
    /// - `cbid`: callback id from calling `sceKernelCreateCallback`
    ///
    /// # Return value
    ///
    /// Returns 0 on success, the slot number if -1 is passed, or < 0 on error.
    pub fn scePowerRegisterCallback(slot: i32, cbid: SceUid) -> i32;

    #[psp(0xDFA8BAF8)]
    /// Unregister Power Callback Function
    ///
    /// # Paramters
    ///
    /// - `slot`: slot of the callback
    ///
    /// # Return Value
    ///
    /// Return 0 on success, < 0 on error
    pub fn scePowerUnregisterCallback(slot: i32) -> i32;

    #[psp(0x87440F5E)]
    /// Check if unit is plugged in
    ///
    /// # Return Value
    ///
    /// Return 1 if plugged in, 0 if not plugged in, < 0 on error
    pub fn scePowerIsPowerOnline() -> i32;

    #[psp(0x0AFD0D8B)]
    /// Check if a battery is present
    ///
    /// # Return Value
    ///
    /// Return 1 if battery present, 0 if battery not present, < 0 on error
    pub fn scePowerIsBatteryExist() -> i32;

    #[psp(0x1E490401)]
    /// Check if the battery is charging
    ///
    /// # Return Value
    ///
    /// Return 1 if battery charging, 0 if battery not charging, < 0 on error
    pub fn scePowerIsBatteryCharging() -> i32;

    #[psp(0xB4432BC8)]
    /// Get the status of battery charging
    pub fn scePowerGetBatteryChargingStatus() -> i32;

    #[psp(0xD3075926)]
    /// Check if the battery is low
    ///
    /// # Return Value
    ///
    /// Return 1 if the battery is low, Return 0 if the battery is not low, < 0 on error
    pub fn scePowerIsLowBattery() -> i32;

    #[psp(0x2085D15D)]
    /// Get battery life as integer percent
    ///
    /// # Return Value
    ///
    /// Return battery charger as a percentage 0-100, < 0 on error
    pub fn scePowerGetBatteryLifePercent() -> i32;

    #[psp(0x8EFB3FA2)]
    /// Get battery life as time
    ///
    /// # Return Value
    ///
    /// Return battery life in minutes, < 0 on error
    pub fn scePowerGetBatteryLifeTime() -> i32;

    #[psp(0x28E12023)]
    /// Get temperature of battery
    pub fn scePowerGetBatteryTemp() -> i32;

    #[psp(0x862AE1A6)]
    /// Unknown - can crash
    pub fn scePowerGetBatteryElec() -> i32;

    #[psp(0x483CE86B)]
    /// Get battery volt level
    pub fn scePowerGetBatteryVolt() -> i32;

    #[psp(0x843FBF43)]
    /// Set CPU Frequency
    ///
    /// # Parameters
    ///
    /// - `cpufreq`: new CPU frequency from 1-333
    pub fn scePowerSetCpuClockFrequency(cpufreq: i32) -> i32;

    #[psp(0xB8D7B3FB)]
    /// Set CPU Frequency
    ///
    /// # Parameters
    ///
    /// - `cpufreq`: new CPU frequency from 1-333
    pub fn scePowerSetBusClockFrequency(busfreq: i32) -> i32;

    #[psp(0xFEE03A2F)]
    /// Get CPU Frequency
    ///
    /// # Return Value
    ///
    /// Returns cpu frequency as an integer
    pub fn scePowerGetCpuClockFrequency() -> i32;

    #[psp(0xFDB5BFE9)]
    /// Get CPU Frequency
    ///
    /// # Return Value
    ///
    /// Returns cpu frequency as an integer
    pub fn scePowerGetCpuClockFrequencyInt() -> i32;

    #[psp(0xB1A52C83)]
    /// Get CPU Frequency
    ///
    /// # Return Value
    ///
    /// Returns cpu frequency as a float
    pub fn scePowerGetCpuClockFrequencyFloat() -> f32;

    #[psp(0x478FE6F5)]
    /// Get bus Frequency
    ///
    /// # Return Value
    ///
    /// Returns bus frequency as an integer
    pub fn scePowerGetBusClockFrequency() -> i32;

    #[psp(0xBD681969)]
    /// Get bus Frequency
    ///
    /// # Return Value
    ///
    /// Returns bus frequency as an integer
    pub fn scePowerGetBusClockFrequencyInt() -> i32;

    #[psp(0x9BADB3EB)]
    /// Get bus Frequency
    ///
    /// # Return Value
    ///
    /// Returns bus frequency as a float
    pub fn scePowerGetBusClockFrequencyFloat() -> f32;

    #[psp(0x737486F2)]
    /// Set Clock Frequencies
    ///
    /// # Parameters
    ///
    /// - `pllfreq`: pll frequency from 19-333
    /// - `cpufreq`: cpu frequency from 1-333
    ///
    /// busfreq - bus frequency from 1-167
    ///
    /// Given:
    ///  cpufreq <= pllfreq
    ///  busfreq*2 <= pllfreq
    pub fn scePowerSetClockFrequency(pllfreq: i32, cpufreq: i32, busfreq: i32) -> i32;

    #[psp(0xD6D016EF)]
    /// Lock Power Switch
    ///
    /// Note: if the power switch is toggled while locked
    /// it will fire immediately after being unlocked
    ///
    /// # Parameters
    ///
    /// unknown - pass 0
    ///
    /// # Return Value
    ///
    /// Return 0 on success, < 0 on error
    pub fn scePowerLock(unknown: i32) -> i32;

    #[psp(0xCA3D34C1)]
    /// Unlock Power Switch
    ///
    /// # Parameters
    ///
    /// unknown - pass 0
    ///
    /// # Return Value
    ///
    /// Return 0 on success, < 0 on error
    pub fn scePowerUnlock(unknown: i32) -> i32;

    #[psp(0xEFD3C963)]
    /// Generate a power tick, preventing unit from
    /// powering off and turning off display
    ///
    /// # Parameters
    ///
    /// `t`: Either All, Suspend, or Display
    ///
    /// # Return Value
    ///
    /// Return 0 on success, < 0 on error.
    pub fn scePowerTick(t: PowerTick) -> i32;

    #[psp(0xEDC13FE5)]
    /// Get Idle Timer
    pub fn scePowerGetIdleTimer() -> i32;

    #[psp(0x7F30B3B1)]
    /// Enable Idle Timer
    ///
    /// # Parameters
    ///
    /// unknown - pass 0
    pub fn scePowerIdleTimerEnable(unknown: i32) -> i32;

    #[psp(0x972CE941)]
    /// Disable Idle Timer
    ///
    /// # Parameters
    ///
    /// unknown - pass 0
    pub fn scePowerIdleTimerDisable(unknown: i32) -> i32;

    #[psp(0x2B7C7CF4)]
    /// Request PSP to go into standby mode
    ///
    /// # Return Value
    ///
    /// Always returns 0
    pub fn scePowerRequestStandby() -> i32;

    #[psp(0xAC32C9CC)]
    /// Request PSP to go into suspend mode
    ///
    /// # Return Value
    ///
    /// Always returns 0
    pub fn scePowerRequestSuspend() -> i32;
}
