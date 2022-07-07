use crate::sys::SceUid;

bitflags::bitflags! {
    /// Power callback flags
    #[repr(transparent)]
    pub struct PowerInfo: u32 {
        /// Indicates the power switch is pushed, putting the unit into suspend
        /// mode.
        const POWER_SWITCH = 0x80000000;
        /// Indicates the hold switch is on.
        const HOLD_SWITCH = 0x40000000;
        /// Indicates the screen is off.
        const STANDBY = 0x00080000;
        /// Indicates the resume process is complete. Only seems to be triggered
        /// when another event happens.
        const RESUME_COMPLETE = 0x00040000;
        /// Indicates the unit is resuming from suspend mode.
        const RESUMING = 0x00020000;
        /// Indicates the unit is suspending, seems to occur due to inactivity.
        const SUSPENDING = 0x00010000;
        /// Indicates the unit is plugged into an AC outlet.
        const AC_POWER = 0x00001000;
        /// Indicates the battery charge level is low.
        const BATTERY_LOW = 0x00000100;
        /// Indicates there is a battery present in the unit.
        const BATTERY_EXIST = 0x00000080;
        /// Indicates that the system is running on battery power.
        const BATTERY_POWER = 0x0000007;
    }
}

/// Type of power tick to generate.
#[repr(u32)]
pub enum PowerTick {
    /// A tick that prevents the PSP from suspending and the display from
    /// turning off.
    All = 0,
    /// A power tick that prevents the PSP from suspending.
    Suspend = 1,
    /// A power tick that prevents the PSP display from turning off.
    Display = 6,
}

/// Power callback function prototype
///
/// # Parameters
///
/// - `unknown`: Unknown function, appears to cycle between 1, 2, and 3.
/// - `power_info`: Combination of `PowerInfo` flags.
pub type PowerCallback = extern "C" fn(unknown: i32, power_info: PowerInfo);

psp_extern! {
    #![name = "scePower"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x04B7766E)]
    /// Register Power Callback Function
    ///
    /// # Parameters
    ///
    /// - `slot`: slot of the callback in the list, 0 to 15, pass -1 to get an
    ///   auto assignment.
    /// - `cbid`: callback id from calling `sceKernelCreateCallback`
    ///
    /// # Return Value
    ///
    /// 0 on success, the slot number if -1 is passed, < 0 on error.
    pub fn scePowerRegisterCallback(
        slot: i32,
        cbid: SceUid,
    ) -> i32;

    #[psp(0xDFA8BAF8)]
    /// Unregister Power Callback Function
    ///
    /// # Parameters
    ///
    /// - `slot`: slot of the callback
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn scePowerUnregisterCallback(slot: i32) -> i32;

    #[psp(0x87440F5E)]
    /// Check if unit is plugged in
    ///
    /// # Return Value
    ///
    /// 1 if plugged in, 0 if not plugged in, < 0 on error.
    pub fn scePowerIsPowerOnline() -> i32;

    #[psp(0x0AFD0D8B)]
    /// Check if a battery is present
    ///
    /// # Return Value
    ///
    /// 1 if battery present, 0 if battery not present, < 0 on error.
    pub fn scePowerIsBatteryExist() -> i32;

    #[psp(0x1E490401)]
    /// Check if the battery is charging
    ///
    /// # Return Value
    ///
    /// 1 if battery charging, 0 if battery not charging, < 0 on error.
    pub fn scePowerIsBatteryCharging() -> i32;

    #[psp(0xB4432BC8)]
    /// Get the status of the battery charging
    pub fn scePowerGetBatteryChargingStatus() -> i32;

    #[psp(0xD3075926)]
    /// Check if the battery is low
    ///
    /// # Return Value
    ///
    /// 1 if the battery is low, 0 if the battery is not low, < 0 on error.
    pub fn scePowerIsLowBattery() -> i32;

    #[psp(0x2085D15D)]
    /// Get battery life as integer percent
    ///
    /// # Return Value
    ///
    /// Battery charge percentage (0-100), < 0 on error.
    pub fn scePowerGetBatteryLifePercent() -> i32;

    #[psp(0x8EFB3FA2)]
    /// Get battery life as time
    ///
    /// # Return Value
    ///
    /// Battery life in minutes, < 0 on error.
    pub fn scePowerGetBatteryLifeTime() -> i32;

    #[psp(0x28E12023)]
    /// Get temperature of the battery
    pub fn scePowerGetBatteryTemp() -> i32;

    #[psp(0x862AE1A6)]
    /// unknown? - crashes PSP in usermode
    pub fn scePowerGetBatteryElec() -> i32;

    #[psp(0x483CE86B)]
    /// Get battery volt level
    pub fn scePowerGetBatteryVolt() -> i32;

    #[psp(0x843FBF43)]
    /// Set CPU Frequency
    ///
    /// # Parameters
    ///
    /// - `cpufreq`: new CPU frequency, valid values are 1 - 333
    pub fn scePowerSetCpuClockFrequency(cpufreq: i32) -> i32;

    #[psp(0xB8D7B3FB)]
    /// Set Bus Frequency
    ///
    /// # Parameters
    ///
    /// - `busfreq`: new BUS frequency, valid values are 1 - 166
    pub fn scePowerSetBusClockFrequency(busfreq: i32) -> i32;

    #[psp(0xFEE03A2F)]
    /// Alias for scePowerGetCpuClockFrequencyInt
    ///
    /// # Return Value
    ///
    /// Frequency as integer
    pub fn scePowerGetCpuClockFrequency() -> i32;

    #[psp(0xFDB5BFE9)]
    /// Get CPU Frequency as Integer
    ///
    /// # Return Value
    ///
    /// Frequency as an integer
    pub fn scePowerGetCpuClockFrequencyInt() -> i32;

    #[psp(0xB1A52C83)]
    /// Get CPU Frequency as Float
    ///
    /// # Return Value
    ///
    /// Frequency as a float
    pub fn scePowerGetCpuClockFrequencyFloat() -> f32;

    #[psp(0x478FE6F5)]
    /// Alias for scePowerGetBusClockFrequencyInt
    ///
    /// # Return Value
    ///
    /// Frequency as an integer
    pub fn scePowerGetBusClockFrequency() -> i32;

    #[psp(0xBD681969)]
    /// Get Bus frequency as Integer
    ///
    /// # Return Value
    ///
    /// Frequency as an integer
    pub fn scePowerGetBusClockFrequencyInt() -> i32;

    #[psp(0x9BADB3EB)]
    /// Get Bus frequency as Float
    ///
    /// # Return Value
    ///
    /// frequency as float
    pub fn scePowerGetBusClockFrequencyFloat() -> f32;

    #[psp(0x737486F2)]
    /// Set Clock Frequencies
    ///
    /// # Parameters
    ///
    /// - `pllfreq`: pll frequency, valid from 19-333
    /// - `cpufreq`: cpu frequency, valid from 1-333
    /// - `busfreq`: bus frequency, valid from 1-166
    ///
    /// and:
    ///
    /// cpufreq <= pllfreq
    /// busfreq*2 <= pllfreq
    ///
    pub fn scePowerSetClockFrequency(
        pllfreq: i32,
        cpufreq: i32,
        busfreq: i32,
    ) -> i32;

    #[psp(0xD6D016EF)]
    /// Lock power switch
    ///
    /// Note: if the power switch is toggled while locked it will fire
    /// immediately after being unlocked.
    ///
    /// # Parameters
    ///
    /// - `unknown`: pass 0
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn scePowerLock(unknown: i32) -> i32;

    #[psp(0xCA3D34C1)]
    /// Unlock power switch
    ///
    /// # Parameters
    ///
    /// - `unknown`: pass 0
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn scePowerUnlock(unknown: i32) -> i32;

    #[psp(0xEFD3C963)]
    /// Generate a power tick, preventing unit from powering off and turning off
    /// display.
    ///
    /// # Parameters
    ///
    /// - `type_`: type of power tick to generate
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn scePowerTick(type_: PowerTick) -> i32;

    #[psp(0xEDC13FE5)]
    /// Get Idle timer
    pub fn scePowerGetIdleTimer() -> i32;

    #[psp(0x7F30B3B1)]
    /// Enable Idle timer
    ///
    /// # Parameters
    ///
    /// - `unknown`: pass 0
    pub fn scePowerIdleTimerEnable(unknown: i32) -> i32;

    #[psp(0x972CE941)]
    /// Disable Idle timer
    ///
    /// # Parameters
    ///
    /// - `unknown`: pass 0
    pub fn scePowerIdleTimerDisable(unknown: i32) -> i32;

    #[psp(0x2B7C7CF4)]
    /// Request the PSP to go into standby
    ///
    /// # Return Value
    ///
    /// 0 always
    pub fn scePowerRequestStandby() -> i32;

    #[psp(0xAC32C9CC)]
    /// Request the PSP to go into suspend
    ///
    /// # Return Value
    ///
    /// 0 always
    pub fn scePowerRequestSuspend() -> i32;
}
