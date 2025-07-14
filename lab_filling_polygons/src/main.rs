mod framebuffer;
mod bhm_line;
mod figures;

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::bhm_line::{bhm_line, LineaBonita};
use crate::figures::{dibujar_triangulo_regular, dibujar_rectangulo, rellenar_poligono};

fn main() {
    let image_width = 800;
    let image_height = 800;
    let mut framebuffer = Framebuffer::new(image_width, image_height);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    
}


