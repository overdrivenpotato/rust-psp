use crate::eabi::{i5, i6};
use core::ffi::c_void;

bitflags::bitflags!{
    pub struct EventFlagAttribute: u32 {
        const WAIT_MULTIPLE = 0x200;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelEventFlagOptParam {
    pub size: usize,
}

sys_lib! {
    #![name = "ThreadManForUser"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x55C20A00)]
    /// Create an event flag.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the event flag.
    /// - `attr`: Attributes from EventFlagAttribute
    /// - `bits`: Initial bit pattern.
    /// - `opt`: Options, set to NULL
    /// # Return Value
    ///
    /// < 0 on error. >= 0 event flag id.
    pub unsafe fn sce_kernel_create_event_flag(
        name: *const u8,
        attr: EventFlagAttribute,
        bits: i32,
        opt: *mut SceKernelEventFlagOptParam,
    ) -> SceUid;

    #[psp(0x446D8DE6, i6)]
    /// Create a thread.
    ///
    /// This function does not directly run a thread, it simply returns a thread
    /// ID which can be used as a handle to start the thread later.
    pub unsafe fn sce_kernel_create_thread(
        name: *const u8,
        entry: fn(argc: u32, argp: *const *const u8) -> u32,
        priority: u32,
        stack_size: u32,
        attributes: u32,

        // TODO
        options: *const u8,
    ) -> u32;

    #[psp(0xF475845D)]
    /// Start a created thread.
    ///
    /// id - Thread ID from `sce_kernel_create_thread`
    /// arglen - Length of the data pointed to by argp
    /// argp - Pointer to the arguments
    pub unsafe fn sce_kernel_start_thread(id: u32, arglen: u32, argp: *const u8) -> u32;

    #[psp(0xE81CAF8F)]
    /// Create callback.
    ///
    /// `name` - A textual name for the callback.
    /// `func` - A pointer to a function that will be called as the callback.
    /// `arg` - Argument for the callback?
    ///
    /// # Return value
    ///
    /// >= 0 A callback id which can be used in subsequent functions, < 0 an error.
    pub unsafe fn sce_kernel_create_callback(
        name: *const u8,
        cb: fn(arg1: u32, arg2: u32, arg: *const u8) -> u32,
        arg: *const u8,
    ) -> u32;

    #[psp(0x82826F70)]
    /// Sleep thread but service any callbacks as necessary.
    ///
    /// Once all callbacks have been setup call this function.
    pub unsafe fn sce_kernel_sleep_thread_cb() -> u32;

    #[psp(0x809CE29B)]
    /// Exit a thread and delete itself.
    ///
    /// # Parameters
    ///
    /// `status` - Exit status
    pub unsafe fn sce_kernel_exit_delete_thread(status: i32) -> i32;
}

sys_lib! {
    #![name = "LoadExecForUser"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x05572A5F)]
    pub unsafe fn sce_kernel_exit_game();

    #[psp(0x4AC57943)]
    /// Register callback.
    ///
    /// By installing the exit callback the home button becomes active. However
    /// if `sce_kernel_exit_game` is not called in the callback it is likely
    /// that the PSP will just crash.
    ///
    /// # Parameters
    /// `cbid` - Callback id
    ///
    /// # Return value
    /// < 0 on error
    pub unsafe fn sce_kernel_register_exit_callback(id: u32) -> u32;
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceUid(pub i32);

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

sys_lib! {
    #![name = "SysMemUserForUser"]
    #![flags = 0x4000]
    #![version = (0, 0)]

    #[psp(0x237DBD4F, i5)]
    /// Allocate a memory block from a memory partition.
    ///
    /// # Parameters
    ///
    /// `partitionid` - The UID of the partition to allocate from.
    ///
    /// `name` - Name assigned to the new block.
    ///
    /// `type` - Specifies how the block is allocated within the partition. One
    ///          of `SysMemBlockTypes`.
    ///
    /// `size` - Size of the memory block, in bytes.
    ///
    /// `addr` - If type is PSP_SMEM_Addr, then addr specifies the lowest address
    ///          allocate the block from.
    ///
    /// # Return value
    ///
    /// The UID of the new block, or if less than 0 an error.
    pub unsafe fn sce_kernel_alloc_partition_memory(
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
    pub unsafe fn sce_kernel_get_block_head_addr(blockid: SceUid) -> *mut c_void;

    #[psp(0xB6D61D02)]
    /// Free a memory block allocated with `sce_kernel_alloc_partition_memory`.
    ///
    /// # Parameters
    ///
    /// `blockid` - UID of the block to free.
    ///
    /// # Return value
    ///
    /// ? on success, less than 0 on error.
    pub unsafe fn sce_kernel_free_partition_memory(blockid: SceUid) -> i32;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeVal {
    pub tv_sec: i32,
    pub tv_usec: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Timezone {
    pub tz_minutes_west: i32,
    pub tz_dst_time: i32,
}

/// Type to hold a sha1 context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sha1Context {
    pub h: [u32; 5usize],
    pub us_remains: u16,
    pub us_computed: u16,
    pub ull_total_len: u64,
    pub buf: [u8; 64usize],
}

/// Structure for holding a mersenne twister context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mt19937Context {
    pub count: u32,
    pub state: [u32; 624usize],
}

/// Structure to hold the MD5 context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Md5Context {
    pub h: [u32; 4usize],
    pub pad: u32,
    pub us_remains: u16,
    pub us_computed: u16,
    pub ull_total_len: u64,
    pub buf: [u8; 64usize],
}

sys_lib! {
    #![name = "UtilsForUser"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x27CC57F0)]
    /// Get the time in seconds since the epoc (1st Jan 1970)
    ///
    pub unsafe fn sce_kernel_libc_time(t: *mut i32) -> i32;

    #[psp(0x91E4F6A7)]
    /// Get the processor clock used since the start of the process
    pub unsafe fn sce_kernel_libc_clock() -> u32;

    #[psp(0x71EC4271)]
    /// Get the current time of time and time zone information
    pub unsafe fn sce_kernel_libc_gettimeofday(tp: *mut TimeVal, tzp: *mut Timezone)
        -> i32;

    #[psp(0x79D1C3FA)]
    /// Write back the data cache to memory
    pub unsafe fn sce_kernel_dcache_writeback_all();

    #[psp(0xB435DEC5)]
    /// Write back and invalidate the data cache
    pub unsafe fn sce_kernel_dcache_writeback_invalidate_all();

    #[psp(0x3EE30821)]
    /// Write back a range of addresses from the data cache to memory
    pub unsafe fn sce_kernel_dcache_writeback_range(
        p: *const c_void,
        size: u32,
    );

    #[psp(0x34B9FA9E)]
    /// Write back and invalidate a range of addresses in the data cache
    pub unsafe fn sce_kernel_dcache_writeback_invalidate_range(
        p: *const c_void,
        size: u32,
    );

    #[psp(0xBFA98062)]
    /// Invalidate a range of addresses in data cache
    pub unsafe fn sce_kernel_dcache_invalidate_range(
        p: *const c_void,
        size: u32,
    );

    #[psp(0x920F104A)]
    /// Invalidate the instruction cache
    pub unsafe fn sce_kernel_icache_invalidate_all();

    #[psp(0xC2DF770E)]
    /// Invalidate a range of addresses in the instruction cache
    pub unsafe fn sce_kernel_icache_invalidate_range(
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
    pub unsafe fn sce_kernel_utils_mt19937_init(
        ctx: *mut Mt19937Context,
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
    pub unsafe fn sce_kernel_utils_mt19937_uint(ctx: *mut Mt19937Context) -> u32;

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
    pub unsafe fn sce_kernel_utils_md5_digest(
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
    pub unsafe fn sce_kernel_utils_md5_block_init(ctx: *mut Md5Context) -> i32;

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
    pub unsafe fn sce_kernel_utils_md5_block_update(
        ctx: *mut Md5Context,
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
    pub unsafe fn sce_kernel_utils_md5_block_result(
        ctx: *mut Md5Context,
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
    pub unsafe fn sce_kernel_utils_sha1_digest(
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
    pub unsafe fn sce_kernel_utils_sha1_block_init(
        ctx: *mut Sha1Context,
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
    pub unsafe fn sce_kernel_utils_sha1_block_update(
        ctx: *mut Sha1Context,
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
    pub unsafe fn sce_kernel_utils_sha1_block_result(
        ctx: *mut Sha1Context,
        digest: *mut u8,
    ) -> i32;
}
