use sdl2::pixels::Color as SdlColor;
use sdl2::render::Canvas;
use sdl2::video::Window;
use glm::{Vec3, vec3};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

pub fn point(canvas: &mut Canvas<Window>, p: Vec3, color: SdlColor) -> Result<(), String> {
    let (w, h) = canvas.output_size()?;
    let x = p.x.round() as i32;
    let y = p.y.round() as i32;
    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
        return Ok(());
    }
    canvas.set_draw_color(color);
    canvas.draw_point(sdl2::rect::Point::new(x, y))
}

pub fn line(canvas: &mut Canvas<Window>, start: Vec3, end: Vec3, color: SdlColor) -> Result<(), String> {
    let mut x1 = start.x.round() as i32;
    let mut y1 = start.y.round() as i32;
    let x2 = end.x.round() as i32;
    let y2 = end.y.round() as i32;

    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        point(canvas, glm::vec3(x1 as f32, y1 as f32, 0.0), color)?;
        if x1 == x2 && y1 == y2 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
    Ok(())
}

pub fn triangle(canvas: &mut Canvas<Window>, a: Vec3, b: Vec3, c: Vec3, color: SdlColor) -> Result<(), String> {
    line(canvas, a, b, color)?;
    line(canvas, b, c, color)?;
    line(canvas, c, a, color)?;
    Ok(())
}

#[inline]
fn bbox3(a: Vec3, b: Vec3, c: Vec3, w: i32, h: i32) -> (i32, i32, i32, i32) {
    let min_x = a.x.min(b.x).min(c.x).floor() as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil()  as i32;
    let min_y = a.y.min(b.y).min(c.y).floor() as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil()  as i32;
    (min_x.clamp(0, w - 1), max_x.clamp(0, w - 1),
     min_y.clamp(0, h - 1), max_y.clamp(0, h - 1))
}

#[inline]
fn barycentric(a: Vec3, b: Vec3, c: Vec3, px: f32, py: f32) -> (f32, f32, f32) {
    let v0x = b.x - a.x; let v0y = b.y - a.y;
    let v1x = c.x - a.x; let v1y = c.y - a.y;
    let v2x = px  - a.x; let v2y = py  - a.y;

    let denom = v0x * v1y - v1x * v0y;
    if denom.abs() < 1e-8 {
        return (-1.0, -1.0, -1.0);
    }
    let u = (v2x * v1y - v1x * v2y) / denom;
    let v = (v0x * v2y - v2x * v0y) / denom;
    let w = 1.0 - u - v;
    (u, v, w)
}

pub fn triangle_filled_bary(
    canvas: &mut Canvas<Window>,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    base_color: SdlColor,
    zbuf: &mut [f32],
) -> Result<(), String> {
    let (w, h) = canvas.output_size()?;
    let w_i = w as i32;
    let h_i = h as i32;

    let area2 = (b.x - a.x)*(c.y - a.y) - (b.y - a.y)*(c.x - a.x);

    let (min_x, max_x, min_y, max_y) = bbox3(a, b, c, w_i, h_i);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = glm::vec3(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            let (u, v, w_) = barycentric(a, b, c, p.x, p.y);

            if u >= 0.0 && v >= 0.0 && w_ >= 0.0 {
                let z = a.z * (1.0 - u - v) + b.z * u + c.z * v;

                let idx = (y * w as i32 + x) as usize;
                if z < zbuf[idx] {
                    zbuf[idx] = z;

                    let near_color = base_color;
                    let far_color  = SdlColor::RGB(102, 153, 153);

                    let tri_zmin = a.z.min(b.z).min(c.z);
                    let tri_zmax = a.z.max(b.z).max(c.z);
                    let mut t = ((z - tri_zmin) / (tri_zmax - tri_zmin + 1e-6)).clamp(0.0, 1.0);

                    let edge0 = 0.10;
                    let edge1 = 0.60;
                    t = ((t - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);

                    t = t.powf(0.4);

                    let mix = |a: u8, b: u8, tt: f32| -> u8 {
                        ((a as f32) * (1.0 - tt) + (b as f32) * tt) as u8
                    };

                    let color = SdlColor::RGB(
                        mix(near_color.r, far_color.r, t),
                        mix(near_color.g, far_color.g, t),
                        mix(near_color.b, far_color.b, t),
                    );

                    point(canvas, vec3(x as f32, y as f32, z), color)?;
                }
            }
        }
    }
    Ok(())
}
