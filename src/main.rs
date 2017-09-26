extern crate cgmath;
extern crate image;
extern crate rand;

mod camera;
mod hit;
mod hittable_list;
mod ray;
mod sphere;

use cgmath::prelude::*;
use cgmath::Vector3;
use image::Pixel;
use rand::Rng;
use std::f32;
use std::time::Instant;

use hit::Hittable;
use ray::Ray;

fn color<H: Hittable>(ray: Ray, world: &H) -> Vector3<f32> {
    if let Some(record) = world.hit(ray, 0.0, f32::MAX) {
        // Normalize normals to be between 0 and 1 for color
        0.5 * record.normal.add_element_wise(1.0)
    } else {
        // Linearly blend white and blue depending on the y coordinate to get background
        // Normalize ray height to -1.0 to 1.0
        let unit_dir = ray.direction().normalize();
        // Scale ray to range 0.0 to 1.0 to get lerp factor
        let t = 0.5 * (unit_dir.y + 1.0);
        // Lerp height to get color
        // Blended Value = (1 - t) * start_value + t * end_value
       (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // Final pixel resolution
    let nx = 200;
    let ny = 100;
    // Number of rays per pixel
    let num_samples = 10;

    let mut image_buffer = image::ImageBuffer::new(nx, ny);

    let camera = camera::Camera::new(nx, ny);

    let small_sphere = sphere::Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let big_sphere = sphere::Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);

    let mut world = hittable_list::HittableList::new();
    world.insert(Box::new(small_sphere));
    world.insert(Box::new(big_sphere));

    let start_time = Instant::now();

    for j in 0..ny {
        for i in 0..nx {
            let mut total_color = Vector3::zero();

            // Create ray based on offsets from origin to point on plane z = -1
            for _ in 0..num_samples {
                // Randomly offset each ray by a tiny, random amount to get nice AA
                let horizontal_offset = (i as f32 + rng.next_f32()) / nx as f32;
                let vertical_offset = (j as f32 + rng.next_f32()) / ny as f32;

                let ray = camera.get_ray_at_coords(horizontal_offset, vertical_offset);

                total_color += color(ray, &world);
            }

            // Final color is the average of all samples on a pixel
            let mut rgb = total_color.div_element_wise(num_samples as f32);

            // Convert from colors in range 0.0..1.0 to 0..255
            rgb *= 255.99;
            // Cast to bytes for storage
            let rgb = rgb.cast::<u8>();
            let pixel = image::Rgba::from_channels(rgb.x, rgb.y, rgb.z, 255);
            // Reverse image beacuse I am indexing from top-down
            image_buffer.put_pixel(i, ny - j - 1, pixel);
        }
    }

    let elapsed_time = start_time.elapsed();
    println!("Raytracing with {} samples took {}", num_samples, format_seconds(elapsed_time.as_secs()));

    let _ = image_buffer.save("output.png");
}

fn format_seconds(secs: u64) -> String {
    let hours = secs / 3600;
    let secs = secs % 3600;
    let minutes = secs / 60;
    let secs = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}