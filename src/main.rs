extern crate cgmath;
extern crate image;

mod ray;

use cgmath::prelude::*;
use cgmath::Vector3;

use image::Pixel;

use ray::Ray;

// Linearly blend white and blue depending on the y coordinate
fn color(ray: Ray) -> Vector3<f32> {
    // Check if ray hit sphere. If it does, it returns the point along the ray
    // that hit the sphere
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, ray);
    // It hit the sphere at some value t so calculate surface normals at that point
    if t > 0.0 {
        // The normal simply points directly from the center to the point on the surface
        // of the sphere. Which can be found by subtracting the center from the point
        // on the surface given by ray.point_at_distance(t)
        let mut normal = ray.point_at_distance(t) - Vector3::new(0.0, 0.0, -1.0);
        normal.normalize();
        return 0.5 * Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    // Normalize ray height to -1.0 to 1.0
    let unit_dir = ray.direction().normalize();
    
    // Scale ray to range 0.0 to 1.0 to get lerp factor
    let t = 0.5 * (unit_dir.y + 1.0);

    // Lerp height to get color
    // Blended Value = (1 - t) * start_value + t * end_value
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vector3<f32>, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn main() {
    let nx = 800;
    let ny = 400;

    let mut image_buffer = image::ImageBuffer::new(nx, ny);

    let width = nx as f32 / ny as f32;
    let height = (ny as f32 / nx as f32) * width;

    // Bounds are 4 wide and 2 vertically
    let lower_left_corner = Vector3::new(-width, -height, -1.0);

    // Horizontal bounds
    let horizontal_scale = Vector3::new(width * 2.0, 0.0, 0.0);
    // Vertical bounds
    let vertical_scale = Vector3::new(0.0, height * 2.0, 0.0);

    let origin = Vector3::new(0.0, 0.0, 0.0);

    for j in 0..ny {
        for i in 0..nx {
            let horizontal_offset = i as f32 / nx as f32;
            let vertical_offset = j as f32 / ny as f32;

            // Create ray based on offsets from origin to point on plane z = -1
            let ray = Ray::new(origin, lower_left_corner +
                               horizontal_offset * horizontal_scale +
                               vertical_offset * vertical_scale);

            let mut rgb = color(ray);

            rgb *= 255.99;
            let rgb = rgb.cast::<u8>();
            let pixel = image::Rgba::from_channels(rgb.x, rgb.y, rgb.z, 255);
            image_buffer.put_pixel(i, ny - j - 1, pixel);
        }
    }

    let _ = image_buffer.save("output.png");
}
