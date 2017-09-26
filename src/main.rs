extern crate cgmath;
extern crate image;

mod ray;

use cgmath::prelude::*;
use cgmath::Vector3;

use image::Pixel;

use ray::Ray;

// Linearly blend white and blue depending on the y coordinate
fn color(ray: Ray) -> Vector3<f32> {
    // Normalize ray height to -1.0 to 1.0
    let unit_dir = ray.direction().normalize();
    
    // Scale ray to range 0.0 to 1.0 to get lerp factor
    let t = 0.5 * (unit_dir.y + 1.0);

    // Lerp height to get color
    // Blended Value = (1 - t) * start_value + t * end_value
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;

    let mut image_buffer = image::ImageBuffer::new(nx, ny);

    // Bounds are 4 wide and 2 vertically
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);

    // Horizontal bounds
    let horizontal_scale = Vector3::new(4.0, 0.0, 0.0);
    // Vertical bounds
    let vertical_scale = Vector3::new(0.0, 2.0, 0.0);

    let origin = Vector3::new(0.0, 0.0, 0.0);

    for j in 0..ny {
        for i in 0..nx {
            let horizontal_offset = i as f32 / nx as f32;
            let vertical_offset = j as f32 / ny as f32;

            // Create ray based on offsets
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
