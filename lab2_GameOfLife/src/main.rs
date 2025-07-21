// src/main.rs

mod framebuffer;
mod bhm_line;
mod render;

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use crate::framebuffer::Framebuffer;
use crate::bhm_line::{bhm_line, LineaBonita};
use crate::render::render;

fn main() {
    let window_width = 1000;
    let window_height = 800;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Figuritas de Colores")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    let mut translate_x: f32 = 0.0;
    let mut translate_y: f32 = 0.0;

    while !window.window_should_close() {
        framebuffer.clear();

        translate_x += 1.0;
        translate_y += 1.0;

        render(&mut framebuffer, translate_x, translate_y);

        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::WHITE);
        framebuffer.swap_buffers(&mut d, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}


