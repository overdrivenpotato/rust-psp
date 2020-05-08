use crate::eabi::{i5, i6};
use core::ffi::c_void;

sys_lib! {
    #![name = "ThreadManForUser"]
    #![flags = 0x4001]
    #![version = (0, 0)]

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



