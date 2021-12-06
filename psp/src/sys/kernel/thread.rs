//! ThreadMan Thread Manager Library
//!
//! Library imports for the kernel threading library.
//!
//! Note: Some of the structures, types, and definitions in this file were
//! extrapolated from symbolic debugging information found in the Japanese
//! version of Puzzle Bobble.

use super::SceUid;
use crate::eabi::i6;
use core::ffi::c_void;

/// Structure to hold the psp profiler register values
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DebugProfilerRegs {
    pub enable: u32,
    pub systemck: u32,
    pub cpuck: u32,
    pub internal: u32,
    pub memory: u32,
    pub copz: u32,
    pub vfpu: u32,
    pub sleep: u32,
    pub bus_access: u32,
    pub uncached_load: u32,
    pub uncached_store: u32,
    pub cached_load: u32,
    pub cached_store: u32,
    pub i_miss: u32,
    pub d_miss: u32,
    pub d_writeback: u32,
    pub cop0_inst: u32,
    pub fpu_inst: u32,
    pub vfpu_inst: u32,
    pub local_bus: u32,
}

/// 64-bit system clock type.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSysClock {
    pub low: u32,
    pub hi: u32,
}

pub type SceKernelThreadEntry = unsafe extern "C" fn(args: usize, argp: *mut c_void) -> i32;

/// Additional options used when creating threads.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadOptParam {
    /// Size of the `SceKernelThreadOptParam` structure.
    pub size: usize,

    /// UID of the memory block (?) allocated for the thread's stack.
    pub stack_mpid: SceUid,
}

bitflags::bitflags! {
    /// Attributes for threads.
    #[repr(transparent)]
    pub struct ThreadAttributes: u32 {
        /// Enable VFPU access for the thread.
        const VFPU = 0x00004000;

        /// Start the thread in user mode (done automatically if the thread
        /// creating it is in user mode).
        const USER = 0x80000000;

        /// Thread is part of the USB/WLAN API.
        const USBWLAN = 0xa0000000;

        /// Thread is part of the VSH API.
        const VSH = 0xc0000000;

        /// Allow using scratchpad memory for a thread, NOT USABLE ON V1.0
        const SCRATCH_SRAM = 0x00008000;

        /// Disables filling the stack with `0xFF` on creation.
        const NO_FILLSTACK = 0x00100000;

        /// Clear the stack when the thread is deleted.
        const CLEAR_STACK = 0x00200000;
    }
}

bitflags::bitflags! {
    /// Event flag creation attributes.
    #[repr(transparent)]
    pub struct EventFlagAttributes: u32 {
        /// Allow the event flag to be waited upon by multiple threads.
        const WAIT_MULTIPLE = 0x200;
    }
}

bitflags::bitflags! {
    /// Event flag wait types
    #[repr(transparent)]
    pub struct EventFlagWaitTypes: u32 {
        /// Wait for all bits in the pattern to be set
        const AND = 0;
        /// Wait for one or more bits in the pattern to be set
        const OR  = 1;
        /// Clear the wait pattern when it matches
        const CLEAR = 0x20;
    }
}

/// Structure to hold the status information for a thread
/// @see sceKernelReferThreadStatus
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadInfo {
    /// Size of the structure
    pub size: usize,
    /// Null terminated name of the thread
    pub name: [u8; 32],
    /// Thread attributes
    pub attr: u32,
    /// Thread status
    pub status: i32,
    /// Thread entry point
    pub entry: SceKernelThreadEntry,
    /// Thread stack pointer
    pub stack: *mut c_void,
    /// Thread stack size
    pub stack_size: i32,
    /// Pointer to the gp
    pub gp_reg: *mut c_void,
    /// Initial priority
    pub init_priority: i32,
    /// Current priority
    pub current_priority: i32,
    /// Wait type
    pub wait_type: i32,
    /// Wait ID
    pub wait_id: SceUid,
    /// Wakeup count
    pub wakeup_count: i32,
    /// Exit status of the thread
    pub exit_status: i32,
    /// Number of clock cycles run
    pub run_clocks: SceKernelSysClock,
    /// Interrupt preemption count
    pub intr_preempt_count: u32,
    /// Thread preemption count
    pub thread_preempt_count: u32,
    /// Release count
    pub release_count: u32,
}

/// Statistics about a running thread. See `sceKernelReferThreadRunStatus`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadRunStatus {
    pub size: usize,
    pub status: i32,
    pub current_priority: i32,
    pub wait_type: i32,
    pub wait_id: i32,
    pub wakeup_count: i32,
    pub run_clocks: SceKernelSysClock,
    pub intr_preempt_count: u32,
    pub thread_preempt_count: u32,
    pub release_count: u32,
}

/// Additional options used when creating semaphores.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSemaOptParam {
    /// Size of the `SceKernelSemaOptParam` structure.
    pub size: usize,
}

/// Current state of a semaphore. See `sceKernelReferSemaStatus`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSemaInfo {
    /// Size of the `SceKernelSemaInfo` structure.
    pub size: usize,
    /// Null terminated name of the semaphore.
    pub name: [u8; 32],
    /// Attributes.
    pub attr: u32,
    /// The initial count the semaphore was created with.
    pub init_count: i32,
    /// The current count.
    pub current_count: i32,
    /// The maximum count.
    pub max_count: i32,
    /// The number of threads waiting on the semaphore.
    pub num_wait_threads: i32,
}

/// Structure to hold the event flag information.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelEventFlagInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub init_pattern: u32,
    pub current_pattern: u32,
    pub num_wait_threads: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelEventFlagOptParam {
    /// Size of the `SceKernelEventFlagOptParam` structure?
    pub size: usize,
}

/// Additional options used when creating messageboxes.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMbxOptParam {
    /// Size of the `SceKernelMbxOptParam` structure.
    pub size: usize,
}

/// Current state of a messagebox. See `sceKernelReferMbxStatus`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMbxInfo {
    /// Size of the `SceKernelMbxInfo` structure.
    pub size: usize,
    /// Null terminated name of the messagebox.
    pub name: [u8; 32usize],
    /// Attributes.
    pub attr: u32,
    /// The number of threads waiting on the messagebox.
    pub num_wait_threads: i32,
    /// Number of messages currently in the messagebox.
    pub num_messages: i32,
    /// The message currently at the head of the queue.
    pub first_message: *mut c_void,
}

pub type SceKernelVTimerHandler = unsafe extern "C" fn(
    uid: SceUid,
    arg1: *mut SceKernelSysClock,
    arg2: *mut SceKernelSysClock,
    arg3: *mut c_void,
) -> u32;

pub type SceKernelVTimerHandlerWide =
    unsafe extern "C" fn(uid: SceUid, arg1: i64, arg2: i64, arg3: *mut c_void) -> u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelVTimerInfo {
    /// Size of the `SceKernelVTimerInfo` structure?
    pub size: usize,
    pub name: [u8; 32],
    pub active: i32,
    pub base: SceKernelSysClock,
    pub current: SceKernelSysClock,
    pub schedule: SceKernelSysClock,
    pub handler: SceKernelVTimerHandler,
    pub common: *mut c_void,
}

// TODO: Is this ok? What if the thread has no event handler registered?
pub type SceKernelThreadEventHandler =
    unsafe extern "C" fn(mask: i32, thid: SceUid, common: *mut c_void) -> i32;

/// Struct for event handler info
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadEventHandlerInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub thread_id: SceUid,
    pub mask: i32,
    // TODO: Make this option?
    pub handler: SceKernelThreadEventHandler,
    pub common: *mut c_void,
}

/// Prototype for alarm handlers.
pub type SceKernelAlarmHandler = unsafe extern "C" fn(common: *mut c_void) -> u32;

/// Struct containing alarm info
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelAlarmInfo {
    /// Size of the structure (should be set before calling `sceKernelReferAlarmStatus`).
    pub size: usize,
    pub schedule: SceKernelSysClock,
    /// Pointer to the alarm handler
    pub handler: SceKernelAlarmHandler,
    /// Common pointer argument
    pub common: *mut c_void,
}

/// Threadman types.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SceKernelIdListType {
    Thread = 1,
    Semaphore = 2,
    EventFlag = 3,
    Mbox = 4,
    Vpl = 5,
    Fpl = 6,
    Mpipe = 7,
    Callback = 8,
    ThreadEventHandler = 9,
    Alarm = 10,
    VTimer = 11,
    SleepThread = 64,
    DelayThread = 65,
    SuspendThread = 66,
    DormantThread = 67,
}

/// Structure to contain the system status returned by `sceKernelReferSystemStatus`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSystemStatus {
    /// Size of the structure (should be set prior to the call).
    pub size: usize,
    /// The status?
    pub status: u32,
    /// The number of cpu clocks in the idle thread
    pub idle_clocks: SceKernelSysClock,
    /// Number of times we resumed from idle
    pub comes_out_of_idle_count: u32,
    /// Number of thread context switches
    pub thread_switch_count: u32,
    /// Number of vfpu switches ?
    pub vfpu_switch_count: u32,
}

/// Message Pipe status info
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMppInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub buf_size: i32,
    pub free_size: i32,
    pub num_send_wait_threads: i32,
    pub num_receive_wait_threads: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelVplOptParam {
    /// Size of the `SceKernelVplOptParam` structure?
    pub size: usize,
}

/// Variable pool status info
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelVplInfo {
    /// Size of the `SceKernelVplInfo` structure?
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub pool_size: i32,
    pub free_size: i32,
    pub num_wait_threads: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelFplOptParam {
    /// Size of the `SceKernelFplOptParam` structure?
    pub size: usize,
}

/// Fixed pool status information
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelFplInfo {
    pub size: usize,
    pub name: [u8; 32usize],
    pub attr: u32,
    pub block_size: i32,
    pub num_blocks: i32,
    pub free_blocks: i32,
    pub num_wait_threads: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelVTimerOptParam {
    /// Size of the `SceKernelVTimerOptParam` structure?
    pub size: usize,
}

/// Callback function prototype
pub type SceKernelCallbackFunction =
    unsafe extern "C" fn(arg1: i32, arg2: i32, arg: *mut c_void) -> i32;

/// Structure to hold the status information for a callback
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelCallbackInfo {
    /// Size of the structure (i.e. `sizeof::<SceKernelCallbackInfo>()`)
    pub size: usize,
    /// The name given to the callback
    pub name: [u8; 32usize],
    /// The thread id associated with the callback
    pub thread_id: SceUid,
    /// Pointer to the callback function
    pub callback: SceKernelCallbackFunction,
    /// User supplied argument for the callback
    pub common: *mut c_void,
    /// Unknown
    pub notify_count: i32,
    /// Unknown
    pub notify_arg: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLwMutexWork {
    /// Count
    pub lock_count: i32,
    /// Locking thread
    pub lock_thread: SceUid,
    /// Attributes
    pub attr: i32,
    /// Number of waiting threads
    pub num_wait_threads: i32,
    /// UID
    pub uid: SceUid,
    /// Padding
    pub pad: [i32; 3],
}

psp_extern! {
    #![name = "ThreadManForUser"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x57CF62DD)]
    /// Get the type of a threadman uid
    ///
    /// # Parameters
    ///
    /// - `uid`: The uid to get the type from
    ///
    /// # Return Value
    ///
    /// The type, < 0 on error
    pub fn sceKernelGetThreadmanIdType(uid: SceUid) -> SceKernelIdListType;

    #[psp(0x446D8DE6, i6)]
    /// Create a thread.
    ///
    /// This function does not directly run a thread, it simply returns a thread
    /// ID which can be used as a handle to start the thread later. See
    /// `sceKernelStartThread`.
    ///
    /// # Parameters
    ///
    /// - `name`: An arbitrary thread name.
    /// - `entry`: The thread function to run when started.
    /// - `init_priority`: The initial priority of the thread. Less if higher priority.
    /// - `stack_size`: The size of the initial stack.
    /// - `attr`: The thread attributes, zero or more of `ThreadAttributes`.
    /// - `option`: Additional options specified by `SceKernelThreadOptParam`.
    ///
    /// # Return Value
    ///
    /// UID of the created thread, or an error code.
    pub fn sceKernelCreateThread(
        name: *const u8,
        entry: SceKernelThreadEntry,
        init_priority: i32,
        stack_size: i32,
        attr: ThreadAttributes,
        option: *mut SceKernelThreadOptParam,
    ) -> SceUid;

    #[psp(0x9FA03CD3)]
    /// Delate a thread
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to be deleted.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelDeleteThread(thid: SceUid) -> i32;

    #[psp(0xF475845D)]
    /// Start a created thread.
    ///
    /// # Parameters
    ///
    /// - `id`: Thread id from sceKernelCreateThread
    /// - `arg_len`: Length of the data pointed to by argp, in bytes
    /// - `argp`: Pointer to the arguments.
    pub fn sceKernelStartThread(
        id: SceUid,
        arg_len: usize,
        arg_p: *mut c_void,
    ) -> i32;

    #[psp(0xAA73C935)]
    /// Exit a thread
    ///
    /// # Parameters
    ///
    /// - `status`: Exit status.
    pub fn sceKernelExitThread(status: i32) -> i32;

    #[psp(0x809CE29B)]
    /// Exit a thread and delete itself.
    ///
    /// # Parameters
    ///
    /// - `status`: Exit status
    pub fn sceKernelExitDeleteThread(status: i32) -> i32;

    #[psp(0x616403BA)]
    /// Terminate a thread.
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to terminate.
    ///
    /// # Return Value
    ///
    /// Success if >= 0, an error if < 0.
    pub fn sceKernelTerminateThread(thid: SceUid) -> i32;

    #[psp(0x383F7BCC)]
    /// Terminate and delete a thread.
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to terminate and delete.
    ///
    /// # Return Value
    ///
    /// Success if >= 0, an error if < 0.
    pub fn sceKernelTerminateDeleteThread(thid: SceUid) -> i32;

    #[psp(0x3AD58B8C)]
    /// Suspend the dispatch thread
    ///
    /// # Return Value
    ///
    /// The current state of the dispatch thread, < 0 on error
    pub fn sceKernelSuspendDispatchThread() -> i32;

    #[psp(0x27E22EC2)]
    /// Resume the dispatch thread
    ///
    /// # Parameters
    ///
    /// - `state`: The state of the dispatch thread
    ///   (from `sceKernelSuspendDispatchThread`)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelResumeDispatchThread(state: i32) -> i32;

    #[psp(0x9ACE131E)]
    /// Sleep thread
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelSleepThread() -> i32;

    #[psp(0x82826F70)]
    /// Sleep thread but service any callbacks as necessary.
    ///
    /// Once all callbacks have been setup call this function.
    pub fn sceKernelSleepThreadCB() -> i32;

    #[psp(0xD59EAD2F)]
    /// Wake a thread previously put into the sleep state.
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to wake.
    ///
    /// # Return Value
    ///
    /// Success if >= 0, an error if < 0.
    pub fn sceKernelWakeupThread(thid: SceUid) -> i32;

    #[psp(0xFCCFAD26)]
    /// Cancel a thread that was to be woken with `sceKernelWakeupThread`.
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to cancel.
    ///
    /// # Return Value
    ///
    /// Success if >= 0, an error if < 0.
    pub fn sceKernelCancelWakeupThread(thid: SceUid) -> i32;

    #[psp(0x9944F31F)]
    /// Suspend a thread.
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to suspend.
    ///
    /// # Return Value
    ///
    /// Success if >= 0, an error if < 0.
    pub fn sceKernelSuspendThread(thid: SceUid) -> i32;

    #[psp(0x75156E8F)]
    /// Resume a thread previously put into a suspended state with `sceKernelSuspendThread`.
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to resume.
    ///
    /// # Return Value
    ///
    /// Success if >= 0, an error if < 0.
    pub fn sceKernelResumeThread(thid: SceUid) -> i32;

    #[psp(0x278C0DF5)]
    /// Wait until a thread has ended.
    ///
    /// # Parameters
    ///
    /// - `thid`: Id of the thread to wait for.
    /// - `timeout`: Timeout in microseconds (assumed).
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelWaitThreadEnd(thid: SceUid, timeout: *mut u32) -> i32;

    #[psp(0x840E8133)]
    /// Wait until a thread has ended and handle callbacks if necessary.
    ///
    /// # Parameters
    ///
    /// - `thid`: Id of the thread to wait for.
    /// - `timeout`: Timeout in microseconds (assumed).
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelWaitThreadEndCB(thid: SceUid, timeout: *mut u32) -> i32;

    #[psp(0xCEADEB47)]
    /// Delay the current thread by a specified number of microseconds
    ///
    /// # Parameters
    ///
    /// - `delay`: Delay in microseconds.
    ///
    pub fn sceKernelDelayThread(delay: u32) -> i32;

    #[psp(0x68DA9E36)]
    /// Delay the current thread by a specified number of microseconds and handle any callbacks.
    ///
    /// # Parameters
    ///
    /// - `delay`: Delay in microseconds.
    ///
    pub fn sceKernelDelayThreadCB(delay: u32) -> i32;

    #[psp(0xBD123D9E)]
    /// Delay the current thread by a specified number of sysclocks
    ///
    /// # Parameters
    ///
    /// - `delay`: Delay in sysclocks
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelDelaySysClockThread(delay: *mut SceKernelSysClock) -> i32;

    #[psp(0x1181E963)]
    /// Delay the current thread by a specified number of sysclocks handling callbacks
    ///
    /// # Parameters
    ///
    /// - `delay`: Delay in sysclocks
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    ///
    pub fn sceKernelDelaySysClockThreadCB(delay: *mut SceKernelSysClock) -> i32;

    #[psp(0xEA748E31)]
    /// Modify the attributes of the current thread.
    ///
    /// # Parameters
    ///
    /// - `unknown`: Set to 0.
    /// - `attr`: The thread attributes to modify.  One of `ThreadAttributes`.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelChangeCurrentThreadAttr(
        unknown: i32,
        attr: ThreadAttributes,
    ) -> i32;

    #[psp(0x71BC9871)]
    /// Change the threads current priority.
    ///
    /// # Parameters
    ///
    /// - `thid`: The ID of the thread (from `sceKernelCreateThread` or `sceKernelGetThreadId`)
    /// - `priority`: The new priority (the lower the number the higher the priority)
    ///
    /// # Return Value
    ///
    /// 0 if successful, otherwise the error code.
    pub fn sceKernelChangeThreadPriority(
        thid: SceUid,
        priority: i32,
    ) -> i32;

    #[psp(0x912354A7)]
    /// Rotate thread ready queue at a set priority
    ///
    /// # Parameters
    ///
    /// - `priority`: The priority of the queue
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceKernelRotateThreadReadyQueue(
        priority: i32,
    ) -> i32;

    #[psp(0x2C34E053)]
    /// Release a thread in the wait state.
    ///
    /// # Parameters
    ///
    /// - `thid`: The UID of the thread.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReleaseWaitThread(thid: SceUid) -> i32;

    #[psp(0x293B45B8)]
    /// Get the current thread Id
    ///
    /// # Return Value
    ///
    /// The thread id of the calling thread.
    pub fn sceKernelGetThreadId() -> i32;

    #[psp(0x94AA61EE)]
    /// Get the current priority of the thread you are in.
    ///
    /// # Return Value
    ///
    /// The current thread priority
    pub fn sceKernelGetThreadCurrentPriority() -> i32;

    #[psp(0x3B183E26)]
    /// Get the exit status of a thread.
    ///
    /// # Parameters
    ///
    /// - `thid`: The UID of the thread to check.
    ///
    /// # Return Value
    ///
    /// The exit status
    pub fn sceKernelGetThreadExitStatus(thid: SceUid) -> i32;

    #[psp(0xD13BDE95)]
    /// Check the thread stack?
    ///
    /// # Return Value
    ///
    /// Unknown.
    pub fn sceKernelCheckThreadStack() -> i32;

    #[psp(0x52089CA1)]
    /// Get the free stack size for a thread.
    ///
    /// # Parameters
    ///
    /// - `thid`: The thread ID. Seem to take current thread if set to 0.
    ///
    /// # Return Value
    ///
    /// The free size.
    pub fn sceKernelGetThreadStackFreeSize(thid: SceUid) -> i32;

    #[psp(0x17C1684E)]
    /// Get the status information for the specified thread.
    ///
    /// # Parameters
    ///
    /// - `thid`: Id of the thread to get status
    /// - `info`: Pointer to the info structure to receive the data.
    ///
    ///   Note: The structures size field should be set to
    ///   `core::mem::size_of(SceKernelThreadInfo)` before calling this function.
    ///
    /// # Return Value
    ///
    /// 0 if successful, otherwise the error code.
    pub fn sceKernelReferThreadStatus(
        thid: SceUid,
        info: *mut SceKernelThreadInfo,
    ) -> i32;

    #[psp(0xFFC36A14)]
    /// Retrive the runtime status of a thread.
    ///
    /// # Parameters
    ///
    /// - `thid`: UID of the thread to retrive status.
    /// - `status`: Pointer to a `SceKernelThreadRunStatus` struct to receive the runtime status.
    ///
    /// # Return Value
    ///
    /// 0 if successful, otherwise the error code.
    pub fn sceKernelReferThreadRunStatus(
        thid: SceUid,
        status: *mut SceKernelThreadRunStatus,
    ) -> i32;

    #[psp(0xD6DA4BA1)]
    /// Creates a new semaphore
    ///
    /// # Parameters
    ///
    /// - `name`: Specifies the name of the sema
    /// - `attr`: Sema attribute flags (normally set to 0)
    /// - `init_val`: Sema initial value
    /// - `max_val`: Sema maximum value
    /// - `option`: Sema options (normally set to 0)
    /// # Return Value
    ///
    /// A semaphore id
    pub fn sceKernelCreateSema(
        name: *const u8,
        attr: u32,
        init_val: i32,
        max_val: i32,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUid;

    #[psp(0x28B6489C)]
    /// Destroy a semaphore
    ///
    /// # Parameters
    ///
    /// - `sema_id`: The semaid returned from a previous create call.
    ///
    /// # Return Value
    ///
    /// Returns the value 0 if it's succesful otherwise -1
    pub fn sceKernelDeleteSema(sema_id: SceUid) -> i32;

    #[psp(0x3F53E640)]
    /// Send a signal to a semaphore
    ///
    /// # Parameters
    ///
    /// - `sema_id`: The sema id returned from `sceKernelCreateSema`
    /// - `signal`: The amount to signal the sema (i.e. if 2 then increment the sema by 2)
    ///
    /// # Return Value
    ///
    /// < 0 On error.
    pub fn sceKernelSignalSema(
        sema_id: SceUid,
        signal: i32,
    ) -> i32;

    #[psp(0x4E3A1105)]
    /// Lock a semaphore
    ///
    /// # Parameters
    ///
    /// - `sema_id`: The sema id returned from `sceKernelCreateSema`
    /// - `signal`: The value to wait for (i.e. if 1 then wait till reaches a signal state of 1)
    /// - `timeout`: Timeout in microseconds (assumed).
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelWaitSema(
        sema_id: SceUid,
        signal: i32,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x6D212BAC)]
    /// Lock a semaphore a handle callbacks if necessary.
    ///
    /// # Parameters
    ///
    /// - `sema_id`: The sema id returned from `sceKernelCreateSema`
    /// - `signal`: The value to wait for (i.e. if 1 then wait till reaches a signal state of 1)
    /// - `timeout`: Timeout in microseconds (assumed).
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelWaitSemaCB(
        sema_id: SceUid,
        signal: i32,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x58B1F937)]
    /// Poll a sempahore.
    ///
    /// # Parameters
    ///
    /// - `sema_id`: UID of the semaphore to poll.
    /// - `signal`: The value to test for.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelPollSema(
        sema_id: SceUid,
        signal: i32,
    ) -> i32;

    #[psp(0xBC6FEBC5)]
    /// Retrieve information about a semaphore.
    ///
    /// # Parameters
    ///
    /// - `sema_id`: UID of the semaphore to retrieve info for.
    /// - `info`: Pointer to a `SceKernelSemaInfo` struct to receive the info.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReferSemaStatus(
        sema_id: SceUid,
        info: *mut SceKernelSemaInfo,
    ) -> i32;

    #[psp(0x19CFF145)]
    /// Create a lightweight mutex
    ///
    /// # Parameters
    ///
    /// - `mutex`: The pointer to the mutex
    /// - `name`: The name of the lightweight mutex
    /// - `attr`: Attributes of the lightweight mutex
    /// TODO: what values does this take?
    /// - `initial_count`: The inital value of the mutex
    /// - `options`: Other options for the mutex
    /// TODO: what values does this take?
    ///
    /// # Return Value
    ///
    /// 0 on success, otherwise an error code
    pub fn sceKernelCreateLwMutex(
        mutex: *mut SceKernelLwMutexWork,
        name: *const u8,
        attr: u32,
        initial_count: i32,
        options: *mut u32,
    ) -> i32;

    #[psp(0x60107536)]
    /// Delete a lightweight mutex
    ///
    /// # Parameters
    ///
    /// - `mutex`: Pointer to a lightweight mutex structure.
    ///
    /// # Return Value
    ///
    /// 0 on success, otherwise an error code
    pub fn sceKernelDeleteLwMutex(mutex: *mut SceKernelLwMutexWork) -> i32;

    #[psp(0xDC692EE3)]
    /// Tries to lock a lightweight mutex.
    ///
    /// This function is non-blocking. If the mutex is flagged as recursive, count can be >1. For more information, see http://linux.die.net/man/3/pthread_mutex_trylock
    ///
    /// # Parameters
    /// - `mutex`: Pointer to a lightweight mutex structure.
    /// - `count`: The lock counter increment.
    ///
    /// # Return Value
    /// SCE_ERROR_OK on success, otherwise SCE_ERROR_KERNEL_LWMUTEX_LOCKED on error.
    pub fn sceKernelTryLockLwMutex(
        mutex: *mut SceKernelLwMutexWork,
        count: i32,
    ) -> i32;

    #[psp(0xBEA46419)]
    /// Locks a lightweight mutex.
    ///
    /// This function can be blocking if the mutex is already locked. If the mutex is flagged as recursive, count can be >1. For more information, see http://linux.die.net/man/3/pthread_mutex_lock
    ///
    /// # Parameters
    /// - `mutex`: Pointer to a lightweight mutex structure.
    /// - `count`: The lock counter increment.
    /// - `timeout`: The timeout to expire after if the mutex cannot be locked.
    ///
    /// # Return Value
    /// SCE_ERROR_OK on success, otherwise <0 on error.
    pub fn sceKernelLockLwMutex(
        mutex: *mut SceKernelLwMutexWork,
        count: i32,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x1FC64E09)]
    /// Locks a lightweight mutex (callback).
    ///
    /// This function can be blocking if the mutex is already locked. If the mutex is flagged as recursive, count can be >1. For more information, see http://linux.die.net/man/3/pthread_mutex_lock
    ///
    /// # Parameters
    /// - `mutex`: Pointer to a lightweight mutex structure.
    /// - `count`: The lock counter increment.
    ///
    /// # Return Value
    /// SCE_ERROR_OK on success, otherwise <0 on error.
    pub fn sceKernelLockMutexCB(mutex: *mut SceKernelLwMutexWork, count: i32) -> i32;

    #[psp(0x15B6446B)]
    /// Unlocks a lightweight mutex.
    ///
    /// This function is non-blocking. If the mutex is flagged as recursive, count can be >1. For more information, see http://linux.die.net/man/3/pthread_mutex_unlock
    ///
    /// # Parameters
    /// - `mutex`: Pointer to a lightweight mutex structure.
    /// - `count`: The lock counter decrement.
    ///
    /// # Return Value
    /// SCE_ERROR_OK on success, otherwise <0 on error.
    pub fn sceKernelUnlockLwMutex(
        mutex: *mut SceKernelLwMutexWork,
        count: i32,
    ) -> i32;

    #[psp(0xC1734599)]
    /// TODO: Refers the lightweight mutex's status.
    ///
    /// # Parameters
    /// - `mutex`: Pointer to a lightweight mutex structure.
    /// - `addr`: Unknown.
    /// # Return Value
    /// Unknown, may be SCE_ERROR_OK on success and <0 on error.
    pub fn sceKernelReferLwMutexStatus(work_area: *mut SceKernelLwMutexWork, addr: *mut u32) -> i32;

    #[psp(0x55C20A00)]
    /// Create an event flag.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the event flag.
    /// - `attr`: Attributes from `EventFlagAttributes`.
    /// - `bits`: Initial bit pattern.
    /// - `opt`: Options, set to null.
    ///
    /// # Return Value
    ///
    /// < 0 on error. >= 0 event flag id.
    pub fn sceKernelCreateEventFlag(
        name: *const u8,
        attr: EventFlagAttributes,
        bits: i32,
        opt: *mut SceKernelEventFlagOptParam,
    ) -> SceUid;

    #[psp(0x1FB15A32)]
    /// Set an event flag bit pattern.
    ///
    /// # Parameters
    ///
    /// - `ev_id`: The event id returned by `sceKernelCreateEventFlag`.
    /// - `bits`: The bit pattern to set.
    ///
    /// # Return Value
    ///
    /// < 0 On error
    pub fn sceKernelSetEventFlag(ev_id: SceUid, bits: u32) -> i32;

    #[psp(0x812346E4)]
    /// Clear a event flag bit pattern
    ///
    /// # Parameters
    ///
    /// - `ev_id`: The event id returned by `sceKernelCreateEventFlag`
    /// - `bits`: The bits to clean
    ///
    /// # Return Value
    ///
    /// < 0 on Error
    pub fn sceKernelClearEventFlag(ev_id: SceUid, bits: u32) -> i32;

    #[psp(0x30FD48F0)]
    /// Poll an event flag for a given bit pattern.
    ///
    /// # Parameters
    ///
    /// - `ev_id`: The event id returned by `sceKernelCreateEventFlag`.
    /// - `bits`: The bit pattern to poll for.
    /// - `wait`: Wait type, one or more of `EventFlagWaitTypes` or'ed together
    /// - `out_bits`: The bit pattern that was matched.
    ///
    /// # Return Value
    ///
    /// < 0 On error
    pub fn sceKernelPollEventFlag(
        ev_id: SceUid,
        bits: u32,
        wait: EventFlagWaitTypes,
        out_bits: *mut u32,
    ) -> i32;

    #[psp(0x402FCF22)]
    /// Wait for an event flag for a given bit pattern.
    ///
    /// # Parameters
    ///
    /// - `ev_id`: The event id returned by sceKernelCreateEventFlag.
    /// - `bits`: The bit pattern to poll for.
    /// - `wait`: Wait type, one or more of `EventFlagWaitTypes` or'ed together
    /// - `out_bits`: The bit pattern that was matched.
    /// - `timeout`: Timeout in microseconds
    ///
    /// # Return Value
    ///
    /// < 0 On error
    pub fn sceKernelWaitEventFlag(
        ev_id: SceUid,
        bits: u32,
        wait: EventFlagWaitTypes,
        out_bits: *mut u32,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x328C546A)]
    /// Wait for an event flag for a given bit pattern with callback.
    ///
    /// # Parameters
    ///
    /// - `ev_id`: The event id returned by `sceKernelCreateEventFlag`.
    /// - `bits`: The bit pattern to poll for.
    /// - `wait`: Wait type, one or more of `EventFlagWaitTypes` or'ed together
    /// - `out_bits`: The bit pattern that was matched.
    /// - `timeout`: Timeout in microseconds
    /// # Return Value
    ///
    /// < 0 On error
    pub fn sceKernelWaitEventFlagCB(
        ev_id: SceUid,
        bits: u32,
        wait: EventFlagWaitTypes,
        out_bits: *mut u32,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0xEF9E4C70)]
    /// Delete an event flag
    ///
    /// # Parameters
    ///
    /// - `ev_id`: The event id returned by `sceKernelCreateEventFlag`.
    ///
    /// # Return Value
    ///
    /// < 0 On error
    pub fn sceKernelDeleteEventFlag(ev_id: SceUid) -> i32;

    #[psp(0xA66B0120)]
    /// Get the status of an event flag.
    ///
    /// # Parameters
    ///
    /// - `event`: The UID of the event.
    /// - `status`: A pointer to a `SceKernelEventFlagInfo` structure.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReferEventFlagStatus(
        event: SceUid,
        status: *mut SceKernelEventFlagInfo,
    ) -> i32;

    #[psp(0x8125221D)]
    /// Creates a new messagebox
    ///
    /// # Parameters
    ///
    /// - `name`: Specifies the name of the mbx
    /// - `attr`: Mbx attribute flags (normally set to 0)
    /// - `option`: Mbx options (normally set to null)
    ///
    /// # Return Value
    ///
    /// A messagebox id
    pub fn sceKernelCreateMbx(
        name: *const u8,
        attr: u32,
        option: *mut SceKernelMbxOptParam,
    ) -> SceUid;

    #[psp(0x86255ADA)]
    /// Destroy a messagebox
    ///
    /// # Parameters
    ///
    /// - `mbx_id`: The mbxid returned from a previous create call.
    ///
    /// # Return Value
    ///
    /// Returns the value 0 if its succesful otherwise an error code
    pub fn sceKernelDeleteMbx(mbx_id: SceUid) -> i32;

    #[psp(0xE9B3061E)]
    /// Send a message to a messagebox
    ///
    /// # Parameters
    ///
    /// - `mbx_id`: The mbx id returned from `sceKernelCreateMbx`
    /// - `message`: A message to be forwarded to the receiver.
    ///
    ///    Note: The start of the message should be the `SceKernelMsgPacket` structure, the rest
    ///    (???) *This documentation appears to have been unfinished*.
    ///
    /// # Return Value
    ///
    /// < 0 On error.
    pub fn sceKernelSendMbx(
        mbx_id: SceUid,
        message: *mut c_void,
    ) -> i32;

    #[psp(0x18260574)]
    /// Wait for a message to arrive in a messagebox
    ///
    /// # Parameters
    ///
    /// - `mbx_id`: The mbx id returned from `sceKernelCreateMbx`
    /// - `message`: A pointer to where a pointer to the received message should be stored
    /// - `timeout`: Timeout in microseconds
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReceiveMbx(
        mbx_id: SceUid,
        message: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0xF3986382)]
    /// Wait for a message to arrive in a messagebox and handle callbacks if necessary.
    ///
    /// # Parameters
    ///
    /// - `mbx_id`: The mbx id returned from `sceKernelCreateMbx`
    /// - `message`: A pointer to where a pointer to the received message should be stored
    /// - `timeout`: Timeout in microseconds
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReceiveMbxCB(
        mbx_id: SceUid,
        message: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x0D81716A)]
    /// Check if a message has arrived in a messagebox
    ///
    /// # Parameters
    ///
    /// - `mbx_id`: The mbx id returned from `sceKernelCreateMbx`
    /// - `message`: A pointer to where a pointer to the received message should be stored
    ///
    /// # Return Value
    ///
    /// < 0 on error (`SCE_KERNEL_ERROR_MBOX_NOMSG` if the mbx is empty).
    pub fn sceKernelPollMbx(
        mbx_id: SceUid,
        pmessage: *mut *mut c_void,
    ) -> i32;

    #[psp(0x87D4DD36)]
    /// Abort all wait operations on a messagebox
    ///
    /// # Parameters
    ///
    /// - `mbx_id`: The mbx id returned from `sceKernelCreateMbx`
    /// - `num`: A pointer to where the number of threads which were waiting on
    ///    the mbx should be stored (null if you don't care)
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sceKernelCancelReceiveMbx(
        mbx_id: SceUid,
        num: *mut i32,
    ) -> i32;

    #[psp(0xA8E8C846)]
    /// Retrieve information about a messagebox.
    ///
    /// # Parameters
    ///
    /// - `mbx_id`: UID of the messagebox to retrieve info for.
    /// - `info`: Pointer to a `SceKernelMbxInfo` struct to receive the info.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReferMbxStatus(
        mbx_id: SceUid,
        info: *mut SceKernelMbxInfo,
    ) -> i32;

    #[psp(0x6652B8CA)]
    /// Set an alarm.
    ///
    /// # Parameters
    ///
    /// - `clock`: The number of micro seconds till the alarm occurrs.
    /// - `handler`: Pointer to a `SceKernelAlarmHandler`
    /// - `common`: Common pointer for the alarm handler
    ///
    /// # Return Value
    ///
    /// A UID representing the created alarm, < 0 on error.
    pub fn sceKernelSetAlarm(
        clock: u32,
        handler: SceKernelAlarmHandler,
        common: *mut c_void,
    ) -> SceUid;

    #[psp(0xB2C25152)]
    /// Set an alarm using a `SceKernelSysClock` structure for the time
    ///
    /// # Parameters
    ///
    /// - `clock`: Pointer to a `SceKernelSysClock` structure
    /// - `handler`: Pointer to a `SceKernelAlarmHandler`
    /// - `common`: Common pointer for the alarm handler.
    ///
    /// # Return Value
    ///
    /// A UID representing the created alarm, < 0 on error.
    pub fn sceKernelSetSysClockAlarm(
        clock: *mut SceKernelSysClock,
        handler: *mut SceKernelAlarmHandler,
        common: *mut c_void,
    ) -> SceUid;

    #[psp(0x7E65B999)]
    /// Cancel a pending alarm.
    ///
    /// # Parameters
    ///
    /// - `alarm_id`: UID of the alarm to cancel.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceKernelCancelAlarm(alarm_id: SceUid) -> i32;

    #[psp(0xDAA3F564)]
    /// Refer the status of a created alarm.
    ///
    /// # Parameters
    ///
    /// - `alarm_id`: UID of the alarm to get the info of
    /// - `info`: Pointer to a `SceKernelAlarmInfo` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceKernelReferAlarmStatus(
        alarm_id: SceUid,
        info: *mut SceKernelAlarmInfo,
    ) -> i32;

    #[psp(0xE81CAF8F)]
    /// Create callback.
    ///
    /// # Parameters
    ///
    /// - `name`: A textual name for the callback.
    /// - `func`: A pointer to a function that will be called as the callback.
    /// - `arg`: Argument for the callback?
    ///
    /// # Return Value
    ///
    /// >= 0 A callback id which can be used in subsequent functions, < 0 an error.
    pub fn sceKernelCreateCallback(
        name: *const u8,
        func: SceKernelCallbackFunction,
        arg: *mut c_void,
    ) -> SceUid;

    #[psp(0x730ED8BC)]
    /// Gets the status of a specified callback.
    ///
    /// # Parameters
    ///
    /// - `cb`: The UID of the callback to refer.
    /// - `status`: Pointer to a status structure. The size parameter should be
    ///   initialised before calling.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReferCallbackStatus(
        cb: SceUid,
        status: *mut SceKernelCallbackInfo,
    ) -> i32;

    #[psp(0xEDBA5844)]
    /// Delete a callback
    ///
    /// # Parameters
    ///
    /// - `cb`: The UID of the specified callback
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelDeleteCallback(cb: SceUid) -> i32;

    #[psp(0xC11BA8C4)]
    /// Notify a callback
    ///
    /// # Parameters
    ///
    /// - `cb`: The UID of the specified callback
    /// - `arg2`: Passed as arg2 into the callback function
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelNotifyCallback(
        cb: SceUid,
        arg2: i32,
    ) -> i32;

    #[psp(0xBA4051D6)]
    /// Cancel a callback?
    ///
    /// # Parameters
    ///
    /// - `cb`: The UID of the specified callback
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelCancelCallback(cb: SceUid) -> i32;

    #[psp(0x2A3D44FF)]
    /// Get the callback count
    ///
    /// # Parameters
    ///
    /// - `cb`: The UID of the specified callback
    ///
    /// # Return Value
    ///
    /// The callback count, < 0 on error
    pub fn sceKernelGetCallbackCount(cb: SceUid) -> i32;

    #[psp(0x349D6D6C)]
    /// Check callback?
    ///
    /// # Return Value
    ///
    /// TODO: Something or another
    pub fn sceKernelCheckCallback() -> i32;

    #[psp(0x94416130)]
    /// Get a list of UIDs from threadman. Allows you to enumerate resources
    /// such as threads or semaphores.
    ///
    /// # Parameters
    ///
    /// - `type`: The type of resource to list, one of `SceKernelIdListType`.
    /// - `read_buf`: A pointer to a buffer to store the list.
    /// - `read_buf_size`: The size of the buffer in `SceUid` units.
    /// - `id_count`: Pointer to an integer in which to return the number of IDs in the list.
    ///
    /// # Return Value
    ///
    /// < 0 on error. Either 0 or the same as idcount on success.
    pub fn sceKernelGetThreadmanIdList(
        type_: SceKernelIdListType,
        read_buf: *mut SceUid,
        read_buf_size: i32,
        id_count: *mut i32,
    ) -> i32;

    #[psp(0x627E6F3A)]
    /// Get the current system status.
    ///
    /// # Parameters
    ///
    /// - `status`: Pointer to a `SceKernelSystemStatus` structure.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelReferSystemStatus(status: *mut SceKernelSystemStatus) -> i32;

    #[psp(0x7C0DC2A0)]
    /// Create a message pipe
    ///
    /// # Parameters
    ///
    /// - `name`: Name of the pipe
    /// - `part`: ID of the memory partition
    /// - `attr`: Set to 0?
    /// - `unk1`: Unknown
    /// - `opt`: Message pipe options (set to null)
    ///
    /// # Return Value
    ///
    /// The UID of the created pipe, < 0 on error
    pub fn sceKernelCreateMsgPipe(
        name: *const u8,
        part: i32,
        attr: i32,
        unk1: *mut c_void,
        opt: *mut c_void,
    ) -> SceUid;

    #[psp(0xF0B7DA1C)]
    /// Delete a message pipe
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pipe
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelDeleteMsgPipe(uid: SceUid) -> i32;

    #[psp(0x876DBFAD)]
    /// Send a message to a pipe
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pipe
    /// - `message`: Pointer to the message
    /// - `size`: Size of the message
    /// - `unk1`: Unknown
    /// - `unk2`: Unknown
    /// - `timeout`: Timeout for send
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelSendMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x7C41F2C2)]
    /// Send a message to a pipe (with callback)
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pipe
    /// - `message`: Pointer to the message
    /// - `size`: Size of the message
    /// - `unk1`: Unknown
    /// - `unk2`: Unknown
    /// - `timeout`: Timeout for send
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelSendMsgPipeCB(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x884C9F90)]
    /// Try to send a message to a pipe
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pipe
    /// - `message`: Pointer to the message
    /// - `size`: Size of the message
    /// - `unk1`: Unknown
    /// - `unk2`: Unknown
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelTrySendMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
    ) -> i32;

    #[psp(0x74829B76)]
    /// Receive a message from a pipe
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pipe
    /// - `message`: Pointer to the message
    /// - `size`: Size of the message
    /// - `unk1`: Unknown
    /// - `unk2`: Unknown
    /// - `timeout`: Timeout for receive
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReceiveMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0xFBFA697D)]
    /// Receive a message from a pipe (with callback)
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pipe
    /// - `message`: Pointer to the message
    /// - `size`: Size of the message
    /// - `unk1`: Unknown
    /// - `unk2`: Unknown
    /// - `timeout`: Timeout for receive
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReceiveMsgPipeCB(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0xDF52098F)]
    /// Receive a message from a pipe
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pipe
    /// - `message`: Pointer to the message
    /// - `size`: Size of the message
    /// - `unk1`: Unknown
    /// - `unk2`: Unknown
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelTryReceiveMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
    ) -> i32;

    #[psp(0x349B864D)]
    /// Cancel a message pipe
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the pipe to cancel
    /// - `send`: Receive number of sending threads?
    /// - `recv`: Receive number of receiving threads?
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelCancelMsgPipe(
        uid: SceUid,
        send: *mut i32,
        recv: *mut i32,
    ) -> i32;

    #[psp(0x33BE4024)]
    /// Get the status of a Message Pipe
    ///
    /// # Parameters
    ///
    /// - `uid`: The uid of the Message Pipe
    /// - `info`: Pointer to a `SceKernelMppInfo` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReferMsgPipeStatus(
        uid: SceUid,
        info: *mut SceKernelMppInfo,
    ) -> i32;

    #[psp(0x56C039B5)]
    /// Create a variable pool
    ///
    /// # Parameters
    ///
    /// - `name`: Name of the pool
    /// - `part`: The memory partition ID
    /// - `attr`: Attributes
    /// - `size`: Size of pool
    /// - `opt`: Options (can be set to null)
    ///
    /// # Return Value
    ///
    /// The UID of the created pool, < 0 on error.
    pub fn sceKernelCreateVpl(
        name: *const u8,
        part: i32,
        attr: i32,
        size: u32,
        opt: *mut SceKernelVplOptParam,
    ) -> SceUid;

    #[psp(0x89B3D48C)]
    /// Delete a variable pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelDeleteVpl(uid: SceUid) -> i32;

    #[psp(0xBED27435)]
    /// Allocate from the pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `size`: The size to allocate
    /// - `data`: Receives the address of the allocated data
    /// - `timeout`: Amount of time to wait for allocation?
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelAllocateVpl(
        uid: SceUid,
        size: u32,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0xEC0A693F)]
    /// Allocate from the pool (with callback)
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `size`: The size to allocate
    /// - `data`: Receives the address of the allocated data
    /// - `timeout`: Amount of time to wait for allocation?
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelAllocateVplCB(
        uid: SceUid,
        size: u32,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0xAF36D708)]
    /// Try to allocate from the pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `size`: The size to allocate
    /// - `data`: Receives the address of the allocated data
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelTryAllocateVpl(
        uid: SceUid,
        size: u32,
        data: *mut *mut c_void,
    ) -> i32;

    #[psp(0xB736E9FF)]
    /// Free a block
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `data`: The data block to deallocate
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelFreeVpl(
        uid: SceUid,
        data: *mut c_void,
    ) -> i32;

    #[psp(0x1D371B8A)]
    /// Cancel a pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `num`: Receives the number of waiting threads
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelCancelVpl(
        uid: SceUid,
        num: *mut i32,
    ) -> i32;

    #[psp(0x39810265)]
    /// Get the status of an VPL
    ///
    /// # Parameters
    ///
    /// - `uid`: The uid of the VPL
    /// - `info`: Pointer to a `SceKernelVplInfo` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReferVplStatus(
        uid: SceUid,
        info: *mut SceKernelVplInfo,
    ) -> i32;

    #[psp(0xC07BB470)]
    /// Create a fixed pool
    ///
    /// # Parameters
    ///
    /// - `name`: Name of the pool
    /// - `part`: The memory partition ID
    /// - `attr`: Attributes
    /// - `size`: Size of pool block
    /// - `blocks`: Number of blocks to allocate
    /// - `opt`: Options (can be set to null)
    ///
    /// # Return Value
    ///
    /// The UID of the created pool, < 0 on error.
    pub fn sceKernelCreateFpl(
        name: *const u8,
        part: i32,
        attr: i32,
        size: u32,
        blocks: u32,
        opt: *mut SceKernelFplOptParam,
    ) -> i32;

    #[psp(0xED1410E0)]
    /// Delete a fixed pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelDeleteFpl(uid: SceUid) -> i32;

    #[psp(0xD979E9BF)]
    /// Allocate from the pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `data`: Receives the address of the allocated data
    /// - `timeout`: Amount of time to wait for allocation?
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelAllocateFpl(
        uid: SceUid,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0xE7282CB6)]
    /// Allocate from the pool (with callback)
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `data`: Receives the address of the allocated data
    /// - `timeout`: Amount of time to wait for allocation?
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelAllocateFplCB(
        uid: SceUid,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;

    #[psp(0x623AE665)]
    /// Try to allocate from the pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `data`: Receives the address of the allocated data
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelTryAllocateFpl(
        uid: SceUid,
        data: *mut *mut c_void,
    ) -> i32;

    #[psp(0xF6414A71)]
    /// Free a block
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `data`: The data block to deallocate
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelFreeFpl(
        uid: SceUid,
        data: *mut c_void,
    ) -> i32;

    #[psp(0xA8AA591F)]
    /// Cancel a pool
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the pool
    /// - `pnum`: Receives the number of waiting threads
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelCancelFpl(
        uid: SceUid,
        pnum: *mut i32,
    ) -> i32;

    #[psp(0xD8199E4C)]
    /// Get the status of an FPL
    ///
    /// # Parameters
    ///
    /// - `uid`: The uid of the FPL
    /// - `info`: Pointer to a `SceKernelFplInfo` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReferFplStatus(
        uid: SceUid,
        info: *mut SceKernelFplInfo,
    ) -> i32;

    #[psp(0x110DEC9A)]
    /// Convert a number of microseconds to a `SceKernelSysClock` structure
    ///
    /// # Parameters
    ///
    /// - `usec`: Number of microseconds
    /// - `clock`: Pointer to a `SceKernelSysClock` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelUSec2SysClock(
        usec: u32,
        clock: *mut SceKernelSysClock,
    ) -> i32;

    #[psp(0xC8CD158C)]
    /// Convert a number of microseconds to a wide time
    ///
    /// # Parameters
    ///
    /// - `usec`: Number of microseconds.
    ///
    /// # Return Value
    ///
    /// The time
    pub fn sceKernelUSec2SysClockWide(usec: u32) -> i64;

    #[psp(0xBA6B92E2)]
    /// Convert a `SceKernelSysClock` structure to microseconds
    ///
    /// # Parameters
    ///
    /// - `clock`: Pointer to a `SceKernelSysClock` structure
    /// - `low`: Pointer to the low part of the time
    /// - `high`: Pointer to the high part of the time
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelSysClock2USec(
        clock: *mut SceKernelSysClock,
        low: *mut u32,
        high: *mut u32,
    ) -> i32;

    #[psp(0xE1619D7C)]
    /// Convert a wide time to microseconds
    ///
    /// # Parameters
    ///
    /// - `clock`: Wide time
    /// - `low`: Pointer to the low part of the time
    /// - `high`: Pointer to the high part of the time
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelSysClock2USecWide(
        clock: i64,
        low: *mut u32,
        high: *mut u32,
    ) -> i32;

    #[psp(0xDB738F35)]
    /// Get the system time
    ///
    /// # Parameters
    ///
    /// - `time`: Pointer to a `SceKernelSysClock` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelGetSystemTime(time: *mut SceKernelSysClock) -> i32;

    #[psp(0x82BC5777)]
    /// Get the system time (wide version)
    ///
    /// # Return Value
    ///
    /// The system time
    pub fn sceKernelGetSystemTimeWide() -> i64;

    #[psp(0x369ED59D)]
    /// Get the low 32bits of the current system time
    ///
    /// # Return Value
    ///
    /// The low 32bits of the system time
    pub fn sceKernelGetSystemTimeLow() -> u32;

    #[psp(0x20FFF560)]
    /// Create a virtual timer
    ///
    /// # Parameters
    ///
    /// - `name`: Name for the timer.
    /// - `opt`: Pointer to an `SceKernelVTimerOptParam` (can be set to null)
    ///
    /// # Return Value
    ///
    /// The VTimer's UID or < 0 on error.
    pub fn sceKernelCreateVTimer(
        name: *const u8,
        opt: *mut SceKernelVTimerOptParam,
    ) -> SceUid;

    #[psp(0x328F9E52)]
    /// Delete a virtual timer
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the timer
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceKernelDeleteVTimer(uid: SceUid) -> i32;

    #[psp(0xB3A59970)]
    /// Get the timer base
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    /// - `base`: Pointer to a `SceKernelSysClock` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelGetVTimerBase(
        uid: SceUid,
        base: *mut SceKernelSysClock,
    ) -> i32;

    #[psp(0xB7C18B77)]
    /// Get the timer base (wide format)
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    ///
    /// # Return Value
    ///
    /// The 64bit timer base
    pub fn sceKernelGetVTimerBaseWide(uid: SceUid) -> i64;

    #[psp(0x034A921F)]
    /// Get the timer time
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    /// - `time`: Pointer to a `SceKernelSysClock` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelGetVTimerTime(
        uid: SceUid,
        time: *mut SceKernelSysClock,
    ) -> i32;

    #[psp(0xC0B3FFD2)]
    /// Get the timer time (wide format)
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    ///
    /// # Return Value
    ///
    /// The 64bit timer time
    pub fn sceKernelGetVTimerTimeWide(uid: SceUid) -> i64;

    #[psp(0x542AD630)]
    /// Set the timer time
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    /// - `time`: Pointer to a `SceKernelSysClock` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelSetVTimerTime(
        uid: SceUid,
        time: *mut SceKernelSysClock,
    ) -> i32;

    #[psp(0xFB6425C3)]
    /// Set the timer time (wide format)
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    /// - `time`: Pointer to a `SceKernelSysClock` structure
    ///
    /// # Return Value
    ///
    /// Possibly the last time
    pub fn sceKernelSetVTimerTimeWide(uid: SceUid, time: i64) -> i64;

    #[psp(0xC68D9437)]
    /// Start a virtual timer
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the timer
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sceKernelStartVTimer(uid: SceUid) -> i32;

    #[psp(0xD0AEEE87)]
    /// Stop a virtual timer
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the timer
    ///
    /// # Return Value
    ///
    /// < 0 on error
    pub fn sceKernelStopVTimer(uid: SceUid) -> i32;

    #[psp(0xD8B299AE)]
    /// Set the timer handler
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    /// - `time`: Time to call the handler?
    /// - `handler`: The timer handler
    /// - `common`: Common pointer
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelSetVTimerHandler(
        uid: SceUid,
        time: *mut SceKernelSysClock,
        handler: SceKernelVTimerHandler,
        common: *mut c_void,
    ) -> i32;

    #[psp(0x53B00E9A)]
    /// Set the timer handler (wide mode)
    ///
    /// # Parameters
    ///
    /// - `uid`: UID of the vtimer
    /// - `time`: Time to call the handler?
    /// - `handler`: The timer handler
    /// - `common`: Common pointer
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelSetVTimerHandlerWide(
        uid: SceUid,
        time: i64,
        handler: SceKernelVTimerHandlerWide,
        common: *mut c_void,
    ) -> i32;

    #[psp(0xD2D615EF)]
    /// Cancel the timer handler
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the vtimer
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelCancelVTimerHandler(uid: SceUid) -> i32;

    #[psp(0x5F32BEAA)]
    /// Get the status of a VTimer
    ///
    /// # Parameters
    ///
    /// - `uid`: The uid of the VTimer
    /// - `info`: Pointer to a `SceKernelVTimerInfo` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReferVTimerStatus(
        uid: SceUid,
        info: *mut SceKernelVTimerInfo,
    ) -> i32;

    #[psp(0x0C106E53)]
    /// Register a thread event handler
    ///
    /// # Parameters
    ///
    /// - `name`: Name for the thread event handler
    /// - `thread_id`: Thread ID to monitor
    /// - `mask`: Bit mask for what events to handle (only lowest 4 bits valid)
    /// - `handler`: Pointer to a `SceKernelThreadEventHandler` function
    /// - `common`: Common pointer
    ///
    /// # Return Value
    ///
    /// The UID of the create event handler, < 0 on error
    pub fn sceKernelRegisterThreadEventHandler(
        name: *const u8,
        thread_id: SceUid,
        mask: i32,
        handler: SceKernelThreadEventHandler,
        common: *mut c_void,
    ) -> SceUid;

    #[psp(0x72F3C145)]
    /// Release a thread event handler.
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the event handler
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReleaseThreadEventHandler(uid: SceUid) -> i32;

    #[psp(0x369EEB6B)]
    /// Refer the status of an thread event handler
    ///
    /// # Parameters
    ///
    /// - `uid`: The UID of the event handler
    /// - `info`: Pointer to a `SceKernelThreadEventHandlerInfo` structure
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceKernelReferThreadEventHandlerStatus(
        uid: SceUid,
        info: *mut SceKernelThreadEventHandlerInfo,
    ) -> i32;

    #[psp(0x64D4540E)]
    /// Get the thread profiler registers.
    ///
    /// # Return Value
    ///
    /// Pointer to the registers, null on error
    pub fn sceKernelReferThreadProfiler() -> *mut DebugProfilerRegs;

    #[psp(0x8218B4DD)]
    /// Get the global profiler registers.
    ///
    /// # Return Value
    ///
    /// Pointer to the registers, null on error
    pub fn sceKernelReferGlobalProfiler() -> *mut DebugProfilerRegs;
}
