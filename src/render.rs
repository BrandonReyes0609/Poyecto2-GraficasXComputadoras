use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::scene::Scene;
use crate::light::Light;
use crate::cast_ray::cast_ray;
use crate::color::Color;


pub fn render(framebuffer: &mut Framebuffer, camera: &Camera, scene: &Scene, lights: &[Light]) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let ray_direction = camera.calculate_ray_direction(x, y, framebuffer);
            
            // Inicializar el color final del píxel
            let mut color = Color::black();
            
            // Iterar sobre cada luz y sumar su contribución
            for light in lights {
                color = color + cast_ray(scene, &camera.eye, &ray_direction, light, 0);
            }

            // Ajustar el color final para evitar saturación
            color = color * (1.0 / lights.len() as f32);
            framebuffer.set_pixel(x, y, color);
        }
    }
} 