extern crate cgmath;
extern crate image;

mod hit;
mod hittable_list;
mod ray;
mod sphere;

use cgmath::prelude::*;
use cgmath::Vector3;

use image::Pixel;

use hit::Hittable;
use ray::Ray;

fn color<H: Hittable>(ray: Ray, world: &H) -> Vector3<f32> {
    if let Some(record) = world.hit(ray, 0.0, 1000.0) {
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

    let small_sphere = sphere::Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let big_sphere = sphere::Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);

    let mut world = hittable_list::HittableList::new();
    world.insert(Box::new(small_sphere));
    world.insert(Box::new(big_sphere));

    for j in 0..ny {
        for i in 0..nx {
            let horizontal_offset = i as f32 / nx as f32;
            let vertical_offset = j as f32 / ny as f32;

            // Create ray based on offsets from origin to point on plane z = -1
            let ray = Ray::new(origin, lower_left_corner +
                               horizontal_offset * horizontal_scale +
                               vertical_offset * vertical_scale);

            let mut rgb = color(ray, &world);

            // Convert from colors in range 0.0..1.0 to 0..255
            rgb *= 255.99;
            // Cast to bytes for storage
            let rgb = rgb.cast::<u8>();
            let pixel = image::Rgba::from_channels(rgb.x, rgb.y, rgb.z, 255);
            // Reverse image beacuse I am indexing from top-down
            image_buffer.put_pixel(i, ny - j - 1, pixel);
        }
    }

    let _ = image_buffer.save("output.png");
}
