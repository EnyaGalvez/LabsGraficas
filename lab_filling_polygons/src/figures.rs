// src/figures.rs

use crate::framebuffer::Framebuffer;
use crate::bhm_line::{bhm_line, LineaBonita};

pub fn dibujar_y_rellenar_poligono(
    framebuffer: &mut Framebuffer,
    vertices: &[LineaBonita],
    grosor: i32,
) {
    // Dibujar contorno
    for i in 0..vertices.len() {
        bhm_line(
            framebuffer,
            vertices[i],
            vertices[(i + 1) % vertices.len()],
            grosor,
        );
    }

    // Rellenar
    let height = framebuffer.height as i32;
    for y in 0..height {
        let mut intersecciones = Vec::new();
        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];
            if (v1.y <= y && v2.y > y) || (v2.y <= y && v1.y > y) {
                let x = v1.x + (y - v1.y) * (v2.x - v1.x) / (v2.y - v1.y);
                intersecciones.push(x);
            }
        }

        intersecciones.sort();
        for i in (0..intersecciones.len()).step_by(2) {
            if i + 1 < intersecciones.len() {
                for x in intersecciones[i]..=intersecciones[i + 1] {
                    framebuffer.set_pixel(x, y, framebuffer.get_current_color());
                }
            }
        }
    }
}