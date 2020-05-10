use crate::sys::kernel::SceUid;

///Power callback flags
#[repr(u32)]
pub enum Callback{
    ///Indicates the power switch it pushed, putting the unit into suspend mode
    PowerSwitch     = 0x80000000,
    
    ///Indicates the hold switch is on
    HoldSwitch      = 0x40000000,

    ///Indicates the PSP has gone to standby (screen off)
    Standby		    = 0x00080000,

    ///Indicates the resume process is complete. (Only triggered when another even happens)
    ResumeComplete  = 0x00040000,

    ///Indicates the unit is resuming from suspend mode.
    Resuming         = 0x00020000,

    ///Indicates the unit is suspending - occurs due to inactivity
    Suspending		= 0x00010000,

    ///Indicates the unit is plugged into an AC outlet
    ACPower         = 0x00001000,

    ///Indicates the battery charge level is low
    BatteryLow      = 0x00000100,
    
    ///Indicates there is a battery present
    BatteryExist    = 0x00000080,

    ///Indicates the unit is relying on a battery for power (instead of AC adapter)
    BatteryPower    = 0x0000007F
}

///Power tick flags
#[repr(u32)]
pub enum Tick{
    ///All
    All = 0,
    ///Suspend
    Suspend = 1,
    ///Display
    Display = 6,
}

sys_lib! {
    #![name = "scePower"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x04B7766E)]
    /// Register Power Callback Function
    ///
    /// # Parameters
    ///
    /// `slot` - slot of the callback in the list, 0 to 15, pass -1 to get an auto assignment.
    ///
    /// `cbid` - callback id from calling sceKernelCreateCallback
    ///
    /// # Return value
    ///
    /// Returns 0 on success, the slot number if -1 is passed, or < 0 on error.
    pub unsafe fn sce_power_register_callback(slot: i32, cbid: SceUid) -> i32;

    #[psp(0xDFA8BAF8)]
    /// Unregister Power Callback Function
    ///
    /// # Paramters
    ///
    /// `slot` - slot of the callback
    ///
    /// # Return Value
    ///
    /// Return 0 on success, < 0 on error
    pub unsafe fn sce_power_unregister_callback(slot: i32) -> i32;

    #[psp(0x87440F5E)]
    /// Check if unit is plugged in
    ///
    /// # Return Value
    ///
    /// Return 1 if plugged in, 0 if not plugged in, < 0 on error
    pub unsafe fn sce_power_is_power_online() -> i32;

    #[psp(0x0AFD0D8B)]
    /// Check if a battery is present
    ///
    /// # Return Value
    ///
    /// Return 1 if battery present, 0 if battery not present, < 0 on error
    pub unsafe fn sce_power_is_battery_exist() -> i32;

    #[psp(0x1E490401)]
    /// Check if the battery is charging
    ///
    /// # Return Value
    ///
    /// Return 1 if battery charging, 0 if battery not charging, < 0 on error
    pub unsafe fn sce_power_is_battery_charging() -> i32;

    #[psp(0xB4432BC8)]
    ///Get the status of battery charging
    pub unsafe fn sce_power_get_battery_charging_status() -> i32;

    #[psp(0xD3075926)]
    ///Check if the battery is low
    ///
    /// # Return Value
    ///
    /// Return 1 if the battery is low, Return 0 if the battery is not low, < 0 on error
    pub unsafe fn sce_power_is_low_battery() -> i32; 

    #[psp(0x2085D15D)]
    ///Get battery life as integer percent
    ///
    /// # Return Value
    ///
    /// Return battery charger as a percentage 0-100, < 0 on error
    pub unsafe fn sce_power_get_battery_life_percent() -> i32;

    #[psp(0x8EFB3FA2)]
    ///Get battery life as time
    ///
    /// # Return Value
    ///
    /// Return battery life in minutes, < 0 on error
    pub unsafe fn sce_power_get_battery_life_time() -> i32;

    #[psp(0x28E12023)]
    ///Get temperature of battery
    pub unsafe fn sce_power_get_battery_temp() -> i32;

    #[psp(0x862AE1A6)]
    ///Unknown - can crash
    pub unsafe fn sce_power_get_battery_elec() -> i32;

    #[psp(0x483CE86B)]
    ///Get battery volt level
    pub unsafe fn sce_power_get_battery_volt() -> i32;

    #[psp(0x843FBF43)]
    ///Set CPU Frequency
    ///
    /// # Parameters
    ///
    /// cpufreq - new CPU frequency from 1-333
    pub unsafe fn sce_power_set_cpu_clock_frequency(cpufreq: i32) -> i32;
    
    #[psp(0xB8D7B3FB)]
    ///Set CPU Frequency
    ///
    /// # Parameters
    ///
    /// cpufreq - new CPU frequency from 1-333
    pub unsafe fn sce_power_set_bus_clock_frequency(busfreq: i32) -> i32;

    #[psp(0xFEE03A2F)]
    ///Get CPU Frequency
    ///
    /// # Return Value
    /// 
    /// Returns cpu frequency as an integer
    pub unsafe fn sce_power_get_cpu_clock_frequency() -> i32;

    
    #[psp(0xFDB5BFE9)]
    ///Get CPU Frequency
    ///
    /// # Return Value
    /// 
    /// Returns cpu frequency as an integer
    pub unsafe fn sce_power_get_cpu_clock_frequency_int() -> i32;
    
    
    #[psp(0xB1A52C83)]
    ///Get CPU Frequency
    ///
    /// # Return Value
    /// 
    /// Returns cpu frequency as a float
    pub unsafe fn sce_power_get_cpu_clock_frequency_float() -> f32;

    
    #[psp(0x478FE6F5)]
    ///Get bus Frequency
    ///
    /// # Return Value
    /// 
    /// Returns bus frequency as an integer
    pub unsafe fn sce_power_get_bus_clock_frequency() -> i32;

    
    #[psp(0xBD681969)]
    ///Get bus Frequency
    ///
    /// # Return Value
    /// 
    /// Returns bus frequency as an integer
    pub unsafe fn sce_power_get_bus_clock_frequency_int() -> i32;
    
    
    #[psp(0x9BADB3EB)]
    ///Get bus Frequency
    ///
    /// # Return Value
    /// 
    /// Returns bus frequency as a float
    pub unsafe fn sce_power_get_bus_clock_frequency_float() -> f32;
    
    #[psp(0x737486F2)]
    ///Set Clock Frequencies
    ///
    /// # Parameters
    ///
    /// pllfreq - pll frequency from 19-333
    ///
    /// cpufreq - cpu frequency from 1-333
    ///
    /// busfreq - bus frequency from 1-167
    ///
    /// Given:
    ///  cpufreq <= pllfreq
    ///  busfreq*2 <= pllfreq
    pub unsafe fn sce_power_set_clock_frequency(pllfreq: i32, cpufreq: i32, busfreq: i32) -> i32;

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
    pub unsafe fn sce_power_lock(unknown: i32) -> i32;

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
    pub unsafe fn sce_power_unlock(unknown: i32) -> i32;

    #[psp(0xEFD3C963)]
    /// Generate a power tick, preventing unit from
    /// powering off and turning off display
    ///
    /// # Parameters
    ///
    /// t - Either All, Suspend, or Display
    ///
    /// # Return Value
    /// 
    /// Return 0 on success, < 0 on error.
    pub unsafe fn sce_power_tick(t: Tick) -> i32;

    #[psp(0xEDC13FE5)]
    /// Get Idle Timer
    pub unsafe fn sce_power_get_idle_timer() -> i32;

    #[psp(0x7F30B3B1)]
    /// Enable Idle Timer
    ///
    /// # Parameters
    /// 
    /// unknown - pass 0
    pub unsafe fn sce_power_idle_timer_enable(unknown: i32) -> i32;
    
    #[psp(0x972CE941)]
    /// Disable Idle Timer
    ///
    /// # Parameters
    /// 
    /// unknown - pass 0
    pub unsafe fn sce_power_idle_timer_disable(unknown: i32) -> i32;

    #[psp(0x2B7C7CF4)]
    /// Request PSP to go into standby mode
    ///
    /// # Return Value
    ///
    /// Always returns 0
    pub unsafe fn sce_power_request_standby() -> i32;
    
    #[psp(0xAC32C9CC)]
    /// Request PSP to go into suspend mode
    ///
    /// # Return Value
    ///
    /// Always returns 0
    pub unsafe fn sce_power_request_suspend() -> i32;
}