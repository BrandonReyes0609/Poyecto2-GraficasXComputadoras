use nalgebra_glm::Vec3;
use crate::intersect::Intersect;

pub trait RayIntersect {
    // Cambiado para devolver Option<Intersect>
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
 