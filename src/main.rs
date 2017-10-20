extern crate cgmath;
extern crate image;
extern crate rand;

mod camera;
mod hit;
mod hittable_list;
mod material;
mod ray;
mod sphere;

use cgmath::prelude::*;
use cgmath::Vector3;
use image::Pixel;
use rand::Rng;
use std::f32;
use std::time::Instant;

use hit::Hittable;
use material::Material;
use ray::Ray;

// Final rendered image resolution
const PIXEL_RES_X: u32 = 400;
const PIXEL_RES_Y: u32 = 200;

// Number of rays per pixel
const NUM_SAMPLES: u32 = 100;

fn main() {
    // Camera contains the ray emitter and calculates colors for a PIXEL_RES_X
    // and PIXEL_RES_Y sized image
    let camera = camera::Camera::new(PIXEL_RES_X, PIXEL_RES_Y);

    let lambertian_red = Material::new_lambertian(0.8, 0.3, 0.3);
    let lambertian_yellow = Material::new_lambertian(0.8, 0.8, 0.0);
    let metallic = Material::new_metallic(0.8, 0.8, 0.8, 0.3);

    let small_sphere_right = sphere::Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, lambertian_red);
    let small_sphere_left = sphere::Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, metallic);
    let big_sphere = sphere::Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, lambertian_yellow);

    let mut world = hittable_list::HittableList::new();
    world.insert(Box::new(small_sphere_left));
    world.insert(Box::new(small_sphere_right));
    world.insert(Box::new(big_sphere));

    // Create PNG for final output
    let mut image_buffer = image::ImageBuffer::new(PIXEL_RES_X, PIXEL_RES_Y);
    
    // Create pRNG for MSAA
    let mut rng = rand::thread_rng();

    // Start timing to see how long the ray trace takes
    let start_time = Instant::now();
    println!("Now ray tracing a {} by {} image with {} samples", PIXEL_RES_X, PIXEL_RES_Y, NUM_SAMPLES);

    for y in 0..PIXEL_RES_Y {
        for x in 0..PIXEL_RES_X {
            // Create accumulator to calculate the average color of the pixel at (i, j)
            let mut total_color = Vector3::zero();

            // Create ray based on offsets from origin to point on plane z = -1
            // The actual ray is exactly the opposite of how it works in real life
            for _ in 0..NUM_SAMPLES {
                // Randomly offset each ray by a tiny, random amount to get nice AA
                let horizontal_offset = (x as f32 + rng.next_f32()) / PIXEL_RES_X as f32;
                let vertical_offset = (y as f32 + rng.next_f32()) / PIXEL_RES_Y as f32;

                // Initialize a ray starting at the camera aimed at these coords
                let ray = camera.get_ray_at_coords(horizontal_offset, vertical_offset);

                // Ray trace the ray and calculate the final color of the ray. color is a 
                // recursive function over the depth. So set it to zero to start with
                total_color += color(ray, &world, 0);
            }

            // Final color is the average of all samples on a pixel
            let mut rgb = total_color.div_element_wise(NUM_SAMPLES as f32);

            // Convert from colors in range 0.0..1.0 to 0..255
            rgb *= 255.99;
            // Cast to bytes for storage in png
            let rgb = rgb.cast::<u8>();
            let pixel = image::Rgba::from_channels(rgb.x, rgb.y, rgb.z, 255);
            // Reverse image beacuse I am indexing from top-down
            image_buffer.put_pixel(x, PIXEL_RES_Y - y - 1, pixel);
        }
    }

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Raytracing took {}", format_seconds(elapsed_time.as_secs()));

    // Save the image to the current working directory
    let _ = image_buffer.save("output.png");
}

// Format seconds into a HH:MM:SS string
fn format_seconds(secs: u64) -> String {
    let hours = secs / 3600;
    let secs = secs % 3600;
    let minutes = secs / 60;
    let secs = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

// Ray takes on the end color after up to 50 scatters/reflections
fn color<H: Hittable>(ray: Ray, world: &H, depth: u32) -> Vector3<f32> {
    // Ray trace through the world and check if it hit anything between 0.001 and f32::MAX distance
    if let Some(record) = world.hit(ray, 0.001, f32::MAX) {
        // Limit the closest distance because otherwise rays would just keep bouncing due to
        // low precision floats or would recurse too deeply and overflow the stack
        if depth < 50 {
            if let Some(scattered_ray) = record.material.scatter(ray, record) {
                // Attenuate ray based on the surface color
                return scattered_ray.attenuation.mul_element_wise(color(scattered_ray.ray, world, depth + 1));
            }
        }

        // Ray has scattered so many times that it has been completely absorbed
        return Vector3::new(0.0, 0.0, 0.0);
    } else {
        // Create a background gradient by lerping white and blue over the height

        // Normalize ray height to -1.0 to 1.0
        let height = ray.direction().normalize().y;
        // Scale ray to range 0.0 to 1.0 to get lerp factor
        let t = 0.5 * (height + 1.0);
        // Lerp height to get color
        // Blended Value = (1 - t) * start_value + t * end_value where t is the lerp factor
       (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}