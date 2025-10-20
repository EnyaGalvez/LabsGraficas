use glm::{Mat4, Vec3};

#[inline]
fn zero_mat4() -> Mat4 {
    glm::mat4(
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
    )
}

#[inline]
fn identity_mat4() -> Mat4 {
    glm::mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn perspective_rh_zo(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (0.5 * fovy).tan();
    let mut m = zero_mat4();
    m[0][0] = f / aspect;
    m[1][1] = f;
    m[2][2] = far / (near - far);
    m[2][3] = -1.0;
    m[3][2] = (far * near) / (near - far);
    m
}

pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let f = glm::normalize(eye - center);
    let r = glm::normalize(glm::cross(up, f));
    let u = glm::cross(f, r);

    let mut m = identity_mat4();

    m[0][0] = r.x; m[0][1] = r.y; m[0][2] = r.z; m[0][3] = 0.0;
    m[1][0] = u.x; m[1][1] = u.y; m[1][2] = u.z; m[1][3] = 0.0;
    m[2][0] = f.x; m[2][1] = f.y; m[2][2] = f.z; m[2][3] = 0.0;

    m[3][0] = -glm::dot(r, eye);
    m[3][1] = -glm::dot(u, eye);
    m[3][2] = -glm::dot(f, eye);
    m[3][3] = 1.0;
    m
}

#[inline]
pub fn translate(t: Vec3) -> Mat4 {
    let mut m = identity_mat4();
    m[3][0] = t.x;
    m[3][1] = t.y;
    m[3][2] = t.z;
    m
}

#[inline]
pub fn scale_uniform(s: f32) -> Mat4 {
    glm::mat4(
        s,   0.0, 0.0, 0.0,
        0.0, s,   0.0, 0.0,
        0.0, 0.0, s,   0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn compute_center_and_scale(vertices: &[Vec3], target: f32) -> (Vec3, f32) {
    let mut min = glm::vec3(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = glm::vec3(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

    for &v in vertices {
        min = glm::min(min, v);
        max = glm::max(max, v);
    }

    let center = (min + max) * 0.5;
    let half_extent = (max - min) * 0.5;
    let r = half_extent.x.max(half_extent.y).max(half_extent.z).max(1e-6);
    let s = target / r;
    (center, s)
}
