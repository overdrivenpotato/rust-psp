use core::ffi::c_void;

/// Stores the state of the GE.
#[repr(C)]
pub struct GeContext {
    pub context: [u32; 512],
}

#[repr(C)]
/// Structure storing a stack (for CALL/RET)
pub struct GeStack {
    pub stack: [u32; 8],
}

#[repr(C)]
/// Structure to hold the callback data
pub struct GeCallbackData {
    pub signal_func: Option<extern "C" fn(id: i32, arg: *mut c_void)>,
    pub signal_arg: *mut c_void,
    pub finish_func: Option<extern "C" fn(id: i32, arg: *mut c_void)>,
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
    #[inline(always)]
    fn default() -> Self {
        Self {
            size: 0,
            context: core::ptr::null_mut(),
            num_stacks: 0,
            stacks: core::ptr::null_mut(),
        }
    }
}

#[repr(C)]
/// Drawing queue interruption parameter
pub struct GeBreakParam {
    pub buf: [u32; 4],
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

/// List status for `sceGeListSync` and `sceGeDrawSync`.
#[repr(i32)]
pub enum GeListState {
    Done = 0,
    Queued,
    DrawingDone,
    StallReached,
    CancelDone,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum GeCommand {
    Nop = 0,
    Vaddr = 0x1,
    Iaddr = 0x2,
    Prim = 0x4,
    Bezier = 0x5,
    Spline = 0x6,
    BoundingBox = 0x7,
    Jump = 0x8,
    BJump = 0x9,
    Call = 0xa,
    Ret = 0xb,
    End = 0xc,
    Signal = 0xe,
    Finish = 0xf,
    Base = 0x10,
    VertexType = 0x12,
    OffsetAddr = 0x13,
    Origin = 0x14,
    Region1 = 0x15,
    Region2 = 0x16,
    LightingEnable = 0x17,
    LightEnable0 = 0x18,
    LightEnable1 = 0x19,
    LightEnable2 = 0x1a,
    LightEnable3 = 0x1b,
    DepthClampEnable = 0x1c,
    CullFaceEnable = 0x1d,
    TextureMapEnable = 0x1e,
    FogEnable = 0x1f,
    DitherEnable = 0x20,
    AlphaBlendEnable = 0x21,
    AlphaTestEnable = 0x22,
    ZTestEnable = 0x23,
    StencilTestEnable = 0x24,
    AntiAliasEnable = 0x25,
    PatchCullEnable = 0x26,
    ColorTestEnable = 0x27,
    LogicOpEnable = 0x28,
    BoneMatrixNumber = 0x2a,
    BoneMatrixData = 0x2b,
    MorphWeight0 = 0x2c,
    MorphWeight1 = 0x2d,
    MorphWeight2 = 0x2e,
    MorphWeight3 = 0x2f,
    MorphWeight4 = 0x30,
    MorphWeight5 = 0x31,
    MorphWeight6 = 0x32,
    MorphWeight7 = 0x33,
    PatchDivision = 0x36,
    PatchPrimitive = 0x37,
    PatchFacing = 0x38,
    WorldMatrixNumber = 0x3a,
    WorldMatrixData = 0x3b,
    ViewMatrixNumber = 0x3c,
    ViewMatrixData = 0x3d,
    ProjMatrixNumber = 0x3e,
    ProjMatrixData = 0x3f,
    TGenMatrixNumber = 0x40,
    TGenMatrixData = 0x41,
    ViewportXScale = 0x42,
    ViewportYScale = 0x43,
    ViewportZScale = 0x44,
    ViewportXCenter = 0x45,
    ViewportYCenter = 0x46,
    ViewportZCenter = 0x47,
    TexScaleU = 0x48,
    TexScaleV = 0x49,
    TexOffsetU = 0x4a,
    TexOffsetV = 0x4b,
    OffsetX = 0x4c,
    OffsetY = 0x4d,
    /// Flat or gouraud.
    ShadeMode = 0x50,
    ReverseNormal = 0x51,
    MaterialUpdate = 0x53,
    MaterialEmissive = 0x54, // not sure about these but this makes sense
    MaterialAmbient = 0x55,  // gotta try enabling lighting and check :)
    MaterialDiffuse = 0x56,
    MaterialSpecular = 0x57,
    MaterialAlpha = 0x58,
    MaterialSpecularCoef = 0x5b,
    AmbientColor = 0x5c,
    AmbientAlpha = 0x5d,
    LightMode = 0x5e,
    LightType0 = 0x5f,
    LightType1 = 0x60,
    LightType2 = 0x61,
    LightType3 = 0x62,
    Light0X = 0x63,
    Light0Y,
    Light0Z,
    Light1X,
    Light1Y,
    Light1Z,
    Light2X,
    Light2Y,
    Light2Z,
    Light3X,
    Light3Y,
    Light3Z,
    Light0DirectionX = 0x6f,
    Light0DirectionY,
    Light0DirectionZ,
    Light1DirectionX,
    Light1DirectionY,
    Light1DirectionZ,
    Light2DirectionX,
    Light2DirectionY,
    Light2DirectionZ,
    Light3DirectionX,
    Light3DirectionY,
    Light3DirectionZ,
    Light0ConstantAtten = 0x7b,
    Light0LinearAtten,
    Light0QuadtraticAtten,
    Light1ConstantAtten,
    Light1LinearAtten,
    Light1QuadtraticAtten,
    Light2ConstantAtten,
    Light2LinearAtten,
    Light2QuadtraticAtten,
    Light3ConstantAtten,
    Light3LinearAtten,
    Light3QuadtraticAtten,
    Light0ExponentAtten = 0x87,
    Light1ExponentAtten,
    Light2ExponentAtten,
    Light3ExponentAtten,
    Light0CutoffAtten = 0x8b,
    Light1CutoffAtten,
    Light2CutoffAtten,
    Light3CutoffAtten,
    Light0Ambient = 0x8f,
    Light0Diffuse,
    Light0Specular,
    Light1Ambient,
    Light1Diffuse,
    Light1Specular,
    Light2Ambient,
    Light2Diffuse,
    Light2Specular,
    Light3Ambient,
    Light3Diffuse,
    Light3Specular,
    Cull = 0x9b,
    FrameBufPtr = 0x9c,
    FrameBufWidth = 0x9d,
    ZBufPtr = 0x9e,
    ZBufWidth = 0x9f,
    TexAddr0 = 0xa0,
    TexAddr1,
    TexAddr2,
    TexAddr3,
    TexAddr4,
    TexAddr5,
    TexAddr6,
    TexAddr7,
    TexBufWidth0 = 0xa8,
    TexBufWidth1,
    TexBufWidth2,
    TexBufWidth3,
    TexBufWidth4,
    TexBufWidth5,
    TexBufWidth6,
    TexBufWidth7,
    ClutAddr = 0xb0,
    ClutAddrUpper = 0xb1,
    TransferSrc,
    TransferSrcW,
    TransferDst,
    TransferDstW,
    TexSize0 = 0xb8,
    TexSize1,
    TexSize2,
    TexSize3,
    TexSize4,
    TexSize5,
    TexSize6,
    TexSize7,
    TexMapMode = 0xc0,
    TexShadeLs = 0xc1,
    TexMode = 0xc2,
    TexFormat = 0xc3,
    LoadClut = 0xc4,
    ClutFormat = 0xc5,
    TexFilter = 0xc6,
    TexWrap = 0xc7,
    TexLevel = 0xc8,
    TexFunc = 0xc9,
    TexEnvColor = 0xca,
    TexFlush = 0xcb,
    TexSync = 0xcc,
    Fog1 = 0xcd,
    Fog2 = 0xce,
    FogColor = 0xcf,
    TexLodSlope = 0xd0,
    FramebufPixFormat = 0xd2,
    ClearMode = 0xd3,
    Scissor1 = 0xd4,
    Scissor2 = 0xd5,
    MinZ = 0xd6,
    MaxZ = 0xd7,
    ColorTest = 0xd8,
    ColorRef = 0xd9,
    ColorTestmask = 0xda,
    AlphaTest = 0xdb,
    StencilTest = 0xdc,
    StencilOp = 0xdd,
    ZTest = 0xde,
    BlendMode = 0xdf,
    BlendFixedA = 0xe0,
    BlendFixedB = 0xe1,
    Dith0 = 0xe2,
    Dith1,
    Dith2,
    Dith3,
    LogicOp = 0xe6,
    ZWriteDisable = 0xe7,
    MaskRgb = 0xe8,
    MaskAlpha = 0xe9,
    TransferStart = 0xea,
    TransferSrcPos = 0xeb,
    TransferDstPos = 0xec,
    TransferSize = 0xee,

    /// Vertex Screen/Texture/Color
    Vscx = 0xf0,
    Vscy = 0xf1,
    Vscz = 0xf2,
    Vtcs = 0xf3,
    Vtct = 0xf4,
    Vtcq = 0xf5,
    Vcv = 0xf6,
    Vap = 0xf7,
    Vfc = 0xf8,
    Vscv = 0xf9,

    Unknown03 = 0x03,
    Unknown0D = 0x0d,
    Unknown11 = 0x11,
    Unknown29 = 0x29,
    Unknown34 = 0x34,
    Unknown35 = 0x35,
    Unknown39 = 0x39,
    Unknown4E = 0x4e,
    Unknown4F = 0x4f,
    Unknown52 = 0x52,
    Unknown59 = 0x59,
    Unknown5A = 0x5a,
    UnknownB6 = 0xb6,
    UnknownB7 = 0xb7,
    UnknownD1 = 0xd1,
    UnknownED = 0xed,
    UnknownEF = 0xef,
    UnknownFA = 0xfa,
    UnknownFB = 0xfb,
    UnknownFC = 0xfc,
    UnknownFD = 0xfd,
    UnknownFE = 0xfe,
    NopFF = 0xff,
}

psp_extern! {
    #![name = "sceGe_user"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x1F6752AD)]
    /// Get the size of VRAM.
    ///
    /// # Return value
    ///
    /// The size of VRAM (in bytes).
    pub fn sceGeEdramGetSize() -> u32;

    #[psp(0xE47E40E4)]
    /// Get the eDRAM address.
    ///
    /// # Return value
    ///
    /// A pointer to the base of the eDRAM.
    pub fn sceGeEdramGetAddr() -> *mut u8;

    #[psp(0xB77905EA)]
    /// Set the eDRAM address translation mode.
    ///
    /// # Parameters
    ///
    /// - `width`: 0 to not set the translation width, otherwise 512, 1024, 2048 or 4096.
    ///
    /// # Return value
    ///
    /// The previous width if it was set, otherwise 0, <0 on error.
    pub fn sceGeEdramSetAddrTranslation(width: i32) -> i32;

    #[psp(0xDC93CFEF)]
    /// Retrieve the current value of a GE command.
    ///
    /// # Parameters
    ///
    /// - `cmd`: The GE command register to retrieve (0 to 0xFF, both included).
    ///
    /// # Return value
    ///
    /// The value of the GE command, < 0 on error.
    pub fn sceGeGetCmd(cmd: i32) -> u32;

    #[psp(0x57C8945B)]
    /// Retrieve a matrix of the given type.
    ///
    /// # Parameters
    ///
    /// - `type_`: One of MatrixTypes.
    /// - `matrix`: Pointer to a variable to store the matrix.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceGeGetMtx(type_: GeMatrixType, matrix: *mut c_void) -> i32;

    #[psp(0xE66CB92E)]
    /// Retrieve the stack of the display list currently being executed.
    ///
    /// # Parameters
    ///
    /// - `stack_id`: The ID of the stack to retrieve.
    /// - `stack`: Pointer to a structure to store the stack, or NULL to not store it.
    ///
    /// # Return value
    ///
    /// The number of stacks of the current display list, < 0 on error.
    pub fn sceGeGetStack(stack_id: i32, stack: *mut GeStack) -> i32;

    #[psp(0x438A385A)]
    /// Save the GE's current state.
    ///
    /// # Parameters
    ///
    /// - `context`: Pointer to a GeContext.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceGeSaveContext(context: *mut GeContext) -> i32;

    #[psp(0x0BF608FB)]
    /// Restore a previously saved GE context.
    ///
    /// # Parameters
    ///
    /// - `context`: Pointer to a GeContext.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceGeRestoreContext(context: *const GeContext) -> i32;

    #[psp(0xAB49E76A)]
    /// Enqueue a display list at the tail of the GE display list queue.
    ///
    /// # Parameters
    ///
    /// - `list`: The head of the list to queue.
    /// - `stall`: The stall address. If NULL then no stall address is set and the list is
    /// transferred  immediately.
    /// - `cbid`: ID of the callback set by calling sceGeSetCallback
    /// - `arg`: Structure containing GE context buffer address
    ///
    /// # Return value
    ///
    /// ID of the queue, < 0 on error.
    pub fn sceGeListEnQueue(
       list: *const c_void,
       stall: *mut c_void,
       cbid: i32,
       arg: *mut GeListArgs,
    ) -> i32;

    #[psp(0x1C0D95A6)]
    /// Enqueue a display list at the head of the GE display list queue.
    ///
    /// # Parameters
    ///
    /// - `list`: The head of the list to queue.
    /// - `stall`: The stall address. If NULL then no stall address is set and
    ///   the list is transferred  immediately.
    /// - `cbid`: ID of the callback set by calling sceGeSetCallback
    /// - `arg`: Structure containing GE context buffer address
    ///
    /// # Return value
    ///
    /// ID of the queue, < 0 on error.
    pub fn sceGeListEnQueueHead(list: *const c_void, stall: *mut c_void, cbid: i32, arg: *mut GeListArgs) -> i32;

   #[psp(0x5FB86AB0)]
    /// Cancel a queued or running list.
    ///
    /// # Parameters
    ///
    /// - `qid`: The ID of the queue.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceGeListDeQueue(qid: i32) -> i32;

    #[psp(0xE0D68148)]
    /// Update the stall address for the specified queue.
    ///
    /// # Parameters
    ///
    /// - `qid`: The ID of the queue.
    /// - `stall`: The new stall address.
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub fn sceGeListUpdateStallAddr(qid: i32, stall: *mut c_void) -> i32;

    #[psp(0x03444EB4)]
    /// Wait for synchronisation of a list.
    ///
    /// # Parameters
    ///
    /// - `qid`: The queue ID of the list to sync.
    /// - `sync_type`: 0 if you want to wait for the list to be completed, or 1
    ///   if you just want to peek the actual state.
    ///
    /// # Return value
    ///
    /// The specified queue status, one of `GeListState`.
    pub fn sceGeListSync(qid: i32, sync_type: i32) -> GeListState;

    #[psp(0xB287BD61)]
    /// Wait for drawing to complete.
    ///
    /// # Parameters
    ///
    /// - `sync_type`: 0 if you want to wait for the drawing to be completed, or
    ///   1 if you just want to peek the actual state.
    ///
    /// # Return value
    ///
    /// The current queue status, one of GeListState.
    pub fn sceGeDrawSync(sync_type: i32) -> GeListState;

    #[psp(0xB448EC0D)]
    /// Interrupt drawing queue.
    ///
    /// # Parameters
    ///
    /// - `mode`: If set to 1, reset all the queues.
    /// - `p_param`: Unused (just K1-checked).
    ///
    /// # Return value
    ///
    /// The stopped queue ID if mode isnt set to 0, otherwise 0, and < 0 on error.
    pub fn sceGeBreak(mode: i32, p_param: *mut GeBreakParam) -> i32;

    #[psp(0x4C06E472)]
    /// Restart drawing queue.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceGeContinue() -> i32;

    #[psp(0xA4FC06A4)]
    /// Register callback handlers for the GE.
    ///
    /// # Parameters
    ///
    /// - `cb`: Configured callback data structure.
    ///
    /// # Return value
    ///
    /// The callback ID, < 0 on error.
    pub fn sceGeSetCallback(cb: *mut GeCallbackData) -> i32;

    #[psp(0x05DB22CE)]
    /// Unregister the callback handlers.
    ///
    /// # Parameters
    ///
    /// - `cbid`: The ID of the callbacks, returned by `sceGeSetCallback()`.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceGeUnsetCallback(cbid: i32) -> i32;
}
