use num_enum::TryFromPrimitive;

///PSP Time Structure
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Time{
    year:           u16,
    month:          u16,
    day:            u16,
    hour:           u16,
    minutes:        u16,
    seconds:        u16,
    microseconds:   u32
}

impl Clone for Time {
    fn clone(&self) -> Self { *self }
}

///Errors
#[repr(i32)]
#[derive(Eq, PartialEq, TryFromPrimitive)]
pub enum CheckValidError{
    InvalidYear         = -1,
    InvalidMonth        = -2,
    InvalidDay          = -3,
    InvalidHour         = -4,
    InvalidMinutes      = -5,
    InvalidSeconds      = -6,
    InvalidMicroseconds = -7
}

sys_lib!{
    #![name = "sceRtc"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0xC41C2853)]
    /// Get the resolution of the tick counter
    ///
    /// # Return Value
    ///
    /// Number of ticks per second
    pub unsafe fn sce_rtc_get_tick_resolution() -> u32;

    #[psp(0x3F7AD767)]
    /// Get current tick count
    ///
    /// # Parameters
    ///
    /// `tick` - Pointer to u64 to receive tick count
    ///
    /// # Return Value
    ///
    ///  0 on success, < 0 on error
    pub unsafe fn sce_rtc_get_current_tick(tick: *mut u64) -> i32;

    #[psp(0x4CFA57B0)]
    /// Get current tick count, adjusted for local time zone
    ///
    /// # Parameters
    ///
    /// `tm` - pointer to Time struct to receive time
    ///
    /// `tz` - time zone to adjust to (minutes from UTC)
    ///
    /// # Return Value
    ///
    ///  0 on success, < 0 on error
    pub unsafe fn sce_rtc_get_current_clock(tm: Time, tz: i32) -> i32;

    #[psp(0xE7C27D1B)]
    /// Get current tick count, adjusted for local time zone
    ///
    /// # Parameters
    ///
    /// `tm` - pointer to Time struct to receive time
    ///
    /// # Return Value
    ///
    ///  0 on success, < 0 on error
    pub unsafe fn sce_rtc_get_current_clock_local_time(tm: Time) -> i32;

    #[psp(0x34885E0D)]
    ///Convert a UTC-based tickcount into a local time tick count
    ///
    /// # Parameters
    ///
    /// `tick_utc` - pointer to u64 tick in UTC time
    ///
    /// `tick_local` - pointer to u64 tick to receive in local time
    ///
    /// # Return Value
    ///
    ///  0 on success, < 0 on error
    pub unsafe fn sce_rtc_convert_utc_to_local_time(tick_utc: *const u64, tick_local: *mut u64) -> i32;

    #[psp(0x779242A2)]
    ///Convert a local time-based tickcount into a UTC time tick count
    ///
    /// # Parameters
    ///
    /// `tick_local` - pointer to u64 tick in UTC time
    ///
    /// `tick_utc` - pointer to u64 tick to receive in local time
    ///
    /// # Return Value
    ///
    ///  0 on success, < 0 on error
    pub unsafe fn sce_rtc_convert_local_to_utc_time(tick_local: *const u64, tick_utc: *mut u64) -> i32;

    #[psp(0x42307A17)]
    ///Check if a year is a leap year
    ///
    /// # Parameters
    ///
    /// `year` - year to check if is a leap year
    ///
    /// # Return Value
    ///
    ///  1 on leapyear, 0 if not
    pub unsafe fn sce_rtc_is_leap_year(year: i32) -> i32;

    #[psp(0x05EF322C)]
    ///Get number of days in a specific month
    ///
    /// # Parameters
    ///
    /// `year` - year in which to check
    ///
    /// `month` - month to get number of days for
    ///
    /// # Return Value
    ///
    /// Number of days in month, < 0 on error
    pub unsafe fn sce_rtc_get_days_in_month(year: i32, month: i32) -> i32;

    #[psp(0x57726BC1)]
    ///Get day of the week for a date
    ///
    /// # Parameters
    ///
    /// `year` - year in which to check
    ///
    /// `month` - month the day is in
    ///
    /// `day` - day to get day of week for
    ///
    /// # Return Value
    ///
    ///  day of week with 0 representing monday
    pub unsafe fn sce_rtc_get_day_of_week(year: i32, month: i32, day: i32) -> i32;

    #[psp(0x4B1B5E82)]
    ///Validate pspDate Component Ranges
    ///
    /// # Parameters
    /// 
    /// `date` - pointer to pspDate struct to be checked
    ///
    /// # Return Value
    ///
    ///  0 on success, one of ::CheckValidErrors on error
    pub unsafe fn sce_rtc_check_valid(date: *const Time) -> i32;

    #[psp(0x7ED29E40)]
    ///Set a pspTime struct based on ticks
    ///
    /// # Parameters
    ///
    /// `date` - pointer to pspTime struct to set
    ///
    /// `tick` - pointer to ticks to convert
    ///
    /// # Return Value
    ///
    ///  0 on success, < 0 on error
    pub unsafe fn sce_rtc_set_tick(date: *mut Time, tick: *const u64) -> i32;

    #[psp(0x6FF40ACC)]
    ///Set ticks based on a pspTime struct
    ///
    /// # Parameters
    ///
    /// `date` - pointer to pspTime to convert
    ///
    /// `tick` - pointer to tick to set
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_get_tick(date: *const Time, tick: *mut u64) -> i32;

    #[psp(0x9ED0AE87)]
    ///Compare two ticks
    ///
    /// # Parameters
    ///
    /// `tick1` - pointer to first tick
    ///
    /// `tick2` - pointer to second tick
    ///
    /// # Return Value
    ///
    ///  0 on equal, < 0 when tick1 < tick2, > 0 when tick1 > tick2
    pub unsafe fn sce_rtc_compare_tick(tick1: *const u64, tick2: *const u64) -> i32;

    #[psp(0x44F45E05)]
    ///Add two ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_ticks` - number of ticks to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_ticks(dest_tick: *mut u64, src_tick: *const u64, num_tick: u64) -> i32;

    #[psp(0x26D25A5D)]
    ///Add an amount of ms to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_tms` - number of ms to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_microseconds(dest_tick: *mut u64, src_tick: *const u64, num_tms: u64) -> i32;

    #[psp(0xF2A4AFE5)]
    ///Add an amount of seconds to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_seconds` - number of seconds to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_seconds(dest_tick: *mut u64, src_tick: *const u64, num_seconds: u64) -> i32;

    #[psp(0xE6605BCA)]
    ///Add an amount of minutes to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_minutes` - number of minutes to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_minutes(dest_tick: *mut u64, src_tick: *const u64, num_minutes: u64) -> i32;

    
    #[psp(0x26D7A24A)]
    ///Add an amount of hours to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_hours` - number of hours to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_hours(dest_tick: *mut u64, src_tick: *const u64, num_hours: u64) -> i32;

    
    #[psp(0xE51B4B7A)]
    ///Add an amount of days to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_days` - number of days to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_days(dest_tick: *mut u64, src_tick: *const u64, num_days: u64) -> i32;

    
    #[psp(0xCF3A2CA8)]
    ///Add an amount of weeks to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_weeks` - number of weeks to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_weeks(dest_tick: *mut u64, src_tick: *const u64, num_weeks: u64) -> i32;

    
    #[psp(0xDBF74F1B)]
    ///Add an amount of months to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_months` - number of months to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_months(dest_tick: *mut u64, src_tick: *const u64, num_months: u64) -> i32;
    
    #[psp(0x42842C77)]
    ///Add an amount of years to a tick ticks
    ///
    /// # Parameters
    ///
    /// `dest_tick` - pointer to tick to hold result
    ///
    /// `src_tick` - pointer to source tick
    /// 
    /// `num_years` - number of years to add
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_rtc_tick_add_years(dest_tick: *mut u64, src_tick: *const u64, num_years: u64) -> i32;

    #[psp(0x3A807CC8)]
    pub unsafe fn sce_rtc_set_time_t(date: *mut Time, time: i64) -> i32;

    #[psp(0x27C4594C)]
    pub unsafe fn sce_rtc_get_time_t(date: *const Time, time: *mut i64) -> i32;

    #[psp(0xF006F264)]
    pub unsafe fn sce_rtc_set_dos_time(date: *mut Time, time: u32) -> i32;
    
    #[psp(0x36075567)]
    pub unsafe fn sce_rtc_get_dos_time(date: *mut Time, time: u32) -> i32;

    #[psp(0x7ACE4C04)]
    pub unsafe fn sce_rtc_set_win32_file_time(date: *mut Time, time: *mut u64) -> i32;
    
    #[psp(0xCF561893)]
    pub unsafe fn sce_rtc_get_win32_file_time(date: *mut Time, time: *mut u64) -> i32;

    #[psp(0xDFBC5F16)]
    pub unsafe fn sce_rtc_parse_date_time(dest_tick: *mut u64, date_string: *const u8) -> i32;

    #[psp(0x0498FB3C)]
    ///Format Tick-representation UTC time in RFC3339(ISO8601) format
    pub unsafe fn sce_rtc_format_rfc3339(psz_date_time: *mut char, p_utc: *const u64, time_zone_minutes: i32) -> i32;

    #[psp(0x27F98543)]
    ///Format Tick-representation UTC time in RFC3339(ISO8601) format
    pub unsafe fn sce_rtc_format_rfc3339_localtime(psz_date_time: *mut char, p_utc: *const u64) -> i32;

    #[psp(0x28E1E988)]
    ///Parse time information represented in RFC3339 format
    pub unsafe fn sce_rtc_parse_rfc3339(p_utc: *mut u64, psz_date_time: *const u8) -> i32;

    #[psp(0xC663B3B9)]
    ///Format Tick-representation UTC time in RFC2822 format
    pub unsafe fn sce_rtc_format_rfc_2822(psz_date_time: *mut char, p_utc: *const u64, time_zone_minutes: i32) -> i32;
    
    #[psp(0x7DE6711B)]
    ///Format Tick-representation UTC time in RFC2822 format
    pub unsafe fn sce_rtc_format_rfc_2822_local_time(psz_date_time: *mut char, p_utc: *const u64) -> i32;
}
