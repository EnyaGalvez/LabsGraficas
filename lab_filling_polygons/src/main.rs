mod framebuffer;
mod bhm_line;
mod figures;

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::bhm_line::LineaBonita;
use crate::figures::dibujar_y_rellenar_poligono;

fn main() {
    let image_width = 800;
    let image_height = 800;
    let mut framebuffer = Framebuffer::new(image_width, image_height);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    let grosor = 1;

    // Polígono 1
    let poligono1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ].into_iter().map(|p| p.into()).collect::<Vec<_>>();
    dibujar_y_rellenar_poligono(&mut framebuffer, &poligono1, grosor, Color::MAGENTA);

    // Polígono 2
    let poligono2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ].into_iter().map(|p| p.into()).collect::<Vec<_>>();
    dibujar_y_rellenar_poligono(&mut framebuffer, &poligono2, grosor, Color::GREEN);

    // Polígono 3
    let poligono3 = vec![
        (377, 249), (411, 197), (436, 249)
    ].into_iter().map(|p| p.into()).collect::<Vec<_>>();
    dibujar_y_rellenar_poligono(&mut framebuffer, &poligono3, grosor, Color::YELLOW);

    // Polígono 4
    let poligono4 = vec![
        (413,177),(448,159),(502,88),(553,53),(535,36),(676,37),(660,52),
        (750,145),(761,179),(672,192),(659,214),(615,214),(632,230),
        (580,230),(597,215),(552,214),(517,144),(466,180)
    ].into_iter().map(|p| p.into()).collect::<Vec<_>>();
    dibujar_y_rellenar_poligono(&mut framebuffer, &poligono4, grosor, Color::CYAN);

    // Polígono 5 (agujero)
    let poligono5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ].into_iter().map(|p| p.into()).collect::<Vec<_>>();
    dibujar_y_rellenar_poligono(&mut framebuffer, &poligono5, grosor, Color::BLACK);

    // Se exporta la imagen
    let output_file_name = "lAB1_image.png";
    framebuffer.render_to_file(output_file_name);
    println!("Imagen exportada a: {}", output_file_name);
    
}


