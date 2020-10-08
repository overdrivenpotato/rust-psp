use core::ptr;
use psp::Align16;
use psp::sys::{
    self, ScePspFVector3,
    GuPrimitive, MipmapLevel, VertexType,
    GuSyncMode, GuSyncBehavior, GuContextType
};

#[repr(align(4))]
pub struct Align4<T>(pub T);

#[repr(C, packed)]
struct Vertex {
    u: f32,
    v: f32,
    color: u32,
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct Sprite<'a, T> {
    texture: &'a T, 
    color: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl<'a, T> Sprite<'a, T> where T: AsRef<[u8]> {
    pub const fn new(texture: &'a T, color: u32, x: u32, y: u32, width: u32, height: u32) -> Self { 
        Self {
            texture,
            color,
            x,
            y,
            width,
            height
        }
    }

    pub fn draw(&self, displaylist: &mut Align16<[u32; 0x40000]>) {
        // build vertices
        let vertices: Align16<[Align4<Vertex>; 2]> = Align16 ([
            Align4(Vertex { 
                u: 0.0,
                v: 0.0,
                color: self.color,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            Align4(Vertex {
                u: self.width as f32,
                v: self.height as f32,
                color: self.color,
                x: self.width as f32,
                y: self.height as f32,
                z: 0.0,
            })
        ]);

        unsafe {
            sys::sceGuStart(GuContextType::Direct, displaylist.0.as_mut_ptr() as *mut _);
            
            sys::sceGumMatrixMode(sys::MatrixMode::Model);
            sys::sceGumLoadIdentity();
            sys::sceGumTranslate(&ScePspFVector3 { x: self.x as f32, y: self.y as f32, z: 0.0});
            // setup texture
            sys::sceGuTexImage(MipmapLevel::None, self.width as i32, self.height as i32, self.width as i32, self.texture.as_ref().as_ptr() as *const _); 
            sys::sceGuTexScale(1.0/self.width as f32, 1.0/self.height as f32);

            sys::sceKernelDcacheWritebackInvalidateAll();

            // draw sprite
            sys::sceGumDrawArray(
                GuPrimitive::Sprites,
                VertexType::TEXTURE_32BITF | VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
                2,
                ptr::null_mut(),
                &vertices as *const Align16<_> as *const _,
            );	
            sys::sceGuFinish();
            sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

        }
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_pos(&mut self) -> (u32, u32) {
        (self.x, self.y)
    }
}
