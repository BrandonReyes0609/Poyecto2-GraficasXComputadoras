use nalgebra_glm::Vec3;
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material, u: f32, v: f32) -> Self {
        Self {
            point,
            normal,
            distance,
            material,
            u,
            v,
        }
    }

    pub fn empty() -> Self {
        Self {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            material: Material::black(),
            u: 0.0,
            v: 0.0,
        }
    }
}
