use crate::graphics::Align4;
use core::f32::consts::PI;

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
pub struct Sprite {
    color: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    rotation_radians: f32,
    scale: f32,
}

impl Sprite {
    /// Creates a new `Sprite`.
    ///
    /// # Parameters
    ///
    /// - `texture`: A reference to a 16-byte aligned `u8` buffer of texture pixels.
    /// Only used by the `draw` function.
    /// - `color`: A 32-bit color in Big-Endian ABGR format (little-endian RGBA).
    /// - `x`: Starting horizontal position, in screen coordinates.
    /// - `y`: Starting vertical position, in screen coordinates.
    /// - `width`: Width of the `Sprite`. Must be a power of 2.
    /// - `height`: Height of the `Sprite`. Must be a power of 2.
    pub const fn new(color: u32, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            color,
            x,
            y,
            width,
            height,
            rotation_radians: 0.0,
            scale: 1.0,
        }
    }

    #[allow(dead_code)]
    /// Returns an `Iterator` over the vertices of a `Sprite`. See `Sprite::Vertex`.
    /// Vertices are aligned to 4 bytes.
    pub fn as_vertex_iter(&self) -> impl Iterator<Item = Align4<Vertex>> {
        Some(Align4(Vertex {
            u: 0.0,
            v: 0.0,
            color: self.color,
            x: self.x as f32,
            y: self.y as f32,
            z: 0.0,
        }))
        .into_iter()
        .chain(Some(Align4(Vertex {
            u: self.width as f32,
            v: self.height as f32,
            color: self.color,
            x: self.x as f32 + self.width as f32,
            y: self.y as f32 + self.height as f32,
            z: 0.0,
        })))
    }

    /// Returns an array of 2 vertices that make up the `Sprite`. See `Sprite::Vertex`.
    /// Vertices are aligned to 4 bytes.
    pub fn as_vertices(&self) -> [Align4<Vertex>; 2] {
        [
            Align4(Vertex {
                u: 0.0,
                v: 0.0,
                color: self.color,
                x: self.x as f32,
                y: self.y as f32,
                z: 0.0,
            }),
            Align4(Vertex {
                u: self.width as f32,
                v: self.height as f32,
                color: self.color,
                x: self.x as f32 + self.width as f32,
                y: self.y as f32 + self.height as f32,
                z: 0.0,
            }),
        ]
    }

    /// Sets the position of a `Sprite`. Position is in screen units.
    ///
    /// # Parameters
    ///
    /// - `x`: Position on the horizontal axis.
    /// - `y`: Position on the vertical axis.
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    #[allow(dead_code)]
    /// Gets the position of a `Sprite`. Position is in screen units.
    ///
    /// # Return Value
    ///
    /// A tuple containing the horizontal position in the first element and the vertical
    /// position in the second element, (x, y).
    pub fn get_pos(&mut self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[allow(dead_code)]
    /// Sets rotation of the `Sprite`. Only used by the `draw` function.
    ///
    /// # Parameters
    ///
    /// - `radians`: Rotation in units of radians.
    pub fn set_rotation_radians(&mut self, radians: f32) {
        self.rotation_radians = radians;
    }

    #[allow(dead_code)]
    /// Gets rotation of the `Sprite`.
    ///
    /// # Return value
    ///
    /// Rotation in units of radians.
    pub fn get_rotation_radians(&mut self) -> f32 {
        self.rotation_radians
    }

    #[allow(dead_code)]
    /// Sets rotation of the `Sprite`. Only used by the `draw` function.
    ///
    /// # Parameters
    ///
    /// - `degrees`: Rotation in units of degrees.
    pub fn set_rotation_degrees(&mut self, degrees: f32) {
        self.rotation_radians = degrees * (PI / 180.0);
    }

    #[allow(dead_code)]
    /// Gets rotation of the `Sprite`.
    ///
    /// # Return value
    ///
    /// Rotation in units of degrees.
    pub fn get_rotation_degrees(&mut self) -> f32 {
        self.rotation_radians * (180.0 / PI)
    }

    #[allow(dead_code)]
    /// Sets scale of the `Sprite`. Only used by the `draw` function.
    ///
    /// # Parameters
    ///
    /// - `scale`: Scale factor, 1.0 is 100% scale.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Self {
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

impl Copy for Sprite {}
