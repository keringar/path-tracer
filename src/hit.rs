use cgmath::Vector3;

use material::Material;
use ray::Ray;

/// Interface of all objects that a ray can interact with
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

/// Struct containg all the data necessary to model a ray-object collision
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    // Distance along ray that it hit
    pub t: f32,
    // Position in the world that ray intersected
    pub position: Vector3<f32>,
    // Surface normal at the point where the ray hit
    pub normal: Vector3<f32>,
    // The material of the surface that the ray last hit
    pub material: Material,
}