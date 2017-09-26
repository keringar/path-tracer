use cgmath::prelude::*;
use cgmath::Vector3;

use ray::Ray;

pub struct Camera {
    position: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal_scale: Vector3<f32>,
    vertical_scale: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector3::zero(),
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal_scale: Vector3::new(4.0, 0.0, 0.0),
            vertical_scale: Vector3::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray_at_coords(&self, horizontal_offset: f32, vertical_offset: f32) -> Ray {
        let destination = self.lower_left_corner + (horizontal_offset * self.horizontal_scale) + (vertical_offset * self.vertical_scale) - self.position;

        Ray::new(self.position, destination)
    }
}