use crate::ray_intersect::RayIntersect;
use crate::intersect::Intersect;
use crate::material::Material;
use nalgebra_glm::{Vec3, dot};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let hit_vec = (point - self.center).normalize();
        let u = 0.5 + hit_vec.z.atan2(hit_vec.x) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - hit_vec.y.asin() / std::f32::consts::PI;
        (u, v)
    }
}
 
impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return None;
        }
        
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t > 0.0 {
            let hit_point = ray_origin + ray_direction * t;
            let normal = (hit_point - self.center).normalize();
            let (u, v) = self.get_uv(&hit_point);
            return Some(Intersect::new(hit_point, normal, t, self.material.clone(), u, v));
        }
        None
    }
}
