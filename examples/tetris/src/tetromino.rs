use crate::sprite::Sprite;
use psp::Align16;
use crate::BLOCK;
use crate::BLOCK_SIZE;

pub struct Tetromino {
    x: i32,
    y: i32,
    color: u32,
    block_locs: [(i32, i32); 4]
}

impl Tetromino {
    pub fn new_o() -> Self {
        Self {
            x: 0,
            y: 0,
            color: 0xff00ffff,
            block_locs: [
                (0, 1),
                (1, 1),
                (0, 0),
                (1, 0),
            ]
        }
    }

    pub fn new_i() -> Self {
        Self {
            x: 0,
            y: 0,
            color: 0xffffff00,
            block_locs: [
                (0, 0),
                (0, 1),
                (0, 2),
                (0, -1),
            ]
        }
    }

    pub fn new_s() -> Self {
        Self {
            x: 0,
            y: 0,
            color: 0xff0000ff,
            block_locs: [
                (0, 1),
                (-1, 1),
                (0, 0),
                (1, 0),
            ]
        }
    }

    pub fn new_z() -> Self {
        Self {
            x: 0,
            y: 0,
            color: 0xff00ff00,
            block_locs: [
                (0, 0),
                (0, 1),
                (-1, 0),
                (1, 1),
            ]
        }
    }

    pub fn new_l() -> Self {
        Self {
            x: 0,
            y: 0,
            color: 0xff008cff,
            block_locs: [
                (0, 1),
                (0, 0),
                (0, -1),
                (-1, -1),
            ]
        }
    }

    pub fn new_j() -> Self {
        Self {
            x: 0,
            y: 0,
            color: 0xffff00ff,
            block_locs: [
                (0, 1),
                (0, 0),
                (0, -1),
                (1, -1),
            ]
        }
    }

    pub fn new_t() -> Self {
        Self {
            x: 0,
            y: 0,
            color: 0xffff0000,
            block_locs: [
                (1, 0),
                (0, 0),
                (-1, 0),
                (0, -1),
            ]
        }
    }

    fn as_sprites<'a>(&self) -> [Sprite<'a, [u8; BLOCK_SIZE as usize * BLOCK_SIZE as usize * 4]>; 4] {
        [
            Sprite::new(&BLOCK.0, self.color, self.block_locs[0].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[0].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Sprite::new(&BLOCK.0, self.color, self.block_locs[1].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[1].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Sprite::new(&BLOCK.0, self.color, self.block_locs[2].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[2].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Sprite::new(&BLOCK.0, self.color, self.block_locs[3].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[3].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ]
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_pos(&mut self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn rotate_ccw(&mut self) {
        for i in 0..4 {
           self.block_locs[i] = (self.block_locs[i].1, self.block_locs[i].0);
        }
    }

    pub fn rotate_cw(&mut self) {
        for i in 0..4 {
           self.block_locs[i] = (0-self.block_locs[i].1, self.block_locs[i].0);
        }
    }

    pub fn draw(&self, displaylist: &mut Align16<[u32;0x40000]>) {
        for block in self.as_sprites().iter_mut() {
            block.set_scale(0.75);
            block.draw(displaylist);
        }
    }   
}

