use core::ffi::c_void;

/// Stores the state of the GE.
#[repr(C)]
pub struct GeContext {
    pub context: [u32; 512]
}

#[repr(C)]
/// Structure storing a stack (for CALL/RET)
pub struct GeStack {
    pub stack: [u32;8]
}

#[repr(C)]
/// Structure to hold the callback data
pub struct GeCallbackData {
    pub signal_func: fn(id: i32, arg: *mut c_void),
    pub signal_arg: *mut c_void,
    pub finish_func: fn(id: i32, arg: *mut c_void),
    pub finish_arg: *mut c_void,
}

#[repr(C)]
pub struct GeListArgs {
    pub size: u32,
    pub context: *mut GeContext,
    pub num_stacks: u32,
    pub stacks: *mut GeStack,
}

impl Default for GeListArgs {
    fn default() -> Self {
        Self {
            size: 0,
            context: core::ptr::null_mut(),
            num_stacks: 0,
            stacks: core::ptr::null_mut()
        }
    }
}

#[repr(C)]
/// Drawing queue interruption parameter
pub struct GeBreakParam {
    pub buf: [u32;4]
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
    pub unsafe fn sce_ge_edram_get_addr() -> *mut u8;

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
    pub unsafe fn sce_ge_list_enqueue(list: *const c_void, stall: *mut c_void, cbid: i32, arg: *mut GeListArgs) -> i32;

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
    pub unsafe fn sce_ge_list_enqueue_head(list: *const c_void, stall: *mut c_void, cbid: i32, arg: *mut GeListArgs) -> i32;

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
    pub unsafe fn sce_ge_list_update_stall_addr(qid: i32, stall: *mut c_void) -> i32;

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
    pub unsafe fn sce_ge_break(mode: i32, p_param: *mut GeBreakParam) -> i32;

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
    pub unsafe fn sce_ge_set_callback(cb: *mut GeCallbackData) -> i32;

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
