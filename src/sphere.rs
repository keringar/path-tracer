use cgmath::prelude::*;
use cgmath::Vector3;

use hit::{HitRecord, Hittable};
use material::Material;
use ray::Ray;

/// The sphere is a position in space, an origin and a material.
/// It implemements the Hittable trait which means that rays can interact
/// with it
pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Calculate a vector from the ray origin to the sphere origin
        let oc = ray.origin() - self.center;

        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        // It hit something
        if discriminant > 0.0 {
            // The hit function is a quadratic so we must find both roots since
            // it is a sphere and therefore a circle from the perspective of a ray

            // Only calculate precise roots after we know it hit as a optimization
            // probably premature but it's not a big deal in this case
            let tmp_t = (-b - discriminant.sqrt()) / a;

            // Check float bounds because of floating point errors
            if tmp_t < t_max && tmp_t > t_min {
                let position = ray.point_at_distance(tmp_t);

                let record = HitRecord {
                    t: tmp_t,
                    position,
                    normal: (position - self.center).normalize(),
                    material: self.material,
                };

                return Some(record);
            };

            // Check second square root
            let tmp_t = (-b + discriminant.sqrt()) / a;
            if tmp_t < t_max && tmp_t > t_min {
                let position = ray.point_at_distance(tmp_t);

                let record = HitRecord {
                    t: tmp_t,
                    position,
                    normal: (position - self.center).normalize(),
                    material: self.material,
                };

                return Some(record);
            };
        }

        // It didn't hit anything so return None
        None
    }
}
