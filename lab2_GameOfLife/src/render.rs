use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::draw_cell::draw_cell;

pub fn render(
    framebuffer: &mut Framebuffer,
    translate_x: i32,
    translate_y: i32,
    size: i32
) {
    framebuffer.set_current_color(Color::LIGHTBLUE);
     // BLOCK
    let block = [(0, 0), (1, 0), (0, 1), (1, 1)];
    for (x, y) in block.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // BEE-HIVE
    let beehive = [(5, 0), (6, 0), (4, 1), (7, 1), (5, 2), (6, 2)];
    for (x, y) in beehive.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // LOAF
    let loaf = [(10, 0), (11, 0), (9, 1), (12, 1), (10, 2), (12, 2), (11, 3)];
    for (x, y) in loaf.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // BLINKER
    let blinker = [(0, 6), (1, 6), (2, 6)];
    for (x, y) in blinker.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // TOAD
    let toad = [(6, 6), (7, 6), (8, 6), (5, 7), (6, 7), (7, 7)];
    for (x, y) in toad.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // BEACON
    let beacon = [(10, 6), (11, 6), (10, 7), (13, 8), (12, 9), (13, 9)];
    for (x, y) in beacon.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // GLIDER
    let glider = [(0, 12), (1, 13), (2, 11), (2, 12), (2, 13)];
    for (x, y) in glider.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // LWSS
    let lwss = [(5, 12), (8, 12), (9, 13), (9, 14), (5, 15), (6, 15), (7, 15), (8, 15), (9, 14)];
    for (x, y) in lwss.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }

    // MWSS
    let mwss = [
        (12, 12), (15, 12),
        (11, 13), (11, 14), (15, 14),
        (11, 15), (12, 15), (13, 15), (14, 15), (15, 14)
    ];
    for (x, y) in mwss.iter() {
        draw_cell(framebuffer, translate_x + x * size, translate_y + y * size, size);
    }
}
