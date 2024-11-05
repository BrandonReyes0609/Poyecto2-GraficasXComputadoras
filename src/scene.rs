use crate::cube::Cube;
use crate::ray_intersect::RayIntersect;  // Importa el rasgo RayIntersect
use crate::intersect::Intersect;
use nalgebra_glm::Vec3;

pub struct Scene {
    pub cubes: Vec<Cube>, // Cambiar de `spheres` a `cubes`
    pub light_position: Vec3,
}

impl Scene {
    pub fn new(cubes: Vec<Cube>, light_position: Vec3) -> Self {
        Self { cubes, light_position }
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
