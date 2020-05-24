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
