use nalgebra_glm::{Vec3, cos, sin}; // Añadir cos y sin aquí
use crate::color::Color;
use crate::material::Material;
use crate::scene::Scene;
use crate::cube::Cube;
use crate::camera::Camera;
use crate::render::render;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent, MouseButton, ElementState, MouseScrollDelta};
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

    // Cargar texturas de tierra
    let dirt_texture = ImageReader::open("assets/dirt/dirt.png").unwrap().decode().unwrap();
    let podzol_top_texture = ImageReader::open("assets/dirt/dirt_podzol_top.png").unwrap().decode().unwrap();
    let podzol_side_texture = ImageReader::open("assets/dirt/dirt_podzol_side.png").unwrap().decode().unwrap();

    // Crear material para el cubo de tierra
    let cube1_dirt = [
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(dirt_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(dirt_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(podzol_top_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(dirt_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(podzol_side_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(podzol_side_texture.clone())]),
    ];

    // Crear la plataforma de tierra
    let mut cubes = Vec::new();
    let spacing = 1.0;
    for i in 0..5 {
        for j in 0..5 {
            let position = Vec3::new(i as f32 * spacing, -1.0, j as f32 * spacing - 5.0);
            let cube = Cube::new(position, 1.0, cube1_dirt.clone());
            cubes.push(cube);
        }
    }

    // Cargar texturas de madera
    let log_spruce_texture = ImageReader::open("assets/oak/log_spruce.png").unwrap().decode().unwrap();
    let log_spruce_top_texture = ImageReader::open("assets/oak/log_spruce_top.png").unwrap().decode().unwrap();

    // Crear material para el cubo de madera
    let cube_wood = [
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(log_spruce_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(log_spruce_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(log_spruce_top_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(log_spruce_top_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(log_spruce_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(log_spruce_texture.clone())]),
    ];

    // Posición inicial para los bloques de madera
    let base_position = Vec3::new(2.0 * spacing, 0.0, 2.0 * spacing - 5.0);

    // Crear un ciclo para apilar 3 bloques de madera
    for i in 0..3 {
        let position = base_position + Vec3::new(0.0, i as f32, 0.0); // Eleva la posición en el eje Y para cada bloque
        let wood_cube = Cube::new(position, 1.0, cube_wood.clone());
        cubes.push(wood_cube);
    }

    //hojs-------------------------
    // Cargar textura de hojas
    let leaves_texture = ImageReader::open("assets/oak/leaves_oak_opaque.png").unwrap().decode().unwrap();

    // Crear material para el cubo de hojas
    let cube_leaves = [
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(leaves_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(leaves_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(leaves_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(leaves_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(leaves_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(leaves_texture.clone())]),
    ];

    // Posición de la plataforma de hojas (3x3) centrada sobre el bloque de madera más alto
    let top_wood_position = base_position + Vec3::new(0.0, 3.0, 0.0); // Posición del bloque de madera más alto
    for i in -1..=1 {
        for j in -1..=1 {
            let position = top_wood_position + Vec3::new(i as f32 * spacing, 0.0, j as f32 * spacing);
            let leaf_cube = Cube::new(position, 1.0, cube_leaves.clone());
            cubes.push(leaf_cube);
        }
    }

    // Agregar un bloque de hojas adicional en el centro, encima de la plataforma de hojas
    let center_leaf_position = top_wood_position + Vec3::new(0.0, 1.0, 0.0); // Posición en el centro, una unidad arriba
    let center_leaf_cube = Cube::new(center_leaf_position, 1.0, cube_leaves.clone());
    cubes.push(center_leaf_cube);
    //FIn hojs--------------------------------

    // panal -----------
    // Cargar texturas del panal de abejas
    let bee_nest_front_texture = ImageReader::open("assets/bee_nest/bee_nest_front.png").unwrap().decode().unwrap();
    let bee_nest_front_honey_texture = ImageReader::open("assets/bee_nest/bee_nest_front_honey.png").unwrap().decode().unwrap();
    let bee_nest_side_texture = ImageReader::open("assets/bee_nest/bee_nest_side.png").unwrap().decode().unwrap();
    let bee_nest_top_texture = ImageReader::open("assets/bee_nest/bee_nest_top.png").unwrap().decode().unwrap();

    // Crear material para el bloque de panal de abejas
    let cube_bee_nest = [
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bee_nest_side_texture.clone())]), // Lado izquierdo
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bee_nest_side_texture.clone())]), // Lado derecho
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bee_nest_top_texture.clone())]),   // Arriba
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bee_nest_front_texture.clone())]), // Abajo
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bee_nest_front_honey_texture.clone())]), // Frente
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bee_nest_side_texture.clone())]),  // Atrás
    ];

    // Posicionar el bloque de panal de abejas al lado del bloque de madera y debajo de la plataforma de hojas
    let bee_nest_position = base_position + Vec3::new(1.0, 0.0, 0.0); // Ajusta la posición según tus necesidades
    let bee_nest_cube = Cube::new(bee_nest_position, 1.0, cube_bee_nest.clone());
    cubes.push(bee_nest_cube);
    // fin panal ------------
    

    // libreria -----------------------------------

    // Cargar texturas de librería
    let planks_texture = ImageReader::open("assets/planks_oak.png").unwrap().decode().unwrap();
    let bookshelf_texture = ImageReader::open("assets/bookshelg/bookshelf.png").unwrap().decode().unwrap();

    // Crear material para el cubo de librería
    let cube_bookshelf = [
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bookshelf_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bookshelf_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(planks_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(planks_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bookshelf_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(bookshelf_texture.clone())]),
    ];

    // Posición de la librería justo encima de la plataforma de tierra
    //                                              ,arriba abajo,
    let bookshelf_position = Vec3::new(0.0 * spacing, 0.0, 1.0 * spacing - 5.0);
    let bookshelf_cube = Cube::new(bookshelf_position, 1.0, cube_bookshelf);
    cubes.push(bookshelf_cube);
    // fin libreria ------------------------------




    
    // Cargar texturas de la mesa de trabajo
    let crafting_table_front_texture = ImageReader::open("assets/crafting/crafting_table_front.png").unwrap().decode().unwrap();
    let crafting_table_side_texture = ImageReader::open("assets/crafting/crafting_table_side.png").unwrap().decode().unwrap();
    let crafting_table_top_texture = ImageReader::open("assets/crafting/crafting_table_top.png").unwrap().decode().unwrap();

    // Crear material para el cubo de la mesa de trabajo
    let cube_crafting_table = [
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(crafting_table_side_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(crafting_table_side_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(crafting_table_top_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(crafting_table_top_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(crafting_table_front_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(crafting_table_front_texture.clone())]),
    ];

    // Posición del bloque de la mesa de trabajo en el centro de la plataforma de tierra
    let crafting_table_position = Vec3::new(0.0 * spacing, 0.0, 2.0 * spacing - 5.0);
    let crafting_table_cube = Cube::new(crafting_table_position, 1.0, cube_crafting_table);
    cubes.push(crafting_table_cube);
    //-------------------------------------------------------
    //mesa de horno----------------



    //--------------------------------
        //mesa de horno----------------

    // Cargar texturas de la mesa de trabajo
    let furnace_front_on_texture = ImageReader::open("assets/furnace/furnace_front_on.png").unwrap().decode().unwrap();
    let furnace_side_texture = ImageReader::open("assets/furnace/furnace_side.png").unwrap().decode().unwrap();
    let furnace_top_texutre = ImageReader::open("assets/furnace/furnace_top.png").unwrap().decode().unwrap();
    // Configurar la escena
    // Definir las coordenadas deseadas para el horno
    let furnace_position = Vec3::new(0.0 * spacing, 0.0, 0.0 * spacing - 5.0);

    // Crear el material del bloque de horno
    let cube_furnace = [
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(furnace_front_on_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(furnace_side_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(furnace_top_texutre.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(furnace_top_texutre.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(furnace_front_on_texture.clone())]),
        Material::new(Color::black(), 1.0, [0.9, 0.1, 0.0, 0.0], 1.0, vec![Some(furnace_side_texture.clone())]),
    ];

    // Crear el cubo de horno y añadirlo a la escena
    let furnace_cube = Cube::new(furnace_position, 1.0, cube_furnace);
    cubes.push(furnace_cube);



    // Inicializamos el ángulo y velocidad de la luz solar
    let mut sun_angle: f32 = 0.0;
    let sun_rotation_speed = 0.5; // Velocidad de rotación de la luz solar
    let sun_light_intensity = 5.0; // Intensidad de la luz solar

    // Crear la fuente de luz del horno
    let furnace_light = Light::new(
        furnace_position + Vec3::new(0.0, 0.5, 0.0),
        Color::new(255.0, 140.0, 0.0),
        2.0,
    );

    // Configuración de la ventana, el búfer de píxeles y la cámara
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Raytracer")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
    let mut framebuffer = Framebuffer::new(WIDTH as usize, HEIGHT as usize);

    let center_position = Vec3::new(2.0, 0.0, -3.0);
    let mut distance_from_center = 10.0;
    let mut camera_yaw: f32 = 0.0;
    let mut camera_pitch: f32 = 0.0;
    let rotation_speed: f32 = 0.005;
    let zoom_speed: f32 = 0.2;
    let mut is_left_mouse_button_pressed = false;
    let mut last_cursor_position: Option<PhysicalPosition<f64>> = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        is_left_mouse_button_pressed = state == ElementState::Pressed;
                    }
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
                WindowEvent::MouseWheel { delta, .. } => {
                    if let MouseScrollDelta::LineDelta(_, scroll) = delta {
                        distance_from_center -= scroll * zoom_speed;
                        distance_from_center = distance_from_center.clamp(2.0, 20.0);
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                if Instant::now() >= next_frame_time {
                    next_frame_time = Instant::now() + frame_duration;

                    // Actualizar el ángulo de la luz solar
                    sun_angle += sun_rotation_speed;
                    let sun_x = 5.0 * sun_angle.cos();
                    let sun_y = 10.0 * sun_angle.sin(); // Altura de la luz, moviéndose de arriba hacia abajo
                    let sun_z = 5.0 * sun_angle.sin();

                    // Configurar la luz del sol en posición circular
                    let main_light = Light::new(
                        Vec3::new(sun_x, sun_y, sun_z),
                        Color::new(255.0, 255.0, 255.0),
                        sun_light_intensity,
                    );

                    // Crear la escena con ambas luces (horno y luz solar en movimiento)
                    let scene = Scene::new(cubes.clone(), vec![main_light, furnace_light.clone()]);

                    // Actualizar la posición de la cámara y renderizar
                    let camera_position = Vec3::new(
                        center_position.x + distance_from_center * camera_yaw.cos() * camera_pitch.cos(),
                        center_position.y + distance_from_center * camera_pitch.sin(),
                        center_position.z + distance_from_center * camera_yaw.sin() * camera_pitch.cos(),
                    );

                    let camera = Camera::new(
                        camera_position,
                        center_position,
                        Vec3::new(0.0, 1.0, 0.0),
                    );

                    render(&mut framebuffer, &camera, &scene, &scene.lights);
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