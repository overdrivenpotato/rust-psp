use crate::graphics::{BLOCK, Align4, sprite::{Sprite, Vertex}};
use crate::BLOCK_SIZE;
use crate::gameboard::Gameboard;
use crate::GAMEBOARD_OFFSET;

use rand_chacha::ChaChaRng;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
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

    pub fn new_random(rng: &mut ChaChaRng) -> Self {
       let rand_num = rng.gen_range(0, 7); 
       match rand_num {
           1 => Tetromino::new_o(),
           2 => Tetromino::new_i(),
           3 => Tetromino::new_s(),
           4 => Tetromino::new_z(),
           5 => Tetromino::new_l(),
           6 => Tetromino::new_j(),
           _ => Tetromino::new_t(),
        }
    }

    pub fn as_sprites<'a>(&self) -> [Sprite<'a, [u8; BLOCK_SIZE as usize * BLOCK_SIZE as usize * 4]>; 4] {
        [
            Sprite::new(&BLOCK, self.color, self.block_locs[0].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[0].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Sprite::new(&BLOCK, self.color, self.block_locs[1].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[1].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Sprite::new(&BLOCK, self.color, self.block_locs[2].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[2].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
            Sprite::new(&BLOCK, self.color, self.block_locs[3].0*BLOCK_SIZE as i32+self.x*BLOCK_SIZE as i32, self.block_locs[3].1*BLOCK_SIZE as i32+self.y*BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE),
        ]
    }

    pub fn as_vertices(&self) -> [Align4<Vertex>; 8] {
        let mut ret = [Align4(Vertex::default()); 8];
        self.as_sprites()
            .iter()
            .flat_map(|s| s.as_vertex_iter())
            .zip(ret.iter_mut())
            .for_each(|(v, dst)| *dst = v);
        ret 
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

    pub fn lock_to_gameboard(&self, gameboard: &mut Gameboard) {
        for block_loc in self.block_locs.iter() {
            gameboard.set_content((block_loc.0+self.x-GAMEBOARD_OFFSET.0 as i32) as usize, (block_loc.1+self.y - GAMEBOARD_OFFSET.1 as i32) as usize, Some(self.color)).unwrap();
        }
    }

    pub fn add_pos(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn get_mapped_locs(&self) -> [(usize, usize) ; 4] {
        [
            ((self.block_locs[0].0 + self.x) as usize - GAMEBOARD_OFFSET.0, (self.block_locs[0].1 + self.y) as usize - GAMEBOARD_OFFSET.1),
            ((self.block_locs[1].0 + self.x) as usize - GAMEBOARD_OFFSET.0, (self.block_locs[1].1 + self.y) as usize - GAMEBOARD_OFFSET.1),
            ((self.block_locs[2].0 + self.x) as usize - GAMEBOARD_OFFSET.0, (self.block_locs[2].1 + self.y) as usize - GAMEBOARD_OFFSET.1),
            ((self.block_locs[3].0 + self.x) as usize - GAMEBOARD_OFFSET.0, (self.block_locs[3].1 + self.y) as usize - GAMEBOARD_OFFSET.1),
        ]
    }
}

