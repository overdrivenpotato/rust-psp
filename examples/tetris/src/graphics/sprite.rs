use core::{ptr, f32::consts::PI};
use psp::Align16;
use psp::sys::{
    self, ScePspFVector3,
    GuPrimitive, MipmapLevel, VertexType,
    GuSyncMode, GuSyncBehavior, GuContextType
};

use crate::graphics::Align4;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vertex {
    pub u: f32,
    pub v: f32,
    pub color: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct Sprite<'a, T> {
    texture: &'a T, 
    color: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    rotation_radians: f32,
    scale: f32,
}

impl<'a, T> Sprite<'a, T> where T: AsRef<[u8]> {
    pub const fn new(texture: &'a T, color: u32, x: i32, y: i32, width: u32, height: u32) -> Self { 
        Self {
            texture,
            color,
            x,
            y,
            width,
            height,
            rotation_radians: 0.0,
            scale: 1.0,
        }
    }

    pub fn as_vertex_iter(&self) -> impl Iterator<Item = Align4<Vertex>> {
        Some(Align4(Vertex { 
            u: 0.0,
            v: 0.0,
            color: self.color,
            x: self.x as f32,
            y: self.y as f32,
            z: 0.0,
        })).into_iter()
        .chain(
            Some(Align4(
                Vertex {
                    u: self.width as f32,
                    v: self.height as f32,
                    color: self.color,
                    x: self.x as f32 + self.width as f32,
                    y: self.y as f32 + self.height as f32,
                    z: 0.0,
                })
            )
        )
    }

    /// Don't use this if you're drawing many sprites, it's really slow
    /// Use fn as_vertex_iter() and collect your sprites into a single buffer to 
    /// draw all at once. Note: as_vertex_iter() does not preserve scaling or rotation.
    pub fn draw(&self, displaylist: &mut Align16<[u32; 0x40000]>) {
        use core::convert::TryInto;
        let vertex_array: alloc::boxed::Box<[Align4<Vertex>; 2]> = self.as_vertex_iter()
            .collect::<alloc::vec::Vec<Align4<Vertex>>>()
            .into_boxed_slice().try_into().unwrap();

        let vertices: Align16<[Align4<Vertex>;2]> = Align16(*vertex_array);

        unsafe {
            sys::sceGuStart(GuContextType::Direct, displaylist.0.as_mut_ptr() as *mut _);
            
            sys::sceGumMatrixMode(sys::MatrixMode::Model);
            sys::sceGumLoadIdentity();
            sys::sceGumScale(&ScePspFVector3 { x: self.scale, y: self.scale, z: 1.0 });
            sys::sceGumRotateZ(self.rotation_radians);
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

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_pos(&mut self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_rotation_radians(&mut self, radians: f32) { 
        self.rotation_radians = radians;
    }

    pub fn get_rotation_radians(&mut self) -> f32 { 
        self.rotation_radians
    }

    pub fn set_rotation_degrees(&mut self, degrees: f32) { 
        self.rotation_radians = degrees * (PI / 180.0);
    }

    pub fn get_rotation_degrees(&mut self) -> f32 { 
        self.rotation_radians * (180.0 / PI)
    }

    pub fn set_scale(&mut self, scale: f32) { 
        self.scale = scale;
    }
}

impl<'a, T> Clone for Sprite<'a, T> where T: AsRef<[u8]> {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            color: self.color,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            rotation_radians: self.rotation_radians,
            scale: self.scale,
        }
    }
}

impl <'a, T> Copy for Sprite<'a, T> where T: AsRef<[u8]> {}
