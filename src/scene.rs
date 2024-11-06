use crate::cube::Cube;
use crate::ray_intersect::RayIntersect;
use crate::intersect::Intersect;
use crate::light::Light;  // Asegúrate de tener la estructura Light importada
use nalgebra_glm::Vec3;

pub struct Scene {
    pub cubes: Vec<Cube>,        // Cubos en la escena
    pub lights: Vec<Light>,      // Vector de luces en la escena
}

impl Scene {
    pub fn new(cubes: Vec<Cube>, lights: Vec<Light>) -> Self {
        Self { cubes, lights }
    }

    pub fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let mut closest_intersection: Option<Intersect> = None;

        for cube in &self.cubes {
            if let Some(intersection) = cube.ray_intersect(ray_origin, ray_direction) {
                if closest_intersection.is_none() || intersection.distance < closest_intersection.as_ref().unwrap().distance {
                    closest_intersection = Some(intersection);
                }
            }
        }

        closest_intersection
    }
}
