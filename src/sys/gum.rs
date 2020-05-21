use crate::vfpu_asm;
use core::ptr;

type VfpuMatrixSet = u8;
pub struct VfpuContext{}

static mut GUM_VFPU_CONTEXT: *mut VfpuContext = ptr::null_mut();
static mut GUM_CURRENT_MATRIX_UPDATE: i32 = 0;
static mut GUM_CURRENT_MATRIX: *mut FMatrix4 = ptr::null_mut();

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

#[repr(C)]
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

#[repr(C)]
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

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct FMatrix4 {
    pub x: FVector4,
    pub y: FVector4,
    pub z: FVector4,
    pub w: FVector4,
}

#[repr(C)]
#[repr(align(16))]
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

pub const VMAT0: u8 = 1<<0;
pub const VMAT1: u8 = 1<<1;
pub const VMAT2: u8 = 1<<2;
pub const VMAT3: u8 = 1<<3;
pub const VMAT4: u8 = 1<<4;
pub const VMAT5: u8 = 1<<5;
pub const VMAT6: u8 = 1<<6;
pub const VMAT7: u8 = 1<<7;

pub fn psp_vfpu_use_matrices(
    c: *mut VfpuContext,
    keep_set: VfpuMatrixSet,
    temp_set: VfpuMatrixSet,
) {
    unimplemented!()
}

pub fn gum_aligned_matrix() -> *mut FMatrix4 {
    unimplemented!()
}

pub unsafe fn gum_scale(m: *mut FMatrix4, v: *const FVector3) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, 0, VMAT0 | VMAT1);
    vfpu_asm!(
        .mips "nop";
        //ulv_q C100,  0;
        //ulv_q C110, 16;
        //ulv_q C120, 32;
        //ulv_q C130, 48;

        //ulv_q C000, %1;

        vscl_t C100, C100, S000;
        vscl_t C110, C110, S001;
        vscl_t C120, C120, S002;

        //usv_q C100,  0;
        //usv_q C110, 16;
        //usv_q C120, 32;
        //usv_q C130, 48;
        //: "+m"(*m) : "m"(*v)
    );
}

pub unsafe fn gum_translate(m: *mut Matrix4, v: *const Vector3) {
    psp_vfpu_use_matrices(
        GUM_VFPU_CONTEXT,
        0,
        VMAT0 | VMAT1 | VMAT2
    );
    
    vfpu_asm!(
        //ulv_q C100,  0;
        //ulv_q C110, 16;
        //ulv_q C120, 32;
        //ulv_q C130, 48;

        vmidt_q M000;
        //ulv_q   C200, 1;
        //vmov_t  C030, C200;
        vmmul_q M200, M100, M000;

        //usv_q C200,  0;
        //usv_q C210, 16;
        //usv_q C220, 32;
        //usv_q C230, 48;
        : "+m"(*m) : "m"(*v)
    );
}

pub unsafe fn gum_rotate_x(m: *mut FMatrix4, angle: f32) {
     psp_vfpu_use_matrices(
        GUM_VFPU_CONTEXT,
        0,
        VMAT0 | VMAT1 | VMAT2
    );
    
    vfpu_asm!(
        //ulv_q C200,  0;
        //ulv_q C210, 1;
        //ulv_q C220, 32;
        //ulv_q C230, 48;

        vmidt_q M000;
        mtv t1, S100; // the t1 formerly known as %1
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        //vrot_q C010, S100, [ 0, c, s, 0];
        //vrot_q C020, S100, [ 0,-s, c, 0];
        vmmul_q M100, M200, M000;

        //"usv.q C100,  0 + %0;
        //"usv.q C110, 16 + %0;
        //"usv.q C120, 32 + %0;
        //"usv.q C130, 48 + %0;
        : "+m"(*m) : "r"(angle)
    );
}

pub unsafe fn gum_rotate_y(m: *mut FMatrix4, angle: f32) {
     psp_vfpu_use_matrices(
        GUM_VFPU_CONTEXT,
        0,
        VMAT0 | VMAT1 | VMAT2
    );
    
    vfpu_asm!(
        //ulv_q C200,  0;
        //ulv_q C210, 16;
        //ulv_q C220, 32;
        //ulv_q C230, 48;

        vmidt_q M000;
        mtv  t1, S100; // the t1 formerly knows as %1
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        //vrot_q C000, S100, [ c, 0,-s, 0];
        //vrot_q C020, S100, [ s, 0, c, 0];
        vmmul_q M100, M200, M000;

        //usv_q C100,  0;
        //usv_q C110, 16;
        //usv_q C120, 32;
        //usv_q C130, 48;
        : "+m"(*m) : "r"(angle)
    );
}

pub unsafe fn gum_rotate_z(m: *mut FMatrix4, angle: f32) {
    psp_vfpu_use_matrices(
        GUM_VFPU_CONTEXT,
        0,
        VMAT0 | VMAT1 | VMAT2
    );

    vfpu_asm!(
        //ulv_q C200,  0;
        //ulv_q C210, 16;
        //ulv_q C220, 32;
        //ulv_q C230, 48;

        vmidt_q M000;
        mtv  t1, S100; // the t1 formerly known as %1
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        //vrot_q C000, S100, [ c, s, 0, 0];
        //vrot_q C010, S100, [-s, c, 0, 0];
        vmmul_q M100, M200, M000;

        //usv_q C100,  0;
        //usv_q C110, 16;
        //usv_q C120, 32;
        //usv_q C130, 48;
	: "+m"(*m) : "r"(angle)
    );
}
 
pub unsafe fn gum_load_identity(m: *mut FMatrix4)
{
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, 0, VMAT0);

    vfpu_asm!(
        vmidt_q M000;
        //usv_q C000,  0;
        //usv_q C010, 16;
        //usv_q C020, 32;
        //usv_q C030, 48;
        : "=m"(*m) : : "memory" 
    );
}

pub unsafe fn gum_fast_inverse(m: *mut FMatrix4, a: *const FMatrix4)
{
    psp_vfpu_use_matrices(
        GUM_VFPU_CONTEXT,
        0,
        VMAT0 | VMAT1 | VMAT2
    );

    vfpu_asm!(
        //ulv_q C200,  0 + %1;
        //ulv_q C210, 16 + %1;
        //ulv_q C220, 32 + %1;
        //ulv_q C230, 48 + %1;

        vmidt_q M000;
        //vmmov_t M000, E200;
        vneg_t C100, C230;
        vtfm3_t C030, M200, C100;

        //usv_q C000,  0;
        //usv_q C010, 16;
        //usv_q C020, 32;
        //usv_q C030, 48;
        : "=m"(*m) : "m"(*a) : "memory" 
    );
}

pub unsafe fn gum_mult_matrix(
    result: *mut FMatrix4,
    a: *mut FMatrix4,
    b: *mut FMatrix4
) {
    psp_vfpu_use_matrices(
        GUM_VFPU_CONTEXT,
        0,
        VMAT0 | VMAT1 | VMAT2
    );

    vfpu_asm!(
        //ulv_q C000,  0 + %1;
        //ulv_q C010, 16 + %1;
        //ulv_q C020, 32 + %1;
        //ulv_q C030, 48 + %1;

        //ulv_q C100,  0 + %2;
        //ulv_q C110, 16 + %2;
        //ulv_q C120, 32 + %2;
        //ulv_q C130, 48 + %2;

        vmmul_q M200, M000, M100;

        //usv_q C200,  0;
        //usv_q C210, 16;
        //usv_q C220, 32;
        //usv_q C230, 48;
        : "=m"(*result) : "m;"(*a), "m"(*b) : "memory"
    );
}

pub unsafe fn sce_gum_fast_inverse() {
    psp_vfpu_use_matrices(
        GUM_VFPU_CONTEXT,
        VMAT3,
        VMAT0 | VMAT1
    );

    vfpu_asm!(
        vmidt_q M000;
        //vmmov_t M000, E300;
        vneg_t  C100, C330;
        vtfm3_t C030, M300, C100;
        //vmmov_q M300, M000;
    );

    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_full_inverse() {
    let t = gum_aligned_matrix();
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);
    
    vfpu_asm!(
        sv_q C300,  t0; // the t0s formerly knows as %0
        sv_q C310, 16(t0);
        sv_q C320, 32(t0);
        sv_q C330, 48(t0);
	: "=m"(*t) : : "memory"
    );

    gum_fast_inverse(t, t);

    vfpu_asm!(
        lv_q C300,  t0; // the t0s formerly knows as %0
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(t0);
        : : "m"(*t) : "memory"
    );

    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_load_identity() {
   //if (gum_vfpucontext == NULL)
   //    gum_vfpucontext = pspvfpu_initcontext(); 
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, 0);
    vfpu_asm!(
        vmidt_q M300;
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_load_matrix(m: *const FMatrix4) {
    //if (gum_vfpucontext == NULL)
    //    gum_vfpucontext = pspvfpu_initcontext(); 
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, 0);
    vfpu_asm!(
        .mips "nop";
        //ulv_q C300.q,  0;
        //ulv_q C310.q, 16;
        //ulv_q C320.q, 32;
        //ulv_q C330.q, 48;
	: : "m"(*m) : "memory"
    );
   GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_look_at(
    eye: *mut FVector3,
    center: *mut FVector3,
    up: *mut FVector3
) {
    let t = gum_aligned_matrix();

    gum_load_identity(t);
    gum_look_at(t, eye, center, up);

    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);

    vfpu_asm!(
         lv_q C000, t0; // TODO t0
         lv_q C010, 16(t0);
         lv_q C020, 32(t0);
         lv_q C030, 48(t0);
         vmmul_q M100, M300, M000;
         vmmov_q M300, M100;
        : : "m"(*t) 
   );

   GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_matrix_mode(mode: i32) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, 0);
    vfpu_asm!(
        sv_q C300,  t0; // TODO t0
        sv_q C310, 16(t0);
        sv_q C320, 32(t0);
        sv_q C330, 48(t0);
	: "=m"(*GUM_CURRENT_MATRIX) : : "memory"
    );

    // fuck this shit
    //gum_matrix_update[gum_current_mode] = gum_current_matrix_update;
    //gum_stack_depth[gum_current_mode] = gum_current_matrix;
    //gum_current_matrix = gum_stack_depth[mode];
    //gum_current_mode = mode;
    //gum_current_matrix_update = gum_matrix_update[gum_current_mode];
    
    vfpu_asm!(
        lv_q C300,  0(t0); //TODO t0
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(t0);
	: : "m"(*GUM_CURRENT_MATRIX) : "memory"
    );
}

pub unsafe fn sce_gum_mult_matrix(m: *const FMatrix4) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);
    vfpu_asm!(
	//ulv_q C000,  0;
        //ulv_q C010, 16;
        //ulv_q C020, 32;
        //ulv_q C030, 48;
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;
	: : "m"(*m)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32
) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);
    
    vfpu_asm!(
        vmidt_q M100;                       // set M100 to identity
        mtv    t1, S000;                   // C000 = [right, ?,      ?,  ]
        mtv    t3, S001;                    // C000 = [right, top,    ?,  ]
        mtv    t5, S002;                    // C000 = [right, top,    far ]
        mtv    t0, S010;                    // C010 = [left,  ?,      ?,  ]
        mtv    t2, S011;                    // C010 = [left,  bottom, ?,  ]
        mtv    t4, S012;                    // C010 = [left,  bottom, near]
        vsub_t  C020, C000, C010;            // C020 = [  dx,   dy,   dz]
        vrcp_t  C020, C020;                  // C020 = [1/dx, 1/dy, 1/dz]
        //vmul_s  S100, S100[2], S020;         // S100 = m->x.x = 2.0 / dx
        //vmul_s  S111, S111[2], S021;         // S110 = m->y.y = 2.0 / dy
        //vmul_s  S122, S122[2], S022[-x];     // S122 = m->z.z = -2.0 / dz
        //vsub_t  C130, C000[-x,-y,-z], C010; // C130 = m->w[x, y, z] = [-(right+left), -(top+bottom), -(far+near)]
                                                // we do vsub here since -(a+b) => (-1*a) + (-1*b) => -a - b
        vmul_t  C130, C130, C020;           // C130 = [-(right+left)/dx, -(top+bottom)/dy, -(far+near)/dz]
	vmmul_q M000, M300, M100;
	vmmov_q M300, M000;
    : : "r"(left), "r"(right), "r"(bottom), "r"(top), "r"(near), "r"(far)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_perspective(fovy: f32, aspect: f32, near: f32, far: f32) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);

    vfpu_asm!(
        vmzero_q M100;                  // set M100 to all zeros
        mtv     t0, S000;                // S000 = fovy
        viim_s  S001, 90;               // S002 = 90.0f
        vrcp_s  S001, S001;              // S002 = 1/90
        //vmul_s  S000, S000, S000[1/2];   // S000 = fovy * 0.5 = fovy/2
        vmul_s  S000, S000, S001;        // S000 = (fovy/2)/90
        //vrot_p  C002, S000, [c, s];      // S002 = cos(angle), S003 = sin(angle)
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
        //vmul_s  S132, S132, S132[2];     // S132 = m->w.z = 2 * (far*near)
        vmul_s  S132, S132, S003;        // S132 = m->w.z = 2 * (far*near) / deltaz
        //vsub_s  S123, S123, S123[1];     // S123 = m->z.w = -1.0
	vmmul_q M000, M300, M100;
        vmmov_q M300, M000;
        : : "r"(fovy),"r"(aspect),"r"(near),"r"(far)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_pop_matrix() {
    GUM_CURRENT_MATRIX = GUM_CURRENT_MATRIX.offset(-1);
    let m = GUM_CURRENT_MATRIX;
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, 0);

    vfpu_asm!(
        lv_q C300,  0(t0); //TODO t0
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(t0);
	: : "m"(*m)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_push_matrix() {
    GUM_CURRENT_MATRIX = GUM_CURRENT_MATRIX.offset(1);
    let m = GUM_CURRENT_MATRIX;
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, 0);

    vfpu_asm!(
        sv_q C300,  0(t0); // TODO t0
        sv_q C310, 16(t0);
        sv_q C320, 32(t0);
        sv_q C330, 48(t0);
	: "=m"(*m) : : "memory"
    );
}

pub unsafe fn sce_gum_rotate_x(angle: f32) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        mtv t0, S100;
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        //vrot_q C010, S100, [ 0, c, s, 0];
        //vrot_q C020, S100, [ 0,-s, c, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;
	: : "r"(angle)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_rotate_y(angle: f32) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        mtv t0, S100;
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        //vrot_q C000, S100, [ c, 0,-s, 0];
        //vrot_q C020, S100, [ s, 0, c, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;
	: : "r"(angle)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_rotate_z(angle: f32) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        mtv t0, S100;
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        //vrot_q C000, S100, [ c, s, 0, 0];
        //vrot_q C010, S100, [-s, c, 0, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;
	: : "r"(angle)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_scale(v: *const FVector3) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0);

    vfpu_asm!(
        //ulv_q C000, t0; //TODO t0;
        vscl_t C300, C300, S000;
        vscl_t C310, C310, S001;
        vscl_t C320, C320, S002;
        : : "m"(*v)
    );
}

pub unsafe fn sce_gum_store_matrix(m: *mut FMatrix4) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0);

    vfpu_asm!(
        .mips "nop";
        //usv_q C300,  0;
        //usv_q C310, 16;
        //usv_q C320, 32;
        //usv_q C330, 48;
	: "=m"(*m) : : "memory"
    );
}

pub unsafe fn sce_gum_translate(v: *const FVector3) {
    psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, VMAT0 | VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        //ulv_q   C100;
        vmov_t  C030, C100;
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;
	: : "m"(*v)
    );
    GUM_CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_update_matrix() {
    //gum_stack_depth[gum_current_mode] = GUM_CURRENT_MATRIX;

    if GUM_CURRENT_MATRIX_UPDATE == 1 {
        psp_vfpu_use_matrices(GUM_VFPU_CONTEXT, VMAT3, 0);
        vfpu_asm!(
            sv_q C300,  0(t0); //TODO t0
            sv_q C310, 16(t0);
            sv_q C320, 32(t0);
            sv_q C330, 48(t0);
            : "=m"(*GUM_CURRENT_MATRIX) : : "memory"
        );
        //gum_matrix_update[gum_current_mode] = gum_current_matrix_update;
        GUM_CURRENT_MATRIX_UPDATE = 0;
    }

    for i in 0..4 {
        //if gum_matrix_update[i] {
            //sce_gu_set_matrix(i, gum_stack_depth[i]);
            //gum_matrix_update[i] = 0;
        //}
    }
}
