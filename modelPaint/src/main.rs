use anyhow::Result;
use glm::{ Vec3, vec3, radians };
use sdl2::pixels::Color as SdlColor;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::mem::ManuallyDrop;

mod vertex;
mod vMesh;
mod draw;
mod framebuffer;
mod math_utils;

use crate::vertex::Vertex;
use crate::draw::{Color, triangle, line, point, triangle_filled_bary};
use crate::vMesh::{load_obj, setup_vertex_array};
use crate::framebuffer::{clear, present, clear_zbuffer};
use crate::math_utils::{perspective_rh_zo, look_at_rh, compute_center_and_scale, translate, scale_uniform};

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

pub fn render(
    canvas: &mut Canvas<Window>,
    vertex: &Vertex,
    vertex_array: &[Vec3],
    obj_loaded: bool,
    draw_color: SdlColor
) -> Result<(), String> {
    let (w, h) = canvas.output_size()?;
    let mut zbuffer = vec![f32::INFINITY; (w * h) as usize];
    clear_zbuffer(&mut zbuffer);

    if obj_loaded {
        for tri in vertex_array.chunks(3) {
            if tri.len() < 3 { continue; }
            let a_scr = vertex.project_screen(tri[0], w, h);
            let b_scr = vertex.project_screen(tri[1], w, h);
            let c_scr = vertex.project_screen(tri[2], w, h);
            triangle_filled_bary(canvas, a_scr, b_scr, c_scr, draw_color, &mut zbuffer)?;
        }
    } else {
        if vertex_array.len() >= 3 {
            let a = vertex.project_screen(vertex_array[0], w, h);
            let b = vertex.project_screen(vertex_array[1], w, h);
            let c = vertex.project_screen(vertex_array[2], w, h);
            triangle_filled_bary(canvas, a, b, c, draw_color, &mut zbuffer)?;
        }

        line(canvas, glm::vec3(10.0, 10.0, 0.0), glm::vec3(200.0, 150.0, 0.0), SdlColor::RGB(0, 255, 255))?;

        let off = glm::vec3(-1000.0, -1000.0, 0.0);
        point(canvas, off, SdlColor::RGB(255, 0, 0))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let sdl = sdl2::init().map_err(|e| anyhow::anyhow!(e))?;
    let video_subsystem = sdl.video().map_err(|e| anyhow::anyhow!(e))?;

    let window = video_subsystem
        .window("Software Renderer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| anyhow::anyhow!(e))?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| anyhow::anyhow!(e))?;

    // Destruccion manual
    let renderer_ptr = canvas.raw();
    let window_ptr = canvas.window().raw();

    let mut canvas = ManuallyDrop::new(canvas);

    // vista
    let aspect = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
    let proj = perspective_rh_zo(radians(60.0), aspect, 0.1, 100.0);
    let eye = vec3(0.0, 0.0, 2.0);
    let center = vec3(0.0, 0.0, 0.0);
    let up = vec3(0.0, 1.0, 0.0);
    let view = look_at_rh(eye, center, up);

    // Cargar modelo
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut faces = Vec::new();
    let obj_loaded = load_obj("res/nave.obj", &mut vertices, &mut faces).unwrap_or(false);
    let vertex_array = if obj_loaded {
        setup_vertex_array(&vertices, &faces)
    } else {
        vec![
            vec3(-0.5, -0.5, 0.0),
            vec3( 0.5, -0.5, 0.0),
            vec3( 0.0,  0.5, 0.0),
        ]
    };

    // Centrar y escalar modelo
    let (mesh_center, mesh_scale) = if obj_loaded {
        compute_center_and_scale(&vertices, 0.8)
    } else {
        (vec3(0.0, 0.0, 0.0), 1.0)
    };


    let model = translate(-mesh_center) * scale_uniform(mesh_scale);

    let vertex = Vertex::new(proj, view, model);

   
    let mut event_pump = sdl.event_pump().map_err(|e| anyhow::anyhow!(e))?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        clear(&mut canvas, SdlColor::RGB(20, 20, 20)).map_err(|e| anyhow::anyhow!(e))?;

        let current = Color::new(255, 255, 0);
        let draw_color = SdlColor::RGB(current.r, current.g, current.b);

        render(&mut canvas, &vertex, &vertex_array, obj_loaded, draw_color).map_err(|e| anyhow::anyhow!(e))?;

        present(&mut canvas);
    }

    unsafe {
        sdl2::sys::SDL_DestroyRenderer(renderer_ptr);
        sdl2::sys::SDL_DestroyWindow(window_ptr);
        sdl2::sys::SDL_Quit();
    }

    Ok(())
}