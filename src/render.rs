use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::scene::Scene;
use crate::light::Light;
use crate::cast_ray::cast_ray;

pub fn render(framebuffer: &mut Framebuffer, camera: &Camera, scene: &Scene, light: &Light) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let ray_direction = camera.calculate_ray_direction(x, y, framebuffer);
            let color = cast_ray(scene, &camera.eye, &ray_direction, light, 0);
            framebuffer.set_pixel(x, y, color);
        }
    }
}
