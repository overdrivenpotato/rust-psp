
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

/* Color Macros, maps 8 bit unsigned channels into one 32-bit value */
pub const fn abgr(a : u8,b : u8,g : u8,r : u8) -> u32{
    let mut res: u32 = 0;
    res += (a as u32) << 24;
    res += (b as u32) << 16;
    res += (g as u32) << 8;
    res += r as u32;
    return res;
}

pub const fn argb(a : u8,r : u8,g : u8,b : u8) -> u32{
    abgr(a,b,g,r)
}
pub const fn rgba(r : u8,g : u8,b : u8,a : u8) -> u32{
    argb(a,r,g,b)
}

/* Color Macro, maps floating point channels (0..1) into one 32-bit value */
pub fn color(r : f32 ,g : f32 ,b : f32 ,a : f32 )	-> u32 {
    rgba( (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8)
}



// Alright, so I just pasted all of pspgu.h that wasn't implemented yet let's goooooooooo


/*

///
/// END OF IMPLEMENTED FUNCTIONS!!!!!!!!!!!
///


/**
  * Set depth buffer parameters
  *
  * @param zbp - VRAM pointer where the depthbuffer should start
  * @param zbw - The width of the depth-buffer (block-aligned)
  *
**/
//void sceGuDepthBuffer(void* zbp, int zbw);


/**
  * Set display buffer parameters
  *
  * @par Example: Setup a standard 16-bit display buffer
  * @code
  * sceGuDispBuffer(480,272,(void*)512*272*2,512); // 480*272, skipping the draw buffer located at address 0
  * @endcode
  *
  * @param width - Width of the display buffer in pixels
  * @param height - Width of the display buffer in pixels
  * @param dispbp - VRAM pointer to where the display-buffer starts
  * @param dispbw - Display buffer width (block aligned)
  *
**/
void sceGuDispBuffer(int width, int height, void* dispbp, int dispbw);

/**
  * Set draw buffer parameters (and store in context for buffer-swap)
  *
  * Available pixel formats are:
  *   - GU_PSM_5650
  *   - GU_PSM_5551
  *   - GU_PSM_4444
  *   - GU_PSM_8888
  *
  * @par Example: Setup a standard 16-bit draw buffer
  * @code
  * sceGuDrawBuffer(GU_PSM_5551,(void*)0,512);
  * @endcode
  *
  * @param psm - Pixel format to use for rendering (and display)
  * @param fbp - VRAM pointer to where the draw buffer starts
  * @param fbw - Frame buffer width (block aligned)
**/
void sceGuDrawBuffer(int psm, void* fbp, int fbw);

/**
  * Set draw buffer directly, not storing parameters in the context
  *
  * @param psm - Pixel format to use for rendering
  * @param fbp - VRAM pointer to where the draw buffer starts
  * @param fbw - Frame buffer width (block aligned)
**/
void sceGuDrawBufferList(int psm, void* fbp, int fbw);

/**
  * Turn display on or off
  *
  * Available states are:
  *   - GU_TRUE (1) - Turns display on
  *   - GU_FALSE (0) - Turns display off
  *
  * @param state - Turn display on or off
  * @return State of the display prior to this call
**/
int sceGuDisplay(int state);

/**
  * Select which depth-test function to use
  *
  * Valid choices for the depth-test are:
  *   - GU_NEVER - No pixels pass the depth-test
  *   - GU_ALWAYS - All pixels pass the depth-test
  *   - GU_EQUAL - Pixels that match the depth-test pass
  *   - GU_NOTEQUAL - Pixels that doesn't match the depth-test pass
  *   - GU_LESS - Pixels that are less in depth passes
  *   - GU_LEQUAL - Pixels that are less or equal in depth passes
  *   - GU_GREATER - Pixels that are greater in depth passes
  *   - GU_GEQUAL - Pixels that are greater or equal passes
  *
  * @param function - Depth test function to use
**/
void sceGuDepthFunc(int function);

/**
  * Mask depth buffer writes
  *
  * @param mask - GU_TRUE(1) to disable Z writes, GU_FALSE(0) to enable
**/
void sceGuDepthMask(int mask);

void sceGuDepthOffset(unsigned int offset);

/**
  * Set which range to use for depth calculations.
  *
  * @note The depth buffer is inversed, and takes values from 65535 to 0.
  *
  * Example: Use the entire depth-range for calculations:
  * @code
  * sceGuDepthRange(65535,0);
  * @endcode
  *
  * @param near - Value to use for the near plane
  * @param far - Value to use for the far plane
**/
void sceGuDepthRange(int near, int far);

void sceGuFog(float near, float far, unsigned int color);

/**
  * Initalize the GU system
  *
  * This function MUST be called as the first function, otherwise state is undetermined.
**/
void sceGuInit(void);

/**
  * Shutdown the GU system
  *
  * Called when GU is no longer needed
**/
void sceGuTerm(void);

void sceGuBreak(int a0);
void sceGuContinue(void);

/**
  * Setup signal handler
  *
  * Available signals are:
  *   - GU_CALLBACK_SIGNAL - Called when sceGuSignal is used
  *   - GU_CALLBACK_FINISH - Called when display list is finished
  *
  * @param signal - Signal index to install a handler for
  * @param callback - Callback to call when signal index is triggered
  * @return The old callback handler
**/
void* sceGuSetCallback(int signal, void (*callback)(int));

/**
  * Trigger signal to call code from the command stream
  *
  * Available behaviors are:
  *   - GU_BEHAVIOR_SUSPEND - Stops display list execution until callback function finished
  *   - GU_BEHAVIOR_CONTINUE - Do not stop display list execution during callback
  *
  * @param signal - Signal to trigger
  * @param behavior - Behavior type
**/
void sceGuSignal(int signal, int behavior);

/**
  * Send raw float-command to the GE
  *
  * The argument is converted into a 24-bit float before transfer.
  *
  * @param cmd - Which command to send
  * @param argument - Argument to pass along
**/
void sceGuSendCommandf(int cmd, float argument);

/**
  * Send raw command to the GE
  *
  * Only the 24 lower bits of the argument is passed along.
  *
  * @param cmd - Which command to send
  * @param argument - Argument to pass along
**/
void sceGuSendCommandi(int cmd, int argument);

/**
  * Allocate memory on the current display list for temporary storage
  *
  * @note This function is NOT for permanent memory allocation, the
  * memory will be invalid as soon as you start filling the same display
  * list again.
  *
  * @param size - How much memory to allocate
  * @return Memory-block ready for use
**/
void* sceGuGetMemory(int size);

/**
  * Start filling a new display-context
  *
  * Contexts available are:
  *   - GU_DIRECT - Rendering is performed as list is filled
  *   - GU_CALL - List is setup to be called from the main list
  *   - GU_SEND - List is buffered for a later call to sceGuSendList()
  *
  * The previous context-type is stored so that it can be restored at sceGuFinish().
  *
  * @param cid - Context Type
  * @param list - Pointer to display-list (16 byte aligned)
**/
void sceGuStart(int cid, void* list);

/**
  * Finish current display list and go back to the parent context
  *
  * If the context is GU_DIRECT, the stall-address is updated so that the entire list will
  * execute. Otherwise, only the terminating action is written to the list, depending on
  * context-type.
  *
  * The finish-callback will get a zero as argument when using this function.
  *
  * This also restores control back to whatever context that was active prior to this call.
  *
  * @return Size of finished display list
**/
int sceGuFinish(void);

/**
  * Finish current display list and go back to the parent context, sending argument id for
  * the finish callback.
  *
  * If the context is GU_DIRECT, the stall-address is updated so that the entire list will
  * execute. Otherwise, only the terminating action is written to the list, depending on
  * context-type.
  *
  * @param id - Finish callback id (16-bit)
  * @return Size of finished display list
**/
int sceGuFinishId(unsigned int id);

/**
  * Call previously generated display-list
  *
  * @param list - Display list to call
**/
void sceGuCallList(const void* list);

/**
  * Set wether to use stack-based calls or signals to handle execution of called lists.
  *
  * @param mode - GU_TRUE(1) to enable signals, GU_FALSE(0) to disable signals and use
  * normal calls instead.
**/
void sceGuCallMode(int mode);

/**
  * Check how large the current display-list is
  *
  * @return The size of the current display list
**/
int sceGuCheckList(void);

/**
  * Send a list to the GE directly
  *
  * Available modes are:
  *   - GU_TAIL - Place list last in the queue, so it executes in-order
  *   - GU_HEAD - Place list first in queue so that it executes as soon as possible
  *
  * @param mode - Whether to place the list first or last in queue
  * @param list - List to send
  * @param context - Temporary storage for the GE context
**/
void sceGuSendList(int mode, const void* list, PspGeContext* context);

/**
  * Swap display and draw buffer
  *
  * @return Pointer to the new drawbuffer
**/
void* sceGuSwapBuffers(void);

/**
  * Wait until display list has finished executing
  *
  * @par Example: Wait for the currently executing display list
  * @code
  * sceGuSync(0,0);
  * @endcode
  *
  * Available what are:
  *   - GU_SYNC_WHAT_DONE
  *   - GU_SYNC_WHAT_QUEUED
  *   - GU_SYNC_WHAT_DRAW
  *   - GU_SYNC_WHAT_STALL
  *   - GU_SYNC_WHAT_CANCEL
  *
  * Available mode are:
  *   - GU_SYNC_FINISH - Wait until the last sceGuFinish command is reached
  *   - GU_SYNC_SIGNAL - Wait until the last (?) signal is executed
  *   - GU_SYNC_DONE - Wait until all commands currently in list are executed
  *   - GU_SYNC_LIST - Wait for the currently executed display list (GU_DIRECT)
  *   - GU_SYNC_SEND - Wait for the last send list
  *
  * @param mode - What to wait for
  * @param what - What to sync to
  * @return Unknown at this time
**/
int sceGuSync(int mode, int what);

/**
  * Draw array of vertices forming primitives
  *
  * Available primitive-types are:
  *   - GU_POINTS - Single pixel points (1 vertex per primitive)
  *   - GU_LINES - Single pixel lines (2 vertices per primitive)
  *   - GU_LINE_STRIP - Single pixel line-strip (2 vertices for the first primitive, 1 for every following)
  *   - GU_TRIANGLES - Filled triangles (3 vertices per primitive)
  *   - GU_TRIANGLE_STRIP - Filled triangles-strip (3 vertices for the first primitive, 1 for every following)
  *   - GU_TRIANGLE_FAN - Filled triangle-fan (3 vertices for the first primitive, 1 for every following)
  *   - GU_SPRITES - Filled blocks (2 vertices per primitive)
  *
  * The vertex-type decides how the vertices align and what kind of information they contain.
  * The following flags are ORed together to compose the final vertex format:
  *   - GU_TEXTURE_8BIT - 8-bit texture coordinates
  *   - GU_TEXTURE_16BIT - 16-bit texture coordinates
  *   - GU_TEXTURE_32BITF - 32-bit texture coordinates (float)
  *
  *   - GU_COLOR_5650 - 16-bit color (R5G6B5A0)
  *   - GU_COLOR_5551 - 16-bit color (R5G5B5A1)
  *   - GU_COLOR_4444 - 16-bit color (R4G4B4A4)
  *   - GU_COLOR_8888 - 32-bit color (R8G8B8A8)
  *
  *   - GU_NORMAL_8BIT - 8-bit normals
  *   - GU_NORMAL_16BIT - 16-bit normals
  *   - GU_NORMAL_32BITF - 32-bit normals (float)
  *
  *   - GU_VERTEX_8BIT - 8-bit vertex position
  *   - GU_VERTEX_16BIT - 16-bit vertex position
  *   - GU_VERTEX_32BITF - 32-bit vertex position (float)
  *
  *   - GU_WEIGHT_8BIT - 8-bit weights
  *   - GU_WEIGHT_16BIT - 16-bit weights
  *   - GU_WEIGHT_32BITF - 32-bit weights (float)
  *
  *   - GU_INDEX_8BIT - 8-bit vertex index
  *   - GU_INDEX_16BIT - 16-bit vertex index
  *
  *   - GU_WEIGHTS(n) - Number of weights (1-8)
  *   - GU_VERTICES(n) - Number of vertices (1-8)
  *
  *   - GU_TRANSFORM_2D - Coordinate is passed directly to the rasterizer
  *   - GU_TRANSFORM_3D - Coordinate is transformed before passed to rasterizer
  *
  * @note Every vertex must align to 32 bits, which means that you HAVE to pad if it does not add up!
  *
  * Vertex order:
  * [for vertices(1-8)]
  * [weights (0-8)]
  * [texture uv]
  * [color]
  * [normal]
  * [vertex]
  * [/for]
  *
  * @par Example: Render 400 triangles, with floating-point texture coordinates, and floating-point position, no indices
  * @code
  * sceGuDrawArray(GU_TRIANGLES,GU_TEXTURE_32BITF|GU_VERTEX_32BITF,400*3,0,vertices);
  * @endcode
  *
  * @param prim - What kind of primitives to render
  * @param vtype - Vertex type to process
  * @param count - How many vertices to process
  * @param indices - Optional pointer to an index-list
  * @param vertices - Pointer to a vertex-list
**/
void sceGuDrawArray(int prim, int vtype, int count, const void* indices, const void* vertices);

/**
  * Begin conditional rendering of object
  *
  * If no vertices passed into this function are inside the scissor region, it will skip rendering
  * the object. There can be up to 32 levels of conditional testing, and all levels HAVE to
  * be terminated by sceGuEndObject().
  *
  * @par Example: test a boundingbox against the frustum, and if visible, render object
  * @code
  * sceGuBeginObject(GU_VERTEX_32BITF,8,0,boundingBox);
  *   sceGuDrawArray(GU_TRIANGLES,GU_TEXTURE_32BITF|GU_VERTEX_32BITF,vertexCount,0,vertices);
  * sceGuEndObject();
  * @endcode
  *
  * @param vtype - Vertex type to process
  * @param count - Number of vertices to test
  * @param indices - Optional list to an index-list
  * @param vertices - Pointer to a vertex-list
**/
void sceGuBeginObject(int vtype, int count, const void* indices, const void* vertices);

/**
  * End conditional rendering of object
**/
void sceGuEndObject(void);

/**
  * Enable or disable GE state
  *
  * Look at sceGuEnable() for a list of states
  *
  * @param state - Which state to change
  * @param status - Wether to enable or disable the state
**/
void sceGuSetStatus(int state, int status);

/**
  * Get if state is currently enabled or disabled
  *
  * Look at sceGuEnable() for a list of states
  *
  * @param state - Which state to query about
  * @return Wether state is enabled or not
**/
int sceGuGetStatus(int state);

/**
  * Set the status on all 22 available states
  *
  * Look at sceGuEnable() for a list of states
  *
  * @param status - Bit-mask (0-21) containing the status of all 22 states
**/
void sceGuSetAllStatus(int status);

/**
  * Query status on all 22 available states
  *
  * Look at sceGuEnable() for a list of states
  *
  * @return Status of all 22 states as a bitmask (0-21)
**/
int sceGuGetAllStatus(void);

/**
  * Enable GE state
  *
  * The currently available states are:
  *   - GU_ALPHA_TEST
  *   - GU_DEPTH_TEST
  *   - GU_SCISSOR_TEST
  *   - GU_BLEND
  *   - GU_CULL_FACE
  *   - GU_DITHER
  *   - GU_CLIP_PLANES
  *   - GU_TEXTURE_2D
  *   - GU_LIGHTING
  *   - GU_LIGHT0
  *   - GU_LIGHT1
  *   - GU_LIGHT2
  *   - GU_LIGHT3
  *   - GU_COLOR_LOGIC_OP
  *
  * @param state - Which state to enable
**/
void sceGuEnable(int state);

/**
  * Disable GE state
  *
  * Look at sceGuEnable() for a list of states
  *
  * @param state - Which state to disable
**/
void sceGuDisable(int state);

/**
  * Set light parameters
  *
  * Available light types are:
  *   - GU_DIRECTIONAL - Directional light
  *   - GU_POINTLIGHT - Single point of light
  *   - GU_SPOTLIGHT - Point-light with a cone
  *
  * Available light components are:
  *   - GU_AMBIENT_AND_DIFFUSE
  *   - GU_DIFFUSE_AND_SPECULAR
  *   - GU_UNKNOWN_LIGHT_COMPONENT
  *
  * @param light - Light index
  * @param type - Light type
  * @param components - Light components
  * @param position - Light position
**/
void sceGuLight(int light, int type, int components, const ScePspFVector3* position);

/**
  * Set light attenuation
  *
  * @param light - Light index
  * @param atten0 - Constant attenuation factor
  * @param atten1 - Linear attenuation factor
  * @param atten2 - Quadratic attenuation factor
**/
void sceGuLightAtt(int light, float atten0, float atten1, float atten2);

/**
  * Set light color
  *
  * Available light components are:
  *   - GU_AMBIENT
  *   - GU_DIFFUSE
  *   - GU_SPECULAR
  *   - GU_AMBIENT_AND_DIFFUSE
  *   - GU_DIFFUSE_AND_SPECULAR
  *
  * @param light - Light index
  * @param component - Which component to set
  * @param color - Which color to use
**/
void sceGuLightColor(int light, int component, unsigned int color);

/**
  * Set light mode
  *
  * Available light modes are:
  *   - GU_SINGLE_COLOR
  *   - GU_SEPARATE_SPECULAR_COLOR
  *
  * Separate specular colors are used to interpolate the specular component
  * independently, so that it can be added to the fragment after the texture color.
  *
  * @param mode - Light mode to use
**/
void sceGuLightMode(int mode);

/**
  * Set spotlight parameters
  *
  * @param light - Light index
  * @param direction - Spotlight direction
  * @param exponent - Spotlight exponent
  * @param cutoff - Spotlight cutoff angle (in radians)
**/
void sceGuLightSpot(int light, const ScePspFVector3* direction, float exponent, float cutoff);

/**
  * Clear current drawbuffer
  *
  * Available clear-flags are (OR them together to get final clear-mode):
  *   - GU_COLOR_BUFFER_BIT - Clears the color-buffer
  *   - GU_STENCIL_BUFFER_BIT - Clears the stencil-buffer
  *   - GU_DEPTH_BUFFER_BIT - Clears the depth-buffer
  *
  * @param flags - Which part of the buffer to clear
**/
void sceGuClear(int flags);

/**
  * Set the current clear-color
  *
  * @param color - Color to clear with
**/
void sceGuClearColor(unsigned int color);

/**
  * Set the current clear-depth
  *
  * @param depth - Set which depth to clear with (0x0000-0xffff)
**/
void sceGuClearDepth(unsigned int depth);

/**
  * Set the current stencil clear value
  *
  * @param stencil - Set which stencil value to clear with (0-255)
  *
**/
void sceGuClearStencil(unsigned int stencil);

/**
  * Set mask for which bits of the pixels to write
  *
  * @param mask - Which bits to filter against writes
  *
**/
void sceGuPixelMask(unsigned int mask);

/**
  * Set current primitive color
  *
  * @param color - Which color to use (overriden by vertex-colors)
**/
void sceGuColor(unsigned int color);

/**
  * Set the color test function
  *
  * The color test is only performed while GU_COLOR_TEST is enabled.
  *
  * Available functions are:
  *   - GU_NEVER
  *   - GU_ALWAYS
  *   - GU_EQUAL
  *   - GU_NOTEQUAL
  *
  * @par Example: Reject any pixel that does not have 0 as the blue channel
  * @code
  * sceGuColorFunc(GU_EQUAL,0,0xff0000);
  * @endcode
  *
  * @param func - Color test function
  * @param color - Color to test against
  * @param mask - Mask ANDed against both source and destination when testing
**/
void sceGuColorFunc(int func, unsigned int color, unsigned int mask);

/**
  * Set which color components that the material will receive
  *
  * The components are ORed together from the following values:
  *   - GU_AMBIENT
  *   - GU_DIFFUSE
  *   - GU_SPECULAR
  *
  * @param components - Which components to receive
**/
void sceGuColorMaterial(int components);

/**
  * Set the alpha test parameters
  * 
  * Available comparison functions are:
  *   - GU_NEVER
  *   - GU_ALWAYS
  *   - GU_EQUAL
  *   - GU_NOTEQUAL
  *   - GU_LESS
  *   - GU_LEQUAL
  *   - GU_GREATER
  *   - GU_GEQUAL
  *
  * @param func - Specifies the alpha comparison function.
  * @param value - Specifies the reference value that incoming alpha values are compared to.
  * @param mask - Specifies the mask that both values are ANDed with before comparison.
**/
void sceGuAlphaFunc(int func, int value, int mask);

void sceGuAmbient(unsigned int color);
void sceGuAmbientColor(unsigned int color);

/**
  * Set the blending-mode
  *
  * Keys for the blending operations:
  *   - Cs - Source color
  *   - Cd - Destination color
  *   - Bs - Blend function for source fragment
  *   - Bd - Blend function for destination fragment
  *
  * Available blending-operations are:
  *   - GU_ADD - (Cs*Bs) + (Cd*Bd)
  *   - GU_SUBTRACT - (Cs*Bs) - (Cd*Bd)
  *   - GU_REVERSE_SUBTRACT - (Cd*Bd) - (Cs*Bs)
  *   - GU_MIN - Cs < Cd ? Cs : Cd
  *   - GU_MAX - Cs < Cd ? Cd : Cs
  *   - GU_ABS - |Cs-Cd|
  *
  * Available blending-functions are:
  *   - GU_SRC_COLOR
  *   - GU_ONE_MINUS_SRC_COLOR
  *   - GU_SRC_ALPHA
  *   - GU_ONE_MINUS_SRC_ALPHA
  *   - GU_DST_ALPHA
  *   - GU_ONE_MINUS_DST_ALPHA
  *   - GU_DST_COLOR
  *   - GU_ONE_MINUS_DST_COLOR
  *   - GU_FIX
  *
  * @param op - Blending Operation
  * @param src - Blending function for source operand
  * @param dest - Blending function for dest operand
  * @param srcfix - Fix value for GU_FIX (source operand)
  * @param destfix - Fix value for GU_FIX (dest operand)
**/
void sceGuBlendFunc(int op, int src, int dest, unsigned int srcfix, unsigned int destfix);

void sceGuMaterial(int mode, int color);

/**
  *
**/
void sceGuModelColor(unsigned int emissive, unsigned int ambient, unsigned int diffuse, unsigned int specular);

/**
  * Set stencil function and reference value for stencil testing
  *
  * Available functions are:
  *   - GU_NEVER
  *   - GU_ALWAYS
  *   - GU_EQUAL
  *   - GU_NOTEQUAL
  *   - GU_LESS
  *   - GU_LEQUAL
  *   - GU_GREATER
  *   - GU_GEQUAL
  *
  * @param func - Test function
  * @param ref - The reference value for the stencil test
  * @param mask - Mask that is ANDed with both the reference value and stored stencil value when the test is done
**/
void sceGuStencilFunc(int func, int ref, int mask);

/**
  * Set the stencil test actions
  *
  * Available actions are:
  *   - GU_KEEP - Keeps the current value
  *   - GU_ZERO - Sets the stencil buffer value to zero
  *   - GU_REPLACE - Sets the stencil buffer value to ref, as specified by sceGuStencilFunc()
  *   - GU_INCR - Increments the current stencil buffer value
  *   - GU_DECR - Decrease the current stencil buffer value
  *   - GU_INVERT - Bitwise invert the current stencil buffer value
  *
  * As stencil buffer shares memory with framebuffer alpha, resolution of the buffer
  * is directly in relation.
  *
  * @param fail - The action to take when the stencil test fails
  * @param zfail - The action to take when stencil test passes, but the depth test fails
  * @param zpass - The action to take when both stencil test and depth test passes
**/
void sceGuStencilOp(int fail, int zfail, int zpass);

/**
  * Set the specular power for the material
  *
  * @param power - Specular power
  *
**/
void sceGuSpecular(float power);

/**
  * Set the current face-order (for culling)
  *
  * This only has effect when culling is enabled (GU_CULL_FACE)
  *
  * Culling order can be:
  *   - GU_CW - Clockwise primitives are not culled
  *   - GU_CCW - Counter-clockwise are not culled
  *
  * @param order - Which order to use
**/
void sceGuFrontFace(int order);

/**
  * Set color logical operation
  *
  * Available operations are:
  *   - GU_CLEAR
  *   - GU_AND
  *   - GU_AND_REVERSE 
  *   - GU_COPY
  *   - GU_AND_INVERTED
  *   - GU_NOOP
  *   - GU_XOR
  *   - GU_OR
  *   - GU_NOR
  *   - GU_EQUIV
  *   - GU_INVERTED
  *   - GU_OR_REVERSE
  *   - GU_COPY_INVERTED
  *   - GU_OR_INVERTED
  *   - GU_NAND
  *   - GU_SET
  *
  * This operation only has effect if GU_COLOR_LOGIC_OP is enabled.
  *
  * @param op - Operation to execute
**/
void sceGuLogicalOp(int op);

/**
  * Set ordered pixel dither matrix
  *
  * This dither matrix is only applied if GU_DITHER is enabled.
  *
  * @param matrix - Dither matrix
**/
void sceGuSetDither(const ScePspIMatrix4* matrix);

/**
  * Set how primitives are shaded
  *
  * The available shading-methods are:
  *   - GU_FLAT - Primitives are flatshaded, the last vertex-color takes effet
  *   - GU_SMOOTH - Primtives are gouraud-shaded, all vertex-colors take effect
  *
  * @param mode - Which mode to use
**/
void sceGuShadeModel(int mode);

/**
  * Image transfer using the GE
  *
  * @note Data must be aligned to 1 quad word (16 bytes)
  *
  * @par Example: Copy a fullscreen 32-bit image from RAM to VRAM
  * @code
  * sceGuCopyImage(GU_PSM_8888,0,0,480,272,512,pixels,0,0,512,(void*)(((unsigned int)framebuffer)+0x4000000));
  * @endcode
  *
  * @param psm - Pixel format for buffer
  * @param sx - Source X
  * @param sy - Source Y
  * @param width - Image width
  * @param height - Image height
  * @param srcw - Source buffer width (block aligned)
  * @param src - Source pointer
  * @param dx - Destination X
  * @param dy - Destination Y
  * @param destw - Destination buffer width (block aligned)
  * @param dest - Destination pointer
**/
void sceGuCopyImage(int psm, int sx, int sy, int width, int height, int srcw, void* src, int dx, int dy, int destw, void* dest);

/**
  * Specify the texture environment color
  *
  * This is used in the texture function when a constant color is needed.
  *
  * See sceGuTexFunc() for more information.
  *
  * @param color - Constant color (0x00BBGGRR)
**/
void sceGuTexEnvColor(unsigned int color);

/**
  * Set how the texture is filtered
  *
  * Available filters are:
  *   - GU_NEAREST
  *   - GU_LINEAR
  *   - GU_NEAREST_MIPMAP_NEAREST
  *   - GU_LINEAR_MIPMAP_NEAREST
  *   - GU_NEAREST_MIPMAP_LINEAR
  *   - GU_LINEAR_MIPMAP_LINEAR
  *
  * @param min - Minimizing filter
  * @param mag - Magnifying filter
**/
void sceGuTexFilter(int min, int mag);

/**
  * Flush texture page-cache
  *
  * Do this if you have copied/rendered into an area currently in the texture-cache
  *
**/
void sceGuTexFlush(void);

/**
  * Set how textures are applied
  *
  * Key for the apply-modes:
  *   - Cv - Color value result
  *   - Ct - Texture color
  *   - Cf - Fragment color
  *   - Cc - Constant color (specified by sceGuTexEnvColor())
  *
  * Available apply-modes are: (TFX)
  *   - GU_TFX_MODULATE - Cv=Ct*Cf TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
  *   - GU_TFX_DECAL - TCC_RGB: Cv=Ct,Av=Af TCC_RGBA: Cv=Cf*(1-At)+Ct*At Av=Af
  *   - GU_TFX_BLEND - Cv=(Cf*(1-Ct))+(Cc*Ct) TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
  *   - GU_TFX_REPLACE - Cv=Ct TCC_RGB: Av=Af TCC_RGBA: Av=At
  *   - GU_TFX_ADD - Cv=Cf+Ct TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
  *
  * The fields TCC_RGB and TCC_RGBA specify components that differ between
  * the two different component modes.
  *
  *   - GU_TFX_MODULATE - The texture is multiplied with the current diffuse fragment
  *   - GU_TFX_REPLACE - The texture replaces the fragment
  *   - GU_TFX_ADD - The texture is added on-top of the diffuse fragment
  *   
  * Available component-modes are: (TCC)
  *   - GU_TCC_RGB - The texture alpha does not have any effect
  *   - GU_TCC_RGBA - The texture alpha is taken into account
  *
  * @param tfx - Which apply-mode to use
  * @param tcc - Which component-mode to use
**/
void sceGuTexFunc(int tfx, int tcc);

/**
  * Set current texturemap
  *
  * Textures may reside in main RAM, but it has a huge speed-penalty. Swizzle textures
  * to get maximum speed.
  *
  * @note Data must be aligned to 1 quad word (16 bytes)
  *
  * @param mipmap - Mipmap level
  * @param width - Width of texture (must be a power of 2)
  * @param height - Height of texture (must be a power of 2)
  * @param tbw - Texture Buffer Width (block-aligned)
  * @param tbp - Texture buffer pointer (16 byte aligned)
**/
void sceGuTexImage(int mipmap, int width, int height, int tbw, const void* tbp);

/**
  * Set texture-level mode (mipmapping)
  *
  * Available modes are:
  *   - GU_TEXTURE_AUTO
  *   - GU_TEXTURE_CONST
  *   - GU_TEXTURE_SLOPE
  *
  * @param mode - Which mode to use
  * @param bias - Which mipmap bias to use
**/
void sceGuTexLevelMode(unsigned int mode, float bias);

/**
  * Set the texture-mapping mode
  *
  * Available modes are:
  *   - GU_TEXTURE_COORDS
  *   - GU_TEXTURE_MATRIX
  *   - GU_ENVIRONMENT_MAP
  *
  * @param mode - Which mode to use
  * @param a1 - Unknown
  * @param a2 - Unknown
**/
void sceGuTexMapMode(int mode, unsigned int a1, unsigned int a2);

/**
  * Set texture-mode parameters
  *
  * Available texture-formats are:
  *   - GU_PSM_5650 - Hicolor, 16-bit
  *   - GU_PSM_5551 - Hicolor, 16-bit
  *   - GU_PSM_4444 - Hicolor, 16-bit
  *   - GU_PSM_8888 - Truecolor, 32-bit
  *   - GU_PSM_T4 - Indexed, 4-bit (2 pixels per byte)
  *   - GU_PSM_T8 - Indexed, 8-bit
  *
  * @param tpsm - Which texture format to use
  * @param maxmips - Number of mipmaps to use (0-8)
  * @param a2 - Unknown, set to 0
  * @param swizzle - GU_TRUE(1) to swizzle texture-reads
**/
void sceGuTexMode(int tpsm, int maxmips, int a2, int swizzle);

/**
  * Set texture offset
  *
  * @note Only used by the 3D T&L pipe, renders done with GU_TRANSFORM_2D are
  * not affected by this.
  *
  * @param u - Offset to add to the U coordinate
  * @param v - Offset to add to the V coordinate
**/
void sceGuTexOffset(float u, float v);

/**
  * Set texture projection-map mode
  *
  * Available modes are:
  *   - GU_POSITION
  *   - GU_UV
  *   - GU_NORMALIZED_NORMAL
  *   - GU_NORMAL
  *
  * @param mode - Which mode to use
**/
void sceGuTexProjMapMode(int mode);

/**
  * Set texture scale
  *
  * @note Only used by the 3D T&L pipe, renders ton with GU_TRANSFORM_2D are
  * not affected by this.
  *
  * @param u - Scalar to multiply U coordinate with
  * @param v - Scalar to multiply V coordinate with
**/
void sceGuTexScale(float u, float v);
void sceGuTexSlope(float slope);

/**
  * Synchronize rendering pipeline with image upload.
  *
  * This will stall the rendering pipeline until the current image upload initiated by
  * sceGuCopyImage() has completed.
**/
void sceGuTexSync();

/**
  * Set if the texture should repeat or clamp
  *
  * Available modes are:
  *   - GU_REPEAT - The texture repeats after crossing the border
  *   - GU_CLAMP - Texture clamps at the border
  *
  * @param u - Wrap-mode for the U direction
  * @param v - Wrap-mode for the V direction
**/
void sceGuTexWrap(int u, int v);

/**
  * Upload CLUT (Color Lookup Table)
  *
  * @note Data must be aligned to 1 quad word (16 bytes)
  *
  * @param num_blocks - How many blocks of 8 entries to upload (32*8 is 256 colors)
  * @param cbp - Pointer to palette (16 byte aligned)
**/
void sceGuClutLoad(int num_blocks, const void* cbp);

/**
  * Set current CLUT mode
  *
  * Available pixel formats for palettes are:
  *   - GU_PSM_5650
  *   - GU_PSM_5551
  *   - GU_PSM_4444
  *   - GU_PSM_8888
  *
  * @param cpsm - Which pixel format to use for the palette
  * @param shift - Shifts color index by that many bits to the right
  * @param mask - Masks the color index with this bitmask after the shift (0-0xFF)
  * @param a3 - Unknown, set to 0
**/
void sceGuClutMode(unsigned int cpsm, unsigned int shift, unsigned int mask, unsigned int a3);

/**
  * Set virtual coordinate offset
  *
  * The PSP has a virtual coordinate-space of 4096x4096, this controls where rendering is performed
  * 
  * @par Example: Center the virtual coordinate range
  * @code
  * sceGuOffset(2048-(480/2),2048-(480/2));
  * @endcode
  *
  * @param x - Offset (0-4095)
  * @param y - Offset (0-4095)
**/
void sceGuOffset(unsigned int x, unsigned int y);

/**
  * Set what to scissor within the current viewport
  *
  * Note that scissoring is only performed if the custom scissoring is enabled (GU_SCISSOR_TEST)
  *
  * @param x - Left of scissor region
  * @param y - Top of scissor region
  * @param w - Width of scissor region
  * @param h - Height of scissor region
**/
void sceGuScissor(int x, int y, int w, int h);

/**
  * Set current viewport
  *
  * @par Example: Setup a viewport of size (480,272) with origo at (2048,2048)
  * @code
  * sceGuViewport(2048,2048,480,272);
  * @endcode
  *
  * @param cx - Center for horizontal viewport
  * @param cy - Center for vertical viewport
  * @param width - Width of viewport
  * @param height - Height of viewport
**/
void sceGuViewport(int cx, int cy, int width, int height);

/**
  * Draw bezier surface
  *
  * @param vtype - Vertex type, look at sceGuDrawArray() for vertex definition
  * @param ucount - Number of vertices used in the U direction
  * @param vcount - Number of vertices used in the V direction
  * @param indices - Pointer to index buffer
  * @param vertices - Pointer to vertex buffer
**/
void sceGuDrawBezier(int vtype, int ucount, int vcount, const void* indices, const void* vertices);

/**
  * Set dividing for patches (beziers and splines)
  *
  * @param ulevel - Number of division on u direction
  * @param vlevel - Number of division on v direction
**/
void sceGuPatchDivide(unsigned int ulevel, unsigned int vlevel);

void sceGuPatchFrontFace(unsigned int a0);

/**
  * Set primitive for patches (beziers and splines)
  *
  * @param prim - Desired primitive type (GU_POINTS | GU_LINE_STRIP | GU_TRIANGLE_STRIP)
**/
void sceGuPatchPrim(int prim);

void sceGuDrawSpline(int vtype, int ucount, int vcount, int uedge, int vedge, const void* indices, const void* vertices);

/**
  * Set transform matrices
  *
  * Available matrices are:
  *   - GU_PROJECTION - View->Projection matrix
  *   - GU_VIEW - World->View matrix
  *   - GU_MODEL - Model->World matrix
  *   - GU_TEXTURE - Texture matrix
  *
  * @param type - Which matrix-type to set
  * @param matrix - Matrix to load
**/
void sceGuSetMatrix(int type, const ScePspFMatrix4* matrix);

/**
  * Specify skinning matrix entry
  *
  * To enable vertex skinning, pass GU_WEIGHTS(n), where n is between
  * 1-8, and pass available GU_WEIGHT_??? declaration. This will change
  * the amount of weights passed in the vertex araay, and by setting the skinning,
  * matrices, you will multiply each vertex every weight and vertex passed.
  *
  * Please see sceGuDrawArray() for vertex format information.
  *
  * @param index - Skinning matrix index (0-7)
  * @param matrix - Matrix to set
**/
void sceGuBoneMatrix(unsigned int index, const ScePspFMatrix4* matrix);

/**
  * Specify morph weight entry
  *
  * To enable vertex morphing, pass GU_VERTICES(n), where n is between
  * 1-8. This will change the amount of vertices passed in the vertex array,
  * and by setting the morph weights for every vertex entry in the array,
  * you can blend between them.
  *
  * Please see sceGuDrawArray() for vertex format information.
  *
  * @param index - Morph weight index (0-7)
  * @param weight - Weight to set
**/
void sceGuMorphWeight(int index, float weight);

void sceGuDrawArrayN(int primitive_type, int vertex_type, int count, int a3, const void* indices, const void* vertices);

/**
  * Set how the display should be set
  *
  * Available behaviours are:
  *   - PSP_DISPLAY_SETBUF_IMMEDIATE - Display is swapped immediately
  *   - PSP_DISPLAY_SETBUF_NEXTFRAME - Display is swapped on the next frame
  *
  * Do remember that this swaps the pointers internally, regardless of setting, so be careful to wait until the next
  * vertical blank or use another buffering algorithm (see guSwapBuffersCallback()).
**/
void guSwapBuffersBehaviour(int behaviour);

/**
  * Set a buffer swap callback to allow for more advanced buffer methods without hacking the library.
  *
  * The GuSwapBuffersCallback is defined like this:
  * @code
  * void swapBuffersCallback(void** display, void** render);
  * @endcode
  * and on entry they contain the variables that are to be set. To change the pointers that will be used, just
  * write the new pointers. Example of a triple-buffering algorithm:
  * @code
  * void* doneBuffer;
  * void swapBuffersCallback(void** display, void** render)
  * {
  *  void* active = doneBuffer;
  *  doneBuffer = *display;
     *display = active;
  * }
  * @endcode
  *
  * @param callback - Callback to access when buffers are swapped. Pass 0 to disable.
**/
void guSwapBuffersCallback(GuSwapBuffersCallback callback);

/*@}*/

*/