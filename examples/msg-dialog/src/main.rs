#![no_std]
#![no_main]

use psp::sys::{
    utility::{
        DialogCommon, MsgDialogParams, MsgDialogMode, MsgDialogPressed,
        SysParamLanguage, DialogButtonAccept, MsgDialogOption, self,
    },
    kernel,
    display,
};

use core::ffi::c_void;

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    static mut LIST: psp::Align16<[u32; 262144]> = psp::Align16([0;262144]);

    unsafe {
        gu::sce_gu_init(); 
        gu::sce_gu_start(gu::Context::Direct, &mut LIST as *mut _ as *mut c_void);
        gu::sce_gu_draw_buffer(gu::PixelFormat::Psm8888, core::ptr::null_mut(), 512);
        gu::sce_gu_finish();
        gu::sce_gu_sync(gu::SyncMode::SyncFinish, gu::SyncBehaviorWhat::SyncWhatDone);
        display::sce_display_wait_vblank_start();
        gu::sce_gu_display(true);
    }

    let dialog_size = core::mem::size_of::<Dialogcommon>();
    let base = DialogCommon {
        size: dialog_size as u32,
        language: SysParamLanguage::English, 
        button_accept: DialogButtonAccept::Cross, // X to accept 
        graphics_thread: 0x11, // magic number stolen from pspsdk example
        access_thread: 0x13,
        font_thread: 0x12,
        sound_thread: 0x10,
        result: 0,
        reserved: [0i32; 4],
    };
    
    let mut msg: [u8; 512] = [0u8; 512];
    msg[..40].copy_from_slice(b"Hello from a Rust-created PSP Msg Dialog");

    let mut msg_dialog = MsgDialogParams {
        base,
        unknown: 0,
        mode: MsgDialogMode::Text,
        error_value: 0,
        message: msg,
        options: MsgDialogOption::TEXT,
        button_pressed: MsgDialogPressed::Unknown1,
    };

    unsafe {
        utility::sce_utility_msg_dialog_init_start(
            &mut msg_dialog as *mut MsgDialogParams
        );
    }

    loop {
        let status = unsafe {utility::sce_utility_msg_dialog_get_status()};
        match status {
            2 => unsafe{utility::sce_utility_msg_dialog_update(1)},
            3 => unsafe{utility::sce_utility_msg_dialog_shutdown_start()},
            0 => {break},
            _ => (),
        }
        unsafe {display::sce_display_wait_vblank_start();}
    }
    unsafe { kernel::sce_kernel_exit_game(); }
}
