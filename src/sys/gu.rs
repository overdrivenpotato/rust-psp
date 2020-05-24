use core::ffi::c_void;
use core::ptr::null_mut;
pub const PI: f32 = 3.141593;

#[repr(u32)]
pub enum Bool {
    False = 0,
    True = 1,
}

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

/// Clear Buffer Mask
#[repr(u32)]
pub enum ClearBuffer {
    ColorBufferBit = 1,
    StencilBufferBit = 2,
    DepthBufferBit = 4,
    FastClearBit = 16,
}

/// Texture Effect
#[repr(u32)]
pub enum TextureEffect {
    TfxModulate = 0,
    TfxDecal = 1,
    TfxBlend = 2,
    TfxReplace = 3,
    TfxAdd = 4,
}

/// Texture Color Component
#[repr(u32)]
pub enum TextureColorComponent {
    TccRgb = 0,
    TccRgba = 1,
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

/// Light Components
#[repr(u32)]
pub enum LightComponent {
    Ambient = 1,
    Diffuse = 2,
    Specular = 4,
    AmbientAndDiffuse = 3,
    DiffuseAndSpecular = 6,
    UnknownLightComponent = 8,
}

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
pub fn color(r: f32, g: f32, b: f32, a: f32) -> u32 {
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
    swapBuffersBehaviour: u32,
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
    pixel_size: i32,
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
    swapBuffersBehaviour: 0,
    swapBuffersCallback: None,
};

static mut gu_list: *mut GuDisplayList = null_mut();
static mut gu_curr_context: i32 = 0;
static mut gu_init: i32 = 0;
static mut gu_display_on: i32 = 0;
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
    pixel_size: 0,
};

static mut gu_object_stack: *mut *mut u32 = null_mut();
static mut gu_object_stack_depth: i32 = 0;
static mut light_settings: [GuLightSettings; 4] = [
    GuLightSettings {
        enable: 0x18,
        tpe: 0x5f,
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
        tpe: 0x60,
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
        tpe: 0x61,
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
        tpe: 0x62,
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
