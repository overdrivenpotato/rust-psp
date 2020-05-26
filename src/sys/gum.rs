use crate::vfpu_asm;
use crate::sys::vfpu_context::{Context, MatrixSet};
use core::{ptr, mem::MaybeUninit, ffi::c_void};

// TODO: Replace this with the definiton in `gu` once merged.
#[repr(i32)]
#[derive(Copy, Debug, Clone)]
pub enum Mode {
    Projection = 0,
    View = 1,
    Model = 2,
    Texture = 3,
}

static mut MATRIX_STACK: [[FMatrix4; 32]; 4] = {
    let zero_vector = FVector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    let zero_matrix = FMatrix4 {
        x: zero_vector,
        y: zero_vector,
        z: zero_vector,
        w: zero_vector,
    };

    let stack = [
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,

        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
    ];

    [stack, stack, stack, stack]
};

static mut MATRIX_UPDATE: [i32; 4] = [0, 0, 0, 0];
static mut CURRENT_MATRIX_UPDATE: i32 = 0;
static mut CURRENT_MATRIX: *mut FMatrix4 = ptr::null_mut();
static mut CURRENT_MODE: Mode = Mode::Projection;
static mut STACK_DEPTH: [*mut FMatrix4; 4] = unsafe {
    [
        &mut MATRIX_STACK[Mode::Projection as usize][0],
        &mut MATRIX_STACK[Mode::View as usize][0],
        &mut MATRIX_STACK[Mode::Model as usize][0],
        &mut MATRIX_STACK[Mode::Texture as usize][0],
    ]
};

static mut VFPU_CONTEXT: Option<Context> = None;
unsafe fn get_context_unchecked() -> &'static mut Context {
    match VFPU_CONTEXT.as_mut() {
        Some(r) => r,
        None => core::intrinsics::unreachable(),
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L64Rect {
    pub x: u64,
    pub y: u64,
    pub w: u64,
    pub h: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SVector2 {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IVector2 {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L64Vector2 {
    pub x: u64,
    pub y: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FVector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Vector2 {
    pub fv: FVector2,
    pub iv: IVector2,
    pub f: [f32; 2usize],
    pub i: [i32; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SVector3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IVector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L64Vector3 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[repr(C, align(4))]
#[derive(Debug, Copy, Clone)]
pub struct FVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Vector3 {
    pub fv: FVector3,
    pub iv: IVector3,
    pub f: [f32; 3usize],
    pub i: [i32; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SVector4 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IVector4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L64Vector4 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}

#[repr(C, align(4))]
#[derive(Debug, Copy, Clone)]
pub struct FVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FVector4Unaligned {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Vector4 {
    pub fv: FVector4,
    pub iv: IVector4,
    pub f: [f32; 4usize],
    pub i: [i32; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMatrix2 {
    pub x: IVector2,
    pub y: IVector2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FMatrix2 {
    pub x: FVector2,
    pub y: FVector2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Matrix2 {
    pub fm: FMatrix2,
    pub im: IMatrix2,
    pub fv: [FVector2; 2usize],
    pub iv: [IVector2; 2usize],
    pub v: [Vector2; 2usize],
    pub f: [[f32; 2usize]; 2usize],
    pub i: [[i32; 2usize]; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMatrix3 {
    pub x: IVector3,
    pub y: IVector3,
    pub z: IVector3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FMatrix3 {
    pub x: FVector3,
    pub y: FVector3,
    pub z: FVector3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Matrix3 {
    pub fm: FMatrix3,
    pub im: IMatrix3,
    pub fv: [FVector3; 3usize],
    pub iv: [IVector3; 3usize],
    pub v: [Vector3; 3usize],
    pub f: [[f32; 3usize]; 3usize],
    pub i: [[i32; 3usize]; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMatrix4 {
    pub x: IVector4,
    pub y: IVector4,
    pub z: IVector4,
    pub w: IVector4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMatrix4Unaligned {
    pub x: IVector4,
    pub y: IVector4,
    pub z: IVector4,
    pub w: IVector4,
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct FMatrix4 {
    pub x: FVector4,
    pub y: FVector4,
    pub z: FVector4,
    pub w: FVector4,
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct FMatrix4Unaligned {
    pub x: FVector4,
    pub y: FVector4,
    pub z: FVector4,
    pub w: FVector4,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Matrix4 {
    pub fm: FMatrix4,
    pub im: IMatrix4,
    pub fv: [FVector4; 4usize],
    pub iv: [IVector4; 4usize],
    pub v: [Vector4; 4usize],
    pub f: [[f32; 4usize]; 4usize],
    pub i: [[i32; 4usize]; 4usize],
}

pub const EPSILON: f32 = 0.00001;

pub unsafe fn sce_gum_draw_array(
    _prim: i32,
    _v_type: i32,
    _count: i32,
    _indices: *const c_void,
    _vertices: *const c_void,
) {
  sce_gum_update_matrix();

  // sceGuDrawArray(prim,v_type,count,indices,vertices);
  todo!()
}

pub unsafe fn sce_gum_draw_array_n(
    _prim: i32,
    _v_type: i32,
    _count: i32,
    _a3: i32,
    _indices: *const c_void,
    _vertices: *const c_void,
) {
  sce_gum_update_matrix();

  // sceGuDrawArrayN(prim,vtype,count,a3,indices,vertices);
  todo!()
}

pub unsafe fn sce_gum_draw_bezier(
    _v_type: i32,
    _u_count: i32,
    _v_count: i32,
    _indices: *const c_void,
    _vertices: *const c_void,
) {
  sce_gum_update_matrix();

  // sceGuDrawBezier(vtype,ucount,vcount,indices,vertices);
  todo!()
}

pub unsafe fn sce_gum_draw_spline(
    _v_type: i32,
    _u_count: i32,
    _v_count: i32,
    _u_edge: i32,
    _v_edge: i32,
    _indices: *const c_void,
    _vertices: *const c_void,
) {
  sce_gum_update_matrix();

  // sceGuDrawSpline(vtype,ucount,vcount,uedge,vedge,indices,vertices);
  todo!()
}

pub unsafe fn sce_gum_fast_inverse() {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        vmmov_t M000, E300;
        vneg_t  C100, C330;
        vtfm3_t C030, M300, C100;
        vmmov_q M300, M000;

        : : : : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_full_inverse() {
    let mut t = MaybeUninit::uninit();

    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        sv_q C300, a0;
        sv_q C310, 16(a0);
        sv_q C320, 32(a0);
        sv_q C330, 48(a0);

        : : "{a0}"(t.as_mut_ptr()) : "memory" : "volatile"
    );

    let t = gum_fast_inverse(&*t.as_ptr());

    vfpu_asm!(
        lv_q C300, t0;
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(a0);

        : : "{t0}"(&t) : "memory" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Load identity matrix
///
/// ```txt
/// [1 0 0 0]
/// [0 1 0 0]
/// [0 0 1 0]
/// [0 0 0 1]
/// ```
pub unsafe fn sce_gum_load_identity() {
    VFPU_CONTEXT
        .get_or_insert_with(Context::new)
        .prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(vmidt_q M300; : : : : "volatile");

    CURRENT_MATRIX_UPDATE = 1;
}

/// Load matrix
///
/// # Parameters
///
/// - `m`: Matrix to load into stack
pub unsafe fn sce_gum_load_matrix(m: &FMatrix4) {
    VFPU_CONTEXT
        .get_or_insert_with(Context::new)
        .prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        lv_q C300,  0(a0);
        lv_q C310, 16(a0);
        lv_q C320, 32(a0);
        lv_q C330, 48(a0);

        : : "{a0}"(m) : "memory" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_look_at(eye: &FVector3, center: &FVector3, up: &FVector3) {
    let mut t = gum_load_identity();
    gum_look_at(&mut t, eye, center, up);

    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        lv_q C000, t0;
        lv_q C010, 16(t0);
        lv_q C020, 32(t0);
        lv_q C030, 48(t0);
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "{t0}"(&t) : : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Select which matrix stack to operate on
///
/// # Parameters
///
/// - `mode`: Matrix mode to use
pub unsafe fn sce_gum_matrix_mode(mode: Mode) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        sv_q C300, t0;
        sv_q C310, 16(t0);
        sv_q C320, 32(t0);
        sv_q C330, 48(t0);

        : : "{t0}"(CURRENT_MATRIX) : "memory" : "volatile"
    );

    MATRIX_UPDATE[CURRENT_MODE as usize] = CURRENT_MATRIX_UPDATE;
    STACK_DEPTH[CURRENT_MODE as usize] = CURRENT_MATRIX;
    CURRENT_MATRIX = STACK_DEPTH[mode as usize];
    CURRENT_MODE = mode;
    CURRENT_MATRIX_UPDATE = MATRIX_UPDATE[CURRENT_MODE as usize];

    vfpu_asm!(
        lv_q C300, t0;
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(t0);

        : : "{t0}"(CURRENT_MATRIX) : "memory" : "volatile"
    );
}

/// Multiply current matrix with input
///
/// # Parameters
///
/// - `m`: Matrix to multiply stack with
pub unsafe fn sce_gum_mult_matrix(m: &FMatrix4) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        lv_q C000,  0(t0);
        lv_q C010, 16(t0);
        lv_q C020, 32(t0);
        lv_q C030, 48(t0);

        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "{t0}"(m) : "memory" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Apply ortho projection matrix
///
/// # Note
///
/// The matrix loses its orthonogal status after executing this function.
pub unsafe fn sce_gum_ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32
) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        .mips "mfc1 $$t1, $1";
        .mips "mfc1 $$t2, $2";
        .mips "mfc1 $$t3, $3";
        .mips "mfc1 $$t4, $4";
        .mips "mfc1 $$t5, $5";

        vmidt_q M100;                         // set M100 to identity
        mtv    t1, S000;                      // C000 = [right, ?,      ?,  ]
        mtv    t3, S001;                      // C000 = [right, top,    ?,  ]
        mtv    t5, S002;                      // C000 = [right, top,    far ]
        mtv    t0, S010;                      // C010 = [left,  ?,      ?,  ]
        mtv    t2, S011;                      // C010 = [left,  bottom, ?,  ]
        mtv    t4, S012;                      // C010 = [left,  bottom, near]
        vsub_t  C020, C000, C010;             // C020 = [  dx,   dy,   dz]
        vrcp_t  C020, C020;                   // C020 = [1/dx, 1/dy, 1/dz]

        vpfxs [2];                            // S100 = m->x.x = 2.0 / dx
        vmul_s S100, S100, S020;

        vpfxs [2];                            // S110 = m->y.y = 2.0 / dy
        vmul_s  S111, S111, S021;

        vpfxs [2];                            // S122 = m->z.z = -2.0 / dz
        vpfxt [-X];
        vmul_s  S122, S122, S022;

        vpfxs [-X], [-Y], [-Z];               // C130 = m->w[x, y, z] = [-(right+left), -(top+bottom), -(far+near)]
        vsub_t  C130, C000, C010;             // we do vsub here since -(a+b) => (-1*a) + (-1*b) => -a - b

        vmul_t  C130, C130, C020;             // C130 = [-(right+left)/dx, -(top+bottom)/dy, -(far+near)/dz]
        vmmul_q M000, M300, M100;
        vmmov_q M300, M000;

        : : "f"(left), "f"(right), "f"(bottom), "f"(top), "f"(near), "f"(far)
        : "t0", "t1", "t2", "t3", "t4", "t5" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Apply perspective projection matrix
///
/// # Note
///
/// The matrix loses its orthonogal status after executing this function.
pub unsafe fn sce_gum_perspective(fovy: f32, aspect: f32, near: f32, far: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        .mips "mfc1 $$t1, $1";
        .mips "mfc1 $$t2, $2";
        .mips "mfc1 $$t3, $3";

        vmzero_q M100;                   // set M100 to all zeros
        mtv     t0, S000;                // S000 = fovy
        viim_s  S001, 90;                // S002 = 90.0f
        vrcp_s  S001, S001;              // S002 = 1/90

        vpfxt [1/2];                     // S000 = fovy * 0.5 = fovy/2
        vmul_s  S000, S000, S000;

        vmul_s  S000, S000, S001;        // S000 = (fovy/2)/90
        vrot_p  C002, S000, [C, S];      // S002 = cos(angle), S003 = sin(angle)
        vdiv_s  S100, S002, S003;        // S100 = m->x.x = cotangent = cos(angle)/sin(angle)
        mtv     t2, S001;                // S001 = near
        mtv     t3, S002;                // S002 = far
        vsub_s  S003, S001, S002;        // S003 = deltaz = near-far
        vrcp_s  S003, S003;              // S003 = 1/deltaz
        mtv     t1, S000;                // S000 = aspect
        vmov_s  S111, S100;              // S111 = m->y.y = cotangent
        vdiv_s  S100, S100, S000;        // S100 = m->x.x = cotangent / aspect
        vadd_s  S122, S001, S002;        // S122 = m->z.z = far + near
        vmul_s  S122, S122, S003;        // S122 = m->z.z = (far+near)/deltaz
        vmul_s  S132, S001, S002;        // S132 = m->w.z = far * near

        vpfxt [2];                       // S132 = m->w.z = 2 * (far*near)
        vmul_s  S132, S132, S132;

        vmul_s  S132, S132, S003;        // S132 = m->w.z = 2 * (far*near) / deltaz

        vpfxt [1];                       // S123 = m->z.w = -1.0
        vsub_s  S123, S123, S123;

        vmmul_q M000, M300, M100;
        vmmov_q M300, M000;

        : : "f"(fovy), "f"(aspect), "f"(near), "f"(far)
        : "t0", "t1", "t2", "t3" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Pop matrix from stack
pub unsafe fn sce_gum_pop_matrix() {
    CURRENT_MATRIX = CURRENT_MATRIX.offset(-1);
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        lv_q C300,  0(t0);
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(t0);

        : : "{t0}"(CURRENT_MATRIX) : : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Push current matrix onto stack
pub unsafe fn sce_gum_push_matrix() {
    CURRENT_MATRIX = CURRENT_MATRIX.offset(1);
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        sv_q C300,  0(t0);
        sv_q C310, 16(t0);
        sv_q C320, 32(t0);
        sv_q C330, 48(t0);

        : : "{t0}"(CURRENT_MATRIX) : "memory" : "volatile"
    );
}

/// Rotate around the X axis
///
/// # Parameters
///
/// - `angle`: Angle in radians
pub unsafe fn sce_gum_rotate_x(angle: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        vmidt_q M000;
        mtv t0, S100;
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        vrot_q C010, S100, [0, C, S, 0];
        vrot_q C020, S100, [0, -S, C, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "f"(angle) : "t0" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Rotate around the Y axis
///
/// # Parameters
///
/// - `angle`: Angle in radians
pub unsafe fn sce_gum_rotate_y(angle: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        vmidt_q M000;
        mtv     t0, S100;
        vcst_s  S101, VFPU_2_PI;
        vmul_s  S100, S101, S100;
        vrot_q  C000, S100, [C, 0,-S, 0];
        vrot_q  C020, S100, [S, 0, C, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "f"(angle) : "t0" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Rotate around the Z axis
///
/// # Parameters
///
/// - `angle`: Angle in radians
pub unsafe fn sce_gum_rotate_z(angle: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        vmidt_q M000;
        mtv     t0, S100;
        vcst_s  S101, VFPU_2_PI;
        vmul_s  S100, S101, S100;
        vrot_q  C000, S100, [ C, S, 0, 0];
        vrot_q  C010, S100, [-S, C, 0, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "f"(angle) : "t0" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Rotate around all 3 axis in order X, Y, Z
///
/// # Parameters
///
/// - `v`: Pointer to vector containing angles
pub unsafe fn sce_gum_rotate_xyz(v: &FVector3) {
    sce_gum_rotate_x(v.x);
    sce_gum_rotate_y(v.y);
    sce_gum_rotate_z(v.z);
}

/// Rotate around all 3 axis in order Z, Y, X
///
/// # Parameters
///
/// - `v`: Pointer to vector containing angles
pub unsafe fn sce_gum_rotate_zyx(v: &FVector3) {
    sce_gum_rotate_z(v.z);
    sce_gum_rotate_y(v.y);
    sce_gum_rotate_x(v.x);
}

/// Scale matrix
///
/// # Note
///
/// The matrix loses its orthonogal status after executing this function.
pub unsafe fn sce_gum_scale(v: &FVector3) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);

    vfpu_asm!(
        lv_q C000, a0;
        vscl_t C300, C300, S000;
        vscl_t C310, C310, S001;
        vscl_t C320, C320, S002;

        : : "{a0}"(v) : : "volatile"
    );
}

/// Store current matrix in the stack
///
/// # Parameters
///
/// - `m`: Matrix to write result to
pub unsafe fn sce_gum_store_matrix(m: &mut FMatrix4) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);

    vfpu_asm!(
        sv_q C300,  0(a0);
        sv_q C310, 16(a0);
        sv_q C320, 32(a0);
        sv_q C330, 48(a0);

        : : "{a0}"(m) : "memory" : "volatile"
    );
}

/// Translate coordinate system
///
/// # Parameters
///
/// - `v`: Translation coordinates
pub unsafe fn sce_gum_translate(v: &FVector3) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        lv_q    C100, a0;
        vmov_t  C030, C100;
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "{a0}"(v) : : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Explicitly flush dirty matrices to the hardware
pub unsafe fn sce_gum_update_matrix() {
    STACK_DEPTH[CURRENT_MODE as usize] = CURRENT_MATRIX;

    if CURRENT_MATRIX_UPDATE == 1 {
        get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

        vfpu_asm!(
            sv_q C300,  0(t0);
            sv_q C310, 16(t0);
            sv_q C320, 32(t0);
            sv_q C330, 48(t0);

            : : "{t0}"(CURRENT_MATRIX) : "memory" : "volatile"
        );

        MATRIX_UPDATE[CURRENT_MODE as usize] = CURRENT_MATRIX_UPDATE;
        CURRENT_MATRIX_UPDATE = 0;
    }

    for i in 0..4 {
        if MATRIX_UPDATE[i] != 0 {
            // TODO: Add this.
            //sce_gu_set_matrix(i, STACK_DEPTH[i]);

            MATRIX_UPDATE[i] = 0;
        }
    }
}

fn gum_normalize(v: &mut FVector3) {
    let l = unsafe {
        use core::intrinsics::sqrtf32;
        sqrtf32((v.x * v.x) + (v.y * v.y) + (v.z * v.z))
    };

    if l > EPSILON {
        let il = 1.0 / l;

        v.x *= il;
        v.y *= il;
        v.z *= il;
    }
}

fn gum_cross_product(a: &FVector3, b: &FVector3) -> FVector3 {
    FVector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

unsafe fn gum_look_at(
    mat: &mut FMatrix4,
    eye: &FVector3,
    center: &FVector3,
    up: &FVector3,
) {
    let mut forward = FVector3 {
        x: center.x - eye.x,
        y: center.y - eye.y,
        z: center.z - eye.z,
    };

    gum_normalize(&mut forward);

    let mut side = gum_cross_product(&forward, &up);
    gum_normalize(&mut side);

    let lup = gum_cross_product(&side, &forward);

    let mut t = gum_load_identity();

    t.x.x = side.x;
    t.y.x = side.y;
    t.z.x = side.z;

    t.x.y = lup.x;
    t.y.y = lup.y;
    t.z.y = lup.z;

    t.x.z = -forward.x;
    t.y.z = -forward.y;
    t.z.z = -forward.z;

    let ieye = FVector3 {
        x: -eye.x,
        y: -eye.y,
        z: -eye.z,
    };

    let mut mat = gum_mult_matrix(mat, &t);
    gum_translate(&mut mat, &ieye);
}

unsafe fn gum_translate(m: &mut FMatrix4, v: &FVector3) {
    get_context_unchecked().prepare(
        MatrixSet::empty(),
        MatrixSet::VMAT0 | MatrixSet::VMAT1 | MatrixSet::VMAT2,
    );

    vfpu_asm!(
        lv_q C100,  0(a0);
        lv_q C110, 16(a0);
        lv_q C120, 32(a0);
        lv_q C130, 48(a0);

        vmidt_q M000;
        lv_q    C200, a1;
        vmov_t  C030, C200;
        vmmul_q M200, M100, M000;

        sv_q C200,  0(a0);
        sv_q C210, 16(a0);
        sv_q C220, 32(a0);
        sv_q C230, 48(a0);

        : : "{a0}"(m), "{a1}"(v) : "memory" : "volatile"
    );
}

unsafe fn gum_load_identity() -> FMatrix4 {
    get_context_unchecked().prepare(MatrixSet::empty(), MatrixSet::VMAT0);

    let mut out = MaybeUninit::uninit();

    vfpu_asm!(
        vmidt_q M000;
        sv_q C000,  0(a0);
        sv_q C010, 16(a0);
        sv_q C020, 32(a0);
        sv_q C030, 48(a0);

        : : "{a0}"(out.as_mut_ptr()) : "memory" : "volatile"
    );

    out.assume_init()
}

unsafe fn gum_fast_inverse(a: &FMatrix4) -> FMatrix4 {
    get_context_unchecked().prepare(
        MatrixSet::empty(),
        MatrixSet::VMAT0 | MatrixSet::VMAT1 | MatrixSet::VMAT2,
    );

    let mut out = MaybeUninit::uninit();

    vfpu_asm!(
        lv_q C200,  0(a1);
        lv_q C210, 16(a1);
        lv_q C220, 32(a1);
        lv_q C230, 48(a1);

        vmidt_q M000;
        vmmov_t M000, E200;
        vneg_t C100, C230;
        vtfm3_t C030, M200, C100;

        sv_q C000,  0(a0);
        sv_q C010, 16(a0);
        sv_q C020, 32(a0);
        sv_q C030, 48(a0);

        : : "{a0}"(out.as_mut_ptr()), "{a1}"(a) : "memory" : "volatile"
    );

    out.assume_init()
}

unsafe fn gum_mult_matrix(a: &FMatrix4, b: &FMatrix4) -> FMatrix4 {
    get_context_unchecked().prepare(
        MatrixSet::empty(),
        MatrixSet::VMAT0 | MatrixSet::VMAT1 | MatrixSet::VMAT2,
    );

    let mut out = MaybeUninit::uninit();

    vfpu_asm!(
        lv_q C000,  0(a1);
        lv_q C010, 16(a1);
        lv_q C020, 32(a1);
        lv_q C030, 48(a1);

        lv_q C100,  0(a2);
        lv_q C110, 16(a2);
        lv_q C120, 32(a2);
        lv_q C130, 48(a2);

        vmmul_q M200, M000, M100;

        sv_q C200,  0(a0);
        sv_q C210, 16(a0);
        sv_q C220, 32(a0);
        sv_q C230, 48(a0);

        : : "{a0}"(&mut out), "{a1}"(a), "{a2}"(b) : "memory" : "volatile"
    );

    out.assume_init()
}
