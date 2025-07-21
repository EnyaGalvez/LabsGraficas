// src/main.rs

mod framebuffer;
mod bhm_line;
mod render;
mod draw_cell;
mod game_ofLife;

use raylib::prelude::*;
use std::thread;
use std::time::Duration;

use crate::framebuffer::Framebuffer;
use crate::game_ofLife::GameOfLife;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    let mut game = GameOfLife::new(10);

    while !window.window_should_close() {
        framebuffer.clear();

        game.update();
        game.render(&mut framebuffer);

        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::WHITE);
        framebuffer.swap_buffers(&mut d, &raylib_thread);
        
        std::thread::sleep(Duration::from_millis(20));
    }
}


