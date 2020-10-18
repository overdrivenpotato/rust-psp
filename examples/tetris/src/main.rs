#![no_std]
#![no_main]

#![allow(dead_code)]
#![feature(const_fn)]

extern crate alloc;

mod tetromino;
mod gameboard;
mod game;
mod graphics;
mod audio;

use psp::vram_alloc::get_vram_allocator;
use psp::Align16;
use psp::sys::{self, TexturePixelFormat, sceAudioChReserve};

use crate::graphics::Align4;
use crate::graphics::sprite::Vertex;
use crate::audio::MAX_SAMPLES;

psp::module!("tetris", 1, 1);

pub const BLOCK_SIZE: u32 = 16;

pub const GAMEBOARD_OFFSET: (usize, usize) = (15, 1);
pub const GAMEBOARD_WIDTH: usize = 10;
pub const GAMEBOARD_HEIGHT: usize = 20;

pub static BLOCK: [u8;BLOCK_SIZE as usize*BLOCK_SIZE as usize*4] = 
    *include_bytes!("../assets/block.bin");

fn psp_main() {
    unsafe {
        psp::enable_home_button();
        let mut allocator = get_vram_allocator().unwrap();
        graphics::setup(&mut allocator);

        let vertex_buffer = allocator.alloc_sized::<Vertex>(418);
        let vertex_buffer = alloc::boxed::Box::from_raw(core::slice::from_raw_parts_mut(vertex_buffer.as_mut_ptr_direct_to_vram() as *mut Align4<Vertex>, 418));
        let mut vertex_buffer = Align16(vertex_buffer);
        let texture_buffer = allocator.alloc_texture_pixels(16, 16, TexturePixelFormat::Psm8888);
        let texture_buffer = alloc::boxed::Box::from_raw(core::slice::from_raw_parts_mut(texture_buffer.as_mut_ptr_direct_to_vram() as *mut u8, 16*16*4));
        let mut texture_buffer = Align16(texture_buffer);

        let channel = sceAudioChReserve(-1, MAX_SAMPLES as i32, psp::sys::AudioFormat::Mono);
        let mut start_pos: usize = 0;
        let mut restlen: i32 = 0;

        let mut game = game::Game::new();
        
        graphics::clear_color(0xff554433);
        graphics::draw_text_at(130, 136, 0xffff_ffff, "Press Start to Play Tetris!"); 
        graphics::finish_frame();

        let ctrl_data = &mut sys::SceCtrlData::default(); 
        while !ctrl_data.buttons.contains(sys::CtrlButtons::START) {
            sys::sceCtrlReadBufferPositive(ctrl_data, 1); 
        }

        let mut loop_end = 0;
        let mut loop_start = 0;
        let ticks_per_sec = sys::sceRtcGetTickResolution();
        let mut seconds_since_last_loop: f32;

        loop {
            seconds_since_last_loop = (loop_end - loop_start) as f32 / ticks_per_sec as f32;
            sys::sceRtcGetCurrentTick(&mut loop_start);

            let audio_ret = audio::process_audio_loop(channel, start_pos, restlen);
            restlen = audio_ret.0;
            start_pos = audio_ret.1;

            graphics::clear_color(0xff554433);
            let game_over = game.process_game_loop(seconds_since_last_loop);
            if game_over  { 
                game.draw(&mut vertex_buffer, &mut texture_buffer);
                graphics::draw_text_at(100, 136, 0xffff_ffff, "Game Over. Press Start to Play Again");

                let ctrl_data = &mut sys::SceCtrlData::default(); 
                sys::sceCtrlReadBufferPositive(ctrl_data, 1); 
                if ctrl_data.buttons.contains(sys::CtrlButtons::START) {
                    game = game::Game::new();
                }
            } else {
                game.draw(&mut vertex_buffer, &mut texture_buffer);
            }
            graphics::finish_frame();
            sys::sceRtcGetCurrentTick(&mut loop_end);
        }
    }
}

