use cgmath::prelude::*;
use cgmath::Vector3;

/// Ray is a simple new type over a Vector3<f32> that contains a direction
/// and an origin.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    // Position
    origin: Vector3<f32>,
    // Normalized vector
    direction: Vector3<f32>,
}

impl Ray {
    /// Create a new ray with the specified origin and direction
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { origin, direction }
    }

    /// Get the ray origin position
    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    /// Get the direction of the ray
    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    /// Get the position of a point along the ray that lies t distance
    /// away from the origin position of the ray
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
