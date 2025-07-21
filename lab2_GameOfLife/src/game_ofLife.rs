use crate::draw_cell::draw_cell;
use crate::render::render;
use crate::framebuffer::Framebuffer;

const WIDTH: usize = 100;
const HEIGHT: usize = 80;

pub struct GameOfLife {
    pub grid: [[u8; WIDTH]; HEIGHT],
    buffer: [[u8; WIDTH]; HEIGHT],
    pub cell_size: i32,
}

impl GameOfLife {
    pub fn new(cell_size: i32) -> Self {
        let mut grid = [[0u8; WIDTH]; HEIGHT];

        // Patrón: Blinker (horizontal)
        grid[10][10] = 1;
        grid[10][11] = 1;
        grid[10][12] = 1;

        // Patrón: Glider
        grid[1][2] = 1;
        grid[2][3] = 1;
        grid[3][1] = 1;
        grid[3][2] = 1;
        grid[3][3] = 1;

        // Patrón: Block
        grid[20][20] = 1;
        grid[20][21] = 1;
        grid[21][20] = 1;
        grid[21][21] = 1;

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