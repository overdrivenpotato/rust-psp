#![no_std]
#![no_main]

use psp::sys::{
    UtilityDialogCommon, UtilityMsgDialogParams, UtilityMsgDialogMode, 
    UtilityMsgDialogPressed, SystemParamLanguage, UtilityDialogButtonAccept, 
    UtilityMsgDialogOption, self,
    DisplayPixelFormat, Context, GuState, DepthFunc, FrontFaceDirection, 
    ShadingModel, SyncMode, SyncBehavior
};

use core::ffi::c_void;

psp::module!("sample_module", 1, 1);

static mut LIST: psp::Align16<[u32; 262144]> = psp::Align16([0;262144]);
const SCR_WIDTH: i32 = 480;
const SCR_HEIGHT: i32 = 272;
const BUF_WIDTH: i32 = 512;

unsafe fn setup_gu() {
    sys::sceGuInit(); 
    sys::sceGuStart(Context::Direct, &mut LIST as *mut _ as *mut c_void);
    sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, core::ptr::null_mut(), BUF_WIDTH);
    sys::sceGuDispBuffer(SCR_WIDTH, SCR_HEIGHT, 0x88000 as *mut c_void, BUF_WIDTH);
    sys::sceGuDepthBuffer(0x110000 as *mut c_void, BUF_WIDTH);
    sys::sceGuOffset(2048 - (SCR_WIDTH as u32 /2), 2048 - (SCR_HEIGHT as u32 /2));
    sys::sceGuViewport(2048, 2048, SCR_WIDTH, SCR_HEIGHT);
    sys::sceGuDepthRange(0xc350, 0x2710);
    sys::sceGuScissor(0, 0, SCR_WIDTH, SCR_HEIGHT);
    sys::sceGuEnable(GuState::ScissorTest);
    sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
    sys::sceGuEnable(GuState::DepthTest);
    sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
    sys::sceGuShadeModel(ShadingModel::Smooth);
    sys::sceGuEnable(GuState::CullFace);
    sys::sceGuEnable(GuState::ClipPlanes);
    sys::sceGuFinish();
    sys::sceGuSync(SyncMode::Finish, SyncBehavior::Wait);

    sys::sceDisplayWaitVblankStart();
    sys::sceGuDisplay(true);
}

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        setup_gu();
    }

    let dialog_size = core::mem::size_of::<UtilityMsgDialogParams>();
    let base = UtilityDialogCommon {
        size: dialog_size as u32,
        language: SystemParamLanguage::English, 
        button_accept: UtilityDialogButtonAccept::Cross, // X to accept 
        graphics_thread: 0x11, // magic number stolen from pspsdk example
        access_thread: 0x13,
        font_thread: 0x12,
        sound_thread: 0x10,
        result: 0,
        reserved: [0i32; 4],
    };
    
    let mut msg: [u8; 512] = [0u8; 512];
    msg[..40].copy_from_slice(b"Hello from a Rust-created PSP Msg Dialog");

    let mut msg_dialog = UtilityMsgDialogParams {
        base,
        unknown: 0,
        mode: UtilityMsgDialogMode::Text,
        error_value: 0,
        message: msg,
        options: UtilityMsgDialogOption::TEXT,
        button_pressed: UtilityMsgDialogPressed::Unknown1,
    };

    unsafe {
        sys::sceUtilityMsgDialogInitStart(
            &mut msg_dialog as *mut UtilityMsgDialogParams
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
