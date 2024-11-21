use crate::ray_intersect::RayIntersect;
use crate::intersect::Intersect;
use crate::material::Material;
use nalgebra_glm::{Vec3, dot};

#[derive(Debug, Clone)]
pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub materials: [Material; 6], // Material específico para cada cara del cubo
}
 
impl Cube {
    pub fn new(center: Vec3, size: f32, materials: [Material; 6]) -> Self {
        Self {
            center,
            size,
            materials,
        }
    }

    /// Calcula la intersección de un rayo con el cubo.
    fn intersect_face(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
        face_normal: Vec3,
        face_center: Vec3,
        material_index: usize,
    ) -> Option<Intersect> {
        let denom = dot(&face_normal, ray_direction);
        if denom.abs() > 1e-6 {
            let t = dot(&(face_center - ray_origin), &face_normal) / denom;
            if t > 0.0 {
                let hit_point = ray_origin + ray_direction * t;
                
                // Verificar si el punto de intersección está dentro de la cara
                let half_size = self.size / 2.0;
                if (hit_point.x - face_center.x).abs() <= half_size &&
                   (hit_point.y - face_center.y).abs() <= half_size &&
                   (hit_point.z - face_center.z).abs() <= half_size {

                    let (u, v) = self.calculate_uv(&hit_point, &face_normal);
                    return Some(Intersect::new(
                        hit_point,
                        face_normal,
                        t,
                        self.materials[material_index].clone(),
                        u,
                        v,
                    ));
                }
            }
        }
        None
    }

    /// Calcula las coordenadas UV para una cara del cubo.
    fn calculate_uv(&self, hit_point: &Vec3, normal: &Vec3) -> (f32, f32) {
        let half_size = self.size / 2.0;
        let local_point = hit_point - self.center;
    
        let (u, v) = if normal.x.abs() > 0.5 {
            // Gira 180 grados en la dirección X
            ((1.0 - (local_point.z + half_size) / self.size), (1.0 - (local_point.y + half_size) / self.size))
        } else if normal.y.abs() > 0.5 {
            // Gira 180 grados en la dirección Y
            ((1.0 - (local_point.x + half_size) / self.size), (1.0 - (local_point.z + half_size) / self.size))
        } else {
            // Gira 180 grados en la dirección Z
            ((1.0 - (local_point.x + half_size) / self.size), (1.0 - (local_point.y + half_size) / self.size))
        };
    
        (u, v)
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let half_size = self.size / 2.0;

        let faces = [
            (Vec3::new(1.0, 0.0, 0.0), self.center + Vec3::new(half_size, 0.0, 0.0), 0), // +X
            (Vec3::new(-1.0, 0.0, 0.0), self.center - Vec3::new(half_size, 0.0, 0.0), 1), // -X
            (Vec3::new(0.0, 1.0, 0.0), self.center + Vec3::new(0.0, half_size, 0.0), 2), // +Y
            (Vec3::new(0.0, -1.0, 0.0), self.center - Vec3::new(0.0, half_size, 0.0), 3), // -Y
            (Vec3::new(0.0, 0.0, 1.0), self.center + Vec3::new(0.0, 0.0, half_size), 4), // +Z
            (Vec3::new(0.0, 0.0, -1.0), self.center - Vec3::new(0.0, 0.0, half_size), 5), // -Z
        ];

        let mut closest_intersect: Option<Intersect> = None;

        for (normal, center, material_index) in faces.iter() {
            if let Some(intersection) = self.intersect_face(ray_origin, ray_direction, *normal, *center, *material_index) {
                if closest_intersect.is_none() || intersection.distance < closest_intersect.as_ref().unwrap().distance {
                    closest_intersect = Some(intersection);
                }
            }
        }

        closest_intersect
    }
}
