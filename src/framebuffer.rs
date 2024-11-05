use crate::color::Color;
use nalgebra_glm::Vec3;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec3>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![Vec3::new(0.0, 0.0, 0.0); width * height];
        Self { width, height, buffer }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let vec_color = Vec3::new(color.r, color.g, color.b);
            self.buffer[y * self.width + x] = vec_color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
        self.buffer[y * self.width + x]
    }
}
