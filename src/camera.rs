use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}
 
impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self { eye, center, up }
    }

    pub fn calculate_ray_direction(&self, x: usize, y: usize, framebuffer: &Framebuffer) -> Vec3 {
        let aspect_ratio = framebuffer.width as f32 / framebuffer.height as f32;
        let sensor_x = (((x as f32 + 0.5) / framebuffer.width as f32) * 2.0 - 1.0) * aspect_ratio;
        let sensor_y = 1.0 - ((y as f32 + 0.5) / framebuffer.height as f32) * 2.0;
        let ray_direction = Vec3::new(sensor_x, sensor_y, -1.0).normalize();
        self.basis_change(&ray_direction)
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();
        let rotated = vector.x * right + vector.y * up - vector.z * forward;
        rotated.normalize()
    }
}
