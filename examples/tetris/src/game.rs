use crate::gameboard::Gameboard;
use crate::tetromino::Tetromino;
use crate::{BLOCK_SIZE, GAMEBOARD_OFFSET, GAMEBOARD_WIDTH, GAMEBOARD_HEIGHT};
use crate::graphics::{Align4, sprite::Vertex, self};

use psp::{sys, Align16, sys::{CtrlButtons, SceCtrlData}};

use rand_chacha::ChaChaRng;
use rand::prelude::*;

pub struct Game {
    score: usize,
    board: Gameboard,
    next_shape: Tetromino,
    current_shape: Tetromino,
    next_shape_offset: (usize, usize),
    seconds_per_tick: f64,
    seconds_since_tick: f64,
    shape_placed: bool,
    rng: ChaChaRng,
    last_input: CtrlButtons,
}

impl Game {
    pub fn new() -> Self {
        let mut seed: u64 = 0;
        unsafe { 
            sys::sceRtcGetCurrentTick(&mut seed as *mut u64);
        }
        let mut rng = ChaChaRng::seed_from_u64(seed);

        let gameboard = Gameboard::new();

        let mut next_shape = Tetromino::new_random(&mut rng);
        next_shape.set_pos(30, 7);

        let mut current_shape = Tetromino::new_random(&mut rng);
        let spawn_loc = gameboard.get_spawn_loc(); 
        current_shape.set_pos(spawn_loc.0 as i32, spawn_loc.1 as i32);

        Self {
            score: 0,
            board: gameboard,
            next_shape,
            current_shape,
            next_shape_offset: (30, 7),
            seconds_per_tick: 0.25,
            seconds_since_tick: 0.0,
            shape_placed: false,
            rng,
            last_input: CtrlButtons::default(),
        }
    }

    pub fn process_input(&mut self) {
        let mut pad_data = SceCtrlData::default();
        unsafe { 
            sys::sceCtrlReadBufferPositive(&mut pad_data, 1);
            if self.last_input.bits() == pad_data.buttons.bits() {
                // no change in input, and I don't feel like doing held down buttons
                return;
            }
            if pad_data.buttons.contains(CtrlButtons::LEFT) && !self.last_input.contains(CtrlButtons::LEFT) {
                attempt_move(&mut self.current_shape, -1, 0, &self.board);
            }
            if pad_data.buttons.contains(CtrlButtons::RIGHT) && !self.last_input.contains(CtrlButtons::RIGHT) {
                attempt_move(&mut self.current_shape, 1, 0, &self.board);
            }
            if pad_data.buttons.contains(CtrlButtons::DOWN) && !self.last_input.contains(CtrlButtons::DOWN) {
                drop(&mut self.current_shape, &self.board);
                self.current_shape.lock_to_gameboard(&mut self.board);
                self.shape_placed = true;
            }
            if pad_data.buttons.contains(CtrlButtons::CROSS) && !self.last_input.contains(CtrlButtons::CROSS)  {
                attempt_rotate_ccw(&mut self.current_shape, &self.board);
            }
            if pad_data.buttons.contains(CtrlButtons::CIRCLE) && !self.last_input.contains(CtrlButtons::CIRCLE)  {
                attempt_rotate_cw(&mut self.current_shape, &self.board);
            }
            self.last_input = pad_data.buttons;
        }
    }

    pub fn process_game_loop(&mut self, seconds_since_last_loop: f32) -> bool {
        self.process_input();
        self.seconds_since_tick += seconds_since_last_loop as f64;
        if self.seconds_since_tick > self.seconds_per_tick {
            self.tick();
            self.seconds_since_tick -= self.seconds_per_tick;
        }
        if self.shape_placed {
            if !self.spawn_next_shape() {
                // game over
                return true;
            } else {
                self.pick_next_shape();
                let rows_complete = self.board.remove_completed_rows().unwrap();
                self.set_score(self.score + 400 * rows_complete);
            }
            self.shape_placed = false;
        }
        false
    }

    pub fn tick(&mut self) {
        if !attempt_move(&mut self.current_shape, 0, 1, &self.board) {
            self.current_shape.lock_to_gameboard(&mut self.board);
            self.shape_placed = true;
        }
    }

    pub fn set_score(&mut self, score: usize) {
        self.score = score;
    }

    pub fn spawn_next_shape(&mut self) -> bool {
        self.current_shape = self.next_shape;
        let spawn_loc = self.board.get_spawn_loc();
        self.current_shape.set_pos(spawn_loc.0 as i32, spawn_loc.1 as i32);
        is_position_legal(&self.current_shape, &self.board)
    }

    pub fn pick_next_shape(&mut self) {
        self.next_shape = Tetromino::new_random(&mut self.rng);
        self.next_shape.set_pos(self.next_shape_offset.0 as i32, self.next_shape_offset.1 as i32);
    }

    pub fn draw(
        &self,
        vertex_buffer: &mut Align16<alloc::boxed::Box<[Align4<Vertex>]>>,
        texture_buffer: &Align16<&mut [u8]>
    ) {

        // background
        vertex_buffer.0[0] = Align4(Vertex { 
            u: 0.0,
            v: 0.0,
            color: 0x7f34_3434,
            x: BLOCK_SIZE as f32 * GAMEBOARD_OFFSET.0 as f32,
            y: BLOCK_SIZE as f32 * GAMEBOARD_OFFSET.1 as f32,
            z: -1.0,
        });
        vertex_buffer.0[1] = Align4(Vertex { 
            u: BLOCK_SIZE as f32 * GAMEBOARD_WIDTH as f32,
            v: BLOCK_SIZE as f32 * GAMEBOARD_HEIGHT as f32,
            color: 0x7f34_3434,
            x: BLOCK_SIZE as f32 * GAMEBOARD_OFFSET.0 as f32 + BLOCK_SIZE as f32 * GAMEBOARD_WIDTH as f32,
            y: BLOCK_SIZE as f32 * GAMEBOARD_OFFSET.1 as f32 + BLOCK_SIZE as f32 * GAMEBOARD_HEIGHT as f32,
            z: -1.0,
        });

        (*vertex_buffer).0[2..402].copy_from_slice(&self.board.as_vertices());
        (*vertex_buffer).0[402..410].copy_from_slice(&self.current_shape.as_vertices());
        (*vertex_buffer).0[410..418].copy_from_slice(&self.next_shape.as_vertices());

        unsafe {
            graphics::draw_vertices(vertex_buffer, texture_buffer, 418);       
            let score_string = alloc::format!("Score: {}", self.score);
            graphics::draw_text_at(327, 40, 0xffff_ffff, score_string.as_str());
            graphics::draw_text_at(327, 60, 0xffff_ffff, "Next Shape:");
        }
    }
}


pub fn attempt_move(shape: &mut Tetromino, x: i32, y: i32, board: &Gameboard) -> bool {
    let mut temp: Tetromino = shape.clone();
    temp.add_pos(x, y);
    if is_position_legal(&temp, board) {
        shape.add_pos(x, y);
        return true;
    }
    false
}

pub fn attempt_rotate_cw(shape: &mut Tetromino, board: &Gameboard) -> bool {
    let mut temp: Tetromino = shape.clone();
    temp.rotate_cw();
    if is_position_legal(&temp, board) {
        shape.rotate_cw();
        return true;
    }
    false
}

pub fn attempt_rotate_ccw(shape: &mut Tetromino, board: &Gameboard) -> bool {
    let mut temp: Tetromino = shape.clone();
    temp.rotate_ccw();
    if is_position_legal(&temp, board) {
        shape.rotate_ccw();
        return true;
    }
    false
}

pub fn is_position_legal(shape: &Tetromino, board: &Gameboard) -> bool {
    is_shape_within_borders(shape, board) 
    && !does_shape_intersect_locked_blocks(shape, board)
}


pub fn is_shape_within_borders(shape: &Tetromino, board: &Gameboard) -> bool {
    let mapped_locs = shape.get_mapped_locs();
    for p in mapped_locs.iter() {
        if !(p.0 < board.get_width() 
        && p.1 < board.get_height()) {
            return false
        }
    }
    true
}


pub fn does_shape_intersect_locked_blocks(shape: &Tetromino, board: &Gameboard) -> bool {
    let mapped_locs = shape.get_mapped_locs();
    !board.are_locs_empty(mapped_locs.to_vec())
}


pub fn drop(shape: &mut Tetromino, board: &Gameboard) {
    while attempt_move(shape, 0, 1, board) {}
}
