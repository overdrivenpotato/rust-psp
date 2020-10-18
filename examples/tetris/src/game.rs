use crate::gameboard::Gameboard;
use crate::tetromino::Tetromino;
use crate::{BLOCK_SIZE, GAMEBOARD_OFFSET, GAMEBOARD_WIDTH, GAMEBOARD_HEIGHT};
use crate::graphics::{Align4, sprite::Vertex, BLOCK, self};

use psp::{sys, Align16, sys::{CtrlButtons, SceCtrlData}};

use rand_chacha::ChaChaRng;
use rand::prelude::*;

/// Stores the state of our entire game
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
    /// Creates a new `Game`
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

    /// Handles user input
    pub fn process_input(&mut self) {
        let mut pad_data = SceCtrlData::default();
        unsafe { 
            sys::sceCtrlReadBufferPositive(&mut pad_data, 1);
            if self.last_input.bits() == pad_data.buttons.bits() {
                // no change in input, and I don't feel like doing held down buttons
                return;
            }
            if pad_data.buttons.contains(CtrlButtons::LEFT) && !self.last_input.contains(CtrlButtons::LEFT) {
                self.attempt_move(-1, 0);
            }
            if pad_data.buttons.contains(CtrlButtons::RIGHT) && !self.last_input.contains(CtrlButtons::RIGHT) {
                self.attempt_move(1, 0);
            }
            if pad_data.buttons.contains(CtrlButtons::DOWN) && !self.last_input.contains(CtrlButtons::DOWN) {
                self.drop();
                self.current_shape.lock_to_gameboard(&mut self.board);
                self.shape_placed = true;
            }
            if pad_data.buttons.contains(CtrlButtons::CROSS) && !self.last_input.contains(CtrlButtons::CROSS)  {
                self.attempt_rotate_ccw();
            }
            if pad_data.buttons.contains(CtrlButtons::CIRCLE) && !self.last_input.contains(CtrlButtons::CIRCLE)  {
                self.attempt_rotate_cw();
            }
            self.last_input = pad_data.buttons;
        }
    }

    /// Called once per loop of the game, does all the biz.
    ///
    /// # Parameters
    ///
    /// - `seconds_since_last_loop`: Seconds that have passed since the last loop
    ///
    /// # Return Value
    ///
    /// `true` if game is over
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
                let rows_complete = self.board.remove_completed_rows();
                self.set_score(self.score + 400 * rows_complete);
            }
            self.shape_placed = false;
        }
        false
    }

    /// Moves `current_shape` down 1 unit and locks to board if it collides.
    pub fn tick(&mut self) {
        if !self.attempt_move(0, 1) {
            self.current_shape.lock_to_gameboard(&mut self.board);
            self.shape_placed = true;
        }
    }

    /// Setter for `score`
    ///
    /// # Parameters
    /// 
    /// - `score`: Score to set.
    pub fn set_score(&mut self, score: usize) {
        self.score = score;
    }

    /// Moves the `next_shape` into the `current_shape` and sets position accordingly.
    pub fn spawn_next_shape(&mut self) -> bool {
        self.current_shape = self.next_shape;
        let spawn_loc = self.board.get_spawn_loc();
        self.current_shape.set_pos(spawn_loc.0 as i32, spawn_loc.1 as i32);
        self.is_position_legal(&self.current_shape)
    }

    /// Picks the next Tetromino, sets it's position on the screen to be in the 
    /// "Next Shape:" section
    pub fn pick_next_shape(&mut self) {
        self.next_shape = Tetromino::new_random(&mut self.rng);
        self.next_shape.set_pos(self.next_shape_offset.0 as i32, self.next_shape_offset.1 as i32);
    }

    /// Draws everything for the game
    ///
    /// # Parameters
    ///
    /// - `vertex_buffer`: Mutable reference to the main vertex buffer.
    /// - `texture_buffer`: Mutable reference to the main texture buffer.
    pub fn draw(
        &self,
        vertex_buffer: &mut Align16<alloc::boxed::Box<[Align4<Vertex>]>>,
        texture_buffer: &mut Align16<alloc::boxed::Box<[u8]>>
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

        (*texture_buffer).0.copy_from_slice(&BLOCK);
        (*vertex_buffer).0[2..402].copy_from_slice(&self.board.as_vertices());
        (*vertex_buffer).0[402..410].copy_from_slice(&self.current_shape.as_vertices());
        (*vertex_buffer).0[410..418].copy_from_slice(&self.next_shape.as_vertices());

        unsafe {
            graphics::draw_vertices(vertex_buffer, texture_buffer, BLOCK_SIZE, BLOCK_SIZE, 0.75, 0.75);       
            let score_string = alloc::format!("Score: {}", self.score);
            graphics::draw_text_at(327, 40, 0xffff_ffff, score_string.as_str());
            graphics::draw_text_at(327, 60, 0xffff_ffff, "Next Shape:");
        }
    }

    /// Attempts to add to the `current_shape` position, returns true if successful.
    ///
    /// # Parameters
    ///
    /// - `x`: horizontal position to add
    /// - `y`: vertical position to add
    ///
    /// # Return Value
    ///
    /// `true` if successful
    pub fn attempt_move(&mut self, x: i32, y: i32) -> bool {
        let mut temp: Tetromino = self.current_shape.clone();
        temp.add_pos(x, y);
        if self.is_position_legal(&temp) {
            self.current_shape.add_pos(x, y);
            return true;
        }
        false
    }

    /// Attempts to rotate `current_shape` clockwise, returns true if successful.
    ///
    /// # Return Value
    ///
    /// `true` if successful
    pub fn attempt_rotate_cw(&mut self) -> bool {
        let mut temp: Tetromino = self.current_shape.clone();
        temp.rotate_cw();
        if self.is_position_legal(&temp) {
            self.current_shape.rotate_cw();
            return true;
        }
        false
    }

    /// Attempts to rotate `current_shape` counterclockwise, returns true if successful.
    ///
    /// # Return Value
    ///
    /// `true` if successful
    pub fn attempt_rotate_ccw(&mut self) -> bool {
        let mut temp: Tetromino = self.current_shape.clone();
        temp.rotate_ccw();
        if self.is_position_legal(&temp) {
            self.current_shape.rotate_ccw();
            return true;
        }
        false
    }

    /// Checks if the position of the given tetromino is within boundaries and does 
    /// not collide.
    /// 
    /// # Parameters
    ///
    /// - `shape`: `Tetromino` to check
    ///
    /// # Return Value
    ///
    /// `true` if position is in bounds and does not collide
    pub fn is_position_legal(&self, shape: &Tetromino) -> bool {
        self.is_shape_within_borders(shape) 
        && !self.does_shape_intersect_locked_blocks(shape)
    }

    /// Checks if the position of the given tetromino is within boundaries of the
    /// gameboard
    ///
    /// # Parameters
    ///
    /// - `shape`: `Tetromino` to check
    ///
    /// # Return Value
    ///
    /// `true` if within boundaries of `board`
    pub fn is_shape_within_borders(&self, shape: &Tetromino) -> bool {
        let mapped_locs = shape.get_mapped_locs();
        for p in mapped_locs.iter() {
            if !(p.0 < GAMEBOARD_WIDTH 
            && p.1 < GAMEBOARD_HEIGHT) {
                return false
            }
        }
        true
    }

    /// Checks if the given tetromino's position collides with a block in the gameboard
    ///
    /// # Parameters
    ///
    /// `shape`: `Tetromino` to check
    /// 
    /// # Return Value
    ///
    /// `true` if shape collides
    pub fn does_shape_intersect_locked_blocks(&self, shape: &Tetromino) -> bool {
        let mapped_locs = shape.get_mapped_locs();
        !self.board.are_locs_empty(mapped_locs.to_vec())
    }

    /// Hard drop function
    pub fn drop(&mut self) {
        while self.attempt_move(0, 1) {}
    }
}


