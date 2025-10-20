use sdl2::pixels::Color as SdlColor;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn clear(canvas: &mut Canvas<Window>, color: SdlColor) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    Ok(())
}

pub fn present(canvas: &mut Canvas<Window>) {
    canvas.present();
}

pub fn clear_zbuffer(zbuf: &mut [f32]) {
    for z in zbuf.iter_mut() { *z = f32::INFINITY; }
}