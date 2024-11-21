use crate::scene::Scene;
use crate::color::Color;
use crate::light::Light;
use crate::intersect::Intersect;
use crate::ray_intersect::RayIntersect;
use nalgebra_glm::Vec3;
use crate::cube::Cube;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let bias = 0.001;
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point + intersect.normal * bias;
    let mut shadow_intensity = 0.9;

    for object in objects {
        if let Some(shadow_intersect) = object.ray_intersect(&shadow_ray_origin, &light_dir) {
            let distance_to_light = (light.position - intersect.point).magnitude();
            let distance_to_object = shadow_intersect.distance;
            if distance_to_object < distance_to_light {
                shadow_intensity = distance_to_object / distance_to_light;
                break;
            }
        }
    }
    
    shadow_intensity
}

pub fn cast_ray(
    scene: &Scene,
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    light: &Light,
    depth: u32,
) -> Color {
    if depth > 3 {
        return Color::new(0.2, 0.7, 1.0); // Color de fondo
    }

    if let Some(intersection) = scene.ray_intersect(ray_origin, ray_direction) {
        let light_dir = (light.position - intersection.point).normalize();
        let diffuse_intensity = intersection.normal.dot(&light_dir).max(0.0);
        let light_intensity = light.intensity;

        // Color difuso
        let diffuse = intersection
            .material
            .get_texture_color(intersection.u, intersection.v, 0) // Usamos un índice predeterminado
            * intersection.material.albedo[0]
            * diffuse_intensity
            * light_intensity;

        // Sombra
        let shadow_intensity = cast_shadow(&intersection, light, &scene.cubes);
        let diffuse_with_shadow = diffuse * shadow_intensity;

        // Reflexión
        let mut final_color = diffuse_with_shadow;
        if intersection.material.albedo[1] > 0.0 {
            let reflected_direction = reflect(ray_direction, &intersection.normal).normalize();
            let reflected_color =
                cast_ray(scene, &intersection.point, &reflected_direction, light, depth + 1);
            final_color = final_color + reflected_color * intersection.material.albedo[1];
        }

        return final_color;
    }

    Color::new(0.2, 0.7, 1.0) // Color de fondo
}
  
fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // El rayo está entrando en el objeto
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // El rayo está saliendo del objeto
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        // Reflexión interna total
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}
