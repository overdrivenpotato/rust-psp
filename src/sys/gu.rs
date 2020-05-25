use core::convert::TryFrom;
use core::ffi::c_void;
use core::ptr::null_mut;

use num_enum::TryFromPrimitive;

use crate::sys::ge::{GeBreakParam, GeCallbackData, GeContext, GeListArgs, GeStack};
use crate::sys::types::{FMatrix4, FVector3, IMatrix4, IVector4};

pub const PI: f32 = 3.141593;

/// Primitive types
#[repr(u32)]
pub enum Primitive {
    Points = 0,
    Lines = 1,
    LineStrip = 2,
    Triangles = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    Sprites = 6,
}

/// States
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum State {
    AlphaTest = 0,
    DepthTest = 1,
    ScissorTest = 2,
    StencilTest = 3,
    Blend = 4,
    CullFace = 5,
    Dither = 6,
    Fog = 7,
    ClipPlanes = 8,
    Texture2D = 9,
    Lighting = 10,
    Light0 = 11,
    Light1 = 12,
    Light2 = 13,
    Light3 = 14,
    LineSmooth = 15,
    PatchCullFace = 16,
    ColorTest = 17,
    ColorLogicOp = 18,
    FaceNormalReverse = 19,
    PatchFace = 20,
    Fragment2X = 21,
}

/// Matrix modes
#[repr(u32)]
pub enum MatrixMode {
    Projection = 0,
    View = 1,
    Model = 2,
    Texture = 3,
}

// Vertex Declarations Begin
const fn texture_shift(n: u32) -> u32 {
    n << 0
}

#[repr(u32)]
pub enum Texture {
    Texture8bit = texture_shift(1),
    Texture16bit = texture_shift(2),
    Texture32bitf = texture_shift(3),
}

const fn color_shift(n: u32) -> u32 {
    n << 2
}

#[repr(u32)]
pub enum Color {
    Color5650 = color_shift(4),
    Color5551 = color_shift(5),
    Color4444 = color_shift(6),
    Color8888 = color_shift(7),
}

const fn normal_shift(n: u32) -> u32 {
    n << 5
}

#[repr(u32)]
pub enum Normal {
    Normal8bit = normal_shift(1),
    Normal16bit = normal_shift(2),
    Normal32bitf = normal_shift(3),
}

const fn vertex_shift(n: u32) -> u32 {
    n << 7
}

#[repr(u32)]
pub enum Vertex {
    Vertex8bit = vertex_shift(1),
    Vertex16bit = vertex_shift(2),
    Vertex32bitf = vertex_shift(3),
}

const fn weight_shift(n: u32) -> u32 {
    n << 9
}

#[repr(u32)]
pub enum Weight {
    Weight8bit = weight_shift(1),
    Weight16bit = weight_shift(2),
    Weight32bitf = weight_shift(3),
}

const fn index_shift(n: u32) -> u32 {
    n << 11
}

#[repr(u32)]
pub enum Index {
    Index8bit = index_shift(1),
    Index16bit = index_shift(2),
}

const fn weights(n: u32) -> u32 {
    (((n) - 1) & 7) << 14
}

const fn vertices(n: u32) -> u32 {
    (((n) - 1) & 7) << 18
}

pub const WEIGHTS_BITS: u32 = weights(8);
pub const VERTICES_BITS: u32 = vertices(8);

const fn transform_shift(n: u32) -> u32 {
    n << 23
}

#[repr(u32)]
pub enum Transform {
    Transform3D = transform_shift(0),
    Transform2D = transform_shift(1),
}

// Vertex Declarations End

/// Pixel Formats
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum PixelFormat {
    Psm5650 = 0,
    Psm5551 = 1,
    Psm4444 = 2,
    Psm8888 = 3,
    PsmT4 = 4,
    PsmT8 = 5,
    PsmT16 = 6,
    PsmT32 = 7,
    PsmDxt1 = 8,
    PsmDxt3 = 9,
    PsmDxt5 = 10,
}

/// Spline Mode
#[repr(u32)]
pub enum SplineMode {
    FillFill = 0,
    OpenFill = 1,
    FillOpen = 2,
    OpenOpen = 3,
}

/// Shading Model
#[repr(u32)]
pub enum ShadingModel {
    Flat = 0,
    Smooth = 1,
}

/// Logical operation
#[repr(u32)]
pub enum LogicalOperation {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy_ = 3,
    AndInverted = 4,
    Noop = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equiv = 9,
    Inverted = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}

/// Texture Filter
#[repr(u32)]
pub enum TextureFilter {
    Nearest = 0,
    Linear = 1,
    NearestMipmapNearest = 4,
    LinearMipmapNearest = 5,
    NearestMipmapLinear = 6,
    LinearMipmapLinear = 7,
}

/// Texture Map Mode
#[repr(u32)]
pub enum TextureMapMode {
    TextureCoords = 0,
    TextureMatrix = 1,
    EnvironmentMap = 2,
}

/// Texture Level Mode
#[repr(u32)]
pub enum TextureLevelMode {
    TextureAuto = 0,
    TextureConst = 1,
    TextureSlope = 2,
}

/// Texture Projection Map Mode
#[repr(u32)]
pub enum TextureProjectionMapMode {
    Position = 0,
    Uv = 1,
    NormalizedNormal = 2,
    Normal = 3,
}

/// Wrap Mode
#[repr(u32)]
pub enum WrapMode {
    Repeat = 0,
    Clamp = 1,
}

/// Front Face Direction
#[repr(u32)]
pub enum FrontFaceDirection {
    CW = 0,
    CCW = 1,
}

/// Test Function
#[repr(u32)]
pub enum TestFunction {
    Never = 0,
    Always = 1,
    Equal = 2,
    Notequal = 3,
    Less = 4,
    Lequal = 5,
    Greater = 6,
    Gequal = 7,
}

bitflags::bitflags!{
    /// Clear Buffer Mask
    pub struct ClearBuffer: u32 {
        const ColorBufferBit = 1;
        const StencilBufferBit = 2;
        const DepthBufferBit = 4;
        const FastClearBit = 16;
    }
}

/// Texture Effect
#[repr(u32)]
pub enum TextureEffect {
    Modulate = 0,
    Decal = 1,
    Blend = 2,
    Replace = 3,
    Add = 4,
}

/// Texture Color Component
#[repr(u32)]
pub enum TextureColorComponent {
    Rgb = 0,
    Rgba = 1,
}

/// Blending Op
#[repr(u32)]
pub enum BlendingOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
    Abs = 5,
}

/// Blending Factor Source
#[repr(u32)]
pub enum BlendingFactorSrc {
    SrcColor = 0,
    OneMinusSrcColor = 1,
    SrcAlpha = 2,
    OneMinusSrcAlpha = 3,
}

/// Blending Factor Destination
pub enum BlendingFactorDst {
    DstColor = 0,
    OneMinusDstColor = 1,
    DstAlpha = 4,
    OneMinusDstAlpha = 5,
    Fix = 10,
}

/// Stencil Operations
#[repr(u32)]
pub enum StencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    Invert = 3,
    Incr = 4,
    Decr = 5,
}

bitflags::bitflags!(
/// Light Components
    pub struct LightComponent: i32 {
        const Ambient = 1;
        const Diffuse = 2;
        const Specular = 4;
        const UnknownLightComponent = 8;
    }
);

/// Light modes
#[repr(u32)]
pub enum LightMode {
    SingleColor = 0,
    SeparateSpecularColor = 1,
}

/// Light Type
#[repr(u32)]
pub enum LightType {
    Directional = 0,
    Pointlight = 1,
    Spotlight = 2,
}

/// Contexts
#[repr(u32)]
#[derive(TryFromPrimitive)]
pub enum Context {
    Direct = 0,
    Call = 1,
    Send_ = 2,
}

/// List Queue
#[repr(u32)]
pub enum ListQueue {
    Tail = 0,
    Head = 1,
}

/// Sync behavior (mode)
#[repr(u32)]
pub enum SyncMode {
    SyncFinish = 0,
    SyncSignal = 1,
    SyncDone = 2,
    SyncList = 3,
    SyncSend = 4,
}

/// behavior (what)
#[repr(u32)]
pub enum SyncModeWhat {
    SyncWait = 0,
    SyncNowait = 1,
}

/// Sync behavior (what)
#[repr(u32)]
pub enum SyncBehaviorWhat {
    SyncWhatDone = 0,
    SyncWhatQueued = 1,
    SyncWhatDraw = 2,
    SyncWhatStall = 3,
    SyncWhatCancel = 4,
}

/// Signals
#[repr(u32)]
pub enum Signal {
    CallbackSignal = 1,
    CallbackFinish = 4,
}

/// Signal behavior
#[repr(u32)]
pub enum SignalBehavior {
    BehaviorSuspend = 1,
    BehaviorContinue = 2,
}

#[inline]
/// Color Macros, maps 8 bit unsigned channels into one 32-bit value */
pub const fn abgr(a: u8, b: u8, g: u8, r: u8) -> u32 {
    let mut res: u32 = 0;
    res += (a as u32) << 24;
    res += (b as u32) << 16;
    res += (g as u32) << 8;
    res += r as u32;
    return res;
}

#[inline]
pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
    abgr(a, b, g, r)
}

#[inline]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    argb(a, r, g, b)
}

#[inline]
/// Color Macro, maps floating point channels (0..1) into one 32-bit value
pub unsafe fn color(r: f32, g: f32, b: f32, a: f32) -> u32 {
    rgba(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    )
}

pub type GuCallback = Option<fn(arg: i32)>;
pub type GuSwapBuffersCallback = Option<fn(display: *mut *mut c_void, render: *mut *mut c_void)>;

//GU INTERNAL
struct GuSettings {
    sig: GuCallback,
    fin: GuCallback,
    signal_history: [i16; 16],
    signal_offset: u32,
    kernel_event_flag: u32,
    ge_callback_id: u32,
    swapBuffersCallback: GuSwapBuffersCallback,
    swapBuffersBehaviour: crate::sys::display::DisplaySetBufSync,
}

struct GuDisplayList {
    start: *mut u32,
    current: *mut u32,
    parent_context: i32,
}

struct GuContext {
    list: GuDisplayList,
    scissor_enable: i32,
    scissor_start: [i32; 2],
    scissor_end: [i32; 2],
    near_plane: i32,
    far_plane: i32,
    depth_offset: i32,
    fragment_2x: i32,
    texture_function: i32,
    texture_proj_map_mode: i32,
    texture_map_mode: i32,
    sprite_mode: [i32; 4],
    clear_color: u32,
    clear_stencil: u32,
    clear_depth: u32,
    texture_mode: i32,
}

struct GuDrawBuffer {
    pixel_size: PixelFormat,
    frame_width: i32,
    frame_buffer: *mut c_void,
    disp_buffer: *mut c_void,
    depth_buffer: *mut c_void,
    depth_width: i32,
    width: i32,
    height: i32,
}

struct GuLightSettings {
    /// Light enable
    enable: u8,
    /// Light type
    type_: u8,
    /// X position
    xpos: u8,
    /// Y position
    ypos: u8,
    /// Z position
    zpos: u8,
    /// X direction
    xdir: u8,
    /// Y direction
    ydir: u8,
    /// Z direction
    zdir: u8,

    /// Ambient color
    ambient: u8,
    /// Diffuse color
    diffuse: u8,
    /// Specular color
    specular: u8,
    /// Constant attenuation
    constant: u8,
    /// Linear attenuation
    linear: u8,
    /// Quadratic attenuation
    quadratic: u8,
    /// Light exponent
    exponent: u8,
    /// Light cutoff
    cutoff: u8,
}

static mut gu_current_frame: u32 = 0;
static mut gu_contexts: [GuContext; 3] = [
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: 0,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: 0,
        texture_map_mode: 0,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: 0,
    },
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: 0,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: 0,
        texture_map_mode: 0,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: 0,
    },
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: 0,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: 0,
        texture_map_mode: 0,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: 0,
    },
];

static mut ge_list_executed: [i32; 2] = [0, 0];
static mut ge_edram_address: *mut c_void = null_mut();

static mut gu_settings: GuSettings = GuSettings {
    sig: None,
    fin: None,
    signal_history: [0; 16],
    signal_offset: 0,
    kernel_event_flag: 0,
    ge_callback_id: 0,
    swapBuffersBehaviour: crate::sys::display::DisplaySetBufSync::Immediate,
    swapBuffersCallback: None,
};

static mut gu_list: *mut GuDisplayList = null_mut();
static mut gu_curr_context: i32 = 0;
static mut gu_init: i32 = 0;
static mut gu_display_on: bool = false;
static mut gu_call_mode: i32 = 0;
static mut gu_states: i32 = 0;

static mut gu_draw_buffer: GuDrawBuffer = GuDrawBuffer {
    depth_buffer: null_mut(),
    frame_buffer: null_mut(),
    disp_buffer: null_mut(),
    width: 0,
    height: 0,
    depth_width: 0,
    frame_width: 0,
    pixel_size: PixelFormat::Psm5650,
};

static mut gu_object_stack: *mut *mut u32 = null_mut();
static mut gu_object_stack_depth: i32 = 0;
static mut light_settings: [GuLightSettings; 4] = [
    GuLightSettings {
        enable: 0x18,
        type_: 0x5f,
        xpos: 0x63,
        ypos: 0x64,
        zpos: 0x65,
        xdir: 0x6f,
        ydir: 0x70,
        zdir: 0x71,
        ambient: 0x8f,
        diffuse: 0x90,
        specular: 0x91,
        constant: 0x7b,
        linear: 0x7c,
        quadratic: 0x7d,
        exponent: 0x87,
        cutoff: 0x8b,
    },
    GuLightSettings {
        enable: 0x19,
        type_: 0x60,
        xpos: 0x66,
        ypos: 0x67,
        zpos: 0x68,
        xdir: 0x72,
        ydir: 0x73,
        zdir: 0x74,
        ambient: 0x92,
        diffuse: 0x93,
        specular: 0x94,
        constant: 0x7e,
        linear: 0x7f,
        quadratic: 0x80,
        exponent: 0x88,
        cutoff: 0x8c,
    },
    GuLightSettings {
        enable: 0x1A,
        type_: 0x61,
        xpos: 0x69,
        ypos: 0x6A,
        zpos: 0x6B,
        xdir: 0x75,
        ydir: 0x76,
        zdir: 0x77,
        ambient: 0x95,
        diffuse: 0x99,
        specular: 0x9A,
        constant: 0x84,
        linear: 0x82,
        quadratic: 0x83,
        exponent: 0x89,
        cutoff: 0x8d,
    },
    GuLightSettings {
        enable: 0x1B,
        type_: 0x62,
        xpos: 0x6c,
        ypos: 0x6d,
        zpos: 0x6e,
        xdir: 0x78,
        ydir: 0x79,
        zdir: 0x7A,
        ambient: 0x98,
        diffuse: 0x99,
        specular: 0x9A,
        constant: 0x84,
        linear: 0x85,
        quadratic: 0x86,
        exponent: 0x8a,
        cutoff: 0x8e,
    },
];

#[inline]
pub unsafe fn send_command_i(cmd: i32, argument: i32) {
    (*(*gu_list).current) = ((cmd << 24) | (argument & 0xffffff)) as u32;
    (*gu_list).current = (*gu_list).current.add(1);
}

#[inline]
pub unsafe fn send_command_f(cmd: i32, argument: f32) {
    send_command_i(cmd, core::mem::transmute::<f32, i32>(argument));
}

#[inline]
pub unsafe fn send_command_i_stall(cmd: i32, argument: i32) {
    send_command_i(cmd, argument);
    if gu_object_stack_depth == 0 && gu_curr_context == 0 {
        crate::sys::ge::sce_ge_list_update_stall_addr(
            ge_list_executed[0],
            (*gu_list).current as *mut c_void
        );        
    }
}

pub unsafe fn draw_region(x: i32, y: i32, width: i32, height: i32) {
    send_command_i(21, (y << 10) | x);
    send_command_i(22, (((y + height)-1) << 10) | ((x + width)-1));
}

/// Set depth buffer parameters
///
/// # Parameters
///
/// - `zbp`: VRAM pointer where the depthbuffer should start
/// - `zbw`: The width of the depth-buffer (block-aligned)
pub unsafe fn sce_gu_depth_buffer(zbp: *mut c_void, zbw: i32) {
    gu_draw_buffer.depth_buffer = zbp;

    if gu_draw_buffer.depth_width == 0 || gu_draw_buffer.depth_width != zbw {
        gu_draw_buffer.depth_width = zbw;
    }
    // not sure if this weird conversion chain does anything but it's what C does
    send_command_i(158, ((zbp as u32) & 0xffffff) as i32);
    send_command_i(159, (((zbp as u32) & 0xff000000) >> 8 | zbw as u32) as i32);
}

/// Set display buffer parameters
///
/// # Parameters
///
/// - `width`: Width of the display buffer in pixels
/// - `height`: Width of the display buffer in pixels
/// - `dispbp`: VRAM pointer to where the display-buffer starts
/// - `dispbw`: Display buffer width (block aligned)
pub unsafe fn sce_gu_disp_buffer(
    width: i32,
    height: i32,
    dispbp: *mut c_void,
    dispbw: i32) {

    gu_draw_buffer.width = width;
    gu_draw_buffer.height = height;
    gu_draw_buffer.disp_buffer = dispbp;

    if gu_draw_buffer.frame_width == 0 || gu_draw_buffer.frame_width != dispbw {
        gu_draw_buffer.frame_width = dispbw;
    }

    draw_region(0, 0, gu_draw_buffer.width, gu_draw_buffer.height);
    crate::sys::display::sce_display_set_mode(
        crate::sys::display::DisplayMode::Lcd,
        gu_draw_buffer.width as usize,
        gu_draw_buffer.height as usize
    );

    if gu_display_on == true {
        crate::sys::display::sce_display_set_frame_buf(
            (ge_edram_address as *mut u8).add(gu_draw_buffer.disp_buffer as usize),
             dispbw as usize,
             crate::sys::display::DisplayPixelFormat::try_from(gu_draw_buffer.pixel_size as u32).unwrap(),
             crate::sys::display::DisplaySetBufSync::NextFrame
        );
    }
}

/// Set draw buffer parameters (and store in context for buffer-swap)
/// - `psm`: Pixel format to use for rendering (and display)
/// - `fbp`: VRAM pointer to where the draw buffer starts
/// - `fbw`: Frame buffer width (block aligned)
pub unsafe fn sce_gu_draw_buffer(psm: PixelFormat, fbp: *mut c_void, fbw: i32) {
    gu_draw_buffer.pixel_size = psm;
    gu_draw_buffer.frame_width = fbw;
    gu_draw_buffer.frame_buffer = fbp;

    if gu_draw_buffer.depth_buffer.is_null() && gu_draw_buffer.height != 0 {
        gu_draw_buffer.depth_buffer = (
            fbp as u32 + (
                (gu_draw_buffer.height * fbw) as u32) << 2u32
            ) as *mut c_void;
    }

    if gu_draw_buffer.depth_width == 0 {
        gu_draw_buffer.depth_width = fbw;
    }

    send_command_i(210, psm as i32);
    send_command_i(156, (gu_draw_buffer.frame_buffer as u32 & 0xffffff) as i32);
    send_command_i(157, (
            ((gu_draw_buffer.frame_buffer as u32 & 0xff000000) >> 8)
            | gu_draw_buffer.frame_width as u32
            ) as i32
    );
    send_command_i(158, (gu_draw_buffer.depth_buffer as u32 & 0xffffff) as i32);
    send_command_i(159, (
            ((gu_draw_buffer.depth_buffer as u32 & 0xff000000) >> 8)
            | gu_draw_buffer.depth_width as u32
            ) as i32
    );
}

/// Set draw buffer directly, not storing parameters in the context
///
/// # Parameters
///
/// - `psm`: Pixel format to use for rendering
/// - `fbp`: VRAM pointer to where the draw buffer starts
/// - `fbw`: Frame buffer width (block aligned)
pub unsafe fn sce_gu_draw_buffer_list(psm: PixelFormat, fbp: *mut c_void, fbw: i32) {
    send_command_i(210, psm as i32);
    send_command_i(156, (fbp as u32 & 0xffffff) as i32);
    send_command_i(157, (((fbp as u32 &  0xff000000) >> 8) | fbw as u32) as i32);
}

/// Turn display on or off
///
/// # Parameters
///
/// - `state`: Turn display on or off
/// # Return Value
///
/// State of the display prior to this call
pub unsafe fn sce_gu_display(state: bool) -> bool {
    if state {
        crate::sys::display::sce_display_set_frame_buf(
            (ge_edram_address as *mut u8).add(gu_draw_buffer.disp_buffer as usize),
             gu_draw_buffer.frame_width as usize,
             crate::sys::display::DisplayPixelFormat::try_from(gu_draw_buffer.pixel_size as u32).unwrap(),
             crate::sys::display::DisplaySetBufSync::NextFrame
        );
    } else {
            crate::sys::display::sce_display_set_frame_buf(
                null_mut(),
                0,
                crate::sys::display::DisplayPixelFormat::_565,
                crate::sys::display::DisplaySetBufSync::NextFrame
            );
        }
        gu_display_on = state;
        state
}

/// Select which depth-test function to use
///
/// Valid choices for the depth-test are:
///   - GU_NEVER - No pixels pass the depth-test
///   - GU_ALWAYS - All pixels pass the depth-test
///   - GU_EQUAL - Pixels that match the depth-test pass
///   - GU_NOTEQUAL - Pixels that doesn't match the depth-test pass
///   - GU_LESS - Pixels that are less in depth passes
///   - GU_LEQUAL - Pixels that are less or equal in depth passes
///   - GU_GREATER - Pixels that are greater in depth passes
///   - GU_GEQUAL - Pixels that are greater or equal passes
///
/// # Parameters
///
/// - `function`: Depth test function to use
pub unsafe fn sce_gu_depth_func(function: i32) {
    send_command_i(222, function);
}

/// Mask depth buffer writes
///
/// # Parameters
///
/// - `mask`: GU_TRUE(1) to disable Z writes, GU_FALSE(0) to enable
pub unsafe fn sce_gu_depth_mask(mask: i32) {
    send_command_i(231, mask);
}

pub unsafe fn sce_gu_depth_offset(offset: u32) {
    let context: &mut GuContext = &mut gu_contexts[gu_curr_context as usize];
    context.depth_offset = offset as i32;
    sce_gu_depth_range(context.near_plane, context.far_plane);
}

/// Set which range to use for depth calculations.
///
/// @note The depth buffer is inversed, and takes values from 65535 to 0.
///
/// Example: Use the entire depth-range for calculations:
/// @code
/// sceGuDepthRange(65535,0) {
/// @endcode
///
/// # Parameters
///
/// - `near`: Value to use for the near plane
/// - `far`: Value to use for the far plane
pub unsafe fn sce_gu_depth_range(near: i32, far: i32) {
    let context: &mut GuContext = &mut gu_contexts[gu_curr_context as usize];
    let max: u32 = near as u32 + far as u32;
    let val: i32 = ((max >> 31) + max) as i32;
    let z: f32 = (val >> 1) as f32;

    context.near_plane = near;
    context.far_plane = far;
    send_command_f(68, z - near as f32);
    send_command_f(71, z + context.depth_offset as f32);

    if near > far {
        let temp = near;
        let near = far;
        let far = temp;
    }
    send_command_i(214, near);
    send_command_i(215, far);
}

pub unsafe fn sce_gu_fog(near: f32, far: f32, color: u32) {
    let mut distance: f32 = far-near;

    if distance != 0.0 {
        distance = 1.0 / distance;
    }
    send_command_i(207, (color & 0xffffff) as i32);
    send_command_f(205, far);
    send_command_f(206, distance);
}

/// Initalize the GU system
/// This function MUST be called as the first function, otherwise state is undetermined.
pub unsafe fn sce_gu_init() {
    unimplemented!()
}

/// Shutdown the GU system
/// Called when GU is no longer needed
pub unsafe fn sce_gu_term() {
    unimplemented!()
}

pub unsafe fn sce_gu_break(a0: i32) {
    // This is actually unimplemented in PSPSDK
    unimplemented!()
}

pub unsafe fn sce_gu_continue() {
    // This is actually unimplemented in PSPSDK
    unimplemented!()
}

/// Setup signal handler
///
/// Available signals are:
///   - GU_CALLBACK_SIGNAL - Called when sceGuSignal is used
///   - GU_CALLBACK_FINISH - Called when display list is finished
///
/// # Parameters
///
/// - `signal`: Signal index to install a handler for
/// - `callback`: Callback to call when signal index is triggered
/// # Return Value
///
/// The old callback handler
pub unsafe fn sce_gu_set_callback(
    signal: i32,
    callback: Option<fn(arg1: i32)>,
) -> Option<fn(arg1: i32)> {
    let mut old_callback: GuCallback = None; 

    match signal {
        GU_CALLBACK_SIGNAL => {
            old_callback = gu_settings.sig;
            gu_settings.sig = callback;
        },
        GU_CALLBACK_FINISH => {
            old_callback = gu_settings.fin;
            gu_settings.fin = callback;
        }
    }

    old_callback
}

/// Trigger signal to call code from the command stream
///
/// Available behaviors are:
///   - GU_BEHAVIOR_SUSPEND - Stops display list execution until callback function finished
///   - GU_BEHAVIOR_CONTINUE - Do not stop display list execution during callback
///
/// # Parameters
///
/// - `signal`: Signal to trigger
/// - `behavior`: Behavior type
pub unsafe fn sce_gu_signal(signal: i32, behavior: i32) {
    send_command_i(14, ((signal & 0xff) << 16) | (behavior & 0xffff));
    send_command_i(12, 0);

    if signal == 3 {
        send_command_i(15, 0);
        send_command_i(12, 0);
    }
    send_command_i_stall(0, 0);
}

/// Send raw float-command to the GE
///
/// The argument is converted into a 24-bit float before transfer.
///
/// # Parameters
///
/// - `cmd`: Which command to send
/// - `argument`: Argument to pass along
pub unsafe fn sce_gu_send_commandf(cmd: i32, argument: f32) {
    send_command_f(cmd, argument);
}

/// Send raw command to the GE
///
/// Only the 24 lower bits of the argument is passed along.
///
/// # Parameters
///
/// - `cmd`: Which command to send
/// - `argument`: Argument to pass along
pub unsafe fn sce_gu_send_commandi(cmd: i32, argument: i32) {
    send_command_i(cmd, argument);
}

/// Allocate memory on the current display list for temporary storage
///
/// @note This function is NOT for permanent memory allocation, the
/// memory will be invalid as soon as you start filling the same display
/// list again.
///
/// # Parameters
///
/// - `size`: How much memory to allocate
/// # Return Value
///
/// Memory-block ready for use
pub unsafe fn sce_gu_get_memory(mut size: i32) -> *mut c_void {
    // some kind of 4-byte alignment?
    size = size + 3;
    size = (((size >> 31) as u32) >> 30) as i32;
    size = (size >> 2) << 2;

    let orig_ptr: *mut u32 = (*gu_list).current;
    let new_ptr: *mut u32 = orig_ptr.add(size as usize + 8);

    let lo: i32 = ((8 << 24) | (new_ptr as u32) & 0xffffff) as i32;
    let hi: i32 = ((16 << 24) | (new_ptr as u32 >> 8) & 0xf0000) as i32;

    (*orig_ptr) = hi as u32;
    (*orig_ptr.offset(1)) = lo as u32;

    (*gu_list).current = new_ptr;

    if gu_curr_context == 0 {
        crate::sys::ge::sce_ge_list_update_stall_addr(
            ge_list_executed[0],
            new_ptr as *mut c_void
        );
    }

    orig_ptr.add(2) as *mut c_void
}

/// Start filling a new display-context
///
/// Contexts available are:
///   - GU_DIRECT - Rendering is performed as list is filled
///   - GU_CALL - List is setup to be called from the main list
///   - GU_SEND - List is buffered for a later call to sceGuSendList()
///
/// The previous context-type is stored so that it can be restored at sceGuFinish().
///
/// # Parameters
///
/// - `cid`: Context Type
/// - `list`: Pointer to display-list (16 byte aligned)
pub unsafe fn sce_gu_start(cid: i32, list: *mut c_void) {
    let mut context: &mut GuContext = &mut gu_contexts[cid as usize];
    let local_list: *mut u32 = ((list as u32) | 0x40000000) as *mut u32;

    // setup display list
    (*context).list.start = local_list;
    (*context).list.current = local_list;
    (*context).list.parent_context = gu_curr_context;
    gu_list = &mut context.list;

    // store current context
    gu_curr_context = cid;

    if cid == 0 {
        ge_list_executed[0] = crate::sys::ge::sce_ge_list_enqueue(
            local_list as *mut c_void,
            local_list as *mut c_void,
            gu_settings.ge_callback_id as i32,
            &mut crate::sys::ge::GeListArgs::default() 
            as *mut crate::sys::ge::GeListArgs
        );
        gu_settings.signal_offset = 0;
    }

    if gu_init == 0 {
        static mut dither_matrix: IMatrix4 = IMatrix4 {
            x: IVector4 {
                x:-4, y: 0, z: -3, w: 1,
            },
            y: IVector4 {
                x: 2, y: -2, z: 3, w: -1,
            },
            z: IVector4 {
                x: -3, y: 1, z: -4, w: 0,
            },
            w: IVector4 {
                x: 3, y: -1, z: 2, w: -2
            }
        };

        sce_gu_set_dither(&mut dither_matrix as *mut IMatrix4);
        sce_gu_patch_divide(16, 16);
        sce_gu_color_material(
            LightComponent::Ambient |
            LightComponent::Diffuse |
            LightComponent::Specular
        );

        sce_gu_specular(1.0);
        sce_gu_tex_scale(1.0, 1.0);

        gu_init = 1;
    }

    if gu_curr_context == 0 {
        if gu_draw_buffer.frame_width != 0 {
            send_command_i(156,
                (gu_draw_buffer.frame_buffer as u32 & 0xffffff) as i32
            );
            send_command_i(157,
                ((gu_draw_buffer.frame_buffer as u32 & 0xff000000) >> 8 
                | (gu_draw_buffer.frame_width as u32)) as i32
            );
        }
    }
}

/// Finish current display list and go back to the parent context
///
/// If the context is GU_DIRECT, the stall-address is updated so that the entire list will
/// execute. Otherwise, only the terminating action is written to the list, depending on
/// context-type.
///
/// The finish-callback will get a zero as argument when using this function.
///
/// This also restores control back to whatever context that was active prior to this call.
///
/// # Return Value
///
/// Size of finished display list
pub unsafe fn sce_gu_finish() -> i32 {
    match Context::try_from(gu_curr_context as u32).unwrap() {
        Context::Direct | Context::Send_ => {
           send_command_i(15, 0);
           send_command_i_stall(12,0);
        },
        Context::Call => {
            if gu_call_mode == 1 {
                send_command_i(14, 0x120000);
                send_command_i(12, 0);
                send_command_i_stall(0,0);
            } else {
                send_command_i(11, 0);
            }
        }

    }

    let size: usize = (*gu_list).current.sub((*gu_list).start as usize) as usize;
    
    // go to parent list
    gu_curr_context = (*gu_list).parent_context;
    gu_list = &mut gu_contexts[gu_curr_context as usize].list;
    size as i32
}

/// Finish current display list and go back to the parent context, sending argument id for
/// the finish callback.
///
/// If the context is GU_DIRECT, the stall-address is updated so that the entire list will
/// execute. Otherwise, only the terminating action is written to the list, depending on
/// context-type.
///
/// # Parameters
///
/// - `id`: Finish callback id (16-bit)
/// # Return Value
///
/// Size of finished display list
pub unsafe fn sce_gu_finish_id(id: u32) -> i32 {
    match Context::try_from(gu_curr_context as u32).unwrap() {
        Context::Direct | Context::Send_ => {
           send_command_i(15, (id & 0xffff) as i32);
           send_command_i_stall(12,0);
        },
        Context::Call => {
            if gu_call_mode == 1 {
                send_command_i(14, 0x120000);
                send_command_i(12, 0);
                send_command_i_stall(0,0);
            } else {
                send_command_i(11, 0);
            }
        }

    }

    let size: usize = (*gu_list).current.sub((*gu_list).start as usize) as usize;
    
    // go to parent list
    gu_curr_context = (*gu_list).parent_context;
    gu_list = &mut gu_contexts[gu_curr_context as usize].list;
    size as i32

}

/// Call previously generated display-list
///
/// # Parameters
///
/// - `list`: Display list to call
pub unsafe fn sce_gu_call_list(list: *const c_void) {
    let list_addr: u32 = list as u32;

    if gu_call_mode == 1 {
        send_command_i(14, ((list_addr >> 16) | 0x110000) as i32);
        send_command_i(12, (list_addr & 0xffff) as i32);
        send_command_i_stall(0, 0);
    } else {
        send_command_i(16, ((list_addr >> 8) & 0xf0000) as i32);
        send_command_i_stall(10, (list_addr & 0xffffff) as i32);
    }
}

/// Set wether to use stack-based calls or signals to handle execution of called lists.
///
/// # Parameters
///
/// - `mode`: GU_TRUE(1) to enable signals, GU_FALSE(0) to disable signals and use
/// normal calls instead.
pub unsafe fn sce_gu_call_mode(mode: i32) {
    gu_call_mode = mode;
}

/// Check how large the current display-list is
///
/// # Return Value
///
/// The size of the current display list
pub unsafe fn sce_gu_check_list() -> i32 {
    (*gu_list).current.sub((*gu_list).start as usize) as i32
}

/// Send a list to the GE directly
///
/// Available modes are:
///   - GU_TAIL - Place list last in the queue, so it executes in-order
///   - GU_HEAD - Place list first in queue so that it executes as soon as possible
///
/// # Parameters
///
/// - `mode`: Whether to place the list first or last in queue
/// - `list`: List to send
/// - `context`: Temporary storage for the GE context
pub unsafe fn sce_gu_send_list(mode: ListQueue, list: *const c_void, context: *mut GeContext) {
    gu_settings.signal_offset = 0;

    let mut args: crate::sys::ge::GeListArgs = GeListArgs::default();
    args.size = 8;
    args.context = context;

    let mut list_id: i32 = 0;
    let callback = gu_settings.ge_callback_id;

    match mode {
        ListQueue::Head => {
            list_id = crate::sys::ge::sce_ge_list_enqueue_head(
                list,
                null_mut(),
                callback as i32,
                &mut args
            );
        },
        ListQueue::Tail => {
            list_id = crate::sys::ge::sce_ge_list_enqueue(
                list,
                null_mut(),
                callback as i32,
                &mut args
            );
        }
    }
    ge_list_executed[1] = list_id;
}

/// Swap display and draw buffer
///
/// # Return Value
///
/// Pointer to the new drawbuffer
pub unsafe fn sce_gu_swap_buffers() -> *mut c_void {
    if gu_settings.swapBuffersCallback != None {
        gu_settings.swapBuffersCallback.unwrap()(
            gu_draw_buffer.disp_buffer as *mut *mut c_void,
            gu_draw_buffer.frame_buffer as *mut *mut c_void
        );
    } else {
        let temp: *mut c_void = gu_draw_buffer.disp_buffer;
        gu_draw_buffer.disp_buffer = gu_draw_buffer.frame_buffer;
        gu_draw_buffer.frame_buffer = temp;
    }

    if gu_display_on {
        crate::sys::display::sce_display_set_frame_buf(
            ge_edram_address.add(gu_draw_buffer.disp_buffer as usize) as *const u8,
            gu_draw_buffer.frame_width as usize,
            crate::sys::display::DisplayPixelFormat::try_from(
                gu_draw_buffer.pixel_size as u32
            ).unwrap(),
            gu_settings.swapBuffersBehaviour
        );
    }

    //PSPSDK says this serves no purpose but fuck it
    gu_current_frame = gu_current_frame ^ 1;

    gu_draw_buffer.frame_buffer
}

/// Wait until display list has finished executing
///
/// # Parameters
///
/// - `mode`: What to wait for, one of SyncMode
/// - `what`: What to sync to, one of SyncBehaviorWhat
/// # Return Value
///
/// Unknown at this time
pub unsafe fn sce_gu_sync(mode: SyncMode, what: SyncBehaviorWhat) -> i32 {
    match mode {
        SyncMode::SyncFinish => 
            crate::sys::ge::sce_ge_draw_sync(what as i32),
        SyncMode::SyncList => 
            crate::sys::ge::sce_ge_list_sync(ge_list_executed[0], what as i32),
        SyncMode::SyncSend => 
            crate::sys::ge::sce_ge_list_sync(ge_list_executed[1], what as i32),
        _ => 0
    }
}

/// Draw array of vertices forming primitives
///
/// Available primitive-types are:
///   - GU_POINTS - Single pixel points (1 vertex per primitive)
///   - GU_LINES - Single pixel lines (2 vertices per primitive)
///   - GU_LINE_STRIP - Single pixel line-strip (2 vertices for the first primitive, 1 for every following)
///   - GU_TRIANGLES - Filled triangles (3 vertices per primitive)
///   - GU_TRIANGLE_STRIP - Filled triangles-strip (3 vertices for the first primitive, 1 for every following)
///   - GU_TRIANGLE_FAN - Filled triangle-fan (3 vertices for the first primitive, 1 for every following)
///   - GU_SPRITES - Filled blocks (2 vertices per primitive)
///
/// The vertex-type decides how the vertices align and what kind of information they contain.
/// The following flags are ORed together to compose the final vertex format:
///   - GU_TEXTURE_8BIT - 8-bit texture coordinates
///   - GU_TEXTURE_16BIT - 16-bit texture coordinates
///   - GU_TEXTURE_32BITF - 32-bit texture coordinates (float)
///
///   - GU_COLOR_5650 - 16-bit color (R5G6B5A0)
///   - GU_COLOR_5551 - 16-bit color (R5G5B5A1)
///   - GU_COLOR_4444 - 16-bit color (R4G4B4A4)
///   - GU_COLOR_8888 - 32-bit color (R8G8B8A8)
///
///   - GU_NORMAL_8BIT - 8-bit normals
///   - GU_NORMAL_16BIT - 16-bit normals
///   - GU_NORMAL_32BITF - 32-bit normals (float)
///
///   - GU_VERTEX_8BIT - 8-bit vertex position
///   - GU_VERTEX_16BIT - 16-bit vertex position
///   - GU_VERTEX_32BITF - 32-bit vertex position (float)
///
///   - GU_WEIGHT_8BIT - 8-bit weights
///   - GU_WEIGHT_16BIT - 16-bit weights
///   - GU_WEIGHT_32BITF - 32-bit weights (float)
///
///   - GU_INDEX_8BIT - 8-bit vertex index
///   - GU_INDEX_16BIT - 16-bit vertex index
///
///   - GU_WEIGHTS(n) - Number of weights (1-8)
///   - GU_VERTICES(n) - Number of vertices (1-8)
///
///   - GU_TRANSFORM_2D - Coordinate is passed directly to the rasterizer
///   - GU_TRANSFORM_3D - Coordinate is transformed before passed to rasterizer
///
/// Note Every vertex must align to 32 bits, which means that you HAVE to pad if it does not add up!
///
/// Vertex order:
/// [for vertices(1-8)]
/// [weights (0-8)]
/// [texture uv]
/// [color]
/// [normal]
/// [vertex]
/// [/for]
///
/// # Parameters
///
/// - `prim`: What kind of primitives to render
/// - `vtype`: Vertex type to process
/// - `count`: How many vertices to process
/// - `indices`: Optional pointer to an index-list
/// - `vertices`: Pointer to a vertex-list
pub unsafe fn sce_gu_draw_array(
    prim: Primitive,
    vtype: i32,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if vtype != 0 {
        send_command_i(18, vtype);
    }

    if !indices.is_null() {
        send_command_i(16, ((indices as u32 >> 8) & 0xf0000) as i32);
        send_command_i(2, (indices as u32 & 0xffffff) as i32);
    }

    if !vertices.is_null() {
        send_command_i(16, ((vertices as u32 >> 8) & 0xf0000) as i32);
        send_command_i(1, (vertices as u32 & 0xffffff) as i32);
    }

    send_command_i_stall(4, (((prim as u32) << 16) | count as u32) as i32);
}

/// Begin conditional rendering of object
///
/// If no vertices passed into this function are inside the scissor region, it will skip rendering
/// the object. There can be up to 32 levels of conditional testing, and all levels HAVE to
/// be terminated by sceGuEndObject().
///
/// /// # Parameters
///
/// - `vtype`: Vertex type to process
/// - `count`: Number of vertices to test
/// - `indices`: Optional list to an index-list
/// - `vertices`: Pointer to a vertex-list
pub unsafe fn sce_gu_begin_object(
    vtype: i32,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if vtype != 0 {
        send_command_i(18, vtype);
    }

    if !indices.is_null() {
        send_command_i(16, ((indices as u32 >> 8) & 0xf0000) as i32);
        send_command_i(2, (indices as u32 & 0xffffff) as i32);
    }

    if !vertices.is_null() {
        send_command_i(16, ((vertices as u32 >> 8) & 0xf0000) as i32);
        send_command_i(1, (vertices as u32 & 0xffffff) as i32);
    }

    send_command_i(7, count);

    // store start to new object

    (*gu_object_stack.offset(gu_object_stack_depth as isize)) = 
        (*gu_list).current as *mut u32;
    gu_object_stack_depth = gu_object_stack_depth + 1;
    
    // dummy commands, overwritten in sce_gu_end_object()
    send_command_i(16, 0);
    send_command_i(9, 0);
}

/// End conditional rendering of object
pub unsafe fn sce_gu_end_object() {
    // rewrite commands from sce_gu_begin_object
    let current = (*gu_list).current;
    (*gu_list).current = *gu_object_stack.offset(gu_object_stack_depth as isize -1);

    send_command_i(16, (current as u32 >> 8 & 0xf0000) as i32);
    send_command_i(9, (current as u32 & 0xffffff) as i32);
    (*gu_list).current = current;
    gu_object_stack_depth = gu_object_stack_depth - 1; 
}

/// Enable or disable GE state
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `state`: Which state to change
/// - `status`: Whether to enable or disable the state
pub unsafe fn sce_gu_set_status(state: State, status: bool) {
    if status {
        sce_gu_enable(state);
    } else {
        sce_gu_disable(state);
    }
}

/// Get if state is currently enabled or disabled
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `state`: Which state to query about
/// # Return Value
///
/// Wether state is enabled or not
pub unsafe fn sce_gu_get_status(state: State) -> bool {
    let state = state as u32;
    if state < 22 {
        return ((gu_states as u32 >> state as u32) & 1) != 0
    }
    false
}

/// Set the status on all 22 available states
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `status`: Bit-mask (0-21) containing the status of all 22 states
pub unsafe fn sce_gu_set_all_status(status: i32) {
    for i in 0..22 {
        if (status as u32 >> i) & 1 != 0 {
            sce_gu_enable(State::try_from(i).unwrap());
        } else {
            sce_gu_disable(State::try_from(i).unwrap());
        }
    }
}

/// Query status on all 22 available states
///
/// Look at sceGuEnable() for a list of states
///
/// # Return Value
///
/// Status of all 22 states as a bitmask (0-21)
pub unsafe fn sce_gu_get_all_status() -> i32 {
    gu_states
}

/// Enable GE state
///
/// # Parameters
///
/// - `state`: Which state to enable, one of State
pub unsafe fn sce_gu_enable(state: State) {
    match state {
        State::AlphaTest => send_command_i(34, 1),
        State::DepthTest => send_command_i(35, 1),
        State::ScissorTest => {
            let context = &mut gu_contexts[gu_curr_context as usize];
            context.scissor_enable = 1;
            send_command_i(
                212, 
                (context.scissor_start[1]<<10) | context.scissor_start[0]
            );
            send_command_i(
                213,
                (context.scissor_end[1]<<10) | context.scissor_end[0]
            );
        },
        State::StencilTest => send_command_i(36, 1),
        State::Blend => send_command_i(33, 1),
        State::CullFace => send_command_i(29, 1),
        State::Dither => send_command_i(32, 1),
        State::Fog => send_command_i(31, 1),
        State::ClipPlanes => send_command_i(28, 1),
        State::Texture2D => send_command_i(30, 1),
        State::Lighting => send_command_i(23, 1),
        State::Light0 => send_command_i(24, 1),
        State::Light1 => send_command_i(25, 1),
        State::Light2 => send_command_i(26, 1),
        State::Light3 => send_command_i(27, 1),
        State::LineSmooth => send_command_i(37, 1),
        State::PatchCullFace => send_command_i(38, 1),
        State::ColorTest => send_command_i(39, 1),
        State::ColorLogicOp => send_command_i(40, 1),
        State::FaceNormalReverse => send_command_i(81, 1),
        State::PatchFace => send_command_i(56, 1),
        State::Fragment2X => {
            let context = &mut gu_contexts[gu_curr_context as usize];
            context.fragment_2x = 0x10000;
            send_command_i(201, 0x10000 | context.texture_function);
        }
    }
    if (state as u32) < 22 {
        gu_states |= 1 << state as u32
    }
}

/// Disable GE state
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `state`: Which state to disable
pub unsafe fn sce_gu_disable(state: State) {
    match state {
        State::AlphaTest => send_command_i(34, 0),
        State::DepthTest => send_command_i(35, 0),
        State::ScissorTest => {
            let context = &mut gu_contexts[gu_curr_context as usize];
            context.scissor_enable = 0;
            send_command_i(212, 0);
            send_command_i(
                213,
                ((gu_draw_buffer.height-1) << 10) | gu_draw_buffer.width-1); 
        },
        State::StencilTest => send_command_i(36, 0),
        State::Blend => send_command_i(33, 0),
        State::CullFace => send_command_i(29, 0),
        State::Dither => send_command_i(32, 0),
        State::Fog => send_command_i(31, 0),
        State::ClipPlanes => send_command_i(28, 0),
        State::Texture2D => send_command_i(30, 0),
        State::Lighting => send_command_i(23, 0),
        State::Light0 => send_command_i(24, 0),
        State::Light1 => send_command_i(25, 0),
        State::Light2 => send_command_i(26, 0),
        State::Light3 => send_command_i(27, 0),
        State::LineSmooth => send_command_i(37, 0),
        State::PatchCullFace => send_command_i(38, 0),
        State::ColorTest => send_command_i(39, 0),
        State::ColorLogicOp => send_command_i(40, 0),
        State::FaceNormalReverse => send_command_i(81, 0),
        State::PatchFace => send_command_i(56, 0),
        State::Fragment2X => {
            let context = &mut gu_contexts[gu_curr_context as usize];
            context.fragment_2x = 0;
            send_command_i(201, context.texture_function);
        }
    }
    if (state as u32) < 22 {
        gu_states |= !(1 << state as u32)
    }
}

/// Set light parameters
///
/// # Parameters
///
/// - `light`: Light index
/// - `type`: Light type, one of LightType
/// - `components`: Light components, one of LightComponent
/// - `position`: Light position - FVector3
pub unsafe fn sce_gu_light(light: i32, type_: LightType, components: LightComponent, position: &FVector3) {
    let settings = &mut light_settings[light as usize];

    send_command_f(settings.xpos as i32, position.x);
    send_command_f(settings.ypos as i32, position.y);
    send_command_f(settings.zpos as i32, position.z);

    let mut kind: i32 = 2;
    if components.bits()  != 8 {
        kind = 
            if ((components.bits()) ^ 6) < 1 {
                1
            } else {
                0
            };
    }

    send_command_i(
        settings.type_ as i32,
        ((type_ as u32 & 0x03 << 8) | kind as u32) as i32
    );
}

/// Set light attenuation
///
/// # Parameters
///
/// - `light`: Light index
/// - `atten0`: Constant attenuation factor
/// - `atten1`: Linear attenuation factor
/// - `atten2`: Quadratic attenuation factor
pub unsafe fn sce_gu_light_att(light: i32, atten0: f32, atten1: f32, atten2: f32) {
    let settings = &mut light_settings[light as usize];
    send_command_f(settings.constant as i32, atten0);
    send_command_f(settings.linear as i32, atten1);
    send_command_f(settings.quadratic as i32, atten2);
}

/// Set light color
///
/// Available light components are:
///   - LightComponent::Ambient
///   - LightComponent::Diffuse
///   - LightComponent::Specular
/// and they may be logically OR'ed together
///
/// # Parameters
///
/// - `light`: Light index
/// - `component`: Which component to set
/// - `color`: Which color to use
pub unsafe fn sce_gu_light_color(light: i32, component: LightComponent, color: u32) {
    let settings = &mut light_settings[light as usize];
    if component.intersects(LightComponent::Ambient) {
        send_command_i(settings.ambient as i32, (color & 0xffffff) as i32);
    }
    if component.intersects(LightComponent::Diffuse) {
        send_command_i(settings.diffuse as i32, (color & 0xffffff) as i32);
    }
    if component.intersects(LightComponent::Specular) {
        send_command_i(settings.specular as i32, (color & 0xffffff) as i32);
    }
}

/// Set light mode
///
/// Available light modes are:
///   - GU_SINGLE_COLOR
///   - GU_SEPARATE_SPECULAR_COLOR
///
/// Separate specular colors are used to interpolate the specular component
/// independently, so that it can be added to the fragment after the texture color.
///
/// # Parameters
///
/// - `mode`: Light mode to use
pub unsafe fn sce_gu_light_mode(mode: i32) {
    send_command_i(94, mode);
}

/// Set spotlight parameters
///
/// # Parameters
///
/// - `light`: Light index
/// - `direction`: Spotlight direction
/// - `exponent`: Spotlight exponent
/// - `cutoff`: Spotlight cutoff angle (in radians)
pub unsafe fn sce_gu_light_spot(
    light: i32,
    direction: &FVector3,
    exponent: f32,
    cutoff: f32,
) {
    let settings = &mut light_settings[light as usize];

    send_command_f(settings.exponent as i32, exponent);
    send_command_f(settings.cutoff as i32, cutoff);

    send_command_f(settings.xdir as i32, direction.x);
    send_command_f(settings.ydir as i32, direction.y);
    send_command_f(settings.zdir as i32, direction.z);
}

/// Clear current drawbuffer
///
/// Available clear-flags are (OR them together to get final clear-mode):
///   - GU_COLOR_BUFFER_BIT - Clears the color-buffer
///   - GU_STENCIL_BUFFER_BIT - Clears the stencil-buffer
///   - GU_DEPTH_BUFFER_BIT - Clears the depth-buffer
///
/// # Parameters
///
/// - `flags`: Which part of the buffer to clear
pub unsafe fn sce_gu_clear(flags: ClearBuffer) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    let filter: u32;
    struct Vertex {
        color: u32,
        x: u16,
        y: u16,
        z: u16,
        _pad: u16
    }

    match gu_draw_buffer.pixel_size {
        PixelFormat::Psm5650 => filter = context.clear_color & 0xffffff,
        PixelFormat::Psm5551 => {
            filter = (context.clear_color & 0xffffff) | (context.clear_stencil << 31);
        },
        PixelFormat::Psm4444 => {
            filter = (context.clear_color & 0xffffff) | (context.clear_stencil << 28);
        },
        PixelFormat::Psm8888 => {
            filter = (context.clear_color & 0xffffff) | (context.clear_stencil << 24);
        },
        _ => {filter = 0;}
    }

    let vertices: *mut Vertex;
    let count: i32;

    if !flags.intersects(ClearBuffer::FastClearBit) {
        vertices = sce_gu_get_memory(
            2 * core::mem::size_of::<Vertex>() as i32
        ) as *mut Vertex;
        count = 2;

        (*vertices.offset(0)).color = 0;
        (*vertices.offset(0)).x = 0;
        (*vertices.offset(0)).y = 0;
        (*vertices.offset(0)).z = context.clear_depth as u16;

        (*vertices.offset(1)).color = filter;
        (*vertices.offset(1)).x = gu_draw_buffer.width as u16;
        (*vertices.offset(1)).y = gu_draw_buffer.height as u16;
        (*vertices.offset(1)).z = context.clear_depth as u16;
    } else {
        let mut curr: *mut Vertex;
        count = ((gu_draw_buffer.width + 63)/64)*2;
        vertices = sce_gu_get_memory(
            count * core::mem::size_of::<Vertex>() as i32
        ) as *mut Vertex;
        curr = vertices;

        for i in 0..count {
            curr = curr.add(1);
            let j: u32 = i as u32 >> 1;
            let k: u32 = i as u32 & 1;

            (*curr).color = filter;
            (*curr).x = (j+k) as u16 * 64;
            (*curr).y = ((k as u32) * (gu_draw_buffer.height as u32)) as u16;
            (*curr).z = context.clear_depth as u16;
        }
    }

    send_command_i(211, 
        ((flags & 
        ClearBuffer::ColorBufferBit |
        ClearBuffer::StencilBufferBit |
        ClearBuffer::DepthBufferBit).bits() << 8 | 0x01) as i32);

    sce_gu_draw_array(
        Primitive::Sprites,
        Color::Color8888  as i32 |
        self::Vertex::Vertex16bit as i32 |
        Transform::Transform2D as i32,
        count,
        null_mut(),
        vertices as *const c_void
    );
    send_command_i(211, 0);
}

/// Set the current clear-color
///
/// # Parameters
///
/// - `color`: Color to clear with
pub unsafe fn sce_gu_clear_color(color: u32) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    context.clear_color = color;
}

/// Set the current clear-depth
///
/// # Parameters
///
/// - `depth`: Set which depth to clear with (0x0000-0xffff)
pub unsafe fn sce_gu_clear_depth(depth: u32) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    context.clear_depth = depth;
}

/// Set the current stencil clear value
///
/// # Parameters
///
/// - `stencil`: Set which stencil value to clear with (0-255)
pub unsafe fn sce_gu_clear_stencil(stencil: u32) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    context.clear_stencil = stencil;
}

/// Set mask for which bits of the pixels to write
///
/// # Parameters
///
/// - `mask`: Which bits to filter against writes
pub unsafe fn sce_gu_pixel_mask(mask: u32) {
    send_command_i(232, (mask & 0xffffff) as i32);
    send_command_i(233, (mask >> 24) as i32);
}

/// Set current primitive color
///
/// # Parameters
///
/// - `color`: Which color to use (overriden by vertex-colors)
pub unsafe fn sce_gu_color(color: u32) {
    sce_gu_material(7, color as i32);
}

/// Set the color test function
///
/// The color test is only performed while GU_COLOR_TEST is enabled.
///
/// Available functions are:
///   - TestFunction::Never
///   - TestFunction::Always
///   - TestFunction::Equal
///   - TestFunction::NotEqual
///
/// # Parameters
///
/// - `func`: Color test function
/// - `color`: Color to test against
/// - `mask`: Mask ANDed against both source and destination when testing
pub unsafe fn sce_gu_color_func(func: TestFunction, color: u32, mask: u32) {
    send_command_i(216, (func as u32 & 0x03) as i32);
    send_command_i(217, (color & 0xffffff) as i32);
    send_command_i(218, mask as i32);
}

/// Set which color components that the material will receive
///
/// The components are ORed together from the following values:
///   - GU_AMBIENT
///   - GU_DIFFUSE
///   - GU_SPECULAR
///
/// # Parameters
///
/// - `components`: Which components to receive
pub unsafe fn sce_gu_color_material(components: LightComponent) {
    send_command_i(83, components.bits() as i32);
}

/// Set the alpha test parameters
///
/// Available comparison functions are:
///   - TestFunction::Never
///   - TestFunction::Always
///   - TestFunction::Equal
///   - TestFunction::NotEqual
///   - TestFunction::Less
///   - TestFunction::Lequal
///   - TestFunction::Greater
///   - TestFunction::Gequal
///
/// # Parameters
///
/// - `func`: Specifies the alpha comparison function.
/// - `value`: Specifies the reference value that incoming alpha values are compared to.
/// - `mask`: Specifies the mask that both values are ANDed with before comparison.
pub unsafe fn sce_gu_alpha_func(func: TestFunction, value: i32, mask: i32) {
    let arg = (func as u32 |
        ((value as u32 & 0xff) << 8) |
        ((mask as u32 & 0xff) << 16)
    )  as i32;
    send_command_i(219, arg);
}

pub unsafe fn sce_gu_ambient(color: u32) {
    send_command_i(92, (color & 0xffffff) as i32);
    send_command_i(93, (color >> 24) as i32);
}

pub unsafe fn sce_gu_ambient_color(color: u32) {
    send_command_i(85, (color & 0xffffff) as i32);
    send_command_i(88, (color >> 24) as i32);
}

/// Set the blending-mode
///
/// Keys for the blending operations:
///   - Cs - Source color
///   - Cd - Destination color
///   - Bs - Blend function for source fragment
///   - Bd - Blend function for destination fragment
///
/// # Parameters
///
/// - `op`: Blending Operation
/// - `src`: Blending function for source operand
/// - `dest`: Blending function for dest operand
/// - `srcfix`: Fix value for GU_FIX (source operand)
/// - `destfix`: Fix value for GU_FIX (dest operand)
pub unsafe fn sce_gu_blend_func(
    op: BlendingOperation,
    src: BlendingFactorSrc,
    dest: BlendingFactorDst,
    srcfix: u32,
    destfix: u32
) {
    send_command_i(
        223,
        (src as u32 | ((dest as u32) << 4) | ((op as u32) << 8)) as i32
    );
    send_command_i(224, (srcfix & 0xffffff) as i32);
    send_command_i(225, (destfix & 0xffffff) as i32);
}

// maybe convert mode to bitflags but idk what the options mean
pub unsafe fn sce_gu_material(mode: i32, color: i32) {
    if (mode & 0x01) != 0 {
        send_command_i(85, (color as u32 & 0xffffff) as i32);
        send_command_i(88, (color as u32 >> 24) as i32);
    }

    if (mode & 0x02) != 0 {
        send_command_i(86, (color as u32 & 0xffffff) as i32);
    }

    if (mode & 0x04) != 0 {
        send_command_i(87, (color as u32 & 0xffffff) as i32);
    }
}

pub unsafe fn sce_gu_model_color(
    emissive: u32,
    ambient: u32,
    diffuse: u32,
    specular: u32
) {
    send_command_i(84, (emissive & 0xffffff) as i32);
    send_command_i(85, (ambient & 0xffffff) as i32);
    send_command_i(86, (diffuse & 0xffffff) as i32);
    send_command_i(87, (specular & 0xffffff) as i32);
}

/// Set stencil function and reference value for stencil testing
///
/// Available functions are:
///   - TestFunction::Never
///   - TestFunction::Always
///   - TestFunction::Equal
///   - TestFunction::NotEqual
///   - TestFunction::Less
///   - TestFunction::Lequal
///   - TestFunction::Greater
///   - TestFunction::Gequal
///
/// # Parameters
///
/// - `func`: Test function
/// - `ref`: The reference value for the stencil test
/// - `mask`: Mask that is ANDed with both the reference value and stored stencil value when the test is done
pub unsafe fn sce_gu_stencil_func(func: TestFunction, ref_: i32, mask: i32) {
    send_command_i(
        220,
        (func as u32 |
        ((ref_ as u32) << 8) |
        ((mask as u32 & 0xff) << 16)) as i32
    );
}

/// Set the stencil test actions
///
/// As stencil buffer shares memory with framebuffer alpha, resolution of the buffer
/// is directly in relation.
///
/// # Parameters
///
/// - `fail`: The action to take when the stencil test fails
/// - `zfail`: The action to take when stencil test passes, but the depth test fails
/// - `zpass`: The action to take when both stencil test and depth test passes
pub unsafe fn sce_gu_stencil_op(
    fail: StencilOperation,
    zfail: StencilOperation,
    zpass: StencilOperation
) {
    send_command_i(
        221, 
        (fail as u32 | ((zfail as u32) << 8) | ((zpass as u32) << 16)) as i32
    );
}

/// Set the specular power for the material
///
/// # Parameters
///
/// - `power`: Specular power
pub unsafe fn sce_gu_specular(power: f32) {
    send_command_f(91, power);
}

/// Set the current face-order (for culling)
///
/// This only has effect when culling is enabled (GU_CULL_FACE)
///
/// Culling order is one of FrontFaceDirection
///
/// # Parameters
///
/// - `order`: Which order to use
pub unsafe fn sce_gu_front_face(order: FrontFaceDirection) {
    match order {
        FrontFaceDirection::CCW => send_command_i(155, 0),
        FrontFaceDirection::CW => send_command_i(155,1),
    }
}

/// Set color logical operation
///
/// Available operations are:
///   - GU_CLEAR
///   - GU_AND
///   - GU_AND_REVERSE
///   - GU_COPY
///   - GU_AND_INVERTED
///   - GU_NOOP
///   - GU_XOR
///   - GU_OR
///   - GU_NOR
///   - GU_EQUIV
///   - GU_INVERTED
///   - GU_OR_REVERSE
///   - GU_COPY_INVERTED
///   - GU_OR_INVERTED
///   - GU_NAND
///   - GU_SET
///
/// This operation only has effect if GU_COLOR_LOGIC_OP is enabled.
///
/// # Parameters
///
/// - `op`: Operation to execute
pub unsafe fn sce_gu_logical_op(op: LogicalOperation) {
    send_command_i(230, ((op as u32) & 0x0f) as i32);
}

/// Set ordered pixel dither matrix
///
/// This dither matrix is only applied if GU_DITHER is enabled.
///
/// # Parameters
///
/// - `matrix`: Dither matrix
pub unsafe fn sce_gu_set_dither(matrix: &IMatrix4) {
    send_command_i(226, (
        (matrix.x.x & 0x0f) |
        ((matrix.x.y & 0x0f) << 4) |
        ((matrix.x.z & 0x0f) << 8) |
        ((matrix.x.w & 0x0f) << 12)
    ) as i32);

    send_command_i(227, (
        (matrix.y.x & 0x0f) |
        ((matrix.y.y & 0x0f) << 4) |
        ((matrix.y.z & 0x0f) << 8) |
        ((matrix.y.w & 0x0f) << 12)
    ) as i32);

    send_command_i(228, (
        (matrix.z.x & 0x0f) |
        ((matrix.z.y & 0x0f) << 4) |
        ((matrix.z.z & 0x0f) << 8) |
        ((matrix.z.w & 0x0f) << 12)
    ) as i32);

    send_command_i(229, (
        (matrix.w.x & 0x0f) |
        ((matrix.w.y & 0x0f) << 4) |
        ((matrix.w.z & 0x0f) << 8) |
        ((matrix.w.w & 0x0f) << 12)
    ) as i32);

}

/// Set how primitives are shaded
///
/// # Parameters
///
/// - `mode`: Which mode to use, one of ShadingModel
pub unsafe fn sce_gu_shade_model(mode: ShadingModel) {
    match mode {
       ShadingModel::Flat => send_command_i(80, 0),
       ShadingModel::Smooth => send_command_i(80, 1),
    }
}

/// Image transfer using the GE
///
/// @note Data must be aligned to 1 quad word (16 bytes)
///
/// /// # Parameters
///
/// - `psm`: Pixel format for buffer
/// - `sx`: Source X
/// - `sy`: Source Y
/// - `width`: Image width
/// - `height`: Image height
/// - `srcw`: Source buffer width (block aligned)
/// - `src`: Source pointer
/// - `dx`: Destination X
/// - `dy`: Destination Y
/// - `destw`: Destination buffer width (block aligned)
/// - `dest`: Destination pointer
pub unsafe fn sce_gu_copy_image(
    psm: PixelFormat,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    srcw: i32,
    src: *mut c_void,
    dx: i32,
    dy: i32,
    destw: i32,
    dest: *mut c_void,
) {
    send_command_i(178, ((src as u32) & 0xffffff) as i32);
    send_command_i(179, ((((src as u32) & 0xff000000) >> 8) | (srcw as u32)) as i32);
    send_command_i(235, (((sy as u32) << 10) | ((sx as u32))) as i32);
    send_command_i(180, ((dest as u32) & 0xffffff) as i32);
    send_command_i(181, ((((dest as u32) & 0xff000000) >> 8) | (destw as u32)) as i32);
    send_command_i(236, (((dy as u32) << 10) | ((dx as u32))) as i32);
    send_command_i(238, ((((height as u32)-1) << 10) | ((width as u32) -1)) as i32);
    send_command_i(234, (!(psm as u32 ^ 0x03)) as i32);
}

/// Specify the texture environment color
///
/// This is used in the texture function when a constant color is needed.
///
/// See sceGuTexFunc() for more information.
///
/// # Parameters
///
/// - `color`: Constant color (0x00BBGGRR)
pub unsafe fn sce_gu_tex_env_color(color: u32) {
    send_command_i(202, (color & 0xffffff) as i32);
}

/// Set how the texture is filtered
///
/// Available filters are:
///   - GU_NEAREST
///   - GU_LINEAR
///   - GU_NEAREST_MIPMAP_NEAREST
///   - GU_LINEAR_MIPMAP_NEAREST
///   - GU_NEAREST_MIPMAP_LINEAR
///   - GU_LINEAR_MIPMAP_LINEAR
///
/// # Parameters
///
/// - `min`: Minimizing filter
/// - `mag`: Magnifying filter
pub unsafe fn sce_gu_tex_filter(min: TextureFilter, mag: TextureFilter) {
    send_command_i(198, (((mag as u32) << 8) | (min as u32)) as i32);
}

/// Flush texture page-cache
///
/// Do this if you have copied/rendered into an area currently in the texture-cache
pub unsafe fn sce_gu_tex_flush() {
    send_command_f(203, 0.0);
}

/// Set how textures are applied
///
/// Key for the apply-modes:
///   - Cv - Color value result
///   - Ct - Texture color
///   - Cf - Fragment color
///   - Cc - Constant color (specified by sceGuTexEnvColor())
///
/// Available apply-modes are: (TFX)
///   - GU_TFX_MODULATE - Cv=Ct*Cf TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
///   - GU_TFX_DECAL - TCC_RGB: Cv=Ct,Av=Af TCC_RGBA: Cv=Cf*(1-At)+Ct*At Av=Af
///   - GU_TFX_BLEND - Cv=(Cf*(1-Ct))+(Cc*Ct) TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
///   - GU_TFX_REPLACE - Cv=Ct TCC_RGB: Av=Af TCC_RGBA: Av=At
///   - GU_TFX_ADD - Cv=Cf+Ct TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
///
/// The fields TCC_RGB and TCC_RGBA specify components that differ between
/// the two different component modes.
///
///   - GU_TFX_MODULATE - The texture is multiplied with the current diffuse fragment
///   - GU_TFX_REPLACE - The texture replaces the fragment
///   - GU_TFX_ADD - The texture is added on-top of the diffuse fragment
///
/// Available component-modes are: (TCC)
///   - GU_TCC_RGB - The texture alpha does not have any effect
///   - GU_TCC_RGBA - The texture alpha is taken into account
///
/// # Parameters
///
/// - `tfx`: Which apply-mode to use
/// - `tcc`: Which component-mode to use
pub unsafe fn sce_gu_tex_func(tfx: TextureEffect, tcc: TextureColorComponent) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    context.texture_function = (((tcc as u32) << 8) | (tfx as u32)) as i32;
    send_command_i(201, (
        (
            (tcc as u32) << 8) |
            (tfx as u32) |
            context.fragment_2x as u32
        ) as i32
    );
}

/// Set current texturemap
///
/// Textures may reside in main RAM, but it has a huge speed-penalty. Swizzle textures
/// to get maximum speed.
///
/// # Note
///
/// Data must be aligned to 1 quad word (16 bytes)
///
/// # Parameters
///
/// - `mipmap`: Mipmap level
/// - `width`: Width of texture (must be a power of 2)
/// - `height`: Height of texture (must be a power of 2)
/// - `tbw`: Texture Buffer Width (block-aligned)
/// - `tbp`: Texture buffer pointer (16 byte aligned)
pub unsafe fn sce_gu_tex_image(mipmap: i32, width: i32, height: i32, tbw: i32, tbp: *const c_void) {
    use core::intrinsics::ctlz;

    static tbpcmd_tbl: [i32; 8] = [0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7];
    static tbwcmd_tbl: [i32; 8] = [0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf];
    static tsizecmd_tbl: [i32; 8] = [0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf];

    send_command_i(tbpcmd_tbl[mipmap as usize], ((tbp as u32) & 0xffffff) as i32);
    send_command_i(
        tbwcmd_tbl[mipmap as usize],
        ((((tbp as u32) >> 8) & 0x0f0000) | 
        tbw as u32) as i32
    );
    send_command_i(
        tsizecmd_tbl[mipmap as usize],
        (31 - ctlz(height & 0x3ff) << 8) |
        (31 - ctlz(width & 0x3ff))
    );
    sce_gu_tex_flush();
}

/// Set texture-level mode (mipmapping)
///
/// # Parameters
///
/// - `mode`: Which mode to use, one of TextureLevelMode
/// - `bias`: Which mipmap bias to use
pub unsafe fn sce_gu_tex_level_mode(mode: TextureLevelMode, bias: f32) {
    let offset = core::intrinsics::truncf32(bias * 16.0) as i32;

    if offset >= 128 {
        offset = 128
    } else if offset < -128 {
        offset = -128;
    }
    send_command_i(200, (((offset as u32) << 16) | mode as u32) as i32);
}

/// Set the texture-mapping mode
///
/// # Parameters
///
/// - `mode`: Which mode to use
/// - `a1`: Unknown
/// - `a2`: Unknown
pub unsafe fn sce_gu_tex_map_mode(mode: TextureMapMode, a1: u32, a2: u32) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    context.texture_map_mode = ((mode as u32) & 0x03) as i32;
    send_command_i(
        192,
        (context.texture_proj_map_mode as u32 | ((mode as u32) & 0x03)) as i32
    );
    send_command_i(193, ((a2 << 8) | (a1 & 0x03)) as i32);
}

/// Set texture-mode parameters
///
/// Available texture-formats are:
///   - GU_PSM_5650 - Hicolor, 16-bit
///   - GU_PSM_5551 - Hicolor, 16-bit
///   - GU_PSM_4444 - Hicolor, 16-bit
///   - GU_PSM_8888 - Truecolor, 32-bit
///   - GU_PSM_T4 - Indexed, 4-bit (2 pixels per byte)
///   - GU_PSM_T8 - Indexed, 8-bit
///
/// # Parameters
///
/// - `tpsm`: Which texture format to use
/// - `maxmips`: Number of mipmaps to use (0-8)
/// - `a2`: Unknown, set to 0
/// - `swizzle`: GU_TRUE(1) to swizzle texture-reads
pub unsafe fn sce_gu_tex_mode(tpsm: PixelFormat, maxmips: i32, a2: i32, swizzle: bool) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    context.texture_mode = tpsm as i32;

    send_command_i(194, (maxmips << 16) | (a2 << 8) | (if swizzle { 1 } else { 0 }));
    send_command_i(195, tpsm as i32);
    sce_gu_tex_flush();
}

/// Set texture offset
///
/// @note Only used by the 3D T&L pipe, renders done with GU_TRANSFORM_2D are
/// not affected by this.
///
/// # Parameters
///
/// - `u`: Offset to add to the U coordinate
/// - `v`: Offset to add to the V coordinate
pub unsafe fn sce_gu_tex_offset(u: f32, v: f32) {
    send_command_f(74, u);
    send_command_f(75, v);
}

/// Set texture projection-map mode
///
/// # Parameters
///
/// - `mode`: Which mode to use
pub unsafe fn sce_gu_tex_proj_map_mode(mode: TextureProjectionMapMode) {
    let context = &mut gu_contexts[gu_curr_context as usize];
    context.texture_proj_map_mode = (((mode as u32) & 0x03) << 8) as i32;
    send_command_i(
        192,
        ((((mode as u32) & 0x03) << 8) |
        context.texture_map_mode as u32) as i32
    );
}

/// Set texture scale
///
/// @note Only used by the 3D T&L pipe, renders ton with GU_TRANSFORM_2D are
/// not affected by this.
///
/// # Parameters
///
/// - `u`: Scalar to multiply U coordinate with
/// - `v`: Scalar to multiply V coordinate with
pub unsafe fn sce_gu_tex_scale(u: f32, v: f32) {
    send_command_f(72, u);
    send_command_f(73, v);
}

pub unsafe fn sce_gu_tex_slope(slope: f32) {
    send_command_f(208, slope);
}

/// Synchronize rendering pipeline with image upload.
///
/// This will stall the rendering pipeline until the current image upload initiated by
/// sceGuCopyImage() has completed.
pub unsafe fn sce_gu_tex_sync() {
    send_command_i(204, 0);
}

/// Set if the texture should repeat or clamp
///
/// Available modes are:
///   - GU_REPEAT - The texture repeats after crossing the border
///   - GU_CLAMP - Texture clamps at the border
///
/// # Parameters
///
/// - `u`: Wrap-mode for the U direction
/// - `v`: Wrap-mode for the V direction
pub unsafe fn sce_gu_tex_wrap(u: WrapMode, v: WrapMode) {
    send_command_i(199, (((v as u32) << 8) | (u as u32)) as i32);
}

/// Upload CLUT (Color Lookup Table)
///
/// @note Data must be aligned to 1 quad word (16 bytes)
///
/// # Parameters
///
/// - `num_blocks`: How many blocks of 8 entries to upload (32*8 is 256 colors)
/// - `cbp`: Pointer to palette (16 byte aligned)
pub unsafe fn sce_gu_clut_load(num_blocks: i32, cbp: *const c_void) {
    send_command_i(176, ((cbp as u32) & 0xffffff) as i32);
    send_command_i(177, (((cbp as u32) >> 8) & 0xf0000) as i32);
    send_command_i(196, num_blocks);
}

/// Set current CLUT mode
///
/// Available pixel formats for palettes are:
///   - GU_PSM_5650
///   - GU_PSM_5551
///   - GU_PSM_4444
///   - GU_PSM_8888
///
/// # Parameters
///
/// - `cpsm`: Which pixel format to use for the palette
/// - `shift`: Shifts color index by that many bits to the right
/// - `mask`: Masks the color index with this bitmask after the shift (0-0xFF)
/// - `a3`: Unknown, set to 0
pub unsafe fn sce_gu_clut_mode(cpsm: PixelFormat, shift: u32, mask: u32, a3: u32) {
    let arg = ((cpsm as u32) | (shift << 2) | (mask << 8) | (a3 << 16)) as i32;
    send_command_i(197, arg);
}

/// Set virtual coordinate offset
///
/// The PSP has a virtual coordinate-space of 4096x4096, this controls where rendering is performed
///
/// @par Example: Center the virtual coordinate range
/// @code
/// sceGuOffset(2048-(480/2),2048-(480/2)) {
/// @endcode
///
/// # Parameters
///
/// - `x`: Offset (0-4095)
/// - `y`: Offset (0-4095)
pub unsafe fn sce_gu_offset(x: u32, y: u32) {
    send_command_i(76, (x << 4) as i32);
    send_command_i(77, (y << 4) as i32);
}

/// Set what to scissor within the current viewport
///
/// Note that scissoring is only performed if the custom scissoring is enabled (GU_SCISSOR_TEST)
///
/// # Parameters
///
/// - `x`: Left of scissor region
/// - `y`: Top of scissor region
/// - `w`: Width of scissor region
/// - `h`: Height of scissor region
pub unsafe fn sce_gu_scissor(x: i32, y: i32, w: i32, h: i32) {
    let context = &mut gu_contexts[gu_curr_context as usize];

    context.scissor_start[0] = x;
    context.scissor_start[1] = y;
    context.scissor_end[0] = w-1;
    context.scissor_end[1] = h-1;

    if context.scissor_enable != 0 {
        send_command_i(
            212,
            (context.scissor_start[1] << 10) |
            context.scissor_start[0]
        );
        send_command_i(
            213,
            (context.scissor_end[1] << 10) |
            context.scissor_end[0]
        );
    }
}

/// Set current viewport
///
/// @par Example: Setup a viewport of size (480,272) with origo at (2048,2048)
/// @code
/// sceGuViewport(2048,2048,480,272) {
/// @endcode
///
/// # Parameters
///
/// - `cx`: Center for horizontal viewport
/// - `cy`: Center for vertical viewport
/// - `width`: Width of viewport
/// - `height`: Height of viewport
pub unsafe fn sce_gu_viewport(cx: i32, cy: i32, width: i32, height: i32) {
    send_command_f(66, (width >> 1) as f32);
    send_command_f(67, (-height >> 1) as f32);
    send_command_f(69, cx as f32);
    send_command_f(70, cy as f32);
}

/// Draw bezier surface
///
/// # Parameters
///
/// - `vtype`: Vertex type, look at sceGuDrawArray() for vertex definition
/// - `ucount`: Number of vertices used in the U direction
/// - `vcount`: Number of vertices used in the V direction
/// - `indices`: Pointer to index buffer
/// - `vertices`: Pointer to vertex buffer
pub unsafe fn sce_gu_draw_bezier(
    vtype: i32,
    ucount: i32,
    vcount: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if vtype != 0 {
        send_command_i(18, vtype);
    }
    
    if !indices.is_null() {
        send_command_i(16, (((indices as u32) >> 8) & 0xf0000) as i32);
        send_command_i(2, ((indices as u32) & 0xffffff) as i32);
    }

    if !vertices.is_null() {
        send_command_i(16, (((vertices as u32) >> 8) & 0xf0000) as i32);
        send_command_i(1, ((vertices as u32) & 0xffffff) as i32);
    }

    send_command_i(5, (((vcount as u32) << 8) | (ucount as u32)) as i32);
}

/// Set dividing for patches (beziers and splines)
///
/// # Parameters
///
/// - `ulevel`: Number of division on u direction
/// - `vlevel`: Number of division on v direction
pub unsafe fn sce_gu_patch_divide(ulevel: u32, vlevel: u32) {
    send_command_i(54, ((vlevel << 8) | ulevel) as i32);
}


pub unsafe fn sce_gu_patch_front_face(a0: u32) {
    send_command_i(56, a0 as i32);
}

/// Set primitive for patches (beziers and splines)
///
/// # Parameters
///
/// - `prim`: Desired primitive type (GU_POINTS | GU_LINE_STRIP | GU_TRIANGLE_STRIP)
pub unsafe fn sce_gu_patch_prim(prim: Primitive) {
    match prim {
        Primitive::Points => send_command_i(55, 2),
        Primitive::LineStrip => send_command_i(55,1),
        Primitive::TriangleStrip => send_command_i(55, 0),
        _ => ()
    }
}

pub unsafe fn sce_gu_draw_spline(
    vtype: i32,
    ucount: i32,
    vcount: i32,
    uedge: i32,
    vedge: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if vtype != 0 {
        send_command_i(18, vtype);
    }
    
    if !indices.is_null() {
        send_command_i(16, (((indices as u32) >> 8) & 0xf0000) as i32);
        send_command_i(2, ((indices as u32) & 0xffffff) as i32);
    }

    if !vertices.is_null() {
        send_command_i(16, (((vertices as u32) >> 8) & 0xf0000) as i32);
        send_command_i(1, ((vertices as u32) & 0xffffff) as i32);
    }

    send_command_i(
        6,
        (
            ((vedge as u32) << 18) |
            ((uedge as u32) << 16) |
            ((vcount as u32) << 8) |
            (ucount as u32)
        ) as i32
    );

}

/// Set transform matrices
///
/// # Parameters
///
/// - `type`: Which matrix-type to set
/// - `matrix`: Matrix to load
pub unsafe fn sce_gu_set_matrix(type_: MatrixMode, matrix: &FMatrix4) {
    let fmatrix = matrix as *const _ as *const f32;

    match type_ {
        MatrixMode::Projection => {
            send_command_f(62, 0.0);
            for i in 0..16 {
                send_command_f(63, *fmatrix.offset(i));
            }
        },
        MatrixMode::View => {
            send_command_f(60, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(61, *fmatrix.offset(j+i*4));
                }
            }
        },
        MatrixMode::Model => {
            send_command_f(58, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(59, *fmatrix.offset(j+i*4));
                }
            }
        },
        MatrixMode::Texture => {
            send_command_f(64, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(65, *fmatrix.offset(j+i*4));
                }
            }
        }
    }
}

/// Specify skinning matrix entry
///
/// To enable vertex skinning, pass GU_WEIGHTS(n), where n is between
/// 1-8, and pass available GU_WEIGHT_??? declaration. This will change
/// the amount of weights passed in the vertex araay, and by setting the skinning,
/// matrices, you will multiply each vertex every weight and vertex passed.
///
/// Please see sceGuDrawArray() for vertex format information.
///
/// # Parameters
///
/// - `index`: Skinning matrix index (0-7)
/// - `matrix`: Matrix to set
pub unsafe fn sce_gu_bone_matrix(index: u32, matrix: *const FMatrix4) {
    let offset = ((index << 1) + index) << 2;
    let fmatrix = matrix as *const _ as *const f32;

    send_command_i(42, offset as i32);
    for i in 0..4 {
        for j in 0..3 {
            send_command_f(43, *fmatrix.offset(j+(i << 2)));
        }
    }
}

/// Specify morph weight entry
///
/// To enable vertex morphing, pass GU_VERTICES(n), where n is between
/// 1-8. This will change the amount of vertices passed in the vertex array,
/// and by setting the morph weights for every vertex entry in the array,
/// you can blend between them.
///
/// Please see sceGuDrawArray() for vertex format information.
///
/// # Parameters
///
/// - `index`: Morph weight index (0-7)
/// - `weight`: Weight to set
pub unsafe fn sce_gu_morph_weight(index: i32, weight: f32) {
    send_command_f(44 + index, weight);
}

pub unsafe fn sce_gu_draw_array_n(
    primitive_type: i32,
    vtype: i32,
    count: i32,
    a3: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if vtype != 0 {
        send_command_i(18, vtype);
    }
    
    if !indices.is_null() {
        send_command_i(16, (((indices as u32) >> 8) & 0xf0000) as i32);
        send_command_i(2, ((indices as u32) & 0xffffff) as i32);
    }

    if !vertices.is_null() {
        send_command_i(16, (((vertices as u32) >> 8) & 0xf0000) as i32);
        send_command_i(1, ((vertices as u32) & 0xffffff) as i32);
    }
    
    if a3 > 0 {
        for i in (0..(a3-1)).rev() {
            send_command_i(4, (primitive_type << 16) | count);
        }
        send_command_i_stall(4, (primitive_type << 16) | count);
    }
}
