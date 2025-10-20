use glm::{Mat4, Vec3, vec3};

pub struct Vertex {
    pub proj: Mat4,
    pub view: Mat4,
    pub model: Mat4
}

impl Vertex {
    pub fn new(proj: Mat4, view: Mat4, model: Mat4) -> Self {
        Self { proj, view, model }
    }

    pub fn project_ndc(&self, p_world: Vec3) -> Vec3 {
        let p4 = self.proj * self.view * self.model * glm::vec4(p_world.x, p_world.y, p_world.z, 1.0);
        let w = if p4.w != 0.0 { p4.w } else { 1.0 };
        vec3(p4.x / w, p4.y / w, p4.z / w)
    }

    pub fn ndc_to_screen(&self, p_ndc: Vec3, width: u32, height: u32) -> Vec3 {
        let x = ((p_ndc.x + 1.0) * 0.5) * (width as f32);
        let y = ((1.0 - (p_ndc.y + 1.0) * 0.5)) * (height as f32); // Y invertido para pantalla
        vec3(x, y, p_ndc.z)
    }

    pub fn project_screen(&self, p_world: Vec3, width: u32, height: u32) -> Vec3 {
        let ndc = self.project_ndc(p_world);
        self.ndc_to_screen(ndc, width, height)
    }

    pub fn print_vertex(&self, label: &str, p_world: Vec3, width: u32, height: u32) {
        let p_ndc = self.project_ndc(p_world);
        let p_scr = self.ndc_to_screen(p_ndc, width, height);
        println!(
            "{}: world=({:.2},{:.2},{:.2}) ndc=({:.2},{:.2},{:.2}) screen=({:.1},{:.1})",
            label, p_world.x, p_world.y, p_world.z, p_ndc.x, p_ndc.y, p_ndc.z, p_scr.x, p_scr.y
        );
    }
}
