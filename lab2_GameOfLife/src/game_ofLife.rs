use crate::draw_cell::draw_cell;
use crate::render::render;
use crate::framebuffer::Framebuffer;

const WIDTH: usize = 80;
const HEIGHT: usize = 60;

pub struct GameOfLife {
    pub grid: [[u8; WIDTH]; HEIGHT],
    buffer: [[u8; WIDTH]; HEIGHT],
    pub cell_size: i32,
}

impl GameOfLife {
    pub fn new(cell_size: i32) -> Self {
        let mut grid = [[0u8; WIDTH]; HEIGHT];

        let offset = [
        (0, 0),     // top-left
        (20, 0),    // top-mid
        (40, 0),    // top-right
        (0, 20),    // mid-left
        (20, 20),   // center
        (40, 20),   // mid-right
        (0, 40),    // bottom-left
        (20, 40),   // bottom-mid
        (40, 40),   // bottom-right
        ];

        let patterns: Vec<Vec<(usize, usize)>> = vec![
        vec![(1, 1), (2, 1), (1, 2), (2, 2)], // BLOCK
        vec![(5, 1), (6, 1), (4, 2), (7, 2), (5, 3), (6, 3)], // BEE-HIVE
        vec![(10, 1), (11, 1), (9, 2), (12, 2), (10, 3), (12, 3), (11, 4)], // LOAF
        vec![(1, 6), (2, 6), (3, 6)], // BLINKER
        vec![(6, 6), (7, 6), (8, 6), (5, 7), (6, 7), (7, 7)], // TOAD
        vec![(10, 6), (11, 6), (10, 7), (13, 8), (12, 9), (13, 9)], // BEACON
        vec![(1, 12), (2, 13), (3, 11), (3, 12), (3, 13)], // GLIDER
        vec![(5, 12), (8, 12), (9, 13), (9, 14), (5, 15), (6, 15), (7, 15), (8, 15)], // LWSS
        vec![(12, 12), (15, 12), (11, 13), (11, 14), (15, 14),(11, 15), (12, 15), (13, 15), (14, 15)], // MWSS 
        ];

        for (i, pattern) in patterns.iter().enumerate() {
            let (ox, oy) = offset[i];
            for (x, y) in pattern.iter() {
                let gx = ox + x;
                let gy = oy + y;
                if gx < WIDTH && gy < HEIGHT {
                    grid[gy][gx] = 1;
                }
            }
        }

        Self {
            grid,
            buffer: [[0u8; WIDTH]; HEIGHT],
            cell_size,
        }
    }

    pub fn update(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let alive_neighbors = self.count_neighbors(x, y);
                let cell = self.grid[y][x];

                self.buffer[y][x] = match (cell, alive_neighbors) {
                    (1, 2) | (1, 3) => 1, // Sobrevive
                    (0, 3) => 1,         // Nace
                    _ => 0,             // Muere o permanece muerta
                };
            }
        }

        self.grid.copy_from_slice(&self.buffer);
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                    count += self.grid[ny as usize][nx as usize];
                }
            }
        }
        count
    }
    pub fn render(&self, fb: &mut Framebuffer) {
        fb.clear();
        fb.set_current_color(raylib::color::Color::GOLD);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.grid[y][x] == 1 {
                    let px = (x as i32) * self.cell_size;
                    let py = (y as i32) * self.cell_size;
                    draw_cell(fb, px, py, self.cell_size);
                }
            }
        }
    }
}