use core::{mem, ffi::c_void, ptr::null_mut};

use num_enum::TryFromPrimitive;

use crate::sys::{
    ge::{GeContext, GeListArgs, Command, GeListState},
    kernel::SceUid,
    display::DisplayPixelFormat,
};
use crate::sys::types::{FMatrix4, FVector3, IMatrix4, IVector4};

pub const PI: f32 = 3.141593;

/// Primitive types
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Primitive {
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

bitflags::bitflags! {
    /// The vertex type decides how the vertices align and what kind of
    /// information they contain.
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
pub enum WrapMode {
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
/// - `Cc`: Constant color (specified by `sce_gu_tex_env_color`)
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

/// Blending factor for source operand
#[repr(u32)]
pub enum BlendSrc {
    SrcColor = 0,
    OneMinusSrcColor = 1,
    SrcAlpha = 2,
    OneMinusSrcAlpha = 3,

    // TODO: There are likely either 4 or 8 missing values here, as the combined
    // enum between source and destination goes 0, 1, 2, 3, 4, 5, 10. What are
    // 6, 7, 8, 9? This can probably be determined with some experimentation.
    // They may also be reserved values.

    /// Use the fixed value provided as `src_fix` in `sce_gu_blend_func`.
    Fix = 10,
}

/// Blending Factor Destination
#[repr(u32)]
pub enum BlendDst {
    DstColor = 0,
    OneMinusDstColor = 1,
    DstAlpha = 4,
    OneMinusDstAlpha = 5,
    /// Use the fixed value provided as `dst_fix` in `sce_gu_blend_func`.
    Fix = 10,
}

/// Stencil Operations
#[repr(u32)]
pub enum StencilOperation {
    /// Keeps the current value
    Keep = 0,
    /// Sets the stencil buffer value to zero
    Zero = 1,
    /// Sets the stencil buffer value to ref, as specified by `sce_gu_stencil_func`
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
pub enum Context {
    Direct = 0,
    Call = 1,
    Send = 2,
}

/// List Queue Mode
#[repr(u32)]
pub enum QueueMode {
    /// Place list last in the queue, so it executes in-order
    Tail = 0,
    /// Place list first in queue so that it executes as soon as possible
    Head = 1,
}

/// Sync mode
#[repr(u32)]
pub enum SyncMode {
    /// Wait until the last sceGuFinish command is reached.
    Finish = 0,
    /// Wait until the last (?) signal is executed.
    Signal = 1,
    /// Wait until all commands currently in list are executed.
    Done = 2,
    /// Wait for the currently executed display list (`Context::Direct`).
    List = 3,
    /// Wait for the last send list.
    Send = 4,
}

/// Sync Behavior
#[repr(u32)]
pub enum SyncBehavior {
    /// Wait for the GE list to be completed.
    Wait = 0,
    /// Just peek at the current state.
    NoWait = 1,
}

/// GU Callback ID
#[repr(u32)]
pub enum CallbackId {
    /// Called when `sce_gu_signal` is used.
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
    (r as u32)
    | ((g as u32) << 8)
    | ((b as u32) << 16)
    | ((a as u32) << 24)
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

pub type GuCallback = Option<extern fn(id: i32, arg: *mut c_void)>;
pub type GuSwapBuffersCallback = Option<extern fn(display: *mut *mut c_void, render: *mut *mut c_void)>;

struct Settings {
    sig: GuCallback,
    fin: GuCallback,
    signal_history: [i16; 16],
    signal_offset: u32,
    kernel_event_flag: SceUid,
    ge_callback_id: i32,
    swap_buffers_callback: GuSwapBuffersCallback,
    swap_buffers_behaviour: crate::sys::display::DisplaySetBufSync,
}

struct GuDisplayList {
    start: *mut u32,
    current: *mut u32,
    parent_context: Context,
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
    type_: Command,
    /// X position
    xpos: Command,
    /// Y position
    ypos: Command,
    /// Z position
    zpos: Command,
    /// X direction
    xdir: Command,
    /// Y direction
    ydir: Command,
    /// Z direction
    zdir: Command,

    /// Ambient color
    ambient: Command,
    /// Diffuse color
    diffuse: Command,
    /// Specular color
    specular: Command,
    /// Constant attenuation
    constant: Command,
    /// Linear attenuation
    linear: Command,
    /// Quadratic attenuation
    quadratic: Command,
    /// Light exponent
    exponent: Command,
    /// Light cutoff
    cutoff: Command,
}

static mut CURRENT_FRAME: u32 = 0;
static mut CONTEXTS: [GuContext; 3] = [
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: Context::Direct,
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
            parent_context: Context::Direct,
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
            parent_context: Context::Direct,
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
    swap_buffers_behaviour: crate::sys::display::DisplaySetBufSync::Immediate,
    swap_buffers_callback: None,
};

static mut LIST: *mut GuDisplayList = null_mut();
static mut CURR_CONTEXT: Context = Context::Direct;
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
        type_: Command::LightType0,
        xpos: Command::Light0X,
        ypos: Command::Light0Y,
        zpos: Command::Light0Z,
        xdir: Command::Light0DirectionX,
        ydir: Command::Light0DirectionY,
        zdir: Command::Light0DirectionZ,
        ambient: Command::Light0Ambient,
        diffuse: Command::Light0Diffuse,
        specular: Command::Light0Specular,
        constant: Command::Light0ConstantAtten,
        linear: Command::Light0LinearAtten,
        quadratic: Command::Light0QuadtraticAtten,
        exponent: Command::Light0ExponentAtten,
        cutoff: Command::Light0CutoffAtten,
    },
    GuLightSettings {
        type_: Command::LightType1,
        xpos: Command::Light1X,
        ypos: Command::Light1Y,
        zpos: Command::Light1Z,
        xdir: Command::Light1DirectionX,
        ydir: Command::Light1DirectionY,
        zdir: Command::Light1DirectionZ,
        ambient: Command::Light1Ambient,
        diffuse: Command::Light1Diffuse,
        specular: Command::Light1Specular,
        constant: Command::Light1ConstantAtten,
        linear: Command::Light1LinearAtten,
        quadratic: Command::Light1QuadtraticAtten,
        exponent: Command::Light1ExponentAtten,
        cutoff: Command::Light1CutoffAtten,
    },
    GuLightSettings {
        type_: Command::LightType2,
        xpos: Command::Light2X,
        ypos: Command::Light2Y,
        zpos: Command::Light2Z,
        xdir: Command::Light2DirectionX,
        ydir: Command::Light2DirectionY,
        zdir: Command::Light2DirectionZ,
        ambient: Command::Light2Ambient,
        diffuse: Command::Light2Diffuse,
        specular: Command::Light2Specular,
        constant: Command::Light2ConstantAtten,
        linear: Command::Light2LinearAtten,
        quadratic: Command::Light2QuadtraticAtten,
        exponent: Command::Light2ExponentAtten,
        cutoff: Command::Light2CutoffAtten,
    },
    GuLightSettings {
        type_: Command::LightType3,
        xpos: Command::Light3X,
        ypos: Command::Light3Y,
        zpos: Command::Light3Z,
        xdir: Command::Light3DirectionX,
        ydir: Command::Light3DirectionY,
        zdir: Command::Light3DirectionZ,
        ambient: Command::Light3Ambient,
        diffuse: Command::Light3Diffuse,
        specular: Command::Light3Specular,
        constant: Command::Light3ConstantAtten,
        linear: Command::Light3LinearAtten,
        quadratic: Command::Light3QuadtraticAtten,
        exponent: Command::Light3ExponentAtten,
        cutoff: Command::Light3CutoffAtten,
    },
];

#[inline]
unsafe fn send_command_i(cmd: Command, argument: i32) {
    (*(*LIST).current) = ((cmd as u32) << 24) | (argument as u32 & 0xffffff);
    (*LIST).current = (*LIST).current.add(1);
}

#[inline]
unsafe fn send_command_f(cmd: Command, argument: f32) {
    send_command_i(cmd, (core::mem::transmute::<_, u32>(argument) >> 8) as i32);
}

#[inline]
unsafe fn send_command_i_stall(cmd: Command, argument: i32) {
    send_command_i(cmd, argument);
    if let (Context::Direct, 0) = (CURR_CONTEXT, OBJECT_STACK_DEPTH) {
        crate::sys::ge::sce_ge_list_update_stall_addr(
            GE_LIST_EXECUTED[0],
            (*LIST).current as *mut c_void,
        );
    }
}

pub unsafe fn draw_region(x: i32, y: i32, width: i32, height: i32) {
    send_command_i(Command::Region1, (y << 10) | x);
    send_command_i(Command::Region2, (((y + height) - 1) << 10) | ((x + width) - 1));
}

pub unsafe fn reset_values() {
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

    for i in 0..3 {
        let context = &mut CONTEXTS[i];
        context.scissor_enable = 0;
        context.scissor_start = [0, 0];
        context.scissor_end = [0, 0];

        context.near_plane = 0;
        context.far_plane = 1;

        context.depth_offset = 0;
        context.fragment_2x = 0;
        context.texture_function = 0;
        context.texture_proj_map_mode = TextureProjectionMapMode::Position;
        context.texture_map_mode = TextureMapMode::TextureCoords;
        context.sprite_mode[0] = 0;
        context.sprite_mode[1] = 0;
        context.sprite_mode[2] = 0;
        context.sprite_mode[3] = 0;
        context.clear_color = 0;
        context.clear_stencil = 0;
        context.clear_depth = 0xffff;
        context.texture_mode = TexturePixelFormat::Psm5650;
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

        if (*settings).sig != None {
            // Convert Option<fn(i32, *mut c_void)> -> fn(i32)
            // This is fine because we are transmuting a nullable function
            // pointer to another function pointer. The requirement here is that
            // it must not be null.
            let f = mem::transmute::<_, extern "C" fn(i32)>((*settings).sig);

            f(id & 0xffff);
        }

        crate::sys::kernel::sce_kernel_set_event_flag((*settings).kernel_event_flag, 1);
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
pub unsafe fn sce_gu_depth_buffer(zbp: *mut c_void, zbw: i32) {
    DRAW_BUFFER.depth_buffer = zbp;

    if DRAW_BUFFER.depth_width == 0 || DRAW_BUFFER.depth_width != zbw {
        DRAW_BUFFER.depth_width = zbw;
    }

    send_command_i(Command::ZBufPtr, zbp as i32 & 0xffffff);
    send_command_i(Command::ZBufWidth, (((zbp as u32 & 0xff000000) >> 8) | zbw as u32) as i32);
}

/// Set display buffer parameters
///
/// # Parameters
///
/// - `width`: Width of the display buffer in pixels
/// - `height`: Width of the display buffer in pixels
/// - `dispbp`: VRAM pointer to where the display-buffer starts
/// - `dispbw`: Display buffer width (block aligned)
pub unsafe fn sce_gu_disp_buffer(width: i32, height: i32, dispbp: *mut c_void, dispbw: i32) {
    use crate::sys::display::DisplaySetBufSync;

    DRAW_BUFFER.width = width;
    DRAW_BUFFER.height = height;
    DRAW_BUFFER.disp_buffer = dispbp;

    if DRAW_BUFFER.frame_width == 0 || DRAW_BUFFER.frame_width != dispbw {
        DRAW_BUFFER.frame_width = dispbw;
    }

    draw_region(0, 0, DRAW_BUFFER.width, DRAW_BUFFER.height);

    crate::sys::display::sce_display_set_mode(
        crate::sys::display::DisplayMode::Lcd,
        DRAW_BUFFER.width as usize,
        DRAW_BUFFER.height as usize,
    );

    if DISPLAY_ON == true {
        crate::sys::display::sce_display_set_frame_buf(
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
pub unsafe fn sce_gu_draw_buffer(psm: DisplayPixelFormat, fbp: *mut c_void, fbw: i32) {
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

    send_command_i(Command::FramebufPixFormat, psm as i32);
    send_command_i(Command::FrameBufPtr, DRAW_BUFFER.frame_buffer as i32 & 0xffffff);
    send_command_i(
        Command::FrameBufWidth,
        ((DRAW_BUFFER.frame_buffer as u32 & 0xff000000) >> 8) as i32
            | DRAW_BUFFER.frame_width as i32,
    );
    send_command_i(Command::ZBufPtr, DRAW_BUFFER.depth_buffer as i32 & 0xffffff);
    send_command_i(
        Command::ZBufWidth,
        ((DRAW_BUFFER.depth_buffer as u32 & 0xff000000) >> 8) as i32
            | DRAW_BUFFER.depth_width as i32,
    );
}

/// Set draw buffer directly, not storing parameters in the context
///
/// # Parameters
///
/// - `psm`: Pixel format to use for rendering
/// - `fbp`: VRAM pointer to where the draw buffer starts
/// - `fbw`: Frame buffer width (block aligned)
pub unsafe fn sce_gu_draw_buffer_list(psm: DisplayPixelFormat, fbp: *mut c_void, fbw: i32) {
    send_command_i(Command::FramebufPixFormat, psm as i32);
    send_command_i(Command::FrameBufPtr, fbp as i32 & 0xffffff);
    send_command_i(Command::FrameBufWidth, ((fbp as u32 & 0xff000000) >> 8) as i32 | fbw);
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
pub unsafe fn sce_gu_display(state: bool) -> bool {
    use crate::sys::display::DisplaySetBufSync;

    if state {
        crate::sys::display::sce_display_set_frame_buf(
            (GE_EDRAM_ADDRESS as *mut u8).add(DRAW_BUFFER.disp_buffer as usize),
            DRAW_BUFFER.frame_width as usize,
            DRAW_BUFFER.pixel_size,
            DisplaySetBufSync::NextFrame,
        );
    } else {
        crate::sys::display::sce_display_set_frame_buf(
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
pub unsafe fn sce_gu_depth_func(function: DepthFunc) {
    send_command_i(Command::ZTest, function as i32);
}

/// Mask depth buffer writes
///
/// # Parameters
///
/// - `mask`: `1` to disable Z writes, `0` to enable
// TODO: Use bool instead?
pub unsafe fn sce_gu_depth_mask(mask: i32) {
    send_command_i(Command::ZWriteDisable, mask);
}

pub unsafe fn sce_gu_depth_offset(offset: i32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.depth_offset = offset;
    sce_gu_depth_range(context.near_plane, context.far_plane);
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
pub unsafe fn sce_gu_depth_range(mut near: i32, mut far: i32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    let max = near as u32 + far as u32;
    let val = ((max >> 31) + max) as i32;
    let z = (val >> 1) as f32;

    context.near_plane = near;
    context.far_plane = far;

    send_command_f(Command::ViewportZScale, z - near as f32);
    send_command_f(Command::ViewportZCenter, z + context.depth_offset as f32);

    if near > far {
        let temp = near;
        near = far;
        far = temp;
    }

    send_command_i(Command::MinZ, near);
    send_command_i(Command::MaxZ, far);
}

pub unsafe fn sce_gu_fog(near: f32, far: f32, color: u32) {
    let mut distance = far - near;

    if distance != 0.0 {
        distance = 1.0 / distance;
    }

    send_command_i(Command::FogColor, (color & 0xffffff) as i32);
    send_command_f(Command::Fog1, far);
    send_command_f(Command::Fog2, distance);
}

/// Initalize the GU system
///
/// This function MUST be called as the first function, otherwise state is undetermined.
pub unsafe fn sce_gu_init() {
    const INIT_COMMANDS: [Command; 223] = [
        Command::Vaddr,
        Command::Iaddr,
        Command::Base,
        Command::VertexType,
        Command::OffsetAddr,
        Command::Region1,
        Command::Region2,
        Command::LightingEnable,
        Command::LightEnable0,
        Command::LightEnable1,
        Command::LightEnable2,
        Command::LightEnable3,
        Command::DepthClampEnable,
        Command::CullFaceEnable,
        Command::TextureMapEnable,
        Command::FogEnable,
        Command::DitherEnable,
        Command::AlphaBlendEnable,
        Command::AlphaTestEnable,
        Command::ZTestEnable,
        Command::StencilTestEnable,
        Command::AntiAliasEnable,
        Command::PatchCullEnable,
        Command::ColorTestEnable,
        Command::LogicOpEnable,
        Command::BoneMatrixNumber,
        Command::BoneMatrixData,
        Command::MorphWeight0,
        Command::MorphWeight1,
        Command::MorphWeight2,
        Command::MorphWeight3,
        Command::MorphWeight4,
        Command::MorphWeight5,
        Command::MorphWeight6,
        Command::MorphWeight7,
        Command::PatchDivision,
        Command::PatchPrimitive,
        Command::PatchFacing,
        Command::WorldMatrixNumber,
        Command::WorldMatrixData,
        Command::ViewMatrixNumber,
        Command::ViewMatrixData,
        Command::ProjMatrixNumber,
        Command::ProjMatrixData,
        Command::TGenMatrixNumber,
        Command::TGenMatrixData,
        Command::ViewportXScale,
        Command::ViewportYScale,
        Command::ViewportZScale,
        Command::ViewportXCenter,
        Command::ViewportYCenter,
        Command::ViewportZCenter,
        Command::TexScaleU,
        Command::TexScaleV,
        Command::TexOffsetU,
        Command::TexOffsetV,
        Command::OffsetX,
        Command::OffsetY,
        Command::ShadeMode,
        Command::ReverseNormal,
        Command::MaterialUpdate,
        Command::MaterialEmissive,
        Command::MaterialAmbient,
        Command::MaterialDiffuse,
        Command::MaterialSpecular,
        Command::MaterialAlpha,
        Command::MaterialSpecularCoef,
        Command::AmbientColor,
        Command::AmbientAlpha,
        Command::LightMode,
        Command::LightType0,
        Command::LightType1,
        Command::LightType2,
        Command::LightType3,
        Command::Light0X,
        Command::Light0Y,
        Command::Light0Z,
        Command::Light1X,
        Command::Light1Y,
        Command::Light1Z,
        Command::Light2X,
        Command::Light2Y,
        Command::Light2Z,
        Command::Light3X,
        Command::Light3Y,
        Command::Light3Z,
        Command::Light0DirectionX,
        Command::Light0DirectionY,
        Command::Light0DirectionZ,
        Command::Light1DirectionX,
        Command::Light1DirectionY,
        Command::Light1DirectionZ,
        Command::Light2DirectionX,
        Command::Light2DirectionY,
        Command::Light2DirectionZ,
        Command::Light3DirectionX,
        Command::Light3DirectionY,
        Command::Light3DirectionZ,
        Command::Light0ConstantAtten,
        Command::Light0LinearAtten,
        Command::Light0QuadtraticAtten,
        Command::Light1ConstantAtten,
        Command::Light1LinearAtten,
        Command::Light1QuadtraticAtten,
        Command::Light2ConstantAtten,
        Command::Light2LinearAtten,
        Command::Light2QuadtraticAtten,
        Command::Light3ConstantAtten,
        Command::Light3LinearAtten,
        Command::Light3QuadtraticAtten,
        Command::Light0ExponentAtten,
        Command::Light1ExponentAtten,
        Command::Light2ExponentAtten,
        Command::Light3ExponentAtten,
        Command::Light0CutoffAtten,
        Command::Light1CutoffAtten,
        Command::Light2CutoffAtten,
        Command::Light3CutoffAtten,
        Command::Light0Ambient,
        Command::Light0Diffuse,
        Command::Light0Specular,
        Command::Light1Ambient,
        Command::Light1Diffuse,
        Command::Light1Specular,
        Command::Light2Ambient,
        Command::Light2Diffuse,
        Command::Light2Specular,
        Command::Light3Ambient,
        Command::Light3Diffuse,
        Command::Light3Specular,
        Command::Cull,
        Command::FrameBufPtr,
        Command::FrameBufWidth,
        Command::ZBufPtr,
        Command::ZBufWidth,
        Command::TexAddr0,
        Command::TexAddr1,
        Command::TexAddr2,
        Command::TexAddr3,
        Command::TexAddr4,
        Command::TexAddr5,
        Command::TexAddr6,
        Command::TexAddr7,
        Command::TexBufWidth0,
        Command::TexBufWidth1,
        Command::TexBufWidth2,
        Command::TexBufWidth3,
        Command::TexBufWidth4,
        Command::TexBufWidth5,
        Command::TexBufWidth6,
        Command::TexBufWidth7,
        Command::ClutAddr,
        Command::ClutAddrUpper,
        Command::TransferSrc,
        Command::TransferSrcW,
        Command::TransferDst,
        Command::TransferDstW,
        Command::TexSize0,
        Command::TexSize1,
        Command::TexSize2,
        Command::TexSize3,
        Command::TexSize4,
        Command::TexSize5,
        Command::TexSize6,
        Command::TexSize7,
        Command::TexMapMode,
        Command::TexShadeLs,
        Command::TexMode,
        Command::TexFormat,
        Command::LoadClut,
        Command::ClutFormat,
        Command::TexFilter,
        Command::TexWrap,
        Command::TexLevel,
        Command::TexFunc,
        Command::TexEnvColor,
        Command::TexFlush,
        Command::TexSync,
        Command::Fog1,
        Command::Fog2,
        Command::FogColor,
        Command::TexLodSlope,
        Command::FramebufPixFormat,
        Command::ClearMode,
        Command::Scissor1,
        Command::Scissor2,
        Command::MinZ,
        Command::MaxZ,
        Command::ColorTest,
        Command::ColorRef,
        Command::ColorTestmask,
        Command::AlphaTest,
        Command::StencilTest,
        Command::StencilOp,
        Command::ZTest,
        Command::BlendMode,
        Command::BlendFixedA,
        Command::BlendFixedB,
        Command::Dith0,
        Command::Dith1,
        Command::Dith2,
        Command::Dith3,
        Command::LogicOp,
        Command::ZWriteDisable,
        Command::MaskRgb,
        Command::MaskAlpha,
        Command::TransferSrcPos,
        Command::TransferDstPos,
        Command::TransferSize,
        Command::Vscx,
        Command::Vscy,
        Command::Vscz,
        Command::Vtcs,
        Command::Vtct,
        Command::Vtcq,
        Command::Vcv,
        Command::Vap,
        Command::Vfc,
        Command::Vscv,
        Command::Finish,
        Command::End,
        Command::Nop,
        Command::Nop,
    ];

    static mut INIT_LIST: crate::Align16<[u32; 223]> = crate::Align16({
        let mut out = [0; 223];

        let mut i = 0;
        while i < 223 {
            out[i] = (INIT_COMMANDS[i] as u32) << 24;
            i += 1;
        }

        out
    });

    let mut callback = crate::sys::ge::GeCallbackData {
        signal_func: Some(callback_sig),
        signal_arg: &mut SETTINGS as *mut _ as *mut c_void,
        finish_func: Some(callback_fin),
        finish_arg: &mut SETTINGS as *mut _ as *mut c_void,
    };

    SETTINGS.ge_callback_id = crate::sys::ge::sce_ge_set_callback(&mut callback);
    SETTINGS.swap_buffers_callback = None;
    SETTINGS.swap_buffers_behaviour = super::display::DisplaySetBufSync::Immediate;

    GE_EDRAM_ADDRESS = super::ge::sce_ge_edram_get_addr() as *mut c_void;

    GE_LIST_EXECUTED[0] = super::ge::sce_ge_list_enqueue(
        (
            &mut INIT_LIST as *mut crate::Align16<[u32;223]> as u32 & 0x1fffffff
        ) as *mut c_void,
        core::ptr::null_mut(),
        SETTINGS.ge_callback_id as i32,
        core::ptr::null_mut()
    );

    reset_values();

    SETTINGS.kernel_event_flag = super::kernel::sce_kernel_create_event_flag(
        b"SceGuSignal\0" as *const u8,
        super::kernel::EventFlagAttributes::WAIT_MULTIPLE,
        3,
        null_mut()
    );

    super::ge::sce_ge_list_sync(GE_LIST_EXECUTED[0], 0);
}

/// Shutdown the GU system
///
/// Called when GU is no longer needed
pub unsafe fn sce_gu_term() {
    use crate::sys::{ge, kernel};

    kernel::sce_kernel_delete_event_flag(SETTINGS.kernel_event_flag);
    ge::sce_ge_unset_callback(SETTINGS.ge_callback_id);
}

pub unsafe fn sce_gu_break(_a0: i32) {
    // This is actually unimplemented in PSPSDK

    // FIXME
    //sceGeBreak(a0,0x527a68);
    unimplemented!()
}

pub unsafe fn sce_gu_continue() {
    // This is actually unimplemented in PSPSDK

    // FIXME
    //sceGeContinue();
    unimplemented!()
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
pub unsafe fn sce_gu_set_callback(
    signal: CallbackId,
    callback: GuCallback,
) -> GuCallback {
    let old_callback;

    match signal {
        CallbackId::Signal => {
            old_callback = SETTINGS.sig;
            SETTINGS.sig = callback;
        }

        CallbackId::Finish => {
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
pub unsafe fn sce_gu_signal(behavior: SignalBehavior, signal: i32) {
    send_command_i(Command::Signal, ((signal & 0xff) << 16) | (behavior as i32 & 0xffff));
    send_command_i(Command::End, 0);

    if signal == 3 {
        send_command_i(Command::Finish, 0);
        send_command_i(Command::End, 0);
    }

    send_command_i_stall(Command::Nop, 0);
}

/// Send raw float command to the GE
///
/// The argument is converted into a 24-bit float before transfer.
///
/// # Parameters
///
/// - `cmd`: Which command to send
/// - `argument`: Argument to pass along
pub unsafe fn sce_gu_send_command_f(cmd: Command, argument: f32) {
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
pub unsafe fn sce_gu_send_command_i(cmd: Command, argument: i32) {
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
pub unsafe fn sce_gu_get_memory(mut size: i32) -> *mut c_void {
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

    if let Context::Direct = CURR_CONTEXT {
        crate::sys::ge::sce_ge_list_update_stall_addr(GE_LIST_EXECUTED[0], new_ptr as *mut _);
    }

    orig_ptr.add(2) as *mut _
}

/// Start filling a new display-context
///
/// The previous context-type is stored so that it can be restored at `sce_gu_finish`.
///
/// # Parameters
///
/// - `cid`: Context Type
/// - `list`: Pointer to display-list (16 byte aligned)
pub unsafe fn sce_gu_start(context_type: Context, list: *mut c_void) {
    let mut context = &mut CONTEXTS[context_type as usize];
    let local_list = ((list as u32) | 0x4000_0000) as *mut u32;

    // setup display list
    context.list.start = local_list;
    context.list.current = local_list;
    context.list.parent_context = CURR_CONTEXT;
    LIST = &mut context.list;

    // store current context
    CURR_CONTEXT = context_type;

    if let Context::Direct = context_type {
        GE_LIST_EXECUTED[0] = crate::sys::ge::sce_ge_list_enqueue(
            local_list as *mut c_void,
            local_list as *mut c_void,
            SETTINGS.ge_callback_id as i32,
            core::ptr::null_mut(),
        );

        SETTINGS.signal_offset = 0;
    }

    if INIT == 0 {
        static DITHER_MATRIX: IMatrix4 = IMatrix4 {
            x: IVector4 { x: -4, y:  0, z: -3, w:  1, },
            y: IVector4 { x:  2, y: -2, z:  3, w: -1, },
            z: IVector4 { x: -3, y:  1, z: -4, w:  0, },
            w: IVector4 { x:  3, y: -1, z:  2, w: -2, },
        };

        sce_gu_set_dither(&DITHER_MATRIX);
        sce_gu_patch_divide(16, 16);
        sce_gu_color_material(
            LightComponent::AMBIENT | LightComponent::DIFFUSE | LightComponent::SPECULAR,
        );

        sce_gu_specular(1.0);
        sce_gu_tex_scale(1.0, 1.0);

        INIT = 1;
    }

    if let Context::Direct = CURR_CONTEXT {
        if DRAW_BUFFER.frame_width != 0 {
            send_command_i(Command::FrameBufPtr, DRAW_BUFFER.frame_buffer as i32 & 0xffffff);
            send_command_i(
                Command::FrameBufWidth,
                ((DRAW_BUFFER.frame_buffer as u32 & 0xff00_0000) >> 8) as i32
                    | (DRAW_BUFFER.frame_width as i32),
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
pub unsafe fn sce_gu_finish() -> i32 {
    match CURR_CONTEXT {
        Context::Direct | Context::Send => {
            send_command_i(Command::Finish, 0);
            send_command_i_stall(Command::End, 0);
        }

        Context::Call => {
            if CALL_MODE == 1 {
                send_command_i(Command::Signal, 0x120000);
                send_command_i(Command::End, 0);
                send_command_i_stall(Command::Nop, 0);
            } else {
                send_command_i(Command::Ret, 0);
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
pub unsafe fn sce_gu_finish_id(id: u32) -> i32 {
    match CURR_CONTEXT {
        Context::Direct | Context::Send => {
            send_command_i(Command::Finish, (id & 0xffff) as i32);
            send_command_i_stall(Command::End, 0);
        }

        Context::Call => {
            if CALL_MODE == 1 {
                send_command_i(Command::Signal, 0x120000);
                send_command_i(Command::End, 0);
                send_command_i_stall(Command::Nop, 0);
            } else {
                send_command_i(Command::Ret, 0);
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
pub unsafe fn sce_gu_call_list(list: *const c_void) {
    let list_addr = list as u32;

    if CALL_MODE == 1 {
        send_command_i(Command::Signal, (list_addr >> 16) as i32 | 0x110000);
        send_command_i(Command::End, list_addr as i32 & 0xffff);
        send_command_i_stall(Command::Nop, 0);
    } else {
        send_command_i(Command::Base, (list_addr >> 8) as i32 & 0xf0000);
        send_command_i_stall(Command::Call, list_addr as i32 & 0xffffff);
    }
}

/// Set whether to use stack-based calls or signals to handle execution of
/// called lists.
///
/// # Parameters
///
/// - `mode`: True (1) to enable signals, false (0) to disable signals and use
///   normal calls instead.
pub unsafe fn sce_gu_call_mode(mode: i32) {
    CALL_MODE = mode;
}

/// Check how large the current display list is
///
/// # Return Value
///
/// The size of the current display list
pub unsafe fn sce_gu_check_list() -> i32 {
    (*LIST).current.sub((*LIST).start as usize) as i32
}

/// Send a list to the GE directly
///
/// # Parameters
///
/// - `mode`: Whether to place the list first or last in queue
/// - `list`: List to send
/// - `context`: Temporary storage for the GE context
pub unsafe fn sce_gu_send_list(mode: QueueMode, list: *const c_void, context: *mut GeContext) {
    SETTINGS.signal_offset = 0;

    let mut args = GeListArgs::default();
    args.size = 8;
    args.context = context;

    let callback = SETTINGS.ge_callback_id;

    let list_id = match mode {
        QueueMode::Head => {
            crate::sys::ge::sce_ge_list_enqueue_head(
                list,
                null_mut(),
                callback as i32,
                &mut args,
            )
        }

        QueueMode::Tail => {
            crate::sys::ge::sce_ge_list_enqueue(list, null_mut(), callback as i32, &mut args)
        }
    };

    GE_LIST_EXECUTED[1] = list_id;
}

/// Swap display and draw buffer
///
/// # Return Value
///
/// Pointer to the new drawbuffer
pub unsafe fn sce_gu_swap_buffers() -> *mut c_void {
    if let Some(cb) = SETTINGS.swap_buffers_callback {
        cb(
            &mut DRAW_BUFFER.disp_buffer as *mut _,
            &mut DRAW_BUFFER.frame_buffer as *mut _,
        );
    } else {
        mem::swap(&mut DRAW_BUFFER.disp_buffer, &mut DRAW_BUFFER.frame_buffer);
    }

    if DISPLAY_ON {
        crate::sys::display::sce_display_set_frame_buf(
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
/// - `mode`: What to wait for, one of `SyncMode`
/// - `behavior`: How to sync, one of `SyncBehavior`
///
/// # Return Value
///
/// Unknown at this time. GeListState?
pub unsafe fn sce_gu_sync(mode: SyncMode, behavior: SyncBehavior) -> GeListState {
    match mode {
        SyncMode::Finish => crate::sys::ge::sce_ge_draw_sync(behavior as i32),
        SyncMode::List => crate::sys::ge::sce_ge_list_sync(GE_LIST_EXECUTED[0], behavior as i32),
        SyncMode::Send => crate::sys::ge::sce_ge_list_sync(GE_LIST_EXECUTED[1], behavior as i32),
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
pub unsafe fn sce_gu_draw_array(
    prim: Primitive,
    vtype: VertexType,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !vtype.is_empty() {
        send_command_i(Command::VertexType, vtype.bits());
    }

    if !indices.is_null() {
        send_command_i(Command::Base, (indices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(Command::Iaddr, indices as i32 & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(Command::Base, (vertices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(Command::Vaddr, vertices as i32 & 0xffffff);
    }

    send_command_i_stall(Command::Prim, ((prim as i32) << 16) | count);
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
pub unsafe fn sce_gu_begin_object(
    vtype: i32,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if vtype != 0 {
        send_command_i(Command::VertexType, vtype);
    }

    if !indices.is_null() {
        send_command_i(Command::Base, (indices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(Command::Iaddr, indices as i32 & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(Command::Base, (vertices as u32 >> 8) as i32 & 0xf0000);
        send_command_i(Command::Vaddr, vertices as i32 & 0xffffff);
    }

    send_command_i(Command::BoundingBox, count);

    // Store start to new object
    (*OBJECT_STACK.offset(OBJECT_STACK_DEPTH as isize)) = (*LIST).current;
    OBJECT_STACK_DEPTH += 1;

    // Dummy commands, overwritten in `sce_gu_end_object`
    send_command_i(Command::Base, 0);
    send_command_i(Command::BJump, 0);
}

/// End conditional rendering of object
pub unsafe fn sce_gu_end_object() {
    // Rewrite commands from `sce_gu_begin_object`

    let current = (*LIST).current;
    (*LIST).current = *OBJECT_STACK.offset(OBJECT_STACK_DEPTH as isize - 1);

    send_command_i(Command::Base, (current as u32 >> 8) as i32 & 0xf0000);
    send_command_i(Command::BJump, current as i32 & 0xffffff);
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
pub unsafe fn sce_gu_set_status(state: State, status: i32) {
    if status != 0 {
        sce_gu_enable(state);
    } else {
        sce_gu_disable(state);
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
pub unsafe fn sce_gu_get_status(state: State) -> bool {
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
pub unsafe fn sce_gu_set_all_status(status: i32) {
    for i in 0..22 {
        if (status >> i) & 1 != 0 {
            sce_gu_enable(mem::transmute(i));
        } else {
            sce_gu_disable(mem::transmute(i));
        }
    }
}

/// Query status on all 22 available states
///
/// # Return Value
///
/// Status of all 22 states as a bitmask (0-21)
pub unsafe fn sce_gu_get_all_status() -> i32 {
    STATES as i32
}

/// Enable GE state
///
/// # Parameters
///
/// - `state`: Which state to enable, one of `State`
pub unsafe fn sce_gu_enable(state: State) {
    match state {
        State::AlphaTest => send_command_i(Command::AlphaTestEnable, 1),
        State::DepthTest => send_command_i(Command::ZTestEnable, 1),
        State::ScissorTest => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.scissor_enable = 1;
            send_command_i(
                Command::Scissor1,
                (context.scissor_start[1] << 10) | context.scissor_start[0],
            );
            send_command_i(Command::Scissor2, (context.scissor_end[1] << 10) | context.scissor_end[0]);
        }
        State::StencilTest => send_command_i(Command::StencilTestEnable, 1),
        State::Blend => send_command_i(Command::AlphaBlendEnable, 1),
        State::CullFace => send_command_i(Command::CullFaceEnable, 1),
        State::Dither => send_command_i(Command::DitherEnable, 1),
        State::Fog => send_command_i(Command::FogEnable, 1),
        State::ClipPlanes => send_command_i(Command::DepthClampEnable, 1),
        State::Texture2D => send_command_i(Command::TextureMapEnable, 1),
        State::Lighting => send_command_i(Command::LightingEnable, 1),
        State::Light0 => send_command_i(Command::LightEnable0, 1),
        State::Light1 => send_command_i(Command::LightEnable1, 1),
        State::Light2 => send_command_i(Command::LightEnable2, 1),
        State::Light3 => send_command_i(Command::LightEnable3, 1),
        State::LineSmooth => send_command_i(Command::AntiAliasEnable, 1),
        State::PatchCullFace => send_command_i(Command::PatchCullEnable, 1),
        State::ColorTest => send_command_i(Command::ColorTestEnable, 1),
        State::ColorLogicOp => send_command_i(Command::LogicOpEnable, 1),
        State::FaceNormalReverse => send_command_i(Command::ReverseNormal, 1),
        State::PatchFace => send_command_i(Command::PatchFacing, 1),
        State::Fragment2X => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.fragment_2x = 0x10000;
            send_command_i(Command::TexFunc, 0x10000 | context.texture_function);
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
/// - `state`: Which state to disable, one of `State`
pub unsafe fn sce_gu_disable(state: State) {
    match state {
        State::AlphaTest => send_command_i(Command::AlphaTestEnable, 0),
        State::DepthTest => send_command_i(Command::ZTestEnable, 0),
        State::ScissorTest => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.scissor_enable = 0;
            send_command_i(Command::Scissor1, 0);
            send_command_i(
                Command::Scissor2,
                ((DRAW_BUFFER.height - 1) << 10) | DRAW_BUFFER.width - 1,
            );
        }
        State::StencilTest => send_command_i(Command::StencilTestEnable, 0),
        State::Blend => send_command_i(Command::AlphaBlendEnable, 0),
        State::CullFace => send_command_i(Command::CullFaceEnable, 0),
        State::Dither => send_command_i(Command::DitherEnable, 0),
        State::Fog => send_command_i(Command::FogEnable, 0),
        State::ClipPlanes => send_command_i(Command::DepthClampEnable, 0),
        State::Texture2D => send_command_i(Command::TextureMapEnable, 0),
        State::Lighting => send_command_i(Command::LightingEnable, 0),
        State::Light0 => send_command_i(Command::LightEnable0, 0),
        State::Light1 => send_command_i(Command::LightEnable1, 0),
        State::Light2 => send_command_i(Command::LightEnable2, 0),
        State::Light3 => send_command_i(Command::LightEnable3, 0),
        State::LineSmooth => send_command_i(Command::AntiAliasEnable, 0),
        State::PatchCullFace => send_command_i(Command::PatchCullEnable, 0),
        State::ColorTest => send_command_i(Command::ColorTestEnable, 0),
        State::ColorLogicOp => send_command_i(Command::LogicOpEnable, 0),
        State::FaceNormalReverse => send_command_i(Command::ReverseNormal, 0),
        State::PatchFace => send_command_i(Command::PatchFacing, 0),
        State::Fragment2X => {
            let context = &mut CONTEXTS[CURR_CONTEXT as usize];
            context.fragment_2x = 0;
            send_command_i(Command::TexFunc, context.texture_function);
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
pub unsafe fn sce_gu_light(
    light: i32,
    type_: LightType,
    components: LightComponent,
    position: &FVector3,
) {
    let settings = &LIGHT_COMMANDS[light as usize];

    send_command_f(settings.xpos, position.x);
    send_command_f(settings.ypos, position.y);
    send_command_f(settings.zpos, position.z);

    let mut kind = 2;
    if components.bits() != 8 {
        kind = if components.bits() ^ 6 < 1 { 1 } else { 0 };
    }

    send_command_i(
        settings.type_,
        ((type_ as i32 & 0x03) << 8) | kind,
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
pub unsafe fn sce_gu_light_color(light: i32, component: LightComponent, color: u32) {
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
pub unsafe fn sce_gu_light_mode(mode: LightMode) {
    send_command_i(Command::LightMode, mode as i32);
}

/// Set spotlight parameters
///
/// # Parameters
///
/// - `light`: Light index
/// - `direction`: Spotlight direction
/// - `exponent`: Spotlight exponent
/// - `cutoff`: Spotlight cutoff angle (in radians)
pub unsafe fn sce_gu_light_spot(light: i32, direction: &FVector3, exponent: f32, cutoff: f32) {
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
pub unsafe fn sce_gu_clear(flags: ClearBuffer) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    let filter: u32;

    struct Vertex {
        color: u32,
        x: u16,
        y: u16,
        z: u16,
        _pad: u16,
    }

    match DRAW_BUFFER.pixel_size {
        DisplayPixelFormat::Psm5650 => filter = context.clear_color & 0xffffff,
        DisplayPixelFormat::Psm5551 => {
            filter = (context.clear_color & 0xffffff) | (context.clear_stencil << 31);
        }
        DisplayPixelFormat::Psm4444 => {
            filter = (context.clear_color & 0xffffff) | (context.clear_stencil << 28);
        }
        DisplayPixelFormat::Psm8888 => {
            filter = (context.clear_color & 0xffffff) | (context.clear_stencil << 24);
        }
    }

    let vertices;
    let count;

    if !flags.intersects(ClearBuffer::FAST_CLEAR_BIT) {
        vertices = sce_gu_get_memory(2 * mem::size_of::<Vertex>() as i32) as *mut Vertex;
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
        vertices = sce_gu_get_memory(count * core::mem::size_of::<Vertex>() as i32) as *mut Vertex;

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
        let relevant_flags = flags & (
            ClearBuffer::COLOR_BUFFER_BIT
            | ClearBuffer::STENCIL_BUFFER_BIT
            | ClearBuffer::DEPTH_BUFFER_BIT
        );

        send_command_i(
            Command::ClearMode,
            (relevant_flags.bits() << 8) as i32 | 0x01,
        );
    }

    sce_gu_draw_array(
        Primitive::Sprites,
        VertexType::COLOR_8888 | VertexType::VERTEX_16BIT | VertexType::TRANSFORM_2D,
        count,
        null_mut(),
        vertices as *mut c_void,
    );

    send_command_i(Command::ClearMode, 0);
}

/// Set the current clear-color
///
/// # Parameters
///
/// - `color`: Color to clear with
pub unsafe fn sce_gu_clear_color(color: u32) {
    CONTEXTS[CURR_CONTEXT as usize].clear_color = color;
}

/// Set the current clear-depth
///
/// # Parameters
///
/// - `depth`: Set which depth to clear with (0x0000-0xffff)
// TODO: Can `depth` be u16 or does this cause issues with FFI ABI compatibility?
pub unsafe fn sce_gu_clear_depth(depth: u32) {
    CONTEXTS[CURR_CONTEXT as usize].clear_depth = depth;
}

/// Set the current stencil clear value
///
/// # Parameters
///
/// - `stencil`: Set which stencil value to clear with (0-255)
// TODO: Can `stencil` be u8 or does this cause issues with FFI ABI compatibility?
pub unsafe fn sce_gu_clear_stencil(stencil: u32) {
    CONTEXTS[CURR_CONTEXT as usize].clear_stencil = stencil;
}

/// Set mask for which bits of the pixels to write
///
/// # Parameters
///
/// - `mask`: Which bits to filter against writes
pub unsafe fn sce_gu_pixel_mask(mask: u32) {
    send_command_i(Command::MaskRgb, mask as i32 & 0xffffff);
    send_command_i(Command::MaskAlpha, (mask >> 24) as i32);
}

/// Set current primitive color
///
/// # Parameters
///
/// - `color`: Which color to use (overridden by vertex colors)
pub unsafe fn sce_gu_color(color: u32) {
    sce_gu_material(
        LightComponent::AMBIENT | LightComponent::DIFFUSE | LightComponent::SPECULAR,
        color,
    );
}

/// Set the color test function
///
/// The color test is only performed while `State::ColorTest` is enabled, e.g.
/// via `sce_gu_enable`.
///
/// # Parameters
///
/// - `func`: Color test function
/// - `color`: Color to test against
/// - `mask`: Mask ANDed against both source and destination when testing
pub unsafe fn sce_gu_color_func(func: ColorFunc, color: u32, mask: u32) {
    send_command_i(Command::ColorTest, func as i32 & 0x03);
    send_command_i(Command::ColorRef, color as i32 & 0xffffff);
    send_command_i(Command::ColorTestmask, mask as i32);
}

/// Set which color components the material will receive
///
/// # Parameters
///
/// - `components`: Which component(s) to receive
pub unsafe fn sce_gu_color_material(components: LightComponent) {
    send_command_i(Command::MaterialUpdate, components.bits() as i32);
}

/// Set the alpha test parameters
///
/// # Parameters
///
/// - `func`: Specifies the alpha comparison function.
/// - `value`: Specifies the reference value that incoming alpha values are compared to.
/// - `mask`: Specifies the mask that both values are ANDed with before comparison.
pub unsafe fn sce_gu_alpha_func(func: AlphaFunc, value: i32, mask: i32) {
    let arg = func as i32 | ((value & 0xff) << 8) | ((mask & 0xff) << 16);
    send_command_i(Command::AlphaTest, arg);
}

pub unsafe fn sce_gu_ambient(color: u32) {
    send_command_i(Command::AmbientColor, color as i32 & 0xffffff);
    send_command_i(Command::AmbientAlpha, (color >> 24) as i32);
}

pub unsafe fn sce_gu_ambient_color(color: u32) {
    send_command_i(Command::MaterialAmbient, color as i32 & 0xffffff);
    send_command_i(Command::MaterialAlpha, (color >> 24) as i32);
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
/// - `srcfix`: Fixed value for `BlendSrc::Fix` (source operand)
/// - `destfix`: Fixed value for `BlendDst::Fix` (dest operand)
pub unsafe fn sce_gu_blend_func(
    op: BlendOp,
    src: BlendSrc,
    dest: BlendDst,
    src_fix: u32,
    dest_fix: u32,
) {
    send_command_i(
        Command::BlendMode,
        src as i32 | ((dest as i32) << 4) | ((op as i32) << 8),
    );
    send_command_i(Command::BlendFixedA, src_fix as i32 & 0xffffff);
    send_command_i(Command::BlendFixedB, dest_fix as i32 & 0xffffff);
}

/// Set current primitive color, for specific light components.
///
/// # Parameters
///
/// - `components`: Which component(s) to set
/// - `color`: Color to set (*likely* overridden by vertex colors)
pub unsafe fn sce_gu_material(components: LightComponent, color: u32) {
    if components.intersects(LightComponent::AMBIENT) {
        send_command_i(Command::MaterialAmbient, color as i32 & 0xffffff);
        send_command_i(Command::MaterialAlpha, (color >> 24) as i32);
    }

    if components.intersects(LightComponent::DIFFUSE) {
        send_command_i(Command::MaterialDiffuse, color as i32 & 0xffffff);
    }

    if components.intersects(LightComponent::SPECULAR) {
        send_command_i(Command::MaterialSpecular, color as i32 & 0xffffff);
    }
}

// TODO: Needs documentation.
pub unsafe fn sce_gu_model_color(emissive: u32, ambient: u32, diffuse: u32, specular: u32) {
    send_command_i(Command::MaterialEmissive, emissive as i32 & 0xffffff);
    send_command_i(Command::MaterialAmbient, ambient as i32 & 0xffffff);
    send_command_i(Command::MaterialDiffuse, diffuse as i32 & 0xffffff);
    send_command_i(Command::MaterialSpecular, specular as i32 & 0xffffff);
}

/// Set stencil function and reference value for stencil testing
///
/// # Parameters
///
/// - `func`: Test function
/// - `ref_`: The reference value for the stencil test
/// - `mask`: Mask that is ANDed with both the reference value and stored
///   stencil value when the test is done
pub unsafe fn sce_gu_stencil_func(func: StencilFunc, ref_: i32, mask: i32) {
    send_command_i(
        Command::StencilTest,
        func as i32 | ((ref_ as i32 & 0xff) << 8) | ((mask as i32 & 0xff) << 16),
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
pub unsafe fn sce_gu_stencil_op(
    fail: StencilOperation,
    zfail: StencilOperation,
    zpass: StencilOperation,
) {
    send_command_i(
        Command::StencilOp,
        fail as i32 | ((zfail as i32) << 8) | ((zpass as i32) << 16),
    );
}

/// Set the specular power for the material
///
/// # Parameters
///
/// - `power`: Specular power
pub unsafe fn sce_gu_specular(power: f32) {
    send_command_f(Command::MaterialSpecularCoef, power);
}

/// Set the current face-order (for culling)
///
/// This only has effect when culling (`State::CullFace`) is enabled, e.g. via
/// `sce_gu_enable`.
///
/// # Parameters
///
/// - `order`: Which order to use, one of `FrontFaceDirection`
pub unsafe fn sce_gu_front_face(order: FrontFaceDirection) {
    match order {
        FrontFaceDirection::CounterClockwise => send_command_i(Command::Cull, 0),
        FrontFaceDirection::Clockwise => send_command_i(Command::Cull, 1),
    }
}

/// Set color logical operation
///
/// This operation only has effect if `State::ColorLogicOp` is enabled, e.g. via
/// `sce_gu_enable`.
///
/// # Parameters
///
/// - `op`: Operation to execute
pub unsafe fn sce_gu_logical_op(op: LogicalOperation) {
    send_command_i(Command::LogicOp, op as i32 & 0x0f);
}

/// Set ordered pixel dither matrix
///
/// This dither matrix is only applied if `State::Dither` is enabled, e.g. via
/// `sce_gu_enable`.
///
/// # Parameters
///
/// - `matrix`: Dither matrix
pub unsafe fn sce_gu_set_dither(matrix: &IMatrix4) {
    send_command_i(
        Command::Dith0,
        (matrix.x.x & 0x0f)
        | ((matrix.x.y & 0x0f) << 4)
        | ((matrix.x.z & 0x0f) << 8)
        | ((matrix.x.w & 0x0f) << 12),
    );

    send_command_i(
        Command::Dith1,
        (matrix.y.x & 0x0f)
        | ((matrix.y.y & 0x0f) << 4)
        | ((matrix.y.z & 0x0f) << 8)
        | ((matrix.y.w & 0x0f) << 12),
    );

    send_command_i(
        Command::Dith2,
        (matrix.z.x & 0x0f)
        | ((matrix.z.y & 0x0f) << 4)
        | ((matrix.z.z & 0x0f) << 8)
        | ((matrix.z.w & 0x0f) << 12),
    );

    send_command_i(
        Command::Dith3,
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
pub unsafe fn sce_gu_shade_model(mode: ShadingModel) {
    match mode {
        ShadingModel::Smooth => send_command_i(Command::ShadeMode, 1),
        ShadingModel::Flat => send_command_i(Command::ShadeMode, 0),
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
pub unsafe fn sce_gu_copy_image(
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
    send_command_i(Command::TransferSrc, (src as i32) & 0xffffff);
    send_command_i(
        Command::TransferSrcW,
        (((src as u32) & 0xff000000) >> 8) as i32 | srcw,
    );
    send_command_i(Command::TransferSrcPos, (sy << 10) | sx);
    send_command_i(Command::TransferDst, (dest as i32) & 0xffffff);
    send_command_i(
        Command::TransferDstW,
        (((dest as u32) & 0xff000000) >> 8) as i32 | destw,
    );
    send_command_i(Command::TransferDstPos, (dy << 10) | dx);
    send_command_i(
        Command::TransferSize,
        ((height as i32 - 1) << 10) | (width - 1),
    );

    let is_32_bit_texel = if let DisplayPixelFormat::Psm8888 = psm {
        1
    } else {
        0
    };

    send_command_i(Command::TransferStart, is_32_bit_texel);
}

/// Specify the texture environment color
///
/// This is used in the texture function when a constant color is needed.
///
/// See `sce_gu_tex_func` for more information.
///
/// # Parameters
///
/// - `color`: Constant color (0x00BBGGRR)
pub unsafe fn sce_gu_tex_env_color(color: u32) {
    send_command_i(Command::TexEnvColor, color as i32 & 0xffffff);
}

/// Set how the texture is filtered
///
/// # Parameters
///
/// - `min`: Minimizing filter
/// - `mag`: Magnifying filter
pub unsafe fn sce_gu_tex_filter(min: TextureFilter, mag: TextureFilter) {
    send_command_i(Command::TexFilter, ((mag as i32) << 8) | (min as i32));
}

/// Flush texture page-cache
///
/// Do this if you have copied/rendered into an area currently in the texture
/// cache.
pub unsafe fn sce_gu_tex_flush() {
    send_command_f(Command::TexFlush, 0.0);
}

/// Set how textures are applied
///
/// # Parameters
///
/// - `tfx`: Which apply-mode to use
/// - `tcc`: Which component-mode to use
pub unsafe fn sce_gu_tex_func(tfx: TextureEffect, tcc: TextureColorComponent) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.texture_function = (((tcc as u32) << 8) | (tfx as u32)) as i32;
    send_command_i(
        Command::TexFunc,
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
/// sce_gu_tex_image(MipmapLevel::None, 512, 512, 340, data);
/// ```
///
/// This will generate a 512x512 pixel texture map, with the remaining horizontal
/// space being filled with the original texture repeating. The remaining
/// vertical space will overflow into the data past the input buffer, which may
/// appear as garbage data. This is not a problem as the UV coordinates on the
/// triangles can be crafted to stay within the image bounds, both vertically and
/// horizontally.
pub unsafe fn sce_gu_tex_image(mipmap: MipmapLevel, width: i32, height: i32, tbw: i32, tbp: *const c_void) {
    use core::intrinsics::ctlz;

    const TBP_CMD_TBL: [Command; 8] = [
        Command::TexAddr0,
        Command::TexAddr1,
        Command::TexAddr2,
        Command::TexAddr3,
        Command::TexAddr4,
        Command::TexAddr5,
        Command::TexAddr6,
        Command::TexAddr7,
    ];

    const TBW_CMD_TBL: [Command; 8] = [
        Command::TexBufWidth0,
        Command::TexBufWidth1,
        Command::TexBufWidth2,
        Command::TexBufWidth3,
        Command::TexBufWidth4,
        Command::TexBufWidth5,
        Command::TexBufWidth6,
        Command::TexBufWidth7,
    ];

    const TSIZE_CMD_TBL: [Command; 8] = [
        Command::TexSize0,
        Command::TexSize1,
        Command::TexSize2,
        Command::TexSize3,
        Command::TexSize4,
        Command::TexSize5,
        Command::TexSize6,
        Command::TexSize7,
    ];

    send_command_i(
        TBP_CMD_TBL[mipmap as usize],
        (tbp as i32) & 0xffffff,
    );
    send_command_i(
        TBW_CMD_TBL[mipmap as usize],
        ((tbp as u32 >> 8) as i32 & 0x0f0000) | tbw as i32,
    );
    send_command_i(
        TSIZE_CMD_TBL[mipmap as usize],
        ((31 - ctlz(height & 0x3ff)) << 8) | (31 - ctlz(width & 0x3ff)),
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
    // Linker error if this is not here.
    #[no_mangle]
    unsafe extern fn truncf(mut x: f32) -> f32 {
        llvm_asm!("cvt.w.s $0, $0" : "+f"(x));
        x
    }

    let mut offset = core::intrinsics::truncf32(bias * 16.0) as i32;

    // PSPSDK: mip map bias?
    if offset >= 128 {
        offset = 128
    } else if offset < -128 {
        offset = -128;
    }

    send_command_i(Command::TexLevel, ((offset as i32) << 16) | mode as i32);
}

/// Set the texture-mapping mode
///
/// # Parameters
///
/// - `mode`: Which mode to use
/// - `a1`: Unknown
/// - `a2`: Unknown
pub unsafe fn sce_gu_tex_map_mode(mode: TextureMapMode, a1: u32, a2: u32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.texture_map_mode = mode;
    send_command_i(
        Command::TexMapMode,
        ((context.texture_proj_map_mode as i32) << 8) | mode as i32,
    );
    send_command_i(Command::TexShadeLs, ((a2 << 8) | (a1 & 0x03)) as i32);
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
pub unsafe fn sce_gu_tex_mode(tpsm: TexturePixelFormat, maxmips: i32, a2: i32, swizzle: i32) {
    CONTEXTS[CURR_CONTEXT as usize].texture_mode = tpsm;

    send_command_i(
        Command::TexMode,
        (maxmips << 16) | (a2 << 8) | swizzle,
    );

    send_command_i(Command::TexFormat, tpsm as i32);
    sce_gu_tex_flush();
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
pub unsafe fn sce_gu_tex_offset(u: f32, v: f32) {
    send_command_f(Command::TexOffsetU, u);
    send_command_f(Command::TexOffsetV, v);
}

/// Set texture projection-map mode
///
/// # Parameters
///
/// - `mode`: Which mode to use
pub unsafe fn sce_gu_tex_proj_map_mode(mode: TextureProjectionMapMode) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];
    context.texture_proj_map_mode = mode;
    send_command_i(
        Command::TexMapMode,
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
pub unsafe fn sce_gu_tex_scale(u: f32, v: f32) {
    send_command_f(Command::TexScaleU, u);
    send_command_f(Command::TexScaleV, v);
}

pub unsafe fn sce_gu_tex_slope(slope: f32) {
    send_command_f(Command::TexLodSlope, slope);
}

/// Synchronize rendering pipeline with image upload.
///
/// This will stall the rendering pipeline until the current image upload initiated by
/// `sce_gu_copy_image` has completed.
pub unsafe fn sce_gu_tex_sync() {
    send_command_i(Command::TexSync, 0);
}

/// Set if the texture should repeat or clamp
///
/// Available modes are:
///
/// # Parameters
///
/// - `u`: Wrap-mode for the U direction
/// - `v`: Wrap-mode for the V direction
pub unsafe fn sce_gu_tex_wrap(u: WrapMode, v: WrapMode) {
    send_command_i(Command::TexWrap, ((v as i32) << 8) | u as i32);
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
pub unsafe fn sce_gu_clut_load(num_blocks: i32, cbp: *const c_void) {
    send_command_i(Command::ClutAddr, (cbp as i32) & 0xffffff);
    send_command_i(Command::ClutAddrUpper, ((cbp as u32) >> 8) as i32 & 0xf0000);
    send_command_i(Command::LoadClut, num_blocks);
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
pub unsafe fn sce_gu_clut_mode(cpsm: ClutPixelFormat, shift: u32, mask: u32, a3: u32) {
    let arg = ((cpsm as u32) | (shift << 2) | (mask << 8) | (a3 << 16)) as i32;
    send_command_i(Command::ClutFormat, arg);
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
/// # use psp::sys::gu::sce_gu_offset;
/// sce_gu_offset(2048 - (480 / 2), 2048 - (272 / 2)) {
/// ```
///
/// # Parameters
///
/// - `x`: Offset (0-4095)
/// - `y`: Offset (0-4095)
pub unsafe fn sce_gu_offset(x: u32, y: u32) {
    send_command_i(Command::OffsetX, (x << 4) as i32);
    send_command_i(Command::OffsetY, (y << 4) as i32);
}

/// Set what to scissor within the current viewport
///
/// Note that scissoring is only performed if the custom scissoring
/// (`State::ScissorTest`) is enabled, e.g. via `sce_gu_enable`.
///
/// # Parameters
///
/// - `x`: Left of scissor region
/// - `y`: Top of scissor region
/// - `w`: Width of scissor region
/// - `h`: Height of scissor region
pub unsafe fn sce_gu_scissor(x: i32, y: i32, w: i32, h: i32) {
    let context = &mut CONTEXTS[CURR_CONTEXT as usize];

    context.scissor_start = [x, y];
    context.scissor_end = [w - 1, h - 1];

    if context.scissor_enable != 0 {
        send_command_i(
            Command::Scissor1,
            (context.scissor_start[1] << 10) | context.scissor_start[0],
        );
        send_command_i(Command::Scissor2, (context.scissor_end[1] << 10) | context.scissor_end[0]);
    }
}

/// Set current viewport
///
/// # Example
///
/// Setup a viewport of size (480,272) with origin at (2048,2048)
///
/// ```no_run
/// # use psp::sys::gu::sce_gu_viewport;
/// sce_gu_viewport(2048, 2048, 480, 272);
/// ```
///
/// # Parameters
///
/// - `cx`: Center for horizontal viewport
/// - `cy`: Center for vertical viewport
/// - `width`: Width of viewport
/// - `height`: Height of viewport
pub unsafe fn sce_gu_viewport(cx: i32, cy: i32, width: i32, height: i32) {
    send_command_f(Command::ViewportXScale, (width >> 1) as f32);
    send_command_f(Command::ViewportYScale, ((-height) >> 1) as f32);
    send_command_f(Command::ViewportXCenter, cx as f32);
    send_command_f(Command::ViewportYCenter, cy as f32);
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
pub unsafe fn sce_gu_draw_bezier(
    v_type: VertexType,
    u_count: i32,
    v_count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !v_type.is_empty() {
        send_command_i(Command::VertexType, v_type.bits());
    }

    if !indices.is_null() {
        send_command_i(Command::Base, ((indices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(Command::Iaddr, (indices as i32) & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(Command::Base, ((vertices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(Command::Vaddr, (vertices as i32) & 0xffffff);
    }

    send_command_i(Command::Bezier, (v_count << 8) | u_count);
}

/// Set dividing for patches (beziers and splines)
///
/// # Parameters
///
/// - `ulevel`: Number of division on u direction
/// - `vlevel`: Number of division on v direction
pub unsafe fn sce_gu_patch_divide(ulevel: u32, vlevel: u32) {
    send_command_i(Command::PatchDivision, ((vlevel << 8) | ulevel) as i32);
}

pub unsafe fn sce_gu_patch_front_face(a0: u32) {
    send_command_i(Command::PatchFacing, a0 as i32);
}

/// Set primitive for patches (beziers and splines)
///
/// # Parameters
///
/// - `prim`: Desired primitive type
pub unsafe fn sce_gu_patch_prim(prim: PatchPrimitive) {
    match prim {
        PatchPrimitive::Points => send_command_i(Command::PatchPrimitive, 2),
        PatchPrimitive::LineStrip => send_command_i(Command::PatchPrimitive, 1),
        PatchPrimitive::TriangleStrip => send_command_i(Command::PatchPrimitive, 0),
    }
}

pub unsafe fn sce_gu_draw_spline(
    v_type: VertexType,
    u_count: i32,
    v_count: i32,
    u_edge: i32,
    v_edge: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !v_type.is_empty() {
        send_command_i(Command::VertexType, v_type.bits());
    }

    if !indices.is_null() {
        send_command_i(Command::Base, ((indices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(Command::Iaddr, (indices as i32) & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(Command::Base, ((vertices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(Command::Vaddr, (vertices as i32) & 0xffffff);
    }

    send_command_i(
        Command::Spline,
        (v_edge << 18) | (u_edge << 16) | (v_count << 8) | u_count,
    );
}

/// Set transform matrices
///
/// # Parameters
///
/// - `type`: Which matrix-type to set
/// - `matrix`: Matrix to load
pub unsafe fn sce_gu_set_matrix(type_: MatrixMode, matrix: &crate::sys::gum::FMatrix4) {
    let fmatrix = matrix as *const _ as *const f32;

    match type_ {
        MatrixMode::Projection => {
            send_command_f(Command::ProjMatrixNumber, 0.0);
            for i in 0..16 {
                send_command_f(Command::ProjMatrixData, *fmatrix.offset(i));
            }
        }

        MatrixMode::View => {
            send_command_f(Command::ViewMatrixNumber, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(Command::ViewMatrixData, *fmatrix.offset(j + i * 4));
                }
            }
        }

        MatrixMode::Model => {
            send_command_f(Command::WorldMatrixNumber, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(Command::WorldMatrixData, *fmatrix.offset(j + i * 4));
                }
            }
        }

        MatrixMode::Texture => {
            send_command_f(Command::TGenMatrixNumber, 0.0);
            for i in 0..4 {
                for j in 0..3 {
                    send_command_f(Command::TGenMatrixData, *fmatrix.offset(j + i * 4));
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
pub unsafe fn sce_gu_bone_matrix(index: u32, matrix: *const FMatrix4) {
    let offset = ((index << 1) + index) << 2; // 3 * 4 matrix
    let fmatrix = matrix as *const f32;

    send_command_i(Command::BoneMatrixNumber, offset as i32);
    for i in 0..4 {
        for j in 0..3 {
            send_command_f(Command::BoneMatrixData, *fmatrix.offset(j + (i << 2)));
        }
    }
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
pub unsafe fn sce_gu_morph_weight(index: i32, weight: f32) {
    let cmd = match index {
        0 => Command::MorphWeight0,
        1 => Command::MorphWeight1,
        2 => Command::MorphWeight2,
        3 => Command::MorphWeight3,
        4 => Command::MorphWeight4,
        5 => Command::MorphWeight5,
        6 => Command::MorphWeight6,
        7 => Command::MorphWeight7,
        _ => core::intrinsics::unreachable(),
    };

    send_command_f(cmd, weight);
}

pub unsafe fn sce_gu_draw_array_n(
    primitive_type: Primitive,
    v_type: VertexType,
    count: i32,
    a3: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    if !v_type.is_empty() {
        send_command_i(Command::VertexType, v_type.bits());
    }

    if !indices.is_null() {
        send_command_i(Command::Base, ((indices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(Command::Iaddr, indices as i32 & 0xffffff);
    }

    if !vertices.is_null() {
        send_command_i(Command::Base, ((vertices as u32) >> 8) as i32 & 0xf0000);
        send_command_i(Command::Vaddr, vertices as i32 & 0xffffff);
    }

    if a3 > 0 {
        // PSPSDK: TODO: not sure about this loop, might be off. review
        for _ in 1..a3 {
            send_command_i(Command::Prim, ((primitive_type as i32) << 16) | count);
        }

        send_command_i_stall(Command::Prim, ((primitive_type as i32) << 16) | count);
    }
}
