
pub const PI: f32 = 3.141593;

#[repr(u32)]
pub enum Bool{
    False = 0,
    True = 1
}

/* Primitive types */
#[repr(u32)]
pub enum Primitive{
    Points = 0,
    Lines = 1,
    LineStrip = 2,
    Triangles = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    Sprites = 6
}

/* States */
#[repr(u32)]
pub enum State{
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
    Fragment2X = 21
}


/* Matrix modes */
#[repr(u32)]
pub enum MatrixMode{
    Projection = 0,
    View = 1,
    Model = 2,
    Texture = 3,
}

/* Vertex Declarations Begin */
const fn texture_shift(n: u32) -> u32{
    n << 0
}

#[repr(u32)]
pub enum Texture{
    Texture8bit		= texture_shift(1),
    Texture16bit	    = texture_shift(2),
    Texture32bitf	    = texture_shift(3),
}

const fn color_shift(n: u32) -> u32{
    n << 2
}
#[repr(u32)]
pub enum Color{
    Color5650		= color_shift(4),
    Color5551		= color_shift(5),
    Color4444		= color_shift(6),
    Color8888		= color_shift(7),
}

const fn normal_shift(n: u32) -> u32{
    n << 5
}
#[repr(u32)]
pub enum Normal{
    Normal8bit		    = normal_shift(1),
    Normal16bit		    = normal_shift(2),
    Normal32bitf	    = normal_shift(3),
}

const fn vertex_shift(n: u32) -> u32{
    n << 7
}
#[repr(u32)]
pub enum Vertex{
    Vertex8bit		= vertex_shift(1),
    Vertex16bit	= vertex_shift(2),
    Vertex32bitf	= vertex_shift(3),
}

const fn weight_shift(n: u32) -> u32{
    n << 9
}

#[repr(u32)]
pub enum Weight{
    Weight8bit		= weight_shift(1),
    Weight16bit	= weight_shift(2),
    Weight32bitf	= weight_shift(3),
}

const fn index_shift(n: u32) -> u32{
    n << 11
}
#[repr(u32)]
pub enum Index{
    Index8bit		= index_shift(1),
    Index16bit		= index_shift(2),
}

const fn weights(n: u32) -> u32{
    (((n)-1)&7)<<14
}

const fn vertices(n: u32) -> u32{
    (((n)-1)&7)<<18
}

pub const WEIGHTS_BITS: u32 = weights(8);
pub const VERTICES_BITS: u32 = vertices(8);

const fn transform_shift(n: u32) -> u32{
    n << 23
}

#[repr(u32)]
pub enum Transform{
    Transform3D		= transform_shift(0),
    Transform2D		= transform_shift(1),
}

/* Vertex Declarations End */

/* Pixel Formats */
#[repr(u32)]
pub enum PixelFormat{
    Psm5650    = 0, /* Display, Texture, Palette */
    Psm5551    = 1, /* Display, Texture, Palette */
    Psm4444    = 2, /* Display, Texture, Palette */
    Psm8888    = 3, /* Display, Texture, Palette */
    PsmT4      = 4, /* Texture */
    PsmT8      = 5, /* Texture */
    PsmT16     = 6, /* Texture */
    PsmT32     = 7, /* Texture */
    PsmDxt1    = 8, /* Texture */
    PsmDxt3    = 9, /* Texture */
    PsmDxt5    = 10 /* Texture */
}


/* Spline Mode */
#[repr(u32)]
pub enum SplineMode{
    FillFill	= 0,
    OpenFill	= 1,
    FillOpen	= 2,
    OpenOpen	= 3,
}

/* Shading Model */
#[repr(u32)]
pub enum ShadingModel{
    Flat = 0,
    Smooth = 1
}

/* Logical operation */
#[repr(u32)]
pub enum LogicalOperation{
    Clear = 0,
    And = 	1,
    AndReverse = 2,
    Copy = 	3,
    AndInverted = 4,
    Noop = 	5,
    Xor = 	6,
    Or = 	7,
    Nor = 	8,
    Equiv = 9,
    Inverted = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 	14,
    Set = 	15
}


/* Texture Filter */
#[repr(u32)]
pub enum TextureFilter{
    Nearest = 0,
    Linear = 1,
    NearestMipmapNearest  = 4,
    LinearMipmapNearest   = 5,
    NearestMipmapLinear   = 6,
    LinearMipmapLinear	= 7
}

/* Texture Map Mode */
#[repr(u32)]
pub enum TextureMapMode{
    TextureCoords	= 0,
    TextureMatrix	= 1,
    EnvironmentMap	= 2
}

/* Texture Level Mode */
#[repr(u32)]
pub enum TextureLevelMode{
    TextureAuto	= 0,
    TextureConst	= 1,
    TextureSlope	= 2,
}

/* Texture Projection Map Mode */
#[repr(u32)]
pub enum TextureProjectionMapMode{
    Position		    = 0,
    Uv			        = 1,
    NormalizedNormal	= 2,
    Normal		        = 3
}

/* Wrap Mode */
#[repr(u32)]
pub enum WrapMode{
    Repeat		= 0,
    Clamp		= 1
}

/* Front Face Direction */
#[repr(u32)]
pub enum FrontFaceDirection{
    CW	= 0,
    CCW	= 1,
}

/* Test Function */
#[repr(u32)]
pub enum TestFunction{
    Never		= 0,
    Always		= 1,
    Equal		= 2,
    Notequal		= 3,
    Less			= 4,
    Lequal		= 5,
    Greater		= 6,
    Gequal		= 7,
}

/* Clear Buffer Mask */
#[repr(u32)]
pub enum ClearBuffer{
    ColorBufferBit	= 1,
    StencilBufferBit	= 2,
    DepthBufferBit	= 4,
    FastClearBit	    = 16,
}

/* Texture Effect */
#[repr(u32)]
pub enum TextureEffect{
    TfxModulate	= 0,
    TfxDecal		= 1,
    TfxBlend		= 2,
    TfxReplace		= 3,
    TfxAdd		    = 4
}

/* Texture Color Component */
#[repr(u32)]
pub enum TextureColorComponent{
    TccRGB	    = 0,
    TccRGBA	= 1
}

/* Blending Op */
#[repr(u32)]
pub enum BlendingOperation{
    Add			        = 0,
    Subtract		    = 1,
    ReverseSubtract	= 2,
    Min			        = 3,
    Max			        = 4,
    Abs			        = 5
}

/* Blending Factor */
#[repr(u32)]
pub enum BlendingFactorSrc{
    SrcColor		    = 0,
    OneMinusSrcColor	= 1,
    SrcAlpha		    = 2,
    OneMinusSrcAlpha	= 3,
}

pub enum BlendingFactorDst{
    DstColor		    = 0,
    OneMinusDstColor	= 1,
    DstAlpha		    = 4,
    OneMinusDstAlpha	= 5,
    Fix			        = 10
}

/* Stencil Operations */
#[repr(u32)]
pub enum StencilOperation{
    Keep			= 0,
    Zero			= 1,
    Replace		    = 2,
    Invert		    = 3,
    Incr			= 4,
    Decr			= 5
}

/* Light Components */
#[repr(u32)]
pub enum LightComponent{
    Ambient		            = 1,
    Diffuse		            = 2,
    Specular		        = 4,
    AmbientAndDiffuse	    = 1|2,
    DiffuseAndSpecular	= 2|4,
    UnknownLightComponent = 8
}

/* Light modes */
#[repr(u32)]
pub enum LightMode{
    SingleColor	    	= 0,
    SeparateSpecularColor = 1
}

/* Light Type */
#[repr(u32)]
pub enum LightType{ 
    Directional  = 0,
    Pointlight   = 1,
    Spotlight    = 2
}

/* Contexts */
#[repr(u32)]
pub enum Context{
    Direct  = 0,
    Call    = 1,
    Send	= 2,
}

/* List Queue */
#[repr(u32)]
pub enum ListQueue{
    Tail = 0,
    Head = 1
}

/* Sync behavior (mode) */
#[repr(u32)]
pub enum SyncMode{
    SyncFinish	= 0,
    SyncSignal	= 1,
    SyncDone	= 2,
    SyncList	= 3,
    SyncSend	= 4
}

/* behavior (what) */
#[repr(u32)]
pub enum SyncModeWhat{
    SyncWait	= 0,
    SyncNowait	= 1
}

/* Sync behavior (what) [see pspge.h] */
#[repr(u32)]
pub enum SyncBehaviorWhat{
    SyncWhatDone   = 0,
    SyncWhatQueued = 1,
    SyncWhatDraw   = 2,
    SyncWhatStall  = 3,
    SyncWhatCancel = 4
}

/* Signals */
#[repr(u32)]
pub enum Signal{
    CallbackSignal	= 1,
    CallbackFinish	= 4
}

/* Signal behavior */
#[repr(u32)]
pub enum SignalBehavior{
    BehaviorSuspend  = 1,
    BehaviorContinue = 2   
}
