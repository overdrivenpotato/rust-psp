#![no_std]
#![no_main]

use psp::sys::{
    DialogCommon, MsgDialogParams, MsgDialogMode, MsgDialogPressed,
    SysParamLanguage, DialogButtonAccept, MsgDialogOption, self,
};

use core::ffi::c_void;

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    static mut LIST: psp::Align16<[u32; 262144]> = psp::Align16([0;262144]);

    unsafe {
        sys::sceGuInit(); 
        sys::sceGuStart(sys::Context::Direct, &mut LIST as *mut _ as *mut c_void);
        sys::sceGuDrawBuffer(sys::DisplayPixelFormat::Psm8888, core::ptr::null_mut(), 512);
        sys::sceGuFinish();
        sys::sceGuSync(sys::SyncMode::Finish, sys::SyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);
    }

    let dialog_size = core::mem::size_of::<MsgDialogParams>();
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
        sys::sceUtilityMsgDialogInitStart(
            &mut msg_dialog as *mut MsgDialogParams
        );
    }

    loop {
        let status = unsafe {sys::sceUtilityMsgDialogGetStatus()};
        match status {
            2 => unsafe{sys::sceUtilityMsgDialogUpdate(1)},
            3 => unsafe{sys::sceUtilityMsgDialogShutdownStart()},
            0 => {break},
            _ => (),
        }
        unsafe {sys::sceDisplayWaitVblankStart();}
    }
    unsafe { sys::sceKernelExitGame(); }
}
