use cgmath::prelude::*;
use cgmath::Vector3;

use ray::Ray;

/// Camera handles creating new rays and ensuring they are all oriented
/// correctly.
pub struct Camera {
    position: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal_scale: Vector3<f32>,
    vertical_scale: Vector3<f32>,
}

impl Camera {
    pub fn new(res_x: u32, res_y: u32) -> Camera {
        let res_x = res_x as f32;
        let res_y = res_y as f32;

        // Only width is modified to change clipping area. If the
        // height is changed, than the final result is scaled up to match
        // Dunno if this is correct, but it seems pretty similar
        // to what other programs do.
        let width = res_x / res_y;

        Camera {
            position: Vector3::zero(),
            lower_left_corner: Vector3::new(-width, -1.0, -1.0),
            horizontal_scale: Vector3::new(width * 2.0, 0.0, 0.0),
            vertical_scale: Vector3::new(0.0, 2.0, 0.0),
        }
    }

    // Initialize a ray starting at the camera position and pointing toward a point on a plane 1 unit away
    // from the camera
    pub fn get_ray_at_coords(&self, horizontal_offset: f32, vertical_offset: f32) -> Ray {
        let destination = self.lower_left_corner + (horizontal_offset * self.horizontal_scale) + (vertical_offset * self.vertical_scale) - self.position;

        Ray::new(self.position, destination)
    }
}