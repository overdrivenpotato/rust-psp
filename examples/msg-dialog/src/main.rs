#![no_std]
#![no_main]

use psp::sys::{
    DialogCommon, MsgDialogParams, MsgDialogMode, MsgDialogPressed,
    SysParamLanguage, DialogButtonAccept, MsgDialogOption, self,
};

use core::ffi::c_void;

psp::module!("sample_module", 1, 1);


static mut LIST: psp::Align16<[u32; 262144]> = psp::Align16([0;262144]);

unsafe fn setup_gu() {
    sys::sceGuInit(); 
    sys::sceGuStart(sys::Context::Direct, &mut LIST as *mut _ as *mut c_void);
    sys::sceGuDrawBuffer(sys::DisplayPixelFormat::Psm8888, core::ptr::null_mut(), 512);
    sys::sceGuDispBuffer(480, 272, 0x88000 as *mut c_void, 512);
    sys::sceGuDepthBuffer(0x110000 as *mut c_void, 512);
    sys::sceGuOffset(2048 - (480/2), 2048 - (272/2));
    sys::sceGuViewport(2048, 2048, 480, 272);
    sys::sceGuDepthRange(0xc350, 0x2710);
    sys::sceGuScissor(0, 0, 480, 272);
    sys::sceGuEnable(sys::GuState::ScissorTest);
    sys::sceGuDepthFunc(sys::DepthFunc::GreaterOrEqual);
    sys::sceGuEnable(sys::GuState::DepthTest);
    sys::sceGuFrontFace(sys::FrontFaceDirection::Clockwise);
    sys::sceGuShadeModel(sys::ShadingModel::Smooth);
    sys::sceGuEnable(sys::GuState::CullFace);
    sys::sceGuEnable(sys::GuState::ClipPlanes);
    sys::sceGuFinish();
    sys::sceGuSync(sys::SyncMode::Finish, sys::SyncBehavior::Wait);

    sys::sceDisplayWaitVblankStart();
    sys::sceGuDisplay(true);
}

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        setup_gu();
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
        unsafe {
            sys::sceGuStart(sys::Context::Direct, &mut LIST as *mut _ as *mut c_void);
            sys::sceGuFinish();
            sys::sceGuSync(sys::SyncMode::Finish, sys::SyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }
    unsafe { sys::sceKernelExitGame(); }
}
