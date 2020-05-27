#![no_std]
#![no_main]

use psp::sys::{
    utility::{
        DialogCommon, MsgDialogParams, MsgDialogMode, MsgDialogPressed,
        self,
    },
    kernel::{self, SceUid},
    display,
};

use core::ffi::c_void;

psp::module!("sample_module", 1, 1);

unsafe extern "C" fn exit_callback(_arg1: i32, _arg2: i32, _common: *mut c_void) -> i32 {
    kernel::sce_kernel_exit_game();
    0
}

unsafe extern "C" fn update_callback(_argc: usize, _argv: *mut c_void) -> i32 {
    let cbid: SceUid;
    let callback_name: &[u8; 14] = b"Exit Callback\0";
    cbid = kernel::sce_kernel_create_callback(
        callback_name as *const u8,
        exit_callback,
        core::ptr::null_mut() as *mut c_void
    );
    kernel::sce_kernel_register_exit_callback(cbid);
    kernel::sce_kernel_sleep_thread_cb();
    0
}

fn psp_main() {
    //psp::enable_home_button();

    let thid: SceUid;
    let callback_name: &[u8; 14] = b"update_thread\0";
    unsafe {
        thid = kernel::sce_kernel_create_thread(
            callback_name as *const u8,
            update_callback,
            0x11,
            0xFA0,
            kernel::ThreadAttributes::from_bits(0).unwrap(),
            core::ptr::null_mut()
        );

        if thid.0 >= 0 {
            kernel::sce_kernel_start_thread(thid, 0, core::ptr::null_mut() as *mut c_void);
        }
    }

    let base_size = core::mem::size_of::<DialogCommon>();
    let base = DialogCommon {
        size: base_size as u32,
        language: 1, // english, TODO add this as an enum later
        button_swap: 1, // X to accept 
        graphics_thread: 0x11, // magic number stolen from pspsdk example
        access_thread: 0x13,
        font_thread: 0x12,
        sound_thread: 0x10,
        result: 0,
        reserved: [0i32; 4],
    };
    
    let mut msg: [u8; 512] = [0u8; 512];
    msg.copy_from_slice(b"Hello from a Rust-created PSP Msg Dialog");

    let mut msg_dialog = MsgDialogParams {
        base,
        unknown: 0,
        mode: MsgDialogMode::Text,
        error_value: 0,
        message: msg,
        options: 1, // Text, TODO add this as an enum later
        button_pressed: MsgDialogPressed::Unknown1,
    };

    unsafe {
        utility::sce_utility_msg_dialog_init_start(
            &mut msg_dialog as *mut MsgDialogParams
        );
    }

    loop {
        // TODO figure out these values
        match unsafe {utility::sce_utility_msg_dialog_get_status()} {
            2 => unsafe{utility::sce_utility_msg_dialog_update(1)},
            3 => unsafe{utility::sce_utility_msg_dialog_shutdown_start()},
            0 => break,
            _ => (),
        }
        unsafe {display::sce_display_wait_vblank_start();}
    }
}
