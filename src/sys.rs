use crate::eabi::{i5, i6};
use core::ffi::c_void;

#[macro_use]
mod macros;

// http://uofw.github.io/uofw/structSceStubLibraryEntryTable.html
#[repr(C)]
pub struct SceStubLibraryEntry {
    pub name: *const u8,
    pub version: [u8; 2],
    pub flags: u16,
    pub len: u8,
    pub v_stub_count: u8,
    pub stub_count: u16,
    pub nid_table: *const u32,
    pub stub_table: *const c_void,
}

unsafe impl Sync for SceStubLibraryEntry {}

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

/// Stores the state of the GE.
#[repr(C)]
pub struct GeContext {
    context: [u32; 512]
}

#[repr(C)]
/// Structure storing a stack (for CALL/RET)
pub struct GeStack {
    stack: [u32;8]
}

#[repr(C)]
/// Structure to hold the callback data
pub struct GeCallbackData {
    signal_func: fn(id: i32, arg: *const c_void),
    signal_arg: *const c_void,
    finish_func: fn(id: i32, arg: *const c_void),
    finish_arg: *const c_void,
}

#[repr(C)]
pub struct GeListArgs {
    size: u32,
    context: *const GeContext,
    num_stacks: u32,
    stacks: *const GeStack,
}

#[repr(C)]
/// Drawing queue interruption parameter
pub struct GeBreakParam {
    buf: [u32;4]
}

/// GE matrix types.
#[repr(i32)]
pub enum GeMatrixType {
    /// Bone matrices.
    Bone0 = 0,
    Bone1,
    Bone2,
    Bone3,
    Bone4,
    Bone5,
    Bone6,
    Bone7,
    /// World matrix
    World,
    /// View matrix
    View,
    /// Projection Matrix
    Projection,
    TexGen,
}


/// List status for sce_ge_list_sync() and sce_ge_draw_sync().
#[repr(i32)]
pub enum GeListState {
    Done = 0,
    Queued,
    DrawingDone,
    StallReached,
    CancelDone,
}

sys_lib! {
    #![name = "sceGe_user"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x1F6752AD)]
    /// Get the size of VRAM.
    ///
    /// # Return value
    ///
    /// The size of VRAM (in bytes).
    pub unsafe fn sce_ge_edram_get_size() -> u32;

    #[psp(0xE47E40E4)]
    /// Get the eDRAM address.
    ///
    /// # Return value
    ///
    /// A pointer to the base of the eDRAM.
    pub unsafe fn sce_ge_edram_get_addr() -> *const u8;

    #[psp(0xB77905EA)]
    /// Set the eDRAM address translation mode.
    ///
    /// # Parameters
    ///
    /// `width` - 0 to not set the translation width, otherwise 512, 1024, 2048 or 4096.
    ///
    /// # Return value
    ///
    /// The previous width if it was set, otherwise 0, <0 on error.
    pub unsafe fn sce_ge_edram_set_addr_translation(width: i32) -> i32;

    #[psp(0xDC93CFEF)]
    /// Retrieve the current value of a GE command.
    ///
    /// # Parameters
    ///
    /// `cmd` - The GE command register to retrieve (0 to 0xFF, both included).
    ///
    /// # Return value
    ///
    /// The value of the GE command, < 0 on error.
    pub unsafe fn sce_ge_get_cmd(cmd: i32) -> u32;

    #[psp(0x57C8945B)]
    /// Retrieve a matrix of the given type.
    ///
    /// # Parameters
    ///
    /// `type_` - One of MatrixTypes.
    /// `matrix` - Pointer to a variable to store the matrix.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ge_get_mtx(type_: GeMatrixType, matrix: *mut c_void) -> i32;

    #[psp(0xE66CB92E)]
    /// Retrieve the stack of the display list currently being executed.
    ///
    /// # Parameters
    ///
    /// `stack_id` - The ID of the stack to retrieve.
    /// `stack` - Pointer to a structure to store the stack, or NULL to not store it.
    ///
    /// # Return value
    ///
    /// The number of stacks of the current display list, < 0 on error.
    pub unsafe fn sce_ge_get_stack(stack_id: i32, stack: *mut GeStack) -> i32;

    #[psp(0x438A385A)]
    /// Save the GE's current state.
    ///
    /// # Parameters
    ///
    /// `context` - Pointer to a GeContext.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ge_save_context(context: *mut GeContext) -> i32;

    #[psp(0x0BF608FB)]
    /// Restore a previously saved GE context.
    ///
    /// # Parameters
    ///
    /// `context` - Pointer to a GeContext.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ge_restore_context(context: *const GeContext) -> i32;

    #[psp(0xAB49E76A)]
    /// Enqueue a display list at the tail of the GE display list queue.
    ///
    /// # Parameters
    ///
    /// `list` - The head of the list to queue.
    /// `stall` - The stall address. If NULL then no stall address is set and the list is
    /// transferred  immediately.
    /// `cbid` - ID of the callback set by calling sce_ge_set_callback
    /// `arg` - Structure containing GE context buffer address
    ///
    /// # Return value
    ///
    /// ID of the queue, < 0 on error.
    pub unsafe fn sce_ge_list_enqueue(list: *const c_void, stall: *const c_void, cbid: i32, arg: GeListArgs) -> i32;

    #[psp(0x1C0D95A6)]
    /// Enqueue a display list at the head of the GE display list queue.
    ///
    /// # Parameters
    ///
    /// `list` - The head of the list to queue.
    /// `stall` - The stall address. If NULL then no stall address is set and the list is
    /// transferred  immediately.
    /// `cbid` - ID of the callback set by calling sce_ge_set_callback
    /// `arg` - Structure containing GE context buffer address
    ///
    /// # Return value
    ///
    /// ID of the queue, < 0 on error.
    pub unsafe fn sce_ge_list_enqueue_head(list: *const c_void, stall: *const c_void, cbid: i32, arg: GeListArgs) -> i32;

   #[psp(0x5FB86AB0)]
    /// Cancel a queued or running list.
    ///
    /// # Parameters
    ///
    /// `qid` - The ID of the queue.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ge_list_dequeue(qid: i32) -> i32;

    #[psp(0xE0D68148)]
    /// Update the stall address for the specified queue.
    ///
    /// # Parameters
    ///
    /// `qid` - The ID of the queue.
    /// `stall` - The new stall address.
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub unsafe fn sce_ge_list_update_stall_addr(qid: i32, stall: *const c_void) -> i32;

    #[psp(0x03444EB4)]
    /// Wait for synchronisation of a list.
    ///
    /// # Parameters
    ///
    /// `qid` - The queue ID of the list to sync.
    /// `sync_type` - 0 if you want to wait for the list to be completed, or 1 if you just 
    /// want to peek the actual state.
    ///
    /// # Return value
    ///
    /// The specified queue status, one of GeListState.
    pub unsafe fn sce_ge_list_sync(qid: i32, sync_type: i32) -> i32;

    #[psp(0xB287BD61)]
    /// Wait for drawing to complete.
    ///
    /// # Parameters
    ///
    /// `syncType` - 0 if you want to wait for the drawing to be completed, or 1 if you
    /// just want to peek the actual state.
    ///
    /// # Return value
    /// 
    /// The current queue status, one of GeListState.
    pub unsafe fn sce_ge_draw_sync(sync_type: i32) -> i32;

    #[psp(0xB448EC0D)]
    /// Interrupt drawing queue.
    ///
    /// # Parameters
    ///
    /// `mode` - If set to 1, reset all the queues.
    /// `p_param` - Unused (just K1-checked).
    ///
    /// # Return value
    ///
    /// The stopped queue ID if mode isnt set to 0, otherwise 0, and < 0 on error.
    pub unsafe fn sce_ge_break(mode: i32, p_param: *const GeBreakParam) -> i32;

    #[psp(0x4C06E472)]
    /// Restart drawing queue.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ge_continue() -> i32;

    #[psp(0xA4FC06A4)]
    /// Register callback handlers for the GE.
    ///
    /// # Parameters
    ///
    /// `cb` - Configured callback data structure.
    ///
    /// # Return value 
    ///
    /// The callback ID, < 0 on error.
    pub unsafe fn sce_ge_set_callback(cb: *const GeCallbackData) -> i32;

    #[psp(0x05DB22CE)]
    /// Unregister the callback handlers.
    ///
    /// # Parameters
    ///
    /// `cbid` - The ID of the callbacks, returned by sce_ge_set_callback().
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub unsafe fn sce_ge_unset_callback(cbid: i32) -> i32;
}

#[repr(u32)]
/// Display mode.
///
/// Display modes other than LCD are unknown.
pub enum DisplayMode {
    // https://github.com/hrydgard/ppsspp/blob/25197451e5cdb1b83dc69fea14c501bdb1e13b1a/Core/HLE/sceDisplay.cpp#L922
    Lcd = 0,

    // TODO: What are the other modes?
}

sys_lib! {
    #![name = "sceDisplay"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x0E20F177)]
    /// Set display mode
    ///
    /// # Parameters
    ///
    /// `mode` - Display mode, normally `DisplayMode::Lcd`.
    /// `width` - Width of screen in pixels.
    /// `height` - Height of screen in pixels.
    ///
    /// # Return value
    ///
    /// ???
    pub unsafe fn sce_display_set_mode(mode: DisplayMode, width: usize, height: usize) -> u32;

    #[psp(0x289D82FE)]
    pub unsafe fn sce_display_set_frame_buf(
        top_addr: *const u8,
        buffer_width: usize,
        pixel_format: u32,
        sync: u32,
    ) -> u32;

    #[psp(0x984C27E7)]
    /// Wait for vertical blank start
    pub unsafe fn sce_display_wait_vblank_start() -> i32; 
}

pub const USBBUS_DRIVERNAME: &str = "USBBusDriver";

sys_lib! {
    #![name = "sceUsb"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0xAE5DE6AF)]
    pub unsafe fn sce_usb_start(driver_name: *const u8, size: u32, args: *const c_void);
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


