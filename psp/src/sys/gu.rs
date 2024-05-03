use crate::sys::{
    self,
    display::DisplayPixelFormat,
    ge::{GeBreakParam, GeCommand, GeContext, GeListArgs, GeListState},
    kernel::SceUid,
    types::{ScePspFMatrix4, ScePspFVector3, ScePspIMatrix4, ScePspIVector4},
};
use core::{ffi::c_void, mem, ptr::addr_of_mut, ptr::null_mut};
use num_enum::TryFromPrimitive;

#[allow(clippy::approx_constant)]
pub const GU_PI: f32 = 3.141593;

/// Primitive types
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum GuPrimitive {
    /// Single pixel points (1 vertex per primitive)
    Points = 0,
    /// Single pixel lines (2 vertices per primitive)
    Lines = 1,
    /// Single pixel line-strip (2 vertices for the first primitive, 1 for every following)
    LineStrip = 2,
    /// Filled triangles (3 vertices per primitive)
    Triangles = 3,
    /// Filled triangles-strip (3 vertices for the first primitive, 1 for every following)
    TriangleStrip = 4,
    /// Filled triangle-fan (3 vertices for the first primitive, 1 for every following)
    TriangleFan = 5,
    /// Filled blocks (2 vertices per primitive)
    Sprites = 6,
}

/// Patch primitive types
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum PatchPrimitive {
    /// Single pixel points (1 vertex per primitive)
    Points = 0,
    /// Single pixel line-strip (2 vertices for the first primitive, 1 for every following)
    LineStrip = 2,
    /// Filled triangles-strip (3 vertices for the first primitive, 1 for every following)
    TriangleStrip = 4,
}

/// States
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum GuState {
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
#[derive(Copy, Clone, Debug)]
pub enum MatrixMode {
    Projection = 0,
    View = 1,
    Model = 2,
    Texture = 3,
}

bitflags::bitflags! {
    /// The vertex type decides how the vertices align and what kind of
    /// information they contain.
    #[repr(transparent)]
    pub struct VertexType: i32 {
        /// 8-bit texture coordinates
        const TEXTURE_8BIT = 1;
        /// 16-bit texture coordinates
        const TEXTURE_16BIT = 2;
        /// 32-bit texture coordinates (float)
        const TEXTURE_32BITF = 3;

        /// 16-bit color (R5G6B5A0)
        const COLOR_5650 = 4 << 2;
        /// 16-bit color (R5G5B5A1)
        const COLOR_5551 = 5 << 2;
        /// 16-bit color (R4G4B4A4)
        const COLOR_4444 = 6 << 2;
        /// 32-bit color (R8G8B8A8)
        const COLOR_8888 = 7 << 2;

        /// 8-bit normals
        const NORMAL_8BIT = 1 << 5;
        /// 16-bit normals
        const NORMAL_16BIT = 2 << 5;
        /// 32-bit normals (float)
        const NORMAL_32BITF = 3 << 5;

        /// 8-bit vertex position
        const VERTEX_8BIT = 1 << 7;
        /// 16-bit vertex position
        const VERTEX_16BIT = 2 << 7;
        /// 32-bit vertex position (float)
        const VERTEX_32BITF = 3 << 7;

        /// 8-bit weights
        const WEIGHT_8BIT = 1 << 9;
        /// 16-bit weights
        const WEIGHT_16BIT = 2 << 9;
        /// 32-bit weights (float)
        const WEIGHT_32BITF = 3 << 9;

        /// 8-bit vertex index
        const INDEX_8BIT = 1 << 11;
        /// 16-bit vertex index
        const INDEX_16BIT = 2 << 11;

        // FIXME: Need to document this.
        // Number of weights (1-8)
        const WEIGHTS1 = Self::num_weights(1);
        const WEIGHTS2 = Self::num_weights(2);
        const WEIGHTS3 = Self::num_weights(3);
        const WEIGHTS4 = Self::num_weights(4);
        const WEIGHTS5 = Self::num_weights(5);
        const WEIGHTS6 = Self::num_weights(6);
        const WEIGHTS7 = Self::num_weights(7);
        const WEIGHTS8 = Self::num_weights(8);

        // Number of vertices (1-8)
        const VERTICES1 = Self::num_vertices(1);
        const VERTICES2 = Self::num_vertices(2);
        const VERTICES3 = Self::num_vertices(3);
        const VERTICES4 = Self::num_vertices(4);
        const VERTICES5 = Self::num_vertices(5);
        const VERTICES6 = Self::num_vertices(6);
        const VERTICES7 = Self::num_vertices(7);
        const VERTICES8 = Self::num_vertices(8);

        /// Coordinate is passed directly to the rasterizer
        const TRANSFORM_2D = 1 << 23;
        /// Coordinate is transformed before being passed to rasterizer
        const TRANSFORM_3D = 0;
    }
}

impl VertexType {
    const fn num_weights(n: u32) -> i32 {
        (((n - 1) & 7) << 14) as i32
    }

    const fn num_vertices(n: u32) -> i32 {
        (((n - 1) & 7) << 18) as i32
    }
}

/// Texture pixel formats
// TODO: Better documentation
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TexturePixelFormat {
    /// Hicolor, 16-bit.
    Psm5650 = 0,
    /// Hicolor, 16-bit
    Psm5551 = 1,
    /// Hicolor, 16-bit
    Psm4444 = 2,
    /// Truecolor, 32-bit
    Psm8888 = 3,
    /// Indexed, 4-bit (2 pixels per byte)
    PsmT4 = 4,
    /// Indexed, 8-bit
    PsmT8 = 5,
    /// Indexed, 16-bit
    PsmT16 = 6,
    /// Indexed, 32-bit
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
// TODO: Should this be `ShadeMode` (no L)?
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
    Copy = 3,
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
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureMapMode {
    TextureCoords = 0,
    TextureMatrix = 1,
    EnvironmentMap = 2,
}

/// Texture Level Mode
#[repr(u32)]
pub enum TextureLevelMode {
    Auto = 0,
    Const = 1,
    Slope = 2,
}

/// Texture Projection Map Mode
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureProjectionMapMode {
    Position = 0,
    Uv = 1,
    NormalizedNormal = 2,
    Normal = 3,
}

/// Wrap Mode
#[repr(u32)]
pub enum GuTexWrapMode {
    /// The texture repeats after crossing the border
    Repeat = 0,

    /// Texture clamps at the border
    Clamp = 1,
}

/// Front Face Direction
#[repr(u32)]
pub enum FrontFaceDirection {
    Clockwise = 0,
    CounterClockwise = 1,
}

/// Test function for alpha test
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum AlphaFunc {
    Never = 0,
    Always,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

/// Test function for stencil test
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum StencilFunc {
    Never = 0,
    Always,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

/// Test function for color test
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ColorFunc {
    Never = 0,
    Always,
    Equal,
    NotEqual,
}

/// Test function for depth test
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum DepthFunc {
    /// No pixels pass the depth-test
    Never = 0,
    /// All pixels pass the depth-test
    Always,
    /// Pixels that match the depth-test pass
    Equal,
    /// Pixels that doesn't match the depth-test pass
    NotEqual,
    /// Pixels that are less in depth passes
    Less,
    /// Pixels that are less or equal in depth passes
    LessOrEqual,
    /// Pixels that are greater in depth passes
    Greater,
    /// Pixels that are greater or equal passes
    GreaterOrEqual,
}

bitflags::bitflags! {
    /// Clear Buffer Mask
    #[repr(transparent)]
    pub struct ClearBuffer: u32 {
        /// Clears the color buffer.
        const COLOR_BUFFER_BIT = 1;
        /// Clears the stencil buffer.
        const STENCIL_BUFFER_BIT = 2;
        /// Clears the depth buffer.
        const DEPTH_BUFFER_BIT = 4;
        /// Enables fast clearing. This divides the screen into 16 parts
        /// and clears them in parallel.
        const FAST_CLEAR_BIT = 16;
    }
}

/// Texture effect apply-modes.
///
/// Key for the apply-modes:
/// - `Cv`: Color value result
/// - `Ct`: Texture color
/// - `Cf`: Fragment color
/// - `Cc`: Constant color (specified by `sceGuTexEnvColor`)
///
/// The fields TCC_RGB and TCC_RGBA specify components that differ between
/// the two different component modes.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureEffect {
    // TODO: Better documentation
    /// The texture is multiplied with the current diffuse fragment.
    ///
    /// `Cv=Ct*Cf TCC_RGB: Av=Af TCC_RGBA: Av=At*Af`
    Modulate = 0,
    /// `TCC_RGB: Cv=Ct,Av=Af TCC_RGBA: Cv=Cf*(1-At)+Ct*At Av=Af`
    Decal = 1,
    /// `Cv=(Cf*(1-Ct))+(Cc*Ct) TCC_RGB: Av=Af TCC_RGBA: Av=At*Af`
    Blend = 2,
    /// The texture replaces the fragment
    ///
    /// `Cv=Ct TCC_RGB: Av=Af TCC_RGBA: Av=At`
    Replace = 3,
    /// The texture is added on-top of the diffuse fragment
    ///
    /// `Cv=Cf+Ct TCC_RGB: Av=Af TCC_RGBA: Av=At*Af`
    Add = 4,
}

/// Texture color component-modes.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureColorComponent {
    /// The texture alpha does not have any effect.
    Rgb = 0,
    /// The texture alpha is taken into account.
    Rgba = 1,
}

/// Mipmap Level
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum MipmapLevel {
    None = 0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}

/// Blending Operation
///
/// Keys for the blending operations:
///
/// - `Cs`: Source color
/// - `Cd`: Destination color
/// - `Bs`: Blend function for source fragment
/// - `Bd`: Blend function for destination fragment
#[repr(u32)]
pub enum BlendOp {
    /// `(Cs*Bs) + (Cd*Bd)`
    Add = 0,
    /// `(Cs*Bs) - (Cd*Bd)`
    Subtract = 1,
    /// `(Cd*Bd) - (Cs*Bs)`
    ReverseSubtract = 2,
    /// `Cs < Cd ? Cs : Cd`
    Min = 3,
    /// `Cs < Cd ? Cd : Cs`
    Max = 4,
    /// `|Cs-Cd|`
    Abs = 5,
}

/// Blending factor
#[repr(u32)]
pub enum BlendFactor {
    Color = 0,
    OneMinusColor = 1,
    SrcAlpha = 2,
    OneMinusSrcAlpha = 3,
    DstAlpha = 4,
    OneMinusDstAlpha = 5,
    // TODO: There are likely 4 missing values here.
    // What are 6, 7, 8, 9? This can probably be determined with some experimentation.
    // They may also be reserved values.
    /// Use the fixed values provided in `sceGuBlendFunc`.
    Fix = 10,
}

/// Stencil Operations
#[repr(u32)]
pub enum StencilOperation {
    /// Keeps the current value
    Keep = 0,
    /// Sets the stencil buffer value to zero
    Zero = 1,
    /// Sets the stencil buffer value to ref, as specified by `sceGuStencilFunc`
    Replace = 2,
    /// Increments the current stencil buffer value
    Invert = 3,
    /// Decrease the current stencil buffer value
    Incr = 4,
    /// Bitwise invert the current stencil buffer value
    Decr = 5,
}

bitflags::bitflags!(
    /// Light Components
    #[repr(transparent)]
    pub struct LightComponent: i32 {
        const AMBIENT = 1;
        const DIFFUSE = 2;
        const SPECULAR = 4;

        // TODO: What is this?
        const UNKNOWN_LIGHT_COMPONENT = 8;
    }
);

/// Light modes
#[repr(u32)]
pub enum LightMode {
    SingleColor = 0,

    /// Separate specular colors are used to interpolate the specular component
    /// independently, so that it can be added to the fragment after the texture
    /// color.
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
#[derive(Copy, Clone, Debug)]
pub enum GuContextType {
    Direct = 0,
    Call = 1,
    Send = 2,
}

/// List Queue Mode
#[repr(u32)]
pub enum GuQueueMode {
    /// Place list last in the queue, so it executes in-order
    Tail = 0,
    /// Place list first in queue so that it executes as soon as possible
    Head = 1,
}

/// Sync mode
#[repr(u32)]
pub enum GuSyncMode {
    /// Wait until the last sceGuFinish command is reached.
    Finish = 0,
    /// Wait until the last (?) signal is executed.
    Signal = 1,
    /// Wait until all commands currently in list are executed.
    Done = 2,
    /// Wait for the currently executed display list (`GuContextType::Direct`).
    List = 3,
    /// Wait for the last send list.
    Send = 4,
}

/// Sync Behavior
#[repr(u32)]
pub enum GuSyncBehavior {
    /// Wait for the GE list to be completed.
    Wait = 0,
    /// Just peek at the current state.
    NoWait = 1,
}

/// GU Callback ID
#[repr(u32)]
pub enum GuCallbackId {
    /// Called when `sceGuSignal` is used.
    Signal = 1,

    /// Called when display list is finished.
    Finish = 4,
}

/// Signal behavior
#[repr(u32)]
pub enum SignalBehavior {
    /// Stops display list execution until callback function finished.
    Suspend = 1,
    /// Do not stop display list execution during callback.
    Continue = 2,
}

/// Map 8-bit color channels into one 32-bit value.
#[inline]
pub const fn abgr(a: u8, b: u8, g: u8, r: u8) -> u32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24)
}

/// Map 8-bit color channels into one 32-bit value.
#[inline]
pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
    abgr(a, b, g, r)
}

/// Map 8-bit color channels into one 32-bit value.
#[inline]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    argb(a, r, g, b)
}

#[inline]
/// Map floating point channels (0..1) into one 32-bit value
pub fn color(r: f32, g: f32, b: f32, a: f32) -> u32 {
    rgba(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    )
}

pub type GuCallback = Option<extern "C" fn(id: i32, arg: *mut c_void)>;
pub type GuSwapBuffersCallback =
    Option<extern "C" fn(display: *mut *mut c_void, render: *mut *mut c_void)>;

struct Settings {
    sig: GuCallback,
    fin: GuCallback,
    signal_history: [i16; 16],
    signal_offset: u32,
    kernel_event_flag: SceUid,
    ge_callback_id: i32,
    swap_buffers_callback: GuSwapBuffersCallback,
    swap_buffers_behaviour: crate::sys::DisplaySetBufSync,
}

struct GuDisplayList {
    start: *mut u32,
    current: *mut u32,
    parent_context: GuContextType,
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
    texture_proj_map_mode: TextureProjectionMapMode,
    texture_map_mode: TextureMapMode,
    sprite_mode: [i32; 4],
    clear_color: u32,
    clear_stencil: u32,
    clear_depth: u32,
    texture_mode: TexturePixelFormat,
}

struct GuDrawBuffer {
    pixel_size: DisplayPixelFormat,
    frame_width: i32,
    frame_buffer: *mut c_void,
    disp_buffer: *mut c_void,
    depth_buffer: *mut c_void,
    depth_width: i32,
    width: i32,
    height: i32,
}

struct GuLightSettings {
    /// Light type
    type_: GeCommand,
    /// X position
    xpos: GeCommand,
    /// Y position
    ypos: GeCommand,
    /// Z position
    zpos: GeCommand,
    /// X direction
    xdir: GeCommand,
    /// Y direction
    ydir: GeCommand,
    /// Z direction
    zdir: GeCommand,

    /// Ambient color
    ambient: GeCommand,
    /// Diffuse color
    diffuse: GeCommand,
    /// Specular color
    specular: GeCommand,
    /// Constant attenuation
    constant: GeCommand,
    /// Linear attenuation
    linear: GeCommand,
    /// Quadratic attenuation
    quadratic: GeCommand,
    /// Light exponent
    exponent: GeCommand,
    /// Light cutoff
    cutoff: GeCommand,
}

static mut CURRENT_FRAME: u32 = 0;
static mut CONTEXTS: [GuContext; 3] = [
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: GuContextType::Direct,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: TextureProjectionMapMode::Position,
        texture_map_mode: TextureMapMode::TextureCoords,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: TexturePixelFormat::Psm5650,
    },
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: GuContextType::Direct,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: TextureProjectionMapMode::Position,
        texture_map_mode: TextureMapMode::TextureCoords,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: TexturePixelFormat::Psm5650,
    },
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: GuContextType::Direct,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: TextureProjectionMapMode::Position,
        texture_map_mode: TextureMapMode::TextureCoords,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: TexturePixelFormat::Psm5650,
    },
];

static mut GE_LIST_EXECUTED: [i32; 2] = [0, 0];
static mut GE_EDRAM_ADDRESS: *mut c_void = null_mut();

static mut SETTINGS: Settings = Settings {
    sig: None,
    fin: None,
    signal_history: [0; 16],
    signal_offset: 0,

    // Invalid UID until initialized.
    kernel_event_flag: SceUid(-1),

    ge_callback_id: 0,
    swap_buffers_behaviour: crate::sys::DisplaySetBufSync::Immediate,
    swap_buffers_callback: None,
};

static mut LIST: *mut GuDisplayList = null_mut();
static mut CURR_CONTEXT: GuContextType = GuContextType::Direct;
static mut INIT: i32 = 0;
static mut DISPLAY_ON: bool = false;
static mut CALL_MODE: i32 = 0;
static mut STATES: u32 = 0;

static mut DRAW_BUFFER: GuDrawBuffer = GuDrawBuffer {
    depth_buffer: null_mut(),
    frame_buffer: null_mut(),
    disp_buffer: null_mut(),
    width: 0,
    height: 0,
    depth_width: 0,
    frame_width: 0,
    pixel_size: DisplayPixelFormat::Psm5650,
};

static mut OBJECT_STACK: *mut *mut u32 = null_mut();
static mut OBJECT_STACK_DEPTH: i32 = 0;

const LIGHT_COMMANDS: [GuLightSettings; 4] = [
    GuLightSettings {
        type_: GeCommand::LightType0,
        xpos: GeCommand::Light0X,
        ypos: GeCommand::Light0Y,
        zpos: GeCommand::Light0Z,
        xdir: GeCommand::Light0DirectionX,
        ydir: GeCommand::Light0DirectionY,
        zdir: GeCommand::Light0DirectionZ,
        ambient: GeCommand::Light0Ambient,
        diffuse: GeCommand::Light0Diffuse,
        specular: GeCommand::Light0Specular,
        constant: GeCommand::Light0ConstantAtten,
        linear: GeCommand::Light0LinearAtten,
        quadratic: GeCommand::Light0QuadtraticAtten,
        exponent: GeCommand::Light0ExponentAtten,
        cutoff: GeCommand::Light0CutoffAtten,
    },
    GuLightSettings {
        type_: GeCommand::LightType1,
        xpos: GeCommand::Light1X,
        ypos: GeCommand::Light1Y,
        zpos: GeCommand::Light1Z,
        xdir: GeCommand::Light1DirectionX,
        ydir: GeCommand::Light1DirectionY,
        zdir: GeCommand::Light1DirectionZ,
        ambient: GeCommand::Light1Ambient,
        diffuse: GeCommand::Light1Diffuse,
        specular: GeCommand::Light1Specular,
        constant: GeCommand::Light1ConstantAtten,
        linear: GeCommand::Light1LinearAtten,
        quadratic: GeCommand::Light1QuadtraticAtten,
        exponent: GeCommand::Light1ExponentAtten,
        cutoff: GeCommand::Light1CutoffAtten,
    },
    GuLightSettings {
        type_: GeCommand::LightType2,
        xpos: GeCommand::Light2X,
        ypos: GeCommand::Light2Y,
        zpos: GeCommand::Light2Z,
        xdir: GeCommand::Light2DirectionX,
        ydir: GeCommand::Light2DirectionY,
        zdir: GeCommand::Light2DirectionZ,
        ambient: GeCommand::Light2Ambient,
        diffuse: GeCommand::Light2Diffuse,
        specular: GeCommand::Light2Specular,
        constant: GeCommand::Light2ConstantAtten,
        linear: GeCommand::Light2LinearAtten,
        quadratic: GeCommand::Light2QuadtraticAtten,
        exponent: GeCommand::Light2ExponentAtten,
        cutoff: GeCommand::Light2CutoffAtten,
    },
    GuLightSettings {
        type_: GeCommand::LightType3,
        xpos: GeCommand::Light3X,
        ypos: GeCommand::Light3Y,
        zpos: GeCommand::Light3Z,
        xdir: GeCommand::Light3DirectionX,
        ydir: GeCommand::Light3DirectionY,
        zdir: GeCommand::Light3DirectionZ,
        ambient: GeCommand::Light3Ambient,
        diffuse: GeCommand::Light3Diffuse,
        specular: GeCommand::Light3Specular,
        constant: GeCommand::Light3ConstantAtten,
        linear: GeCommand::Light3LinearAtten,
        quadratic: GeCommand::Light3QuadtraticAtten,
        exponent: GeCommand::Light3ExponentAtten,
        cutoff: GeCommand::Light3CutoffAtten,
    },
];

#[inline]
unsafe fn send_command_i(cmd: GeCommand, argument: i32) {
    (*(*LIST).current) = ((cmd as u32) << 24) | (argument as u32 & 0xffffff);
    (*LIST).current = (*LIST).current.add(1);
}

#[inline]
unsafe fn send_command_f(cmd: GeCommand, argument: f32) {
    send_command_i(cmd, (argument.to_bits() >> 8) as i32);
}

#[inline]
unsafe fn send_command_i_stall(cmd: GeCommand, argument: i32) {
    send_command_i(cmd, argument);
    if let (GuContextType::Direct, 0) = (CURR_CONTEXT, OBJECT_STACK_DEPTH) {
        crate::sys::sceGeListUpdateStallAddr(GE_LIST_EXECUTED[0], (*LIST).current as *mut c_void);
    }
}

unsafe fn draw_region(x: i32, y: i32, width: i32, height: i32) {
    send_command_i(GeCommand::Region1, (y << 10) | x);
    send_command_i(
        GeCommand::Region2,
        (((y + height) - 1) << 10) | ((x + width) - 1),
    );
}

unsafe fn reset_values() {
    INIT = 0;
    STATES = 0;
    CURRENT_FRAME = 0;
    OBJECT_STACK_DEPTH = 0;
    DISPLAY_ON = false;
    CALL_MODE = 0;
    DRAW_BUFFER.pixel_size = DisplayPixelFormat::Psm5551;
    DRAW_BUFFER.frame_width = 0;
    DRAW_BUFFER.frame_buffer = null_mut();
    DRAW_BUFFER.disp_buffer = null_mut();
    DRAW_BUFFER.depth_buffer = null_mut();
    DRAW_BUFFER.depth_width = 0;
    DRAW_BUFFER.width = 480;
    DRAW_BUFFER.height = 272;

    for i in 0..CONTEXTS.len() {
        let context = addr_of_mut!(CONTEXTS[i]);
        (*context).scissor_enable = 0;
        (*context).scissor_start = [0, 0];
        (*context).scissor_end = [0, 0];

        (*context).near_plane = 0;
        (*context).far_plane = 1;

        (*context).depth_offset = 0;
        (*context).fragment_2x = 0;
        (*context).texture_function = 0;
        (*context).texture_proj_map_mode = TextureProjectionMapMode::Position;
        (*context).texture_map_mode = TextureMapMode::TextureCoords;
        (*context).sprite_mode[0] = 0;
        (*context).sprite_mode[1] = 0;
        (*context).sprite_mode[2] = 0;
        (*context).sprite_mode[3] = 0;
        (*context).clear_color = 0;
        (*context).clear_stencil = 0;
        (*context).clear_depth = 0xffff;
        (*context).texture_mode = TexturePixelFormat::Psm5650;
    }

    SETTINGS.sig = None;
    SETTINGS.fin = None;
}

extern "C" fn callback_sig(id: i32, arg: *mut c_void) {
    let settings = arg as *mut Settings;

    unsafe {
        let idx = ((*settings).signal_offset & 15) as usize;
        (*settings).signal_history[idx] = (id & 0xffff) as i16;
        (*settings).signal_offset += 1;

        if (*settings).sig.is_some() {
            // Convert Option<fn(i32, *mut c_void)> -> fn(i32)
            // This is fine because we are transmuting a nullable function
            // pointer to another function pointer. The requirement here is that
            // it must not be null.
            let f = mem::transmute::<_, extern "C" fn(i32)>((*settings).sig);

            f(id & 0xffff);
        }

        crate::sys::sceKernelSetEventFlag((*settings).kernel_event_flag, 1);
    }
}

extern "C" fn callback_fin(id: i32, arg: *mut c_void) {
    unsafe {
        let settings = arg as *mut Settings;

        if let Some(fin) = (*settings).fin {
            // Convert Option<fn(i32, *mut c_void)> -> fn(i32)
            // This is fine because we are transmuting a nullable function
            // pointer to another function pointer. The requirement here is that
            // it must not be null.
            let f = core::mem::transmute::<_, extern "C" fn(i32)>(fin);

            f(id & 0xffff)
        }
    }
}

/// Set depth buffer parameters
///
/// # Parameters
///
/// - `zbp`: VRAM pointer where the depth buffer should start
/// - `zbw`: The width of the depth buffer (block-aligned)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDepthBuffer(zbp: *mut c_void, zbw: i32) {
    DRAW_BUFFER.depth_buffer = zbp;

    if DRAW_BUFFER.depth_width == 0 || DRAW_BUFFER.depth_width != zbw {
        DRAW_BUFFER.depth_width = zbw;
    }

    send_command_i(GeCommand::ZBufPtr, zbp as i32 & 0xffffff);
    send_command_i(
        GeCommand::ZBufWidth,
        (((zbp as u32 & 0xff000000) >> 8) | zbw as u32) as i32,
    );
}

/// Set display buffer parameters
///
/// # Parameters
///
/// - `width`: Width of the display buffer in pixels
/// - `height`: Width of the display buffer in pixels
/// - `dispbp`: VRAM pointer to where the display-buffer starts
/// - `dispbw`: Display buffer width (block aligned)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDispBuffer(
    width: i32,
    height: i32,
    dispbp: *mut c_void,
    dispbw: i32,
) {
    use crate::sys::DisplaySetBufSync;

    DRAW_BUFFER.width = width;
    DRAW_BUFFER.height = height;
    DRAW_BUFFER.disp_buffer = dispbp;

    if DRAW_BUFFER.frame_width == 0 || DRAW_BUFFER.frame_width != dispbw {
        DRAW_BUFFER.frame_width = dispbw;
    }

    draw_region(0, 0, DRAW_BUFFER.width, DRAW_BUFFER.height);

    crate::sys::sceDisplaySetMode(
        crate::sys::DisplayMode::Lcd,
        DRAW_BUFFER.width as usize,
        DRAW_BUFFER.height as usize,
    );

    if DISPLAY_ON {
        crate::sys::sceDisplaySetFrameBuf(
            (GE_EDRAM_ADDRESS as *mut u8).add(DRAW_BUFFER.disp_buffer as usize),
            dispbw as usize,
            DRAW_BUFFER.pixel_size,
            DisplaySetBufSync::NextFrame,
        );
    }
}

/// Set draw buffer parameters (and store in context for buffer-swap)
///
/// # Parameters
///
/// - `psm`: Pixel format to use for rendering (and display)
/// - `fbp`: VRAM pointer to where the draw buffer starts
/// - `fbw`: Frame buffer width (block aligned)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDrawBuffer(psm: DisplayPixelFormat, fbp: *mut c_void, fbw: i32) {
    DRAW_BUFFER.pixel_size = psm;
    DRAW_BUFFER.frame_width = fbw;
    DRAW_BUFFER.frame_buffer = fbp;

    if DRAW_BUFFER.depth_buffer.is_null() && DRAW_BUFFER.height != 0 {
        DRAW_BUFFER.depth_buffer =
            (fbp as u32 + (((DRAW_BUFFER.height * fbw) as u32) << 2u32)) as *mut c_void;
    }

    if DRAW_BUFFER.depth_width == 0 {
        DRAW_BUFFER.depth_width = fbw;
    }

    send_command_i(GeCommand::FramebufPixFormat, psm as i32);
    send_command_i(
        GeCommand::FrameBufPtr,
        DRAW_BUFFER.frame_buffer as i32 & 0xffffff,
    );
    send_command_i(
        GeCommand::FrameBufWidth,
        ((DRAW_BUFFER.frame_buffer as u32 & 0xff000000) >> 8) as i32 | DRAW_BUFFER.frame_width,
    );
    send_command_i(
        GeCommand::ZBufPtr,
        DRAW_BUFFER.depth_buffer as i32 & 0xffffff,
    );
    send_command_i(
        GeCommand::ZBufWidth,
        ((DRAW_BUFFER.depth_buffer as u32 & 0xff000000) >> 8) as i32 | DRAW_BUFFER.depth_width,
    );
}

/// Set draw buffer directly, not storing parameters in the context
///
/// # Parameters
///
/// - `psm`: Pixel format to use for rendering
/// - `fbp`: VRAM pointer to where the draw buffer starts
/// - `fbw`: Frame buffer width (block aligned)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDrawBufferList(psm: DisplayPixelFormat, fbp: *mut c_void, fbw: i32) {
    send_command_i(GeCommand::FramebufPixFormat, psm as i32);
    send_command_i(GeCommand::FrameBufPtr, fbp as i32 & 0xffffff);
    send_command_i(
        GeCommand::FrameBufWidth,
        ((fbp as u32 & 0xff000000) >> 8) as i32 | fbw,
    );
}

/// Turn display on or off
///
/// # Parameters
///
/// - `state`: Turn display on or off
///
/// # Return Value
///
/// State of the display prior to this call
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDisplay(state: bool) -> bool {
    use crate::sys::DisplaySetBufSync;

    if state {
        crate::sys::sceDisplaySetFrameBuf(
            (GE_EDRAM_ADDRESS as *mut u8).add(DRAW_BUFFER.disp_buffer as usize),
            DRAW_BUFFER.frame_width as usize,
            DRAW_BUFFER.pixel_size,
            DisplaySetBufSync::NextFrame,
        );
    } else {
        crate::sys::sceDisplaySetFrameBuf(
            null_mut(),
            0,
            DisplayPixelFormat::Psm5650,
            DisplaySetBufSync::NextFrame,
        );
    }

    DISPLAY_ON = state;
    state
}

/// Select which depth-test function to use
///
/// # Parameters
///
/// - `function`: Depth test function to use
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDepthFunc(function: DepthFunc) {
    send_command_i(GeCommand::ZTest, function as i32);
}

/// Mask depth buffer writes
///
/// # Parameters
///
/// - `mask`: `1` to disable Z writes, `0` to enable
// TODO: Use bool instead?
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDepthMask(mask: i32) {
    send_command_i(GeCommand::ZWriteDisable, mask);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDepthOffset(offset: i32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.depth_offset = offset;
    sceGuDepthRange(context.near_plane, context.far_plane);
}

/// Set which range to use for depth calculations.
///
/// # Note
///
/// The depth buffer is inversed, and takes values from 65535 to 0.
///
/// # Parameters
///
/// - `near`: Value to use for the near plane
/// - `far`: Value to use for the far plane
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDepthRange(mut near: i32, mut far: i32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    let max = near as u32 + far as u32;
    let val = ((max >> 31) + max) as i32;
    let z = (val >> 1) as f32;

    context.near_plane = near;
    context.far_plane = far;

    send_command_f(GeCommand::ViewportZScale, z - near as f32);
    send_command_f(GeCommand::ViewportZCenter, z + context.depth_offset as f32);

    if near > far {
        mem::swap(&mut near, &mut far);
    }

    send_command_i(GeCommand::MinZ, near);
    send_command_i(GeCommand::MaxZ, far);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuFog(near: f32, far: f32, color: u32) {
    let mut distance = far - near;

    if distance != 0.0 {
        distance = 1.0 / distance;
    }

    send_command_i(GeCommand::FogColor, (color & 0xffffff) as i32);
    send_command_f(GeCommand::Fog1, far);
    send_command_f(GeCommand::Fog2, distance);
}

/// Initalize the GU system
///
/// This function MUST be called as the first function, otherwise state is undetermined.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuInit() {
    const INIT_COMMANDS: [GeCommand; 223] = [
        GeCommand::Vaddr,
        GeCommand::Iaddr,
        GeCommand::Base,
        GeCommand::VertexType,
        GeCommand::OffsetAddr,
        GeCommand::Region1,
        GeCommand::Region2,
        GeCommand::LightingEnable,
        GeCommand::LightEnable0,
        GeCommand::LightEnable1,
        GeCommand::LightEnable2,
        GeCommand::LightEnable3,
        GeCommand::DepthClampEnable,
        GeCommand::CullFaceEnable,
        GeCommand::TextureMapEnable,
        GeCommand::FogEnable,
        GeCommand::DitherEnable,
        GeCommand::AlphaBlendEnable,
        GeCommand::AlphaTestEnable,
        GeCommand::ZTestEnable,
        GeCommand::StencilTestEnable,
        GeCommand::AntiAliasEnable,
        GeCommand::PatchCullEnable,
        GeCommand::ColorTestEnable,
        GeCommand::LogicOpEnable,
        GeCommand::BoneMatrixNumber,
        GeCommand::BoneMatrixData,
        GeCommand::MorphWeight0,
        GeCommand::MorphWeight1,
        GeCommand::MorphWeight2,
        GeCommand::MorphWeight3,
        GeCommand::MorphWeight4,
        GeCommand::MorphWeight5,
        GeCommand::MorphWeight6,
        GeCommand::MorphWeight7,
        GeCommand::PatchDivision,
        GeCommand::PatchPrimitive,
        GeCommand::PatchFacing,
        GeCommand::WorldMatrixNumber,
        GeCommand::WorldMatrixData,
        GeCommand::ViewMatrixNumber,
        GeCommand::ViewMatrixData,
        GeCommand::ProjMatrixNumber,
        GeCommand::ProjMatrixData,
        GeCommand::TGenMatrixNumber,
        GeCommand::TGenMatrixData,
        GeCommand::ViewportXScale,
        GeCommand::ViewportYScale,
        GeCommand::ViewportZScale,
        GeCommand::ViewportXCenter,
        GeCommand::ViewportYCenter,
        GeCommand::ViewportZCenter,
        GeCommand::TexScaleU,
        GeCommand::TexScaleV,
        GeCommand::TexOffsetU,
        GeCommand::TexOffsetV,
        GeCommand::OffsetX,
        GeCommand::OffsetY,
        GeCommand::ShadeMode,
        GeCommand::ReverseNormal,
        GeCommand::MaterialUpdate,
        GeCommand::MaterialEmissive,
        GeCommand::MaterialAmbient,
        GeCommand::MaterialDiffuse,
        GeCommand::MaterialSpecular,
        GeCommand::MaterialAlpha,
        GeCommand::MaterialSpecularCoef,
        GeCommand::AmbientColor,
        GeCommand::AmbientAlpha,
        GeCommand::LightMode,
        GeCommand::LightType0,
        GeCommand::LightType1,
        GeCommand::LightType2,
        GeCommand::LightType3,
        GeCommand::Light0X,
        GeCommand::Light0Y,
        GeCommand::Light0Z,
        GeCommand::Light1X,
        GeCommand::Light1Y,
        GeCommand::Light1Z,
        GeCommand::Light2X,
        GeCommand::Light2Y,
        GeCommand::Light2Z,
        GeCommand::Light3X,
        GeCommand::Light3Y,
        GeCommand::Light3Z,
        GeCommand::Light0DirectionX,
        GeCommand::Light0DirectionY,
        GeCommand::Light0DirectionZ,
        GeCommand::Light1DirectionX,
        GeCommand::Light1DirectionY,
        GeCommand::Light1DirectionZ,
        GeCommand::Light2DirectionX,
        GeCommand::Light2DirectionY,
        GeCommand::Light2DirectionZ,
        GeCommand::Light3DirectionX,
        GeCommand::Light3DirectionY,
        GeCommand::Light3DirectionZ,
        GeCommand::Light0ConstantAtten,
        GeCommand::Light0LinearAtten,
        GeCommand::Light0QuadtraticAtten,
        GeCommand::Light1ConstantAtten,
        GeCommand::Light1LinearAtten,
        GeCommand::Light1QuadtraticAtten,
        GeCommand::Light2ConstantAtten,
        GeCommand::Light2LinearAtten,
        GeCommand::Light2QuadtraticAtten,
        GeCommand::Light3ConstantAtten,
        GeCommand::Light3LinearAtten,
        GeCommand::Light3QuadtraticAtten,
        GeCommand::Light0ExponentAtten,
        GeCommand::Light1ExponentAtten,
        GeCommand::Light2ExponentAtten,
        GeCommand::Light3ExponentAtten,
        GeCommand::Light0CutoffAtten,
        GeCommand::Light1CutoffAtten,
        GeCommand::Light2CutoffAtten,
        GeCommand::Light3CutoffAtten,
        GeCommand::Light0Ambient,
        GeCommand::Light0Diffuse,
        GeCommand::Light0Specular,
        GeCommand::Light1Ambient,
        GeCommand::Light1Diffuse,
        GeCommand::Light1Specular,
        GeCommand::Light2Ambient,
        GeCommand::Light2Diffuse,
        GeCommand::Light2Specular,
        GeCommand::Light3Ambient,
        GeCommand::Light3Diffuse,
        GeCommand::Light3Specular,
        GeCommand::Cull,
        GeCommand::FrameBufPtr,
        GeCommand::FrameBufWidth,
        GeCommand::ZBufPtr,
        GeCommand::ZBufWidth,
        GeCommand::TexAddr0,
        GeCommand::TexAddr1,
        GeCommand::TexAddr2,
        GeCommand::TexAddr3,
        GeCommand::TexAddr4,
        GeCommand::TexAddr5,
        GeCommand::TexAddr6,
        GeCommand::TexAddr7,
        GeCommand::TexBufWidth0,
        GeCommand::TexBufWidth1,
        GeCommand::TexBufWidth2,
        GeCommand::TexBufWidth3,
        GeCommand::TexBufWidth4,
        GeCommand::TexBufWidth5,
        GeCommand::TexBufWidth6,
        GeCommand::TexBufWidth7,
        GeCommand::ClutAddr,
        GeCommand::ClutAddrUpper,
        GeCommand::TransferSrc,
        GeCommand::TransferSrcW,
        GeCommand::TransferDst,
        GeCommand::TransferDstW,
        GeCommand::TexSize0,
        GeCommand::TexSize1,
        GeCommand::TexSize2,
        GeCommand::TexSize3,
        GeCommand::TexSize4,
        GeCommand::TexSize5,
        GeCommand::TexSize6,
        GeCommand::TexSize7,
        GeCommand::TexMapMode,
        GeCommand::TexShadeLs,
        GeCommand::TexMode,
        GeCommand::TexFormat,
        GeCommand::LoadClut,
        GeCommand::ClutFormat,
        GeCommand::TexFilter,
        GeCommand::TexWrap,
        GeCommand::TexLevel,
        GeCommand::TexFunc,
        GeCommand::TexEnvColor,
        GeCommand::TexFlush,
        GeCommand::TexSync,
        GeCommand::Fog1,
        GeCommand::Fog2,
        GeCommand::FogColor,
        GeCommand::TexLodSlope,
        GeCommand::FramebufPixFormat,
        GeCommand::ClearMode,
        GeCommand::Scissor1,
        GeCommand::Scissor2,
        GeCommand::MinZ,
        GeCommand::MaxZ,
        GeCommand::ColorTest,
        GeCommand::ColorRef,
        GeCommand::ColorTestmask,
        GeCommand::AlphaTest,
        GeCommand::StencilTest,
        GeCommand::StencilOp,
        GeCommand::ZTest,
        GeCommand::BlendMode,
        GeCommand::BlendFixedA,
        GeCommand::BlendFixedB,
        GeCommand::Dith0,
        GeCommand::Dith1,
        GeCommand::Dith2,
        GeCommand::Dith3,
        GeCommand::LogicOp,
        GeCommand::ZWriteDisable,
        GeCommand::MaskRgb,
        GeCommand::MaskAlpha,
        GeCommand::TransferSrcPos,
        GeCommand::TransferDstPos,
        GeCommand::TransferSize,
        GeCommand::Vscx,
        GeCommand::Vscy,
        GeCommand::Vscz,
        GeCommand::Vtcs,
        GeCommand::Vtct,
        GeCommand::Vtcq,
        GeCommand::Vcv,
        GeCommand::Vap,
        GeCommand::Vfc,
        GeCommand::Vscv,
        GeCommand::Finish,
        GeCommand::End,
        GeCommand::Nop,
        GeCommand::Nop,
    ];

    static INIT_LIST: crate::Align16<[u32; 223]> = crate::Align16({
        let mut out = [0; 223];

        let mut i = 0;
        while i < 223 {
            out[i] = (INIT_COMMANDS[i] as u32) << 24;
            i += 1;
        }

        out
    });

    let mut callback = crate::sys::GeCallbackData {
        signal_func: Some(callback_sig),
        signal_arg: addr_of_mut!(SETTINGS).cast::<c_void>(),
        finish_func: Some(callback_fin),
        finish_arg: addr_of_mut!(SETTINGS).cast::<c_void>(),
    };

    SETTINGS.ge_callback_id = crate::sys::sceGeSetCallback(&mut callback);
    SETTINGS.swap_buffers_callback = None;
    SETTINGS.swap_buffers_behaviour = super::display::DisplaySetBufSync::Immediate;

    GE_EDRAM_ADDRESS = sys::sceGeEdramGetAddr().cast::<c_void>();

    GE_LIST_EXECUTED[0] = sys::sceGeListEnQueue(
        (&INIT_LIST as *const _ as u32 & 0x1fffffff) as *const _,
        core::ptr::null_mut(),
        SETTINGS.ge_callback_id,
        core::ptr::null_mut(),
    );

    reset_values();

    SETTINGS.kernel_event_flag = super::kernel::sceKernelCreateEventFlag(
        b"SceGuSignal\0" as *const u8,
        super::kernel::EventFlagAttributes::WAIT_MULTIPLE,
        3,
        null_mut(),
    );

    sys::sceGeListSync(GE_LIST_EXECUTED[0], 0);
}

/// Shutdown the GU system
///
/// Called when GU is no longer needed
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTerm() {
    sys::sceKernelDeleteEventFlag(SETTINGS.kernel_event_flag);
    sys::sceGeUnsetCallback(SETTINGS.ge_callback_id);
}

/// # Parameters
///
/// - `mode`: If set to 1, reset all the queues.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuBreak(mode: i32) {
    static mut UNUSED_BREAK: GeBreakParam = GeBreakParam { buf: [0; 4] };

    sys::sceGeBreak(mode, addr_of_mut!(UNUSED_BREAK));
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuContinue() {
    // Return this?
    sys::sceGeContinue();
}

// FIXME: This documentation is confusing.
/// Setup signal handler
///
/// # Parameters
///
/// - `signal`: Signal index to install a handler for
/// - `callback`: Callback to call when signal index is triggered
///
/// # Return Value
///
/// The old callback handler
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSetCallback(
    signal: GuCallbackId,
    callback: GuCallback,
) -> GuCallback {
    let old_callback;

    match signal {
        GuCallbackId::Signal => {
            old_callback = SETTINGS.sig;
            SETTINGS.sig = callback;
        }

        GuCallbackId::Finish => {
            old_callback = SETTINGS.fin;
            SETTINGS.fin = callback;
        }
    }

    old_callback
}

// FIXME: This documentation is confusing.
/// Trigger signal to call code from the command stream
///
/// # Parameters
///
/// - `behavior`: Behavior type
/// - `signal`: Signal to trigger
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSignal(behavior: SignalBehavior, signal: i32) {
    send_command_i(
        GeCommand::Signal,
        ((signal & 0xff) << 16) | (behavior as i32 & 0xffff),
    );
    send_command_i(GeCommand::End, 0);

    if signal == 3 {
        send_command_i(GeCommand::Finish, 0);
        send_command_i(GeCommand::End, 0);
    }

    send_command_i_stall(GeCommand::Nop, 0);
}

/// Send raw float command to the GE
///
/// The argument is converted into a 24-bit float before transfer.
///
/// # Parameters
///
/// - `cmd`: Which command to send
/// - `argument`: Argument to pass along
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSendCommandf(cmd: GeCommand, argument: f32) {
    send_command_f(cmd, argument);
}

/// Send raw command to the GE
///
/// Only the 24 lower bits of the argument are passed along.
///
/// # Parameters
///
/// - `cmd`: Which command to send
/// - `argument`: Argument to pass along
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSendCommandi(cmd: GeCommand, argument: i32) {
    send_command_i(cmd, argument);
}

/// Allocate memory on the current display list for temporary storage
///
/// # Note
///
/// This function is NOT for permanent memory allocation, the memory will be
/// invalid as soon as you start filling the same display list again.
///
/// # Parameters
///
/// - `size`: How much memory to allocate
///
/// # Return Value
///
/// Memory-block ready for use
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuGetMemory(mut size: i32) -> *mut c_void {
    // Some kind of 4-byte alignment?
    size += 3;
    size += (((size >> 31) as u32) >> 30) as i32;
    size = (size >> 2) << 2;

    let orig_ptr = (*LIST).current;
    let new_ptr = (orig_ptr as usize + size as usize + 8) as *mut u32;

    let lo = (8 << 24) | (new_ptr as i32 & 0xffffff);
    let hi = ((16 << 24) | ((new_ptr as u32 >> 8) & 0xf0000)) as i32;

    *orig_ptr = hi as u32;
    *orig_ptr.offset(1) = lo as u32;

    (*LIST).current = new_ptr;

    if let GuContextType::Direct = CURR_CONTEXT {
        crate::sys::sceGeListUpdateStallAddr(GE_LIST_EXECUTED[0], new_ptr.cast::<c_void>());
    }

    orig_ptr.add(2).cast::<c_void>()
}

/// Start filling a new display-context
///
/// The previous context-type is stored so that it can be restored at `sceGuFinish`.
///
/// # Parameters
///
/// - `cid`: Context Type
/// - `list`: Pointer to display-list (16 byte aligned)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuStart(context_type: GuContextType, list: *mut c_void) {
    let context = &mut CONTEXTS[context_type as usize];
    let local_list = ((list as u32) | 0x4000_0000) as *mut u32;

    // setup display list
    context.list.start = local_list;
    context.list.current = local_list;
    context.list.parent_context = CURR_CONTEXT;
    LIST = &mut context.list;

    // store current context
    CURR_CONTEXT = context_type;

    if let GuContextType::Direct = context_type {
        GE_LIST_EXECUTED[0] = crate::sys::sceGeListEnQueue(
            local_list as *mut c_void,
            local_list as *mut c_void,
            SETTINGS.ge_callback_id,
            core::ptr::null_mut(),
        );

        SETTINGS.signal_offset = 0;
    }

    if INIT == 0 {
        static DITHER_MATRIX: ScePspIMatrix4 = ScePspIMatrix4 {
            x: ScePspIVector4 {
                x: -4,
                y: 0,
                z: -3,
                w: 1,
            },
            y: ScePspIVector4 {
                x: 2,
                y: -2,
                z: 3,
                w: -1,
            },
            z: ScePspIVector4 {
                x: -3,
                y: 1,
                z: -4,
                w: 0,
            },
            w: ScePspIVector4 {
                x: 3,
                y: -1,
                z: 2,
                w: -2,
            },
        };

        sceGuSetDither(&DITHER_MATRIX);
        sceGuPatchDivide(16, 16);
        sceGuColorMaterial(
            LightComponent::AMBIENT | LightComponent::DIFFUSE | LightComponent::SPECULAR,
        );

        sceGuSpecular(1.0);
        sceGuTexScale(1.0, 1.0);

        INIT = 1;
    }

    if let GuContextType::Direct = CURR_CONTEXT {
        if DRAW_BUFFER.frame_width != 0 {
            send_command_i(
                GeCommand::FrameBufPtr,
                DRAW_BUFFER.frame_buffer as i32 & 0xffffff,
            );
            send_command_i(
                GeCommand::FrameBufWidth,
                ((DRAW_BUFFER.frame_buffer as u32 & 0xff00_0000) >> 8) as i32
                    | DRAW_BUFFER.frame_width,
            );
        }
    }
}

/// Finish current display list and go back to the parent context
///
/// If the context is `Direct`, the stall-address is updated so that the entire
/// list will execute. Otherwise, only the terminating action is written to the
/// list, depending on the context type.
///
/// The finish-callback will get a zero as argument when using this function.
///
/// This also restores control back to whatever context that was active prior to
/// this call.
///
/// # Return Value
///
/// Size of finished display list
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuFinish() -> i32 {
    match CURR_CONTEXT {
        GuContextType::Direct | GuContextType::Send => {
            send_command_i(GeCommand::Finish, 0);
            send_command_i_stall(GeCommand::End, 0);
        }

        GuContextType::Call => {
            if CALL_MODE == 1 {
                send_command_i(GeCommand::Signal, 0x120000);
                send_command_i(GeCommand::End, 0);
                send_command_i_stall(GeCommand::Nop, 0);
            } else {
                send_command_i(GeCommand::Ret, 0);
            }
        }
    }

    let size = ((*LIST).current as usize) - ((*LIST).start as usize);

    // Go to parent list
    CURR_CONTEXT = (*LIST).parent_context;
    LIST = &mut CONTEXTS[CURR_CONTEXT as usize].list;
    size as i32
}

/// Finish current display list and go back to the parent context, sending
/// argument id for the finish callback.
///
/// If the context is `Direct`, the stall-address is updated so that the entire
/// list will execute. Otherwise, only the terminating action is written to the
/// list, depending on the context type.
///
/// # Parameters
///
/// - `id`: Finish callback id (16-bit)
///
/// # Return Value
///
/// Size of finished display list
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuFinishId(id: u32) -> i32 {
    match CURR_CONTEXT {
        GuContextType::Direct | GuContextType::Send => {
            send_command_i(GeCommand::Finish, (id & 0xffff) as i32);
            send_command_i_stall(GeCommand::End, 0);
        }

        GuContextType::Call => {
            if CALL_MODE == 1 {
                send_command_i(GeCommand::Signal, 0x120000);
                send_command_i(GeCommand::End, 0);
                send_command_i_stall(GeCommand::Nop, 0);
            } else {
                send_command_i(GeCommand::Ret, 0);
            }
        }
    }

    let size = ((*LIST).current as usize) - ((*LIST).start as usize);

    // Go to parent list
    CURR_CONTEXT = (*LIST).parent_context;
    LIST = &mut CONTEXTS[CURR_CONTEXT as usize].list;
    size as i32
}

/// Call previously generated display-list
///
/// # Parameters
///
/// - `list`: Display list to call
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuCallList(list: *const c_void) {
    let list_addr = list as u32;

    if CALL_MODE == 1 {
        send_command_i(GeCommand::Signal, (list_addr >> 16) as i32 | 0x110000);
        send_command_i(GeCommand::End, list_addr as i32 & 0xffff);
        send_command_i_stall(GeCommand::Nop, 0);
    } else {
        send_command_i(GeCommand::Base, (list_addr >> 8) as i32 & 0xf0000);
        send_command_i_stall(GeCommand::Call, list_addr as i32 & 0xffffff);
    }
}

/// Set whether to use stack-based calls or signals to handle execution of
/// called lists.
///
/// # Parameters
///
/// - `mode`: True (1) to enable signals, false (0) to disable signals and use
///   normal calls instead.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuCallMode(mode: i32) {
    CALL_MODE = mode;
}

/// Check how large the current display list is
///
/// # Return Value
///
/// The size of the current display list
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuCheckList() -> i32 {
    (*LIST).current.sub((*LIST).start as usize) as i32
}

/// Send a list to the GE directly
///
/// # Parameters
///
/// - `mode`: Whether to place the list first or last in queue
/// - `list`: List to send
/// - `context`: Temporary storage for the GE context
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSendList(
    mode: GuQueueMode,
    list: *const c_void,
    context: *mut GeContext,
) {
    SETTINGS.signal_offset = 0;

    let mut args = GeListArgs {
        size: 8,
        context,
        ..<_>::default()
    };

    let callback = SETTINGS.ge_callback_id;

    let list_id = match mode {
        GuQueueMode::Head => {
            crate::sys::sceGeListEnQueueHead(list, null_mut(), callback, &mut args)
        }

        GuQueueMode::Tail => crate::sys::sceGeListEnQueue(list, null_mut(), callback, &mut args),
    };

    GE_LIST_EXECUTED[1] = list_id;
}

/// Swap display and draw buffer
///
/// # Return Value
///
/// Pointer to the new drawbuffer
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSwapBuffers() -> *mut c_void {
    if let Some(cb) = SETTINGS.swap_buffers_callback {
        cb(
            addr_of_mut!(DRAW_BUFFER.disp_buffer),
            addr_of_mut!(DRAW_BUFFER.frame_buffer),
        );
    } else {
        mem::swap(&mut DRAW_BUFFER.disp_buffer, &mut DRAW_BUFFER.frame_buffer);
    }

    if DISPLAY_ON {
        crate::sys::sceDisplaySetFrameBuf(
            GE_EDRAM_ADDRESS.add(DRAW_BUFFER.disp_buffer as usize) as *const u8,
            DRAW_BUFFER.frame_width as usize,
            DRAW_BUFFER.pixel_size,
            SETTINGS.swap_buffers_behaviour,
        );
    }

    // Comment from the C PSPSDK: remove this? it serves no real purpose
    CURRENT_FRAME ^= 1;

    DRAW_BUFFER.frame_buffer
}

/// Wait until display list has finished executing
///
/// # Parameters
///
/// - `mode`: What to wait for, one of `GuSyncMode`
/// - `behavior`: How to sync, one of `GuSyncBehavior`
///
/// # Return Value
///
/// Unknown at this time. GeListState?
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSync(mode: GuSyncMode, behavior: GuSyncBehavior) -> GeListState {
    match mode {
        GuSyncMode::Finish => crate::sys::sceGeDrawSync(behavior as i32),
        GuSyncMode::List => crate::sys::sceGeListSync(GE_LIST_EXECUTED[0], behavior as i32),
        GuSyncMode::Send => crate::sys::sceGeListSync(GE_LIST_EXECUTED[1], behavior as i32),
        _ => GeListState::Done,
    }
}

// FIXME: Confusing documentation.
/// Draw array of vertices forming primitives
///
/// Vertex order:
///
/// - Weights (0-8)
/// - Texture UV
/// - Color
/// - Normal
/// - Position
///
/// # Note
///
/// Every vertex must align to 32 bits, which means that you HAVE to pad if it does not add up!
///
/// # Parameters
///
/// - `prim`: What kind of primitives to render
/// - `vtype`: Vertex type to process
/// - `count`: How many vertices to process
/// - `indices`: Optional pointer to an index-list
/// - `vertices`: Pointer to a vertex-list
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDrawArray(
    prim: GuPrimitive,
    vtype: VertexType,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !vtype.is_empty() {
        send_command_i(GeCommand::VertexType, vtype.bits());
    }

    if !indices.is_null() {
        send_command_i(GeCommand::Base, (indices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Iaddr, indices as i32 & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(GeCommand::Base, (vertices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Vaddr, vertices as i32 & 0xffffff);
    }

    send_command_i_stall(GeCommand::Prim, ((prim as i32) << 16) | count);
}

/// Begin conditional rendering of object
///
/// If no vertices passed into this function are inside the scissor region, it
/// will skip rendering the object. There can be up to 32 levels of conditional
/// testing, and all levels HAVE to be terminated by sceGuEndObject().
///
/// # Parameters
///
/// - `vtype`: Vertex type to process
/// - `count`: Number of vertices to test
/// - `indices`: Optional list to an index-list
/// - `vertices`: Pointer to a vertex-list
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuBeginObject(
    vtype: i32,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if vtype != 0 {
        send_command_i(GeCommand::VertexType, vtype);
    }

    if !indices.is_null() {
        send_command_i(GeCommand::Base, (indices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Iaddr, indices as i32 & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(GeCommand::Base, (vertices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Vaddr, vertices as i32 & 0xffffff);
    }

    send_command_i(GeCommand::BoundingBox, count);

    // Store start to new object
    (*OBJECT_STACK.offset(OBJECT_STACK_DEPTH as isize)) = (*LIST).current;
    OBJECT_STACK_DEPTH += 1;

    // Dummy commands, overwritten in `sceGuEndObject`
    send_command_i(GeCommand::Base, 0);
    send_command_i(GeCommand::BJump, 0);
}

/// End conditional rendering of object
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuEndObject() {
    // Rewrite commands from `sceGuBeginObject`

    let current = (*LIST).current;
    (*LIST).current = *OBJECT_STACK.offset(OBJECT_STACK_DEPTH as isize - 1);

    send_command_i(GeCommand::Base, (current as u32 >> 8) as i32 & 0xf0000);
    send_command_i(GeCommand::BJump, current as i32 & 0xffffff);
    (*LIST).current = current;
    OBJECT_STACK_DEPTH -= 1;
}

/// Enable or disable GE state
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `state`: Which state to change
/// - `status`: `1` to enable or `0` to disable the state
// TODO: bool for ABI?
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSetStatus(state: GuState, status: i32) {
    if status != 0 {
        sceGuEnable(state);
    } else {
        sceGuDisable(state);
    }
}

/// Get if state is currently enabled or disabled
///
/// # Parameters
///
/// - `state`: Which state to query about
///
/// # Return Value
///
/// Whether state is enabled or not
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuGetStatus(state: GuState) -> bool {
    let state = state as u32;

    if state < 22 {
        return (STATES >> state) & 1 != 0;
    }

    false
}

/// Set the status on all 22 available states
///
/// # Parameters
///
/// - `status`: Bitmask (0-21) containing the status of all 22 states
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSetAllStatus(status: i32) {
    for i in 0..22 {
        if (status >> i) & 1 != 0 {
            sceGuEnable(mem::transmute(i));
        } else {
            sceGuDisable(mem::transmute(i));
        }
    }
}

/// Query status on all 22 available states
///
/// # Return Value
///
/// Status of all 22 states as a bitmask (0-21)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuGetAllStatus() -> i32 {
    STATES as i32
}

/// Enable GE state
///
/// # Parameters
///
/// - `state`: Which state to enable, one of `GuState`
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuEnable(state: GuState) {
    match state {
        GuState::AlphaTest => send_command_i(GeCommand::AlphaTestEnable, 1),
        GuState::DepthTest => send_command_i(GeCommand::ZTestEnable, 1),
        GuState::ScissorTest => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.scissor_enable = 1;
            send_command_i(
                GeCommand::Scissor1,
                (context.scissor_start[1] << 10) | context.scissor_start[0],
            );
            send_command_i(
                GeCommand::Scissor2,
                (context.scissor_end[1] << 10) | context.scissor_end[0],
            );
        }
        GuState::StencilTest => send_command_i(GeCommand::StencilTestEnable, 1),
        GuState::Blend => send_command_i(GeCommand::AlphaBlendEnable, 1),
        GuState::CullFace => send_command_i(GeCommand::CullFaceEnable, 1),
        GuState::Dither => send_command_i(GeCommand::DitherEnable, 1),
        GuState::Fog => send_command_i(GeCommand::FogEnable, 1),
        GuState::ClipPlanes => send_command_i(GeCommand::DepthClampEnable, 1),
        GuState::Texture2D => send_command_i(GeCommand::TextureMapEnable, 1),
        GuState::Lighting => send_command_i(GeCommand::LightingEnable, 1),
        GuState::Light0 => send_command_i(GeCommand::LightEnable0, 1),
        GuState::Light1 => send_command_i(GeCommand::LightEnable1, 1),
        GuState::Light2 => send_command_i(GeCommand::LightEnable2, 1),
        GuState::Light3 => send_command_i(GeCommand::LightEnable3, 1),
        GuState::LineSmooth => send_command_i(GeCommand::AntiAliasEnable, 1),
        GuState::PatchCullFace => send_command_i(GeCommand::PatchCullEnable, 1),
        GuState::ColorTest => send_command_i(GeCommand::ColorTestEnable, 1),
        GuState::ColorLogicOp => send_command_i(GeCommand::LogicOpEnable, 1),
        GuState::FaceNormalReverse => send_command_i(GeCommand::ReverseNormal, 1),
        GuState::PatchFace => send_command_i(GeCommand::PatchFacing, 1),
        GuState::Fragment2X => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.fragment_2x = 0x10000;
            send_command_i(GeCommand::TexFunc, 0x10000 | context.texture_function);
        }
    }

    if (state as u32) < 22 {
        STATES |= 1 << state as u32
    }
}

/// Disable GE state
///
/// # Parameters
///
/// - `state`: Which state to disable, one of `GuState`
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDisable(state: GuState) {
    match state {
        GuState::AlphaTest => send_command_i(GeCommand::AlphaTestEnable, 0),
        GuState::DepthTest => send_command_i(GeCommand::ZTestEnable, 0),
        GuState::ScissorTest => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.scissor_enable = 0;
            send_command_i(GeCommand::Scissor1, 0);
            send_command_i(
                GeCommand::Scissor2,
                ((DRAW_BUFFER.height - 1) << 10) | (DRAW_BUFFER.width - 1),
            );
        }
        GuState::StencilTest => send_command_i(GeCommand::StencilTestEnable, 0),
        GuState::Blend => send_command_i(GeCommand::AlphaBlendEnable, 0),
        GuState::CullFace => send_command_i(GeCommand::CullFaceEnable, 0),
        GuState::Dither => send_command_i(GeCommand::DitherEnable, 0),
        GuState::Fog => send_command_i(GeCommand::FogEnable, 0),
        GuState::ClipPlanes => send_command_i(GeCommand::DepthClampEnable, 0),
        GuState::Texture2D => send_command_i(GeCommand::TextureMapEnable, 0),
        GuState::Lighting => send_command_i(GeCommand::LightingEnable, 0),
        GuState::Light0 => send_command_i(GeCommand::LightEnable0, 0),
        GuState::Light1 => send_command_i(GeCommand::LightEnable1, 0),
        GuState::Light2 => send_command_i(GeCommand::LightEnable2, 0),
        GuState::Light3 => send_command_i(GeCommand::LightEnable3, 0),
        GuState::LineSmooth => send_command_i(GeCommand::AntiAliasEnable, 0),
        GuState::PatchCullFace => send_command_i(GeCommand::PatchCullEnable, 0),
        GuState::ColorTest => send_command_i(GeCommand::ColorTestEnable, 0),
        GuState::ColorLogicOp => send_command_i(GeCommand::LogicOpEnable, 0),
        GuState::FaceNormalReverse => send_command_i(GeCommand::ReverseNormal, 0),
        GuState::PatchFace => send_command_i(GeCommand::PatchFacing, 0),
        GuState::Fragment2X => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.fragment_2x = 0;
            send_command_i(GeCommand::TexFunc, context.texture_function);
        }
    }

    if (state as u32) < 22 {
        STATES &= !(1 << state as u32)
    }
}

/// Set light parameters
///
/// # Parameters
///
/// - `light`: Light index
/// - `type`: Light type, one of `LightType`
/// - `components`: Light components, one or more of `LightComponent`
/// - `position`: Light position
// FIXME: light components seem to be a subset here.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuLight(
    light: i32,
    type_: LightType,
    components: LightComponent,
    position: &ScePspFVector3,
) {
    let settings = &LIGHT_COMMANDS[light as usize];

    send_command_f(settings.xpos, position.x);
    send_command_f(settings.ypos, position.y);
    send_command_f(settings.zpos, position.z);

    let mut kind = 2;
    if components.bits() != 8 {
        kind = if components.bits() ^ 6 < 1 { 1 } else { 0 };
    }

    send_command_i(settings.type_, ((type_ as i32 & 0x03) << 8) | kind);
}

/// Set light attenuation
///
/// # Parameters
///
/// - `light`: Light index
/// - `atten0`: Constant attenuation factor
/// - `atten1`: Linear attenuation factor
/// - `atten2`: Quadratic attenuation factor
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuLightAtt(light: i32, atten0: f32, atten1: f32, atten2: f32) {
    let settings = &LIGHT_COMMANDS[light as usize];
    send_command_f(settings.constant, atten0);
    send_command_f(settings.linear, atten1);
    send_command_f(settings.quadratic, atten2);
}

/// Set light color
///
/// # Parameters
///
/// - `light`: Light index
/// - `component`: Which component(s) to set
/// - `color`: Which color to use
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuLightColor(light: i32, component: LightComponent, color: u32) {
    let settings = &LIGHT_COMMANDS[light as usize];

    // PSPSDK implements this as a jump table, probably for speed. Should we do
    // this too?
    //
    // TODO: Or maybe only certain combinations are valid?

    if component.intersects(LightComponent::AMBIENT) {
        send_command_i(settings.ambient, (color & 0xffffff) as i32);
    }

    if component.intersects(LightComponent::DIFFUSE) {
        send_command_i(settings.diffuse, (color & 0xffffff) as i32);
    }

    if component.intersects(LightComponent::SPECULAR) {
        send_command_i(settings.specular, (color & 0xffffff) as i32);
    }
}

/// Set light mode
///
/// # Parameters
///
/// - `mode`: Light mode to use
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuLightMode(mode: LightMode) {
    send_command_i(GeCommand::LightMode, mode as i32);
}

/// Set spotlight parameters
///
/// # Parameters
///
/// - `light`: Light index
/// - `direction`: Spotlight direction
/// - `exponent`: Spotlight exponent
/// - `cutoff`: Spotlight cutoff angle (in radians)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuLightSpot(
    light: i32,
    direction: &ScePspFVector3,
    exponent: f32,
    cutoff: f32,
) {
    let settings = &LIGHT_COMMANDS[light as usize];

    send_command_f(settings.exponent, exponent);
    send_command_f(settings.cutoff, cutoff);

    send_command_f(settings.xdir, direction.x);
    send_command_f(settings.ydir, direction.y);
    send_command_f(settings.zdir, direction.z);
}

/// Clear current drawbuffer
///
/// # Parameters
///
/// - `flags`: Which part of the buffer to clear
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuClear(flags: ClearBuffer) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];

    struct Vertex {
        color: u32,
        x: u16,
        y: u16,
        z: u16,
        _pad: u16,
    }

    let filter: u32 = match DRAW_BUFFER.pixel_size {
        DisplayPixelFormat::Psm5650 => context.clear_color & 0xffffff,
        DisplayPixelFormat::Psm5551 => {
            (context.clear_color & 0xffffff) | (context.clear_stencil << 31)
        }
        DisplayPixelFormat::Psm4444 => {
            (context.clear_color & 0xffffff) | (context.clear_stencil << 28)
        }
        DisplayPixelFormat::Psm8888 => {
            (context.clear_color & 0xffffff) | (context.clear_stencil << 24)
        }
    };

    let vertices;
    let count;

    if !flags.intersects(ClearBuffer::FAST_CLEAR_BIT) {
        vertices = sceGuGetMemory(2 * mem::size_of::<Vertex>() as i32) as *mut Vertex;
        count = 2;

        (*vertices.offset(0)).color = 0;
        (*vertices.offset(0)).x = 0;
        (*vertices.offset(0)).y = 0;
        (*vertices.offset(0)).z = context.clear_depth as u16;

        (*vertices.offset(1)).color = filter;
        (*vertices.offset(1)).x = DRAW_BUFFER.width as u16;
        (*vertices.offset(1)).y = DRAW_BUFFER.height as u16;
        (*vertices.offset(1)).z = context.clear_depth as u16;
    } else {
        count = ((DRAW_BUFFER.width + 63) / 64) * 2;
        vertices = sceGuGetMemory(count * core::mem::size_of::<Vertex>() as i32) as *mut Vertex;

        let mut curr = vertices;

        for i in 0..count {
            let j = i >> 1;
            let k = i & 1;

            (*curr).color = filter;
            (*curr).x = (j + k) as u16 * 64;
            (*curr).y = (k * DRAW_BUFFER.height) as u16;
            (*curr).z = context.clear_depth as u16;

            curr = curr.add(1);
        }
    }

    {
        let relevant_flags = flags
            & (ClearBuffer::COLOR_BUFFER_BIT
                | ClearBuffer::STENCIL_BUFFER_BIT
                | ClearBuffer::DEPTH_BUFFER_BIT);

        send_command_i(
            GeCommand::ClearMode,
            (relevant_flags.bits() << 8) as i32 | 0x01,
        );
    }

    sceGuDrawArray(
        GuPrimitive::Sprites,
        VertexType::COLOR_8888 | VertexType::VERTEX_16BIT | VertexType::TRANSFORM_2D,
        count,
        null_mut(),
        vertices as *mut c_void,
    );

    send_command_i(GeCommand::ClearMode, 0);
}

/// Set the current clear-color
///
/// # Parameters
///
/// - `color`: Color to clear with
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuClearColor(color: u32) {
    CONTEXTS[CURR_CONTEXT as usize].clear_color = color;
}

/// Set the current clear-depth
///
/// # Parameters
///
/// - `depth`: Set which depth to clear with (0x0000-0xffff)
// TODO: Can `depth` be u16 or does this cause issues with FFI ABI compatibility?
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuClearDepth(depth: u32) {
    CONTEXTS[CURR_CONTEXT as usize].clear_depth = depth;
}

/// Set the current stencil clear value
///
/// # Parameters
///
/// - `stencil`: Set which stencil value to clear with (0-255)
// TODO: Can `stencil` be u8 or does this cause issues with FFI ABI compatibility?
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuClearStencil(stencil: u32) {
    CONTEXTS[CURR_CONTEXT as usize].clear_stencil = stencil;
}

/// Set mask for which bits of the pixels to write
///
/// # Parameters
///
/// - `mask`: Which bits to filter against writes
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuPixelMask(mask: u32) {
    send_command_i(GeCommand::MaskRgb, mask as i32 & 0xffffff);
    send_command_i(GeCommand::MaskAlpha, (mask >> 24) as i32);
}

/// Set current primitive color
///
/// # Parameters
///
/// - `color`: Which color to use (overridden by vertex colors)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuColor(color: u32) {
    sceGuMaterial(
        LightComponent::AMBIENT | LightComponent::DIFFUSE | LightComponent::SPECULAR,
        color,
    );
}

/// Set the color test function
///
/// The color test is only performed while `GuState::ColorTest` is enabled, e.g.
/// via `sceGuEnable`.
///
/// # Parameters
///
/// - `func`: Color test function
/// - `color`: Color to test against
/// - `mask`: Mask ANDed against both source and destination when testing
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuColorFunc(func: ColorFunc, color: u32, mask: u32) {
    send_command_i(GeCommand::ColorTest, func as i32 & 0x03);
    send_command_i(GeCommand::ColorRef, color as i32 & 0xffffff);
    send_command_i(GeCommand::ColorTestmask, mask as i32);
}

/// Set which color components the material will receive
///
/// # Parameters
///
/// - `components`: Which component(s) to receive
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuColorMaterial(components: LightComponent) {
    send_command_i(GeCommand::MaterialUpdate, components.bits());
}

/// Set the alpha test parameters
///
/// # Parameters
///
/// - `func`: Specifies the alpha comparison function.
/// - `value`: Specifies the reference value that incoming alpha values are compared to.
/// - `mask`: Specifies the mask that both values are ANDed with before comparison.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuAlphaFunc(func: AlphaFunc, value: i32, mask: i32) {
    let arg = func as i32 | ((value & 0xff) << 8) | ((mask & 0xff) << 16);
    send_command_i(GeCommand::AlphaTest, arg);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuAmbient(color: u32) {
    send_command_i(GeCommand::AmbientColor, color as i32 & 0xffffff);
    send_command_i(GeCommand::AmbientAlpha, (color >> 24) as i32);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuAmbientColor(color: u32) {
    send_command_i(GeCommand::MaterialAmbient, color as i32 & 0xffffff);
    send_command_i(GeCommand::MaterialAlpha, (color >> 24) as i32);
}

/// Set the blending mode
///
/// This is similar to `glBlendEquation` combined with `glBlendFunc` in OpenGL.
///
/// # Parameters
///
/// - `op`: Blending Operation
/// - `src`: Blending function for source operand
/// - `dest`: Blending function for dest operand
/// - `srcfix`: Fixed value for `BlendFactor::Fix` (source operand)
/// - `destfix`: Fixed value for `BlendFactor::Fix` (dest operand)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuBlendFunc(
    op: BlendOp,
    src: BlendFactor,
    dest: BlendFactor,
    src_fix: u32,
    dest_fix: u32,
) {
    send_command_i(
        GeCommand::BlendMode,
        src as i32 | ((dest as i32) << 4) | ((op as i32) << 8),
    );
    send_command_i(GeCommand::BlendFixedA, src_fix as i32 & 0xffffff);
    send_command_i(GeCommand::BlendFixedB, dest_fix as i32 & 0xffffff);
}

/// Set current primitive color, for specific light components.
///
/// # Parameters
///
/// - `components`: Which component(s) to set
/// - `color`: Color to set (*likely* overridden by vertex colors)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuMaterial(components: LightComponent, color: u32) {
    if components.intersects(LightComponent::AMBIENT) {
        send_command_i(GeCommand::MaterialAmbient, color as i32 & 0xffffff);
        send_command_i(GeCommand::MaterialAlpha, (color >> 24) as i32);
    }

    if components.intersects(LightComponent::DIFFUSE) {
        send_command_i(GeCommand::MaterialDiffuse, color as i32 & 0xffffff);
    }

    if components.intersects(LightComponent::SPECULAR) {
        send_command_i(GeCommand::MaterialSpecular, color as i32 & 0xffffff);
    }
}

// TODO: Needs documentation.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuModelColor(emissive: u32, ambient: u32, diffuse: u32, specular: u32) {
    send_command_i(GeCommand::MaterialEmissive, emissive as i32 & 0xffffff);
    send_command_i(GeCommand::MaterialAmbient, ambient as i32 & 0xffffff);
    send_command_i(GeCommand::MaterialDiffuse, diffuse as i32 & 0xffffff);
    send_command_i(GeCommand::MaterialSpecular, specular as i32 & 0xffffff);
}

/// Set stencil function and reference value for stencil testing
///
/// # Parameters
///
/// - `func`: Test function
/// - `ref_`: The reference value for the stencil test
/// - `mask`: Mask that is ANDed with both the reference value and stored
///   stencil value when the test is done
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuStencilFunc(func: StencilFunc, ref_: i32, mask: i32) {
    send_command_i(
        GeCommand::StencilTest,
        func as i32 | ((ref_ & 0xff) << 8) | ((mask & 0xff) << 16),
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
/// - `zfail`: The action to take when the stencil test passes, but the depth test fails
/// - `zpass`: The action to take when both the stencil test and depth test pass
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuStencilOp(
    fail: StencilOperation,
    zfail: StencilOperation,
    zpass: StencilOperation,
) {
    send_command_i(
        GeCommand::StencilOp,
        fail as i32 | ((zfail as i32) << 8) | ((zpass as i32) << 16),
    );
}

/// Set the specular power for the material
///
/// # Parameters
///
/// - `power`: Specular power
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSpecular(power: f32) {
    send_command_f(GeCommand::MaterialSpecularCoef, power);
}

/// Set the current face-order (for culling)
///
/// This only has effect when culling (`GuState::CullFace`) is enabled, e.g. via
/// `sceGuEnable`.
///
/// # Parameters
///
/// - `order`: Which order to use, one of `FrontFaceDirection`
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuFrontFace(order: FrontFaceDirection) {
    match order {
        FrontFaceDirection::CounterClockwise => send_command_i(GeCommand::Cull, 0),
        FrontFaceDirection::Clockwise => send_command_i(GeCommand::Cull, 1),
    }
}

/// Set color logical operation
///
/// This operation only has effect if `GuState::ColorLogicOp` is enabled, e.g. via
/// `sceGuEnable`.
///
/// # Parameters
///
/// - `op`: Operation to execute
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuLogicalOp(op: LogicalOperation) {
    send_command_i(GeCommand::LogicOp, op as i32 & 0x0f);
}

/// Set ordered pixel dither matrix
///
/// This dither matrix is only applied if `GuState::Dither` is enabled, e.g. via
/// `sceGuEnable`.
///
/// # Parameters
///
/// - `matrix`: Dither matrix
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSetDither(matrix: &ScePspIMatrix4) {
    send_command_i(
        GeCommand::Dith0,
        (matrix.x.x & 0x0f)
            | ((matrix.x.y & 0x0f) << 4)
            | ((matrix.x.z & 0x0f) << 8)
            | ((matrix.x.w & 0x0f) << 12),
    );

    send_command_i(
        GeCommand::Dith1,
        (matrix.y.x & 0x0f)
            | ((matrix.y.y & 0x0f) << 4)
            | ((matrix.y.z & 0x0f) << 8)
            | ((matrix.y.w & 0x0f) << 12),
    );

    send_command_i(
        GeCommand::Dith2,
        (matrix.z.x & 0x0f)
            | ((matrix.z.y & 0x0f) << 4)
            | ((matrix.z.z & 0x0f) << 8)
            | ((matrix.z.w & 0x0f) << 12),
    );

    send_command_i(
        GeCommand::Dith3,
        (matrix.w.x & 0x0f)
            | ((matrix.w.y & 0x0f) << 4)
            | ((matrix.w.z & 0x0f) << 8)
            | ((matrix.w.w & 0x0f) << 12),
    );
}

/// Set how primitives are shaded
///
/// # Parameters
///
/// - `mode`: Which mode to use, one of `ShadingModel`.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuShadeModel(mode: ShadingModel) {
    match mode {
        ShadingModel::Smooth => send_command_i(GeCommand::ShadeMode, 1),
        ShadingModel::Flat => send_command_i(GeCommand::ShadeMode, 0),
    }
}

// TODO: Maybe add examples in documentation?
/// Image transfer using the GE
///
/// # Note
///
/// Data must be aligned to 1 quad word (16 bytes)
///
/// # Parameters
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
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuCopyImage(
    psm: DisplayPixelFormat,
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
    send_command_i(GeCommand::TransferSrc, (src as i32) & 0xffffff);
    send_command_i(
        GeCommand::TransferSrcW,
        (((src as u32) & 0xff000000) >> 8) as i32 | srcw,
    );
    send_command_i(GeCommand::TransferSrcPos, (sy << 10) | sx);
    send_command_i(GeCommand::TransferDst, (dest as i32) & 0xffffff);
    send_command_i(
        GeCommand::TransferDstW,
        (((dest as u32) & 0xff000000) >> 8) as i32 | destw,
    );
    send_command_i(GeCommand::TransferDstPos, (dy << 10) | dx);
    send_command_i(GeCommand::TransferSize, ((height - 1) << 10) | (width - 1));

    let is_32_bit_texel = if let DisplayPixelFormat::Psm8888 = psm {
        1
    } else {
        0
    };

    send_command_i(GeCommand::TransferStart, is_32_bit_texel);
}

/// Specify the texture environment color
///
/// This is used in the texture function when a constant color is needed.
///
/// See `sceGuTexFunc` for more information.
///
/// # Parameters
///
/// - `color`: Constant color (0x00BBGGRR)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexEnvColor(color: u32) {
    send_command_i(GeCommand::TexEnvColor, color as i32 & 0xffffff);
}

/// Set how the texture is filtered
///
/// # Parameters
///
/// - `min`: Minimizing filter
/// - `mag`: Magnifying filter
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexFilter(min: TextureFilter, mag: TextureFilter) {
    send_command_i(GeCommand::TexFilter, ((mag as i32) << 8) | (min as i32));
}

/// Flush texture page-cache
///
/// Do this if you have copied/rendered into an area currently in the texture
/// cache.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexFlush() {
    send_command_f(GeCommand::TexFlush, 0.0);
}

/// Set how textures are applied
///
/// # Parameters
///
/// - `tfx`: Which apply-mode to use
/// - `tcc`: Which component-mode to use
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexFunc(tfx: TextureEffect, tcc: TextureColorComponent) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.texture_function = (((tcc as u32) << 8) | (tfx as u32)) as i32;
    send_command_i(
        GeCommand::TexFunc,
        (((tcc as u32) << 8) | (tfx as u32) | context.fragment_2x as u32) as i32,
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
/// - `width`: Width of texture map (must be a power of 2)
/// - `height`: Height of texture map (must be a power of 2)
/// - `tbw`: Texture Buffer Width (block-aligned). For `PixelFormat::8888`, this
///    seems to have to be a multiple of 4. This value indicates the actual
///    width of the image, not the width of the texture map.
/// - `tbp`: Texture buffer pointer (16 byte aligned)
///
/// # `width` vs `tbw`
///
/// The parameters `height` and `width` indicate the size of the texture map. It
/// is possible to pass in oddly sized textures, however, using the `tbw`
/// parameter, as long as the true width is divisible by (... block size? The
/// block size seems to be 4 for `PixelFormat::8888`).
///
/// As an example, say you have a 340x340 image. This image can be passed in
/// like so:
///
/// ```ignore
/// let image_data = ...; // Some 16-byte aligned source.
/// sceGuTexImage(MipmapLevel::None, 512, 512, 340, data);
/// ```
///
/// This will generate a 512x512 pixel texture map, with the remaining horizontal
/// space being filled with the original texture repeating. The remaining
/// vertical space will overflow into the data past the input buffer, which may
/// appear as garbage data. This is not a problem as the UV coordinates on the
/// triangles can be crafted to stay within the image bounds, both vertically and
/// horizontally.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexImage(
    mipmap: MipmapLevel,
    width: i32,
    height: i32,
    tbw: i32,
    tbp: *const c_void,
) {
    use core::intrinsics::ctlz;

    const TBP_CMD_TBL: [GeCommand; 8] = [
        GeCommand::TexAddr0,
        GeCommand::TexAddr1,
        GeCommand::TexAddr2,
        GeCommand::TexAddr3,
        GeCommand::TexAddr4,
        GeCommand::TexAddr5,
        GeCommand::TexAddr6,
        GeCommand::TexAddr7,
    ];

    const TBW_CMD_TBL: [GeCommand; 8] = [
        GeCommand::TexBufWidth0,
        GeCommand::TexBufWidth1,
        GeCommand::TexBufWidth2,
        GeCommand::TexBufWidth3,
        GeCommand::TexBufWidth4,
        GeCommand::TexBufWidth5,
        GeCommand::TexBufWidth6,
        GeCommand::TexBufWidth7,
    ];

    const TSIZE_CMD_TBL: [GeCommand; 8] = [
        GeCommand::TexSize0,
        GeCommand::TexSize1,
        GeCommand::TexSize2,
        GeCommand::TexSize3,
        GeCommand::TexSize4,
        GeCommand::TexSize5,
        GeCommand::TexSize6,
        GeCommand::TexSize7,
    ];

    send_command_i(TBP_CMD_TBL[mipmap as usize], (tbp as i32) & 0xffffff);
    send_command_i(
        TBW_CMD_TBL[mipmap as usize],
        ((tbp as u32 >> 8) as i32 & 0x0f0000) | tbw,
    );
    send_command_i(
        TSIZE_CMD_TBL[mipmap as usize],
        (((31 - ctlz(height & 0x3ff)) << 8) | (31 - ctlz(width & 0x3ff))) as i32,
    );
    sceGuTexFlush();
}

/// Set texture-level mode (mipmapping)
///
/// # Parameters
///
/// - `mode`: Which mode to use, one of TextureLevelMode
/// - `bias`: Which mipmap bias to use
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexLevelMode(mode: TextureLevelMode, bias: f32) {
    // Linker error if this is not here.
    #[no_mangle]
    #[cfg(target_os = "psp")]
    #[allow(deprecated)]
    unsafe extern "C" fn truncf(mut x: f32) -> f32 {
        core::arch::asm!("cvt.w.s {0}, {0}", inout(freg) x);
        x
    }

    let mut offset = core::intrinsics::truncf32(bias * 16.0) as i32;

    // PSPSDK: mip map bias?
    if offset >= 128 {
        offset = 128
    } else if offset < -128 {
        offset = -128;
    }

    send_command_i(GeCommand::TexLevel, ((offset as i32) << 16) | mode as i32);
}

/// Set the texture-mapping mode
///
/// # Parameters
///
/// - `mode`: Which mode to use
/// - `a1`: Unknown
/// - `a2`: Unknown
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexMapMode(mode: TextureMapMode, a1: u32, a2: u32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.texture_map_mode = mode;
    send_command_i(
        GeCommand::TexMapMode,
        ((context.texture_proj_map_mode as i32) << 8) | mode as i32,
    );
    send_command_i(GeCommand::TexShadeLs, ((a2 << 8) | (a1 & 0x03)) as i32);
}

/// Set texture-mode parameters
///
/// # Parameters
///
/// - `tpsm`: Which texture format to use
/// - `maxmips`: Number of mipmaps to use (0-7)
/// - `a2`: Unknown, set to 0
/// - `swizzle`: `1` to swizzle texture-reads.
// TODO: Are boolean parameters ABI compatibile with FFI i32 parameters?
// TODO: Better documentation for `maxmips`. What does it do? Maybe it should be
//       of type `MipmapLevel`?
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexMode(
    tpsm: TexturePixelFormat,
    maxmips: i32,
    a2: i32,
    swizzle: i32,
) {
    CONTEXTS[CURR_CONTEXT as usize].texture_mode = tpsm;

    send_command_i(GeCommand::TexMode, (maxmips << 16) | (a2 << 8) | swizzle);

    send_command_i(GeCommand::TexFormat, tpsm as i32);
    sceGuTexFlush();
}

/// Set texture offset
///
/// # Note
///
/// Only used by the 3D T&L pipe, renders done with `VertexType::TRANSFORM_2D`
/// are not affected by this.
///
/// # Parameters
///
/// - `u`: Offset to add to the U coordinate
/// - `v`: Offset to add to the V coordinate
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexOffset(u: f32, v: f32) {
    send_command_f(GeCommand::TexOffsetU, u);
    send_command_f(GeCommand::TexOffsetV, v);
}

/// Set texture projection-map mode
///
/// # Parameters
///
/// - `mode`: Which mode to use
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexProjMapMode(mode: TextureProjectionMapMode) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.texture_proj_map_mode = mode;
    send_command_i(
        GeCommand::TexMapMode,
        ((mode as i32) << 8) | context.texture_map_mode as i32,
    );
}

/// Set texture scale
///
/// # Note
///
/// Only used by the 3D T&L pipe, renders ton with `VertexType::TRANSFORM_2D`
/// are not affected by this.
///
/// # Parameters
///
/// - `u`: Scalar to multiply U coordinate with
/// - `v`: Scalar to multiply V coordinate with
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexScale(u: f32, v: f32) {
    send_command_f(GeCommand::TexScaleU, u);
    send_command_f(GeCommand::TexScaleV, v);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexSlope(slope: f32) {
    send_command_f(GeCommand::TexLodSlope, slope);
}

/// Synchronize rendering pipeline with image upload.
///
/// This will stall the rendering pipeline until the current image upload initiated by
/// `sceGuCopyImage` has completed.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexSync() {
    send_command_i(GeCommand::TexSync, 0);
}

/// Set if the texture should repeat or clamp
///
/// Available modes are:
///
/// # Parameters
///
/// - `u`: Wrap-mode for the U direction
/// - `v`: Wrap-mode for the V direction
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuTexWrap(u: GuTexWrapMode, v: GuTexWrapMode) {
    send_command_i(GeCommand::TexWrap, ((v as i32) << 8) | u as i32);
}

/// Upload CLUT (Color Lookup Table)
///
/// # Note
///
/// Data must be aligned to 1 quad word (16 bytes)
///
/// # Parameters
///
/// - `num_blocks`: How many blocks of 8 entries to upload (32*8 is 256 colors)
/// - `cbp`: Pointer to palette (16 byte aligned)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuClutLoad(num_blocks: i32, cbp: *const c_void) {
    send_command_i(GeCommand::ClutAddr, (cbp as i32) & 0xffffff);
    send_command_i(
        GeCommand::ClutAddrUpper,
        ((cbp as u32) >> 8) as i32 & 0xf0000,
    );
    send_command_i(GeCommand::LoadClut, num_blocks);
}

/// CLUT palette pixel formats.
///
/// This is the pixel format for the input palette when setting up a CLUT.
#[repr(u32)]
pub enum ClutPixelFormat {
    /// Hicolor, 16-bit, RGB 5:6:5
    Psm5650 = 0,
    /// Hicolor, 16-bit, RGBA 5:5:5:1
    Psm5551 = 1,
    /// Hicolor, 16-bit, RGBA 4:4:4:4
    Psm4444 = 2,
    /// Truecolor, 32-bit, RGBA 8:8:8:8
    Psm8888 = 3,
}

/// Set current CLUT mode
///
/// # Parameters
///
/// - `cpsm`: Which pixel format to use for the palette
/// - `shift`: Shifts color index by that many bits to the right
/// - `mask`: Masks the color index with this bitmask after the shift (0-0xFF)
/// - `a3`: Unknown, set to 0
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuClutMode(cpsm: ClutPixelFormat, shift: u32, mask: u32, a3: u32) {
    let arg = ((cpsm as u32) | (shift << 2) | (mask << 8) | (a3 << 16)) as i32;
    send_command_i(GeCommand::ClutFormat, arg);
}

/// Set virtual coordinate offset
///
/// The PSP has a virtual coordinate-space of 4096x4096, this controls where
/// rendering is performed.
///
/// # Example
///
/// Center the virtual coordinate range:
///
/// ```no_run
/// # use psp::sys::sceGuOffset;
/// sceGuOffset(2048 - (480 / 2), 2048 - (272 / 2));
/// ```
///
/// # Parameters
///
/// - `x`: Offset (0-4095)
/// - `y`: Offset (0-4095)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuOffset(x: u32, y: u32) {
    send_command_i(GeCommand::OffsetX, (x << 4) as i32);
    send_command_i(GeCommand::OffsetY, (y << 4) as i32);
}

/// Set what to scissor within the current viewport
///
/// Note that scissoring is only performed if the custom scissoring
/// (`GuState::ScissorTest`) is enabled, e.g. via `sceGuEnable`.
///
/// # Parameters
///
/// - `x`: Left of scissor region
/// - `y`: Top of scissor region
/// - `w`: Width of scissor region
/// - `h`: Height of scissor region
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuScissor(x: i32, y: i32, w: i32, h: i32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];

    context.scissor_start = [x, y];
    context.scissor_end = [w - 1, h - 1];

    if context.scissor_enable != 0 {
        send_command_i(
            GeCommand::Scissor1,
            (context.scissor_start[1] << 10) | context.scissor_start[0],
        );
        send_command_i(
            GeCommand::Scissor2,
            (context.scissor_end[1] << 10) | context.scissor_end[0],
        );
    }
}

/// Set current viewport
///
/// # Example
///
/// Setup a viewport of size (480,272) with origin at (2048,2048)
///
/// ```no_run
/// # use psp::sys::sceGuViewport;
/// sceGuViewport(2048, 2048, 480, 272);
/// ```
///
/// # Parameters
///
/// - `cx`: Center for horizontal viewport
/// - `cy`: Center for vertical viewport
/// - `width`: Width of viewport
/// - `height`: Height of viewport
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuViewport(cx: i32, cy: i32, width: i32, height: i32) {
    send_command_f(GeCommand::ViewportXScale, (width >> 1) as f32);
    send_command_f(GeCommand::ViewportYScale, ((-height) >> 1) as f32);
    send_command_f(GeCommand::ViewportXCenter, cx as f32);
    send_command_f(GeCommand::ViewportYCenter, cy as f32);
}

/// Draw bezier surface
///
/// # Parameters
///
/// - `vtype`: Vertex type
/// - `ucount`: Number of vertices used in the U direction
/// - `vcount`: Number of vertices used in the V direction
/// - `indices`: Pointer to index buffer
/// - `vertices`: Pointer to vertex buffer
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDrawBezier(
    v_type: VertexType,
    u_count: i32,
    v_count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !v_type.is_empty() {
        send_command_i(GeCommand::VertexType, v_type.bits());
    }

    if !indices.is_null() {
        send_command_i(GeCommand::Base, ((indices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Iaddr, (indices as i32) & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(GeCommand::Base, ((vertices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Vaddr, (vertices as i32) & 0xffffff);
    }

    send_command_i(GeCommand::Bezier, (v_count << 8) | u_count);
}

/// Set dividing for patches (beziers and splines)
///
/// # Parameters
///
/// - `ulevel`: Number of division on u direction
/// - `vlevel`: Number of division on v direction
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuPatchDivide(ulevel: u32, vlevel: u32) {
    send_command_i(GeCommand::PatchDivision, ((vlevel << 8) | ulevel) as i32);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuPatchFrontFace(a0: u32) {
    send_command_i(GeCommand::PatchFacing, a0 as i32);
}

/// Set primitive for patches (beziers and splines)
///
/// # Parameters
///
/// - `prim`: Desired primitive type
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuPatchPrim(prim: PatchPrimitive) {
    match prim {
        PatchPrimitive::Points => send_command_i(GeCommand::PatchPrimitive, 2),
        PatchPrimitive::LineStrip => send_command_i(GeCommand::PatchPrimitive, 1),
        PatchPrimitive::TriangleStrip => send_command_i(GeCommand::PatchPrimitive, 0),
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDrawSpline(
    v_type: VertexType,
    u_count: i32,
    v_count: i32,
    u_edge: i32,
    v_edge: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !v_type.is_empty() {
        send_command_i(GeCommand::VertexType, v_type.bits());
    }

    if !indices.is_null() {
        send_command_i(GeCommand::Base, ((indices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Iaddr, (indices as i32) & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(GeCommand::Base, ((vertices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Vaddr, (vertices as i32) & 0xffffff);
    }

    send_command_i(
        GeCommand::Spline,
        (v_edge << 18) | (u_edge << 16) | (v_count << 8) | u_count,
    );
}

/// Set transform matrices
///
/// # Parameters
///
/// - `type`: Which matrix-type to set
/// - `matrix`: Matrix to load
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuSetMatrix(type_: MatrixMode, matrix: &ScePspFMatrix4) {
    let fmatrix = matrix as *const _ as *const f32;

    match type_ {
        MatrixMode::Projection => {
            send_command_f(GeCommand::ProjMatrixNumber, 0.0);
            for i in 0..16 {
                send_command_f(GeCommand::ProjMatrixData, *fmatrix.offset(i));
            }
        }

        MatrixMode::View => {
            send_command_f(GeCommand::ViewMatrixNumber, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(GeCommand::ViewMatrixData, *fmatrix.offset(j + i * 4));
                }
            }
        }

        MatrixMode::Model => {
            send_command_f(GeCommand::WorldMatrixNumber, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(GeCommand::WorldMatrixData, *fmatrix.offset(j + i * 4));
                }
            }
        }

        MatrixMode::Texture => {
            send_command_f(GeCommand::TGenMatrixNumber, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(GeCommand::TGenMatrixData, *fmatrix.offset(j + i * 4));
                }
            }
        }
    }
}

/// Specify skinning matrix entry
///
/// To enable vertex skinning, use `VertexType::WEIGHTSn`, where n is between
/// 1-8, and pass an applicable `VertexType::WEIGHT_?BIT` declaration. This will
/// change the amount of weights passed in the vertex array, and by setting the
/// skinning matrices, you will multiply each vertex every weight and vertex
/// passed.
///
/// Please see `VertexType` for vertex format information.
///
/// # Parameters
///
/// - `index`: Skinning matrix index (0-7)
/// - `matrix`: Matrix to set
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuBoneMatrix(index: u32, matrix: &ScePspFMatrix4) {
    send_command_i(GeCommand::BoneMatrixNumber, index as i32 * 12); // 3 * 4 matrix

    send_command_f(GeCommand::BoneMatrixData, matrix.x.x);
    send_command_f(GeCommand::BoneMatrixData, matrix.x.y);
    send_command_f(GeCommand::BoneMatrixData, matrix.x.z);

    send_command_f(GeCommand::BoneMatrixData, matrix.y.x);
    send_command_f(GeCommand::BoneMatrixData, matrix.y.y);
    send_command_f(GeCommand::BoneMatrixData, matrix.y.z);

    send_command_f(GeCommand::BoneMatrixData, matrix.z.x);
    send_command_f(GeCommand::BoneMatrixData, matrix.z.y);
    send_command_f(GeCommand::BoneMatrixData, matrix.z.z);

    send_command_f(GeCommand::BoneMatrixData, matrix.w.x);
    send_command_f(GeCommand::BoneMatrixData, matrix.w.y);
    send_command_f(GeCommand::BoneMatrixData, matrix.w.z);
}

/// Specify morph weight entry
///
/// To enable vertex morphing, use `VertexType::VERTICESn`, where n is between
/// 1-8. This will change the amount of vertices passed in the vertex array,
/// and by setting the morph weights for every vertex entry in the array,
/// you can blend between them.
///
/// Please see `VertexType` for vertex format information.
///
/// # Parameters
///
/// - `index`: Morph weight index (0-7)
/// - `weight`: Weight to set
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuMorphWeight(index: i32, weight: f32) {
    let cmd = match index {
        0 => GeCommand::MorphWeight0,
        1 => GeCommand::MorphWeight1,
        2 => GeCommand::MorphWeight2,
        3 => GeCommand::MorphWeight3,
        4 => GeCommand::MorphWeight4,
        5 => GeCommand::MorphWeight5,
        6 => GeCommand::MorphWeight6,
        7 => GeCommand::MorphWeight7,
        _ => core::intrinsics::unreachable(),
    };

    send_command_f(cmd, weight);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDrawArrayN(
    primitive_type: GuPrimitive,
    v_type: VertexType,
    count: i32,
    a3: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !v_type.is_empty() {
        send_command_i(GeCommand::VertexType, v_type.bits());
    }

    if !indices.is_null() {
        send_command_i(GeCommand::Base, ((indices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Iaddr, indices as i32 & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(GeCommand::Base, ((vertices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(GeCommand::Vaddr, vertices as i32 & 0xffffff);
    }

    if a3 > 0 {
        // PSPSDK: TODO: not sure about this loop, might be off. review
        for _ in 1..a3 {
            send_command_i(GeCommand::Prim, ((primitive_type as i32) << 16) | count);
        }

        send_command_i_stall(GeCommand::Prim, ((primitive_type as i32) << 16) | count);
    }
}

static mut CHAR_BUFFER_USED: u32 = 0;
static mut CHAR_BUFFER: [DebugCharStruct; 2048] = [DebugCharStruct {
    x: 0,
    y: 0,
    color: 0,
    character: b'\0',
    unused: [0, 0, 0],
}; 2048];
static FONT: [u8; 768] = *include_bytes!("./debugfont.bin");

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct DebugCharStruct {
    x: i32,
    y: i32,
    color: u32,
    character: u8,
    unused: [u8; 3],
}

/// Add characters to an internal buffer for later printing with sceGuDebugFlush
///
/// # Parameters
///
/// - `x`: Horizontal start position
/// - `y`: Vertical start position
/// - `color`: Text color, ABGR
/// - `msg`: C-style string
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDebugPrint(x: i32, mut y: i32, mut color: u32, mut msg: *const u8) {
    let mut cur_char: u8;
    let mut uVar1: u32;
    let iVar2: i32;
    let mut cur_x: i32;
    let mut char_struct_ptr: *mut DebugCharStruct =
        addr_of_mut!(CHAR_BUFFER).cast::<DebugCharStruct>();

    let mut i = CHAR_BUFFER_USED;
    if i >= 0x3ff {
        return;
    }
    uVar1 = color >> 8 & 0xff;
    let uVar3 = color >> 16 & 0xff;
    let iVar4 = (uVar3 >> 3) as i32;
    cur_x = x;
    match DRAW_BUFFER.pixel_size {
        DisplayPixelFormat::Psm5650 => {
            iVar2 = (uVar1 as i32) >> 2;
            uVar1 = (iVar4 as u32) << 0xb;
            uVar1 = (uVar1 | iVar2 as u32) << 5;
            color = (color & 0xff) >> 3;
            color |= uVar1;
        }
        DisplayPixelFormat::Psm5551 => {
            iVar2 = (uVar1 >> 3) as i32;
            uVar1 = ((color >> 24) >> 7) << 0xf | (iVar4 as u32) << 10;
            uVar1 = (uVar1 | iVar2 as u32) << 5;
            color = (color & 0xff) >> 3;
            color |= uVar1;
        }
        DisplayPixelFormat::Psm8888 => {}
        DisplayPixelFormat::Psm4444 => {
            uVar1 = ((color >> 0x18) >> 4) << 0xc | (uVar3 >> 4) << 8 | (uVar1 >> 4) << 4;
            color &= 0xff >> 4;
            color |= uVar1;
        }
    }
    cur_char = *msg;
    while cur_char != b'\0' {
        if cur_char == b'\n' {
            y += 8;
            cur_x = x;
        } else {
            (*char_struct_ptr).x = cur_x;
            i += 1;
            (*char_struct_ptr).character = cur_char - 0x20;
            (*char_struct_ptr).y = y;
            (*char_struct_ptr).color = color;
            char_struct_ptr = (char_struct_ptr as u32 + 16) as *mut DebugCharStruct;
            cur_x += 8;
        }
        msg = msg.add(1);
        cur_char = *msg;
    }
    CHAR_BUFFER_USED = i;
}

/// Flush character buffer created by sceGuDebugPrint to the draw buffer
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn sceGuDebugFlush() {
    let edram_address = GE_EDRAM_ADDRESS;
    let mut pixel_size: DisplayPixelFormat;
    let mut frame_width: i32;
    let mut frame_buffer: *mut c_void;
    let draw_buffer_height = DRAW_BUFFER.height;
    let mut char_index: i32;
    let mut pos: i32;
    let mut x_pixel_counter: i32;
    let mut glyph_pos: u32 = 0;
    let mut color: u32;
    let mut font_glyph: u32 = 0;
    let mut y_pixel_counter: i32;
    let mut x: i32;
    let mut char_buffer_used = CHAR_BUFFER_USED;
    let mut y: i32;
    let mut char_struct_ptr: *mut DebugCharStruct =
        addr_of_mut!(CHAR_BUFFER).cast::<DebugCharStruct>();

    if char_buffer_used != 0 {
        loop {
            frame_buffer = DRAW_BUFFER.frame_buffer;
            frame_width = DRAW_BUFFER.frame_width;
            pixel_size = DRAW_BUFFER.pixel_size;
            y = (*char_struct_ptr).y;
            x = (*char_struct_ptr).x;
            if (y + 7 < draw_buffer_height)
                && (((x + 7 < DRAW_BUFFER.width) as i32 & !y >> 0x1f) != 0)
                && -1 < x
            {
                color = (*char_struct_ptr).color;
                char_index = ((*char_struct_ptr).character) as i32 * 8;
                y_pixel_counter = 0;
                loop {
                    if y_pixel_counter == 0 {
                        font_glyph =
                            *(((&FONT as *const _ as u32) + char_index as u32) as *const u32);
                        glyph_pos = 1;
                    } else if y_pixel_counter == 4 {
                        font_glyph =
                            *(((&FONT as *const _ as u32) + 4 + char_index as u32) as *const u32);
                        glyph_pos = 1
                    }
                    x_pixel_counter = 7;
                    pos = x + (y + y_pixel_counter) * frame_width;
                    pos = pos * 4 + edram_address as i32 + frame_buffer as i32;
                    loop {
                        match pixel_size {
                            DisplayPixelFormat::Psm8888 => {
                                if font_glyph & glyph_pos != 0 {
                                    *((pos as u32 + 0x4000_0000) as *mut u32) = color;
                                }
                            }
                            _ => {
                                *((pos as u32 + 0x4000_0002) as *mut u16) = color as u16;
                            }
                        }
                        x_pixel_counter -= 1;
                        glyph_pos <<= 1;
                        pos += 4;
                        if x_pixel_counter <= -1 {
                            break;
                        }
                    }
                    y_pixel_counter += 1;
                    if 8 <= y_pixel_counter {
                        break;
                    }
                }
            }
            char_buffer_used -= 1;
            char_struct_ptr = ((char_struct_ptr as u32) + 16) as *mut DebugCharStruct;
            if char_buffer_used == 0 {
                break;
            }
        }
        CHAR_BUFFER_USED = 0;
    }
}
