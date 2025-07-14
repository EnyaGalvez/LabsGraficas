use raylib::prelude::*;
use image::{ImageBuffer, Rgba};

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let background_color = Color::BLACK;
        let pixels = vec![vec![background_color; width as usize]; height as usize];

        Framebuffer {
            width,
            height,
            pixels,
            background_color,
            current_color: background_color,
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = self.background_color;
            }
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
            self.pixels[y as usize][x as usize] = color;
        }
    }

    pub fn set_thick_pixel(&mut self, x: i32, y: i32, radius: i32, color: Color) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    self.set_pixel(x + dx, y + dy, color);
                }
            }
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_current_color(&self) -> Color {
        self.current_color
    }

    pub fn render_to_file(&self, file_path: &str) {
        let mut imgbuf = ImageBuffer::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixels[y as usize][x as usize];
                imgbuf.put_pixel(x, y, Rgba([pixel.r, pixel.g, pixel.b, 255]));
            }
        }
        imgbuf.save(file_path).unwrap();
    }
}