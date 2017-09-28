use cgmath::prelude::*;
use cgmath::Vector3;

use hit::{Hittable, HitRecord};
use material::Material;
use ray::Ray;

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
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        // It hit something
        if discriminant > 0.0 {
            // The hit function is a quadratic so we must find both roots since
            // it is a sphere
            let tmp_t = (-b - discriminant.sqrt()) / a;
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