use core::ptr;
use psp::Align16;
use psp::sys::{
    self, ScePspFVector3, GuContextType, GuSyncMode, GuSyncBehavior,
    GuPrimitive, TextureFilter, TextureEffect, TextureColorComponent,
    TexturePixelFormat, MipmapLevel, VertexType,
};

#[repr(C, align(4))]
struct Vertex {
    u: f32,
    v: f32,
    color: u32,
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct Sprite<T> {
    texture: T, 
    color: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl<T> Sprite<T> where T: AsRef<[u8]> {
    pub const fn new(texture: T, color: u32, x: u32, y: u32, width: u32, height: u32) -> Self { 
        Self {
            texture,
            color,
            x,
            y,
            width,
            height
        }
    }

    pub fn draw(&self, displaylist: &mut [u32; 0x40000]) {
        // build vertices
        let vertices: Align16<[Vertex; 2]> = Align16 ([
            Vertex { 
                u: 0.0,
                v: 0.0,
                color: self.color,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vertex {
                u: self.width as f32,
                v: self.height as f32,
                color: self.color,
                x: self.width as f32,
                y: self.height as f32,
                z: 1.0,
            }
        ]);

        unsafe {
            sys::sceGuStart(GuContextType::Direct, displaylist.as_mut_ptr() as *mut _);

            // setup matrices for cube

            sys::sceGumMatrixMode(sys::MatrixMode::Projection);
            sys::sceGumLoadIdentity();
            sys::sceGumOrtho(0.0,480.0,272.0,0.0,-30.0,30.0);

            sys::sceGumMatrixMode(sys::MatrixMode::View);
            sys::sceGumLoadIdentity();

            sys::sceGumMatrixMode(sys::MatrixMode::Model);
            sys::sceGumLoadIdentity();
            sys::sceGumTranslate(&ScePspFVector3 { x: self.x as f32, y: self.y as f32, z: 0.0});

            // setup texture
            sys::sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
            sys::sceGuTexImage(MipmapLevel::None, self.width as i32, self.height as i32, self.width as i32, self.texture.as_ref().as_ptr() as *const _); 

            sys::sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgb);
            sys::sceGuTexFilter(TextureFilter::Nearest, TextureFilter::Nearest);
            sys::sceGuTexScale(1.0/self.width as f32, 1.0/self.height as f32);
            sys::sceGuTexOffset(0.0, 0.0);

            // draw sprite
            sys::sceGumDrawArray(
                GuPrimitive::Sprites,
                VertexType::TEXTURE_32BITF | VertexType::VERTEX_32BITF | VertexType::COLOR_8888 | VertexType::TRANSFORM_3D,
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
