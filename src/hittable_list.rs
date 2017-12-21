use cgmath::prelude::*;

use hit::{HitRecord, Hittable};
use ray::Ray;

// A collection of Hittable objects
pub struct HittableList {
    hittable: Vec<Box<Hittable + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            hittable: Vec::new(),
        }
    }

    pub fn insert(&mut self, obj: Box<Hittable + Sync>) {
        self.hittable.push(obj);
    }
}

// Returns the closest object in the colleciton to the camera
// since all others would be occluded
impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut current_closest_hit = None;

        // Iterate through the list of objects and check if they were it
        for hittable in &self.hittable {
            if let Some(record) = hittable.hit(ray, t_min, t_max) {
                // Calculate which one was closest to us
                match current_closest_hit {
                    None => current_closest_hit = Some(record),
                    Some(closest) => {
                        // Calculate distances from origin to determine which is closer
                        let closest_distance = closest.position.distance2(ray.origin());
                        let current_distance = record.position.distance2(ray.origin());

                        if current_distance < closest_distance {
                            current_closest_hit = Some(record);
                        }
                    }
                }
            }
        }

        current_closest_hit
    }
}
