use image::{DynamicImage, GenericImageView};
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub textures: Vec<Option<DynamicImage>>,
}
 
impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        textures: Vec<Option<DynamicImage>>,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            textures,
        }
    }

    pub fn get_texture_color(&self, u: f32, v: f32, face_index: usize) -> Color {
        if let Some(texture) = &self.textures.get(face_index).unwrap_or(&None) {
            let x = (u * (texture.width() as f32 - 1.0)).round() as u32;
            let y = (v * (texture.height() as f32 - 1.0)).round() as u32;
            let tex_color = texture.get_pixel(x, y);
            Color::new(tex_color[0] as f32 / 255.0, tex_color[1] as f32 / 255.0, tex_color[2] as f32 / 255.0)
        } else {
            self.diffuse
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::black(),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            textures: vec![None; 6], // Asigna 6 caras sin textura para el cubo
        }
    }
}
