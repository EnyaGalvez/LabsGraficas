// src/draw_cell.rs

use crate::bhm_line::{bhm_line, LineaBonita};
use crate::framebuffer::Framebuffer;

pub fn draw_cell(
    framebuffer: &mut Framebuffer, 
    x: i32, 
    y: i32, 
    size: i32
) {
    let color = framebuffer.get_current_color();
    for dy in 0..size {
        bhm_line(
            framebuffer,
            LineaBonita::new(x, y + dy),
            LineaBonita::new(x + size - 1, y + dy),
            1,
        );
    }
    fb.set_current_color(color);
}
