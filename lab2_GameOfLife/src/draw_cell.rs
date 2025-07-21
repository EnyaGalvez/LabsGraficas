mod framebuffer;
mod bhm_line;
mod render;

use bhm_line::{bhm_line, LineaBonita};
use framebuffer::Framebuffer;
use raylib::prelude::*;

pub fn draw_cell(
    framebuffer: &mut Framebuffer, 
    x: i32, 
    y: i32, 
    size: i32
) {
    let color = fb.get_current_color();
    for dx in 0..size {
        bhm_line(
            fb,
            LineaBonita::new(x, y + dx),
            LineaBonita::new(x + size - 1, y + dx),
            1,
        );
    }
    fb.set_current_color(color);
}
