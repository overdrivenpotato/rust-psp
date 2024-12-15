use crate::eabi::i5;
use core::{ffi::c_void, fmt};

mod thread;
pub use thread::*;

/// Structure to pass to `sceKernelLoadExec`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLoadExecParam {
    /// Size of the structure.
    pub size: usize,
    /// Size of the arg string.
    pub args: usize,
    /// Pointer to the arg string.
    pub argp: *mut c_void,
    /// Encryption key ?
    pub key: *const u8,
}

psp_extern! {
    #![name = "LoadExecForUser"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x05572A5F)]
    /// Exit game and go back to the PSP browser.
    ///
    /// You need to be in a thread in order for this function to work.
    pub fn sceKernelExitGame();

    #[psp(0x4AC57943)]
    /// Register callback.
    ///
    /// By installing the exit callback the home button becomes active. However
    /// if `sceKernelExitGame` is not called in the callback it is likely
    /// that the PSP will just crash.
    ///
    /// # Parameters
    ///
    /// `id` - Callback id
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub fn sceKernelRegisterExitCallback(id: SceUid) -> i32;

    #[psp(0xBD2F1094)]
    /// Execute a new game executable, limited when not running in kernel mode.
    ///
    /// # Parameters
    ///
    /// - `file`: The file to execute.
    /// - `param`: Pointer to a `SceKernelLoadExecParam` structure, or NULL.
    ///
    /// # Return Value
    ///
    /// < 0 on error, probably.
    ///
    pub fn sceKernelLoadExec(
        file: *const u8,
        param: *mut SceKernelLoadExecParam,
    ) -> i32;
}

/// UIDs are used to describe many different kernel objects.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SceUid(pub i32);

impl From<SceUid> for i32 {
    fn from(uid: SceUid) -> i32 {
        uid.0
    }
}

impl From<i32> for SceUid {
    fn from(uid: i32) -> SceUid {
        SceUid(uid)
    }
}

// https://github.com/uofw/uofw/blob/f099b78dc0937df4e7346e2e417b63f471f8a3af/include/sysmem_user.h#L12
#[repr(i32)]
pub enum SceSysMemPartitionId {
    SceKernelUnknownPartition = 0,
    SceKernelPrimaryKernelPartition = 1,
    SceKernelPrimaryUserPartition = 2,
    SceKernelOtherKernelPartition1 = 3, //PRIMARY_ME_KERNEL_PARTITION according to TyRaNiD
    SceKernelOtherKernelPartition2 = 4,
    SceKernelVshellPARTITION = 5,
    SceKernelScUserPartition = 6,
    SceKernelMeUserPartition = 7,
    SceKernelExtendedScKernelPartition = 8,
    SceKernelExtendedSc2KernelPartition = 9,
    SceKernelExtendedMeKernelPartition = 10,
    SceKernelVshellKernelPartition = 11,
    SceKernelExtendedKernelPartition = 12,
}

/// Specifies the type of allocation used for memory blocks.
#[repr(i32)]
pub enum SceSysMemBlockTypes {
    /// Allocate from the lowest available address.
    Low = 0,

    /// Allocate from the highest available address.
    High,

    /// Allocate from the specified address.
    Addr,
}

psp_extern! {
    #![name = "SysMemUserForUser"]
    #![flags = 0x4000]
    #![version = (0, 0)]

    #[psp(0x237DBD4F, i5)]
    /// Allocate a memory block from a memory partition.
    ///
    /// # Parameters
    ///
    /// - `partition`: The UID of the partition to allocate from.
    /// - `name`: Name assigned to the new block.
    /// - `type`: Specifies how the block is allocated within the partition. One
    ///           of `SysMemBlockTypes`.
    /// - `size`: Size of the memory block, in bytes.
    /// - `addr`: If type is `Addr`, then addr specifies the lowest address
    ///           allocate the block from.
    ///
    /// # Return value
    ///
    /// The UID of the new block, or if less than 0 an error.
    pub fn sceKernelAllocPartitionMemory(
        partition: SceSysMemPartitionId,
        name: *const u8,
        type_: SceSysMemBlockTypes,
        size: u32,
        addr: *mut c_void,
    ) -> SceUid;

    #[psp(0x9D9A5BA1)]
    /// Get the address of a memory block.
    ///
    /// # Parameters
    ///
    /// `blockid` - UID of the memory block.
    ///
    /// # Return value
    ///
    /// The lowest address belonging to the memory block.
    pub fn sceKernelGetBlockHeadAddr(blockid: SceUid) -> *mut c_void;

    #[psp(0xB6D61D02)]
    /// Free a memory block allocated with `sceKernelAllocPartitionMemory`.
    ///
    /// # Parameters
    ///
    /// `blockid` - UID of the block to free.
    ///
    /// # Return value
    ///
    /// ? on success, less than 0 on error.
    pub fn sceKernelFreePartitionMemory(blockid: SceUid) -> i32;

    #[psp(0xF919F628)]
    /// Get the total amount of free memory.
    ///
    /// # Return Value
    ///
    /// The total amount of free memory, in bytes.
    pub fn sceKernelTotalFreeMemSize() -> usize;

    #[psp(0xA291F107)]
    /// Get the size of the largest free memory block.
    ///
    /// # Return Value
    ///
    /// The size of the largest free memory block, in bytes.
    pub fn sceKernelMaxFreeMemSize() -> usize;

    #[psp(0x3FC9AE6A)]
    /// Get the firmware version.
    ///
    /// # Return Value
    ///
    /// The firmware version.
    ///
    /// - `0x01000300` on v1.00 unit,
    /// - `0x01050001` on v1.50 unit,
    /// - `0x01050100` on v1.51 unit,
    /// - `0x01050200` on v1.52 unit,
    /// - `0x02000010` on v2.00/v2.01 unit,
    /// - `0x02050010` on v2.50 unit,
    /// - `0x02060010` on v2.60 unit,
    /// - `0x02070010` on v2.70 unit,
    /// - `0x02070110` on v2.71 unit.
    pub fn sceKernelDevkitVersion() -> u32;

    #[psp(0x7591C7DB)]
    /// Set the version of the SDK with which the caller was compiled.
    ///
    /// Version numbers are the same as for `sceKernelDevkitVersion`.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceKernelSetCompiledSdkVersion(version: u32) -> i32;

    #[psp(0xFC114573)]
    /// Get the SDK version set with `sceKernelSetCompiledSdkVersion`.
    ///
    /// # Return Value
    ///
    /// Version number, or 0 if unset.
    pub fn sceKernelGetCompiledSdkVersion() -> u32;

    // TODO: sceKernelPrintf cannot be implemented yet as this macro does not
    // yet support vararg functions.
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timezone {
    pub tz_minutes_west: i32,
    pub tz_dst_time: i32,
}

/// Type to hold a sha1 context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelUtilsSha1Context {
    pub h: [u32; 5usize],
    pub us_remains: u16,
    pub us_computed: u16,
    pub ull_total_len: u64,
    pub buf: [u8; 64usize],
}

/// Structure for holding a mersenne twister context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelUtilsMt19937Context {
    pub count: u32,
    pub state: [u32; 624usize],
}

/// Structure to hold the MD5 context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelUtilsMd5Context {
    pub h: [u32; 4usize],
    pub pad: u32,
    pub us_remains: u16,
    pub us_computed: u16,
    pub ull_total_len: u64,
    pub buf: [u8; 64usize],
}

psp_extern! {
    #![name = "UtilsForUser"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x27CC57F0)]
    /// Get the time in seconds since the epoch (1st Jan 1970)
    pub fn sceKernelLibcTime(t: *mut i32) -> i32;

    #[psp(0x91E4F6A7)]
    /// Get the processor clock used since the start of the process
    pub fn sceKernelLibcClock() -> u32;

    #[psp(0x71EC4271)]
    /// Get the current time of time and time zone information
    pub fn sceKernelLibcGettimeofday(tp: *mut timeval, tzp: *mut timezone) -> i32;

    #[psp(0x79D1C3FA)]
    /// Write back the data cache to memory
    pub fn sceKernelDcacheWritebackAll();

    #[psp(0xB435DEC5)]
    /// Write back and invalidate the data cache
    pub fn sceKernelDcacheWritebackInvalidateAll();

    #[psp(0x3EE30821)]
    /// Write back a range of addresses from the data cache to memory
    pub fn sceKernelDcacheWritebackRange(
        p: *const c_void,
        size: u32,
    );

    #[psp(0x34B9FA9E)]
    /// Write back and invalidate a range of addresses in the data cache
    pub fn sceKernelDcacheWritebackInvalidateRange(
        p: *const c_void,
        size: u32,
    );

    #[psp(0xBFA98062)]
    /// Invalidate a range of addresses in data cache
    pub fn sceKernelDcacheInvalidateRange(
        p: *const c_void,
        size: u32,
    );

    #[psp(0x920F104A)]
    /// Invalidate the instruction cache
    pub fn sceKernelIcacheInvalidateAll();

    #[psp(0xC2DF770E)]
    /// Invalidate a range of addresses in the instruction cache
    pub fn sceKernelIcacheInvalidateRange(
        p: *const c_void,
        size: u32,
    );

    #[psp(0xE860E75E)]
    /// Function to initialise a mersenne twister context.
    ///
    /// # Parameters
    ///
    /// - `ctx`: Pointer to a context
    /// - `seed`: A seed for the random function.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsMt19937Init(
        ctx: *mut SceKernelUtilsMt19937Context,
        seed: u32,
    ) -> i32;

    #[psp(0x06FB8A63)]
    /// Function to return a new psuedo random number.
    ///
    /// # Parameters
    ///
    /// - `ctx`: Pointer to a pre-initialised context.
    /// # Return Value
    ///
    /// A pseudo random number (between 0 and MAX_INT).
    pub fn sceKernelUtilsMt19937UInt(ctx: *mut SceKernelUtilsMt19937Context) -> u32;

    #[psp(0xC8186A58)]
    /// Function to perform an MD5 digest of a data block.
    ///
    /// # Parameters
    ///
    /// - `data`: Pointer to a data block to make a digest of.
    /// - `size`: Size of the data block.
    /// - `digest`: Pointer to a 16byte buffer to store the resulting digest
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsMd5Digest(
        data: *mut u8,
        size: u32,
        digest: *mut u8,
    ) -> i32;

    #[psp(0x9E5C5086)]
    /// Function to initialise a MD5 digest context
    ///
    /// # Parameters
    ///
    /// - `ctx`: A context block to initialise
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsMd5BlockInit(ctx: *mut SceKernelUtilsMd5Context) -> i32;

    #[psp(0x61E1E525)]
    /// Function to update the MD5 digest with a block of data.
    ///
    /// # Parameters
    ///
    /// - `ctx`: A filled in context block.
    /// - `data`: The data block to hash.
    /// - `size`: The size of the data to hash
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsMd5BlockUpdate(
        ctx: *mut SceKernelUtilsMd5Context,
        data: *mut u8,
        size: u32,
    ) -> i32;

    #[psp(0xB8D24E78)]
    /// Function to get the digest result of the MD5 hash.
    ///
    /// # Parameters
    ///
    /// - `ctx`: A filled in context block.
    /// - `digest`: A 16 byte array to hold the digest.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsMd5BlockResult(
        ctx: *mut SceKernelUtilsMd5Context,
        digest: *mut u8,
    ) -> i32;

    #[psp(0x840259F1)]
    /// Function to SHA1 hash a data block.
    ///
    /// # Parameters
    ///
    /// - `data`: The data to hash.
    /// - `size`: The size of the data.
    /// - `digest`: Pointer to a 20 byte array for storing the digest
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsSha1Digest(
        data: *mut u8,
        size: u32,
        digest: *mut u8,
    ) -> i32;

    #[psp(0xF8FCD5BA)]
    /// Function to initialise a context for SHA1 hashing.
    ///
    /// # Parameters
    ///
    /// - `ctx`: Pointer to a context.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsSha1BlockInit(
        ctx: *mut SceKernelUtilsSha1Context,
    ) -> i32;

    #[psp(0x346F6DA8)]
    /// Function to update the current hash.
    ///
    /// # Parameters
    ///
    /// - `ctx`: Pointer to a prefilled context.
    /// - `data`: The data block to hash.
    /// - `size`: The size of the data block
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsSha1BlockUpdate(
        ctx: *mut SceKernelUtilsSha1Context,
        data: *mut u8,
        size: u32,
    ) -> i32;

    #[psp(0x585F1C09)]
    /// Function to get the result of the SHA1 hash.
    ///
    /// # Parameters
    ///
    /// - `ctx`: Pointer to a prefilled context.
    /// - `digest`: A pointer to a 20 byte array to contain the digest.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelUtilsSha1BlockResult(
        ctx: *mut SceKernelUtilsSha1Context,
        digest: *mut u8,
    ) -> i32;
}

#[repr(packed, C)]
pub struct IntrHandlerOptionParam {
    size: i32,           //+00
    entry: u32,          //+04
    common: u32,         //+08
    gp: u32,             //+0C
    intr_code: u16,      //+10
    sub_count: u16,      //+12
    intr_level: u16,     //+14
    enabled: u16,        //+16
    calls: u32,          //+18
    field_1c: u32,       //+1C
    total_clock_lo: u32, //+20
    total_clock_hi: u32, //+24
    min_clock_lo: u32,   //+28
    min_clock_hi: u32,   //+2C
    max_clock_lo: u32,   //+30
    max_clock_hi: u32,   //+34
} //=38

#[repr(u32)]
pub enum Interrupt {
    Gpio = 4,
    Ata = 5,
    Umd = 6,
    Mscm0 = 7,
    Wlan = 8,
    Audio = 10,
    I2c = 12,
    Sircs = 14,
    Systimer0 = 15,
    Systimer1 = 16,
    Systimer2 = 17,
    Systimer3 = 18,
    Thread0 = 19,
    Nand = 20,
    Dmacplus = 21,
    Dma0 = 22,
    Dma1 = 23,
    Memlmd = 24,
    Ge = 25,
    Vblank = 30,
    Mecodec = 31,
    Hpremote = 36,
    Mscm1 = 60,
    Mscm2 = 61,
    Thread1 = 65,
    Interrupt = 66,
}

#[cfg(not(feature = "stub-only"))]
impl fmt::Display for Interrupt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        core::write!(
            f,
            "{}",
            match self {
                Self::Gpio => "GPIO",
                Self::Ata => "ATA_ATAPI",
                Self::Umd => "UmdMan",
                Self::Mscm0 => "MScmNone",
                Self::Wlan => "Wlan",
                Self::Audio => "Audio",
                Self::I2c => "I2C",
                Self::Sircs => "SIRCS_IrDA",
                Self::Systimer0 => "SystimerNone",
                Self::Systimer1 => "Systimer1",
                Self::Systimer2 => "Systimer2",
                Self::Systimer3 => "Systimer3",
                Self::Thread0 => "ThreadNone",
                Self::Nand => "NAND",
                Self::Dmacplus => "DMACPLUS",
                Self::Dma0 => "DMANone",
                Self::Dma1 => "DMA1",
                Self::Memlmd => "Memlmd",
                Self::Ge => "GE",
                Self::Vblank => "Display",
                Self::Mecodec => "MeCodec",
                Self::Hpremote => "HP_Remote",
                Self::Mscm1 => "MScm1",
                Self::Mscm2 => "MScm2",
                Self::Thread1 => "Thread1",
                Self::Interrupt => "Interrupt",
            }
        )
    }
}

#[repr(u32)]
pub enum SubInterrupt {
    Gpio = Interrupt::Gpio as u32,
    Ata = Interrupt::Ata as u32,
    Umd = Interrupt::Umd as u32,
    Dmacplus = Interrupt::Dmacplus as u32,
    Ge = Interrupt::Ge as u32,
    Display = Interrupt::Vblank as u32,
}

psp_extern! {
    #![name = "InterruptManager"]
    #![flags = 0x4000]
    #![version = (0x00, 0x00)]

    #[psp(0xCA04A2B9)]
    /// Register a sub interrupt handler.
    ///
    /// # Parameters
    ///
    /// - `int_no`: The interrupt number to register.
    /// - `no`: The sub interrupt handler number (user controlled)
    /// - `handler`: The interrupt handler
    /// - `arg`: An argument passed to the interrupt handler
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelRegisterSubIntrHandler(
        int_no: i32,
        no: i32,
        handler: *mut c_void,
        arg: *mut c_void,
    ) -> i32;

    #[psp(0xD61E6961)]
    /// Release a sub interrupt handler.
    ///
    /// # Parameters
    ///
    /// - `int_no`: The interrupt number to register.
    /// - `no`: The sub interrupt handler number
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReleaseSubIntrHandler(
        int_no: i32,
        no: i32,
    ) -> i32;

    #[psp(0xFB8E22EC)]
    /// Enable a sub interrupt.
    ///
    /// # Parameters
    ///
    /// - `int_no`: The sub interrupt to enable.
    /// - `no`: The sub interrupt handler number
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelEnableSubIntr(
        int_no: i32,
        no: i32,
    ) -> i32;

    #[psp(0x8A389411)]
    /// Disable a sub interrupt handler.
    ///
    /// # Parameters
    ///
    /// - `int_no`: The sub interrupt to disable.
    /// - `no`: The sub interrupt handler number
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelDisableSubIntr(
        int_no: i32,
        no: i32,
    ) -> i32;

    #[psp(0xD2E8363F)]
    pub fn QueryIntrHandlerInfo(
        intr_code: SceUid,
        sub_intr_code: SceUid,
        data: *mut IntrHandlerOptionParam,
    ) -> i32;
}

psp_extern! {
    #![name = "Kernel_Library"]
    #![flags = 0x0001]
    #![version = (0x00, 0x00)]

    #[psp(0x092968F4)]
    /// Suspend all interrupts.
    ///
    /// # Return Value
    ///
    /// The current state of the interrupt controller, to be used with `sceKernelCpuResumeIntr`.
    pub fn sceKernelCpuSuspendIntr() -> u32;

    #[psp(0x5F10D406)]
    /// Resume all interrupts.
    ///
    /// # Parameters
    ///
    /// - `flags`: The value returned from `sceKernelCpuSuspendIntr`.
    pub fn sceKernelCpuResumeIntr(flags: u32);

    #[psp(0x3B84732D)]
    /// Resume all interrupts (using sync instructions).
    ///
    /// # Parameters
    ///
    /// - `flags`: The value returned from `sceKernelCpuSuspendIntr`.
    pub fn sceKernelCpuResumeIntrWithSync(flags: u32);

    #[psp(0x47A0B729)]
    /// Determine if interrupts are suspended or active, based on the given flags.
    ///
    /// # Parameters
    ///
    /// - `flags`: The value returned from `sceKernelCpuSuspendIntr`.
    ///
    /// # Return Value
    ///
    /// 1 if flags indicate that interrupts were not suspended, 0 otherwise.
    pub fn sceKernelIsCpuIntrSuspended(flags: u32) -> i32;

    #[psp(0xB55249D2)]
    /// Determine if interrupts are enabled or disabled.
    ///
    /// # Return Value
    ///
    /// 1 if interrupts are currently enabled.
    pub fn sceKernelIsCpuIntrEnable() -> i32;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLMOption {
    pub size: usize,
    pub m_pid_text: SceUid,
    pub m_pid_data: SceUid,
    pub flags: u32,
    pub position: u8,
    pub access: u8,
    pub c_reserved: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSMOption {
    pub size: usize,
    pub m_pid_stack: SceUid,
    pub stack_size: usize,
    pub priority: i32,
    pub attribute: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleInfo {
    pub size: usize,
    pub n_segment: u8,
    pub reserved: [u8; 3usize],
    pub segment_addr: [i32; 4usize],
    pub segment_size: [i32; 4usize],
    pub entry_addr: u32,
    pub gp_value: u32,
    pub text_addr: u32,
    pub text_size: u32,
    pub data_size: u32,
    pub bss_size: u32,
    /// The following is only available in the v1.5 firmware and above, but as
    /// `sceKernelQueryModuleInfo` is broken in v1.0 it doesn't matter.
    pub attribute: u16,
    pub version: [u8; 2usize],
    pub name: [u8; 28usize],
}

psp_extern! {
    #![name = "ModuleMgrForUser"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x977DE386)]
    /// Load a module.
    ///
    /// This function restricts where it can load from (such as from flash0)
    /// unless you call it in kernel mode. It also must be called from a thread.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the module to load.
    /// - `flags`: Unused, always 0 .
    /// - `option`: Pointer to a `SceKernelLMOption` structure. Can be null.
    ///
    /// # Return Value
    ///
    /// The UID of the loaded module on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelLoadModule(
        path: *const u8,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;

    #[psp(0x710F61B5)]
    /// Load a module from MS.
    ///
    /// This function restricts what it can load, e.g. it wont load plain executables.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the module to load.
    /// - `flags`: Unused, set to 0.
    /// - `option`: Pointer to a `SceKernelLMOption` structure. Can be NULL.
    ///
    /// # Return Value
    ///
    /// The UID of the loaded module on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelLoadModuleMs(
        path: *const u8,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;

    #[psp(0xB7F46618)]
    /// Load a module from the given file UID.
    ///
    /// # Parameters
    ///
    /// - `fid`: The module's file UID.
    /// - `flags`: Unused, always 0.
    /// - `option`: Pointer to an optional `SceKernelLMOption` structure.
    ///
    /// # Return Value
    ///
    /// The UID of the loaded module on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelLoadModuleByID(
        fid: SceUid,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;

    #[psp(0xF9275D98)]
    /// Load a module from a buffer using the USB/WLAN API.
    ///
    /// Can only be called from kernel mode, or from a thread that has attributes
    /// of `0xa0000000`.
    ///
    /// # Parameters
    ///
    /// - `buf_size`: Size (in bytes) of the buffer pointed to by buf.
    /// - `buf`: Pointer to a buffer containing the module to load. The buffer
    ///          must reside at an address that is a multiple of 64 bytes.
    /// - `flags`: Unused, always 0.
    /// - `option`: Pointer to an optional `SceKernelLMOption` structure.
    ///
    /// # Return Value
    ///
    /// The UID of the loaded module on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelLoadModuleBufferUsbWlan(
        buf_size: usize,
        buf: *mut c_void,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;

    #[psp(0x50F0C1EC)]
    /// Start a loaded module.
    ///
    /// # Parameters
    ///
    /// - `mod_id`: The ID of the module returned from `sceKernelLoadModule*`.
    /// - `arg_size`: Length of the args.
    /// - `argp`: A pointer to the arguments to the module.
    /// - `status`: Returns the status of the start.
    /// - `option`: Pointer to an optional `SceKernelSMOption` structure.
    ///
    /// # Return Value
    ///
    /// ??? on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelStartModule(
        mod_id: SceUid,
        arg_size: usize,
        argp: *mut c_void,
        status: *mut i32,
        option: *mut SceKernelSMOption,
    ) -> i32;

    #[psp(0xD1FF982A)]
    /// Stop a running module.
    ///
    /// # Parameters
    ///
    /// - `mod_id`: The UID of the module to stop.
    /// - `arg_size`: The length of the arguments pointed to by argp.
    /// - `argp`: Pointer to arguments to pass to the module's `module_stop` routine.
    /// - `status`: Return value of the module's `module_stop` routine.
    /// - `option`: Pointer to an optional `SceKernelSMOption` structure.
    ///
    /// # Return Value
    ///
    /// ??? on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelStopModule(
        mod_id: SceUid,
        arg_size: usize,
        argp: *mut c_void,
        status: *mut i32,
        option: *mut SceKernelSMOption,
    ) -> i32;

    #[psp(0x2E0911AA)]
    /// Unload a stopped module.
    ///
    /// # Parameters
    ///
    /// - `mod_id`: The UID of the module to unload.
    ///
    /// # Return Value
    ///
    /// ??? on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelUnloadModule(mod_id: SceUid) -> i32;

    #[psp(0xD675EBB8)]
    /// Stop and unload the current module.
    ///
    /// # Parameters
    ///
    /// - `unknown`: Unknown (I've seen 1 passed).
    /// - `arg_size`: Size (in bytes) of the arguments that will be passed to `module_stop`.
    /// - `argp`: Pointer to arguments that will be passed to `module_stop`.
    ///
    /// # Return Value
    ///
    /// ??? on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelSelfStopUnloadModule(
        unknown: i32,
        arg_size: usize,
        argp: *mut c_void,
    ) -> i32;

    #[psp(0xCC1D3699)]
    /// Stop and unload the current module.
    ///
    /// # Parameters
    ///
    /// - `arg_size`: Size (in bytes) of the arguments that will be passed to `module_stop`.
    /// - `argp`: Poitner to arguments that will be passed to `module_stop`.
    /// - `status`: Return value from `module_stop`.
    /// - `option`: Pointer to an optional `SceKernelSMOption` structure.
    ///
    /// # Return Value
    ///
    /// ??? on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelStopUnloadSelfModule(
        arg_size: usize,
        argp: *mut c_void,
        status: *mut i32,
        option: *mut SceKernelSMOption,
    ) -> i32;

    #[psp(0x748CBED9)]
    /// Query the information about a loaded module from its UID.
    ///
    /// This fails on v1.0 firmware (and even it worked has a limited structure)
    /// so if you want to be compatible with both 1.5 and 1.0 (and you are
    /// running in kernel mode), then call this function first then
    /// `pspSdkQueryModuleInfoV1` if it fails, or make separate v1 and v1.5+
    /// builds.
    ///
    /// # Parameters
    ///
    /// - `mod_id`: The UID of the loaded module.
    /// - `info`: Pointer to a `SceKernelModuleInfo` structure.
    ///
    /// # Return Value
    ///
    /// 0 on success, otherwise one of `KernelErrorCodes`.
    pub fn sceKernelQueryModuleInfo(
        mod_id: SceUid,
        info: *mut SceKernelModuleInfo,
    ) -> i32;

    #[psp(0x644395E2)]
    /// Get a list of module IDs.
    ///
    /// This is only available on 1.5 firmware and above. For V1 use
    /// `pspSdkGetModuleIdList`.
    ///
    /// # Parameters
    ///
    /// - `read_buf`: Buffer to store the module list.
    /// - `read_buf_size`: Number of elements in the readbuffer.
    /// - `id_count`: Returns the number of module ids
    ///
    /// # Return Value
    ///
    /// >= 0 on success
    pub fn sceKernelGetModuleIdList(
        read_buf: *mut SceUid,
        read_buf_size: i32,
        id_count: *mut i32,
    ) -> i32;
}

psp_extern! {
    #![name = "sceSuspendForUser"]
    #![flags = 0x4000]
    #![version = (0x00, 0x00)]

    #[psp(0x3E0271D3)]
    /// Allocate the extra 4megs of RAM
    ///
    /// # Parameters
    ///
    /// - `unk`: No idea as it is never used, set to anything
    /// - `ptr`: Pointer to a pointer to hold the address of the memory
    /// - `size`: Pointer to an int which will hold the size of the memory
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceKernelVolatileMemLock(
        unk: i32,
        ptr: *mut *mut c_void,
        size: *mut i32,
    ) -> i32;

    #[psp(0xA14F40B2)]
    /// Try and allocate the extra 4megs of RAM, will return an error if
    /// something has already allocated it
    ///
    /// # Parameters
    ///
    /// - `unk`: No idea as it is never used, set to anything
    /// - `ptr`: Pointer to a pointer to hold the address of the memory
    /// - `size`: Pointer to an int which will hold the size of the memory
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceKernelVolatileMemTryLock(
        unk: i32,
        ptr: *mut *mut c_void,
        size: *mut i32,
    ) -> i32;

    #[psp(0xA569E425)]
    /// Deallocate the extra 4 megs of RAM
    ///
    /// # Parameters
    ///
    /// - `unk`: Set to 0, otherwise it fails in 3.52+, possibly earlier
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceKernelVolatileMemUnlock(unk: i32) -> i32;
}

psp_extern! {
    #![name = "StdioForUser"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x172D316E)]
    /// Function to get the current standard in file no
    ///
    /// # Return Value
    ///
    /// The stdin fileno
    pub fn sceKernelStdin() -> SceUid;

    #[psp(0xA6BAB2E9)]
    /// Function to get the current standard out file no
    ///
    /// # Return Value
    ///
    /// The stdout fileno
    pub fn sceKernelStdout() -> SceUid;

    #[psp(0xF78BA90A)]
    /// Function to get the current standard err file no
    ///
    /// # Return Value
    ///
    /// The stderr fileno
    pub fn sceKernelStderr() -> SceUid;
}
