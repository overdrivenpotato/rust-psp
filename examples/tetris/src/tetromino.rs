use crate::sprite::Sprite;
use crate::BLOCK;
use crate::BLOCK_SIZE;

#[repr(C)]
pub struct Tetromino<'a> {
    blocks: [Sprite<'a, [u8; BLOCK_SIZE as usize * BLOCK_SIZE as usize * 4]>; 4],
} 

impl<'a> Tetromino <'a> {
    pub fn new_o() -> Tetromino<'a> {
        Tetromino {
           blocks: [
               Sprite::new(&BLOCK.0, 0xff00ffff, 0, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff00ffff, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff00ffff, 0, 0, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff00ffff,  BLOCK_SIZE, 0, BLOCK_SIZE, BLOCK_SIZE),
            ]
        }
    }

    pub fn new_i() -> Tetromino<'a> {
        Tetromino {
           blocks: [
               Sprite::new(&BLOCK.0, 0xffffff00, 0, 0, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffffff00, 0, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffffff00, 0, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffffff00, 0, BLOCK_SIZE*3, BLOCK_SIZE, BLOCK_SIZE),
            ]
        }
    }

    pub fn new_s() -> Tetromino<'a> {
        Tetromino {
           blocks: [
               Sprite::new(&BLOCK.0, 0xff0000ff, BLOCK_SIZE, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff0000ff, 0, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff0000ff, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff0000ff, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ]
        }
    }

    pub fn new_z() -> Tetromino<'a> {
        Tetromino {
           blocks: [
               Sprite::new(&BLOCK.0, 0xff00ff00, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff00ff00, BLOCK_SIZE, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff00ff00, 0, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff00ff00, BLOCK_SIZE*2, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE),
            ]
        }
    }

    pub fn new_l() -> Tetromino<'a> {
        Tetromino {
           blocks: [
               Sprite::new(&BLOCK.0, 0xff008cff, BLOCK_SIZE, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff008cff, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff008cff, BLOCK_SIZE, 0, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xff008cff, 0, 0, BLOCK_SIZE, BLOCK_SIZE),
            ]
        }
    }

    pub fn new_j() -> Tetromino<'a> {
        Tetromino {
           blocks: [
               Sprite::new(&BLOCK.0, 0xffff00ff, BLOCK_SIZE, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffff00ff, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffff00ff, BLOCK_SIZE, 0, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffff00ff,  BLOCK_SIZE*2, 0, BLOCK_SIZE, BLOCK_SIZE),
            ]
        }
    }

    pub fn new_t() -> Tetromino<'a> {
        Tetromino {
           blocks: [
               Sprite::new(&BLOCK.0, 0xffff0000, BLOCK_SIZE*2, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffff0000,  BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffff0000, 0, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
               Sprite::new(&BLOCK.0, 0xffff0000, BLOCK_SIZE, 0, BLOCK_SIZE, BLOCK_SIZE),
            ]
        }
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        for block in self.blocks.iter_mut() {
            let (block_x, block_y) = block.get_pos();
            block.set_pos(block_x+x*BLOCK_SIZE, block_y + y*BLOCK_SIZE);
        }
    }

    pub fn draw(&self) {
        for block in self.blocks.iter() {
            block.draw();
        }
    }
}
