use cgmath::prelude::*;
use cgmath::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    // Position
    origin: Vector3<f32>,
    // Non-normalized vector
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    pub fn point_at_distance(&self, t: f32) -> Vector3<f32> {
        self.origin + (t * self.direction)
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Vector3::<f32>::zero(),
            direction: Vector3::<f32>::zero(),
        }
    }
}