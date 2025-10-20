use anyhow::{anyhow, Result};
use glm::{Vec3, vec3};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Face {
    pub vertex_indices: Vec<[usize; 3]>,
}

pub fn load_obj(path: &str, out_vertices: &mut Vec<Vec3>, out_faces: &mut Vec<Face>) -> Result<bool> {
    let file = File::open(path).map_err(|e| anyhow!("No se pudo abrir {}: {}", path, e))?;
    let reader = BufReader::new(file);

    let mut raw_faces: Vec<Vec<usize>> = Vec::new();
    out_vertices.clear();
    out_faces.clear();

    for line in reader.lines() {
        let l = line?;
        let l = l.trim();
        if l.is_empty() || l.starts_with('#') { continue; }

        if l.starts_with("v ") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() < 4 { continue; }
            let x: f32 = parts[1].parse().unwrap_or(0.0);
            let y: f32 = parts[2].parse().unwrap_or(0.0);
            let z: f32 = parts[3].parse().unwrap_or(0.0);
            out_vertices.push(vec3(x, y, z));
        } else if l.starts_with("f ") {
            let parts: Vec<&str> = l.split_whitespace().skip(1).collect();
            let mut idxs: Vec<usize> = Vec::new();
            for p in parts {
                let first = p.split('/').next().unwrap_or("");
                if first.is_empty() { continue; }
                if let Ok(v) = first.parse::<isize>() {
                    let zidx = if v > 0 {
                        (v - 1) as usize
                    } else {
                        (out_vertices.len() as isize + v) as usize
                    };
                    idxs.push(zidx);
                }
            }
            if idxs.len() >= 3 {
                raw_faces.push(idxs);
            }
        }
    }

    for poly in raw_faces {
        let mut tris: Vec<[usize; 3]> = Vec::new();
        for i in 1..(poly.len() - 1) {
            tris.push([poly[0], poly[i], poly[i + 1]]);
        }
        out_faces.push(Face { vertex_indices: tris });
    }

    Ok(true)
}

pub fn setup_vertex_array(vertices: &[Vec3], faces: &[Face]) -> Vec<Vec3> {
    let mut vertex_array = Vec::new();
    for face in faces {
        for tri in &face.vertex_indices {
            let v0 = vertices[tri[0]];
            let v1 = vertices[tri[1]];
            let v2 = vertices[tri[2]];
            vertex_array.push(v0);
            vertex_array.push(v1);
            vertex_array.push(v2);
        }
    }
    vertex_array
}