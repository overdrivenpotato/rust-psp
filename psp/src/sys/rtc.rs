use num_enum::TryFromPrimitive;

/// PSP Time structure
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ScePspDateTime {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub microseconds: u32,
}

/// Errors which may be returned from `sceRtc*` functions.
#[repr(i32)]
#[derive(Eq, PartialEq, TryFromPrimitive)]
pub enum RtcCheckValidError {
    InvalidYear = -1,
    InvalidMonth = -2,
    InvalidDay = -3,
    InvalidHour = -4,
    InvalidMinutes = -5,
    InvalidSeconds = -6,
    InvalidMicroSeconds = -7,
}

psp_extern! {
    #![name = "sceRtc"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0xC41C2853)]
    /// Get the resolution of the tick counter
    ///
    /// # Return Value
    ///
    /// Number of ticks per second
    pub fn sceRtcGetTickResolution() -> u32;

    #[psp(0x3F7AD767)]
    /// Get current tick count
    ///
    /// # Parameters
    ///
    /// - `tick`: pointer to u64 to receive tick count
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcGetCurrentTick(tick: *mut u64) -> i32;

    #[psp(0x4CFA57B0)]
    /// Get current tick count, adjusted for local time zone
    ///
    /// # Parameters
    ///
    /// - `time`: pointer to `ScePspDateTime` struct to receive time
    /// - `tz`: time zone to adjust to (minutes from UTC)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcGetCurrentClock(time: *mut ScePspDateTime, tz: i32) -> i32;

    #[psp(0xE7C27D1B)]
    /// Get current local time into a `ScePspDateTime` struct
    ///
    /// # Parameters
    ///
    /// - `time`: pointer to `ScePspDateTime` struct to receive time
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcGetCurrentClockLocalTime(time: *mut ScePspDateTime) -> i32;

    #[psp(0x34885E0D)]
    /// Convert a UTC-based tickcount into a local time tick count
    ///
    /// # Parameters
    ///
    /// - `tick_utc`: pointer to u64 tick in UTC time
    /// - `tick_local`: pointer to u64 to receive tick in local time
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcConvertUtcToLocalTime(
        tick_utc: *const u64,
        tick_local: *mut u64,
    ) -> i32;

    #[psp(0x779242A2)]
    /// Convert a local time based tickcount into a UTC-based tick count
    ///
    /// # Parameters
    ///
    /// - `tick_local`: pointer to u64 tick in local time
    /// - `tick_utc`: pointer to u64 to receive tick in UTC based time
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcConvertLocalTimeToUTC(
        tick_local: *const u64,
        tick_utc: *mut u64,
    ) -> i32;

    #[psp(0x42307A17)]
    /// Check if a year is a leap year
    ///
    /// # Parameters
    ///
    /// - `year`: year to check if is a leap year
    ///
    /// # Return Value
    ///
    /// 1 on leapyear, 0 if not
    pub fn sceRtcIsLeapYear(year: i32) -> i32;

    #[psp(0x05EF322C)]
    /// Get number of days in a specific month
    ///
    /// # Parameters
    ///
    /// - `year`: year in which to check (accounts for leap year)
    /// - `month`: month to get number of days for
    ///
    /// # Return Value
    ///
    /// Number of days in month, < 0 on error (?)
    pub fn sceRtcGetDaysInMonth(year: i32, month: i32) -> i32;

    #[psp(0x57726BC1)]
    /// Get day of the week for a date
    ///
    /// # Parameters
    ///
    /// - `year`: year in which to check (accounts for leap year)
    /// - `month`: month that day is in
    /// - `day`: day to get day of week for
    ///
    /// # Return Value
    ///
    /// day of week with 0 representing Monday
    pub fn sceRtcGetDayOfWeek(year: i32, month: i32, day: i32) -> i32;

    #[psp(0x4B1B5E82)]
    /// Validate `ScePspDateTime` component ranges
    ///
    /// # Parameters
    ///
    /// - `date`: pointer to `ScePspDateTime` struct to be checked
    ///
    /// # Return Value
    ///
    /// 0 on success, one of ::pspRtcCheckValidErrors on error
    pub fn sceRtcCheckValid(date: *const ScePspDateTime) -> i32;

    #[psp(0x7ED29E40)]
    /// Set a `ScePspDateTime` struct based on ticks
    ///
    /// # Parameters
    ///
    /// - `date`: pointer to `ScePspDateTime` struct to set
    /// - `tick`: pointer to ticks to convert
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcSetTick(date: *mut ScePspDateTime, tick: *const u64) -> i32;

    #[psp(0x6FF40ACC)]
    /// Set ticks based on a `ScePspDateTime` struct
    ///
    /// # Parameters
    ///
    /// - `date`: pointer to `ScePspDateTime` to convert
    /// - `tick`: pointer to tick to set
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcGetTick(date: *const ScePspDateTime, tick: *mut u64) -> i32;

    #[psp(0x9ED0AE87)]
    /// Compare two ticks
    ///
    /// # Parameters
    ///
    /// - `tick1`: pointer to first tick
    /// - `tick2`: poiinter to second tick
    ///
    /// # Return Value
    ///
    /// 0 on equal, < 0 when tick1 < tick2, >0 when tick1 > tick2
    pub fn sceRtcCompareTick(tick1: *const u64, tick2: *const u64) -> i32;

    #[psp(0x44F45E05)]
    /// Add two ticks
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_ticks`: number of ticks to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddTicks(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_ticks: u64,
    ) -> i32;

    #[psp(0x26D25A5D)]
    /// Add an amount of ms to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_ms`: number of ms to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddMicroseconds(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_ms: u64,
    ) -> i32;

    #[psp(0xF2A4AFE5)]
    /// Add an amount of seconds to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_secs`: number of seconds to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddSeconds(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_secs: u64,
    ) -> i32;

    #[psp(0xE6605BCA)]
    /// Add an amount of minutes to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_mins`: number of minutes to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddMinutes(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_mins: u64,
    ) -> i32;

    #[psp(0x26D7A24A)]
    /// Add an amount of hours to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_hours`: number of hours to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddHours(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_hours: i32,
    ) -> i32;

    #[psp(0xE51B4B7A)]
    /// Add an amount of days to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_days`: number of days to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddDays(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_days: i32,
    ) -> i32;

    #[psp(0xCF3A2CA8)]
    /// Add an amount of weeks to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_weeks`: number of weeks to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddWeeks(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_weeks: i32,
    ) -> i32;

    #[psp(0xDBF74F1B)]
    /// Add an amount of months to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_months`: number of months to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddMonths(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_months: i32,
    ) -> i32;

    #[psp(0x42842C77)]
    /// Add an amount of years to a tick
    ///
    /// # Parameters
    ///
    /// - `dest_tick`: pointer to tick to hold result
    /// - `src_tick`: pointer to source tick
    /// - `num_years`: number of years to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRtcTickAddYears(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_years: i32,
    ) -> i32;

    #[psp(0x3A807CC8)]
    pub fn sceRtcSetTime_t(date: *mut ScePspDateTime, time: u32) -> i32;

    #[psp(0x27C4594C)]
    pub fn sceRtcGetTime_t(date: *const ScePspDateTime, time: *mut u32) -> i32;

    #[psp(0x1909C99B)]
    pub fn sceRtcSetTime64_t(date: *mut ScePspDateTime, time: u64) -> i32;

    #[psp(0xE1C93E47)]
    pub fn sceRtcGetTime64_t(date: *const ScePspDateTime, time: *mut u64) -> i32;

    #[psp(0xF006F264)]
    pub fn sceRtcSetDosTime(date: *mut ScePspDateTime, dos_time: u32) -> i32;

    #[psp(0x36075567)]
    pub fn sceRtcGetDosTime(date: *mut ScePspDateTime, dos_time: u32) -> i32;

    #[psp(0x7ACE4C04)]
    pub fn sceRtcSetWin32FileTime(date: *mut ScePspDateTime, win32_time: *mut u64) -> i32;

    #[psp(0xCF561893)]
    pub fn sceRtcGetWin32FileTime(date: *mut ScePspDateTime, win32_time: *mut u64) -> i32;

    #[psp(0xDFBC5F16)]
    pub fn sceRtcParseDateTime(
        dest_tick: *mut u64,
        date_string: *const u8,
    ) -> i32;

    #[psp(0xC663B3B9)]
    /// Format Tick-representation UTC time in RFC2822 format
    pub fn sceRtcFormatRFC2822(
        p_sz_date_time: *mut u8,
        p_utc: *const u64,
        time_zone_minutes: i32,
    ) -> i32;

    #[psp(0x7DE6711B)]
    /// Format Tick-representation UTC time in RFC2822 format
    pub fn sceRtcFormatRFC2822LocalTime(
        p_sz_date_time: *mut u8,
        p_utc: *const u64,
    ) -> i32;

    #[psp(0x0498FB3C)]
    /// Format Tick-representation UTC time in RFC3339(ISO8601) format
    pub fn sceRtcFormatRFC3339(
        p_sz_date_time: *mut u8,
        p_utc: *const u64,
        time_zone_minutes: i32,
    ) -> i32;

    #[psp(0x27F98543)]
    /// Format Tick-representation UTC time in RFC3339(ISO8601) format
    pub fn sceRtcFormatRFC3339LocalTime(
        p_sz_date_time: *mut u8,
        p_utc: *const u64,
    ) -> i32;

    #[psp(0x28E1E988)]
    /// Parse time information represented in RFC3339 format
    pub fn sceRtcParseRFC3339(
        p_utc: *mut u64,
        p_sz_date_time: *const u8,
    ) -> i32;
}
