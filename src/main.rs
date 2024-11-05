use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::material::Material;
use crate::scene::Scene;
use crate::cube::Cube;
use crate::camera::Camera;
use crate::render::render;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent, MouseButton, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::dpi::PhysicalPosition;

use image::io::Reader as ImageReader;
use std::time::{Duration, Instant};

mod camera;
mod color;
mod framebuffer;
mod material;
mod render;
mod scene;
mod cube;
mod intersect;
mod cast_ray;
mod light;
mod ray_intersect;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let frame_duration = Duration::from_secs_f32(1.0 / 60.0); // 60 FPS
    let mut next_frame_time = Instant::now() + frame_duration;

    // Cargar texturas
    let dirt_texture = ImageReader::open("assets/dirt/dirt.png").unwrap().decode().unwrap();
    let podzol_top_texture = ImageReader::open("assets/dirt/dirt_podzol_top.png").unwrap().decode().unwrap();
    let podzol_side_texture = ImageReader::open("assets/dirt/dirt_podzol_side.png").unwrap().decode().unwrap();

    // Crear materiales para cada textura
    let dirt_material = Material::new(
        Color::black(),
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        1.0,
        vec![Some(dirt_texture.clone())]
    );

    let podzol_top_material = Material::new(
        Color::black(),
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        1.0,
        vec![Some(podzol_top_texture.clone())]
    );

    let podzol_side_material = Material::new(
        Color::black(),
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        1.0,
        vec![Some(podzol_side_texture.clone())]
    );

    // Crear materiales para cada cara del cubo
    let cube_materials = [
        dirt_material.clone(),
        dirt_material.clone(),
        podzol_top_material.clone(),
        dirt_material.clone(),
        podzol_side_material.clone(),
        podzol_side_material.clone(),
    ];

    let cube = Cube::new(Vec3::new(0.0, 0.0, -5.0), 1.0, cube_materials);

    // Configurar la escena
    let objects = vec![cube];
    let light = Light::new(Vec3::new(5.0, 5.0, 5.0), Color::new(255.0, 255.0, 255.0), 1.0);
    let scene = Scene::new(objects, Vec3::new(0.0, 5.0, 0.0));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Raytracer")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
    let mut framebuffer = Framebuffer::new(WIDTH as usize, HEIGHT as usize);

    // Control de cámara
    let mut camera_yaw: f32 = 0.0;
    let mut camera_pitch: f32 = 0.0;
    let rotation_speed: f32 = 0.005;
    let mut is_left_mouse_button_pressed = false;
    let mut last_cursor_position: Option<PhysicalPosition<f64>> = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                    is_left_mouse_button_pressed = state == ElementState::Pressed;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if is_left_mouse_button_pressed {
                        if let Some(last_pos) = last_cursor_position {
                            let dx = (position.x - last_pos.x) as f32;
                            let dy = (position.y - last_pos.y) as f32;
                            camera_yaw += dx * rotation_speed;
                            camera_pitch = (camera_pitch + dy * rotation_speed)
                                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
                        }
                        last_cursor_position = Some(position);
                    } else {
                        last_cursor_position = Some(position);
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                if Instant::now() >= next_frame_time {
                    next_frame_time = Instant::now() + frame_duration;
                    let eye_x = camera_yaw.cos() * camera_pitch.cos();
                    let eye_y = camera_pitch.sin();
                    let eye_z = camera_yaw.sin() * camera_pitch.cos();

                    let camera = Camera::new(
                        Vec3::new(eye_x, eye_y, eye_z),
                        Vec3::new(0.0, 0.0, -5.0),
                        Vec3::new(0.0, 1.0, 0.0),
                    );

                    render(&mut framebuffer, &camera, &scene, &light);
                    render_framebuffer_to_pixels(&mut framebuffer, pixels.frame_mut());

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            _ => {}
        }
        window.request_redraw();
    });
}

// Función para copiar el contenido del framebuffer al array de píxeles
fn render_framebuffer_to_pixels(framebuffer: &Framebuffer, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH as usize;
        let y = i / WIDTH as usize;

        let color = framebuffer.get_pixel(x, y);
        let rgba = [
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
            255,
        ];

        pixel.copy_from_slice(&rgba);
    }
}
