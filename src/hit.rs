use cgmath::Vector3;
use cgmath::Zero;

use ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    // Distance along ray that it hit
    pub t: f32,
    // Position in the world that ray intersected
    pub position: Vector3<f32>,
    // Surface normal at the point where the ray hit
    pub normal: Vector3<f32>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            t: 0.0,
            position: Vector3::<f32>::zero(),
            normal: Vector3::<f32>::zero(),
        }
    }
}