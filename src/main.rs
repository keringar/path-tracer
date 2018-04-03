extern crate cgmath;
extern crate clap;
extern crate image;
extern crate rand;
extern crate rayon;

mod camera;
mod hit;
mod hittable_list;
mod material;
mod ray;
mod scene;
mod sphere;
mod util;

use cgmath::prelude::*;
use cgmath::Vector3;

use clap::{App, Arg};

use image::Pixel;

use rand::Rng;
use rayon::prelude::*;

use std::f32;

use hit::Hittable;
use ray::Ray;

fn main() {
    // Set up clap
    let matches = App::new("ray-tracer")
        .version("0.2")
        .about("Ray traces a scene")
        .author("Warren")
        .arg(
            Arg::with_name("samples")
                .short("s")
                .long("samples")
                .value_name("SAMPLES")
                .help("Sets the number of samples per pixel")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("resx")
                .short("x")
                .long("resx")
                .value_name("RES X")
                .help("Sets the output resolution on the x-axis")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("resy")
                .short("y")
                .long("resy")
                .value_name("RES Y")
                .help("Sets the output resolution on the y-axis")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Convert arg to a u32
    let pixel_res_x = match matches.value_of("resx").unwrap().parse() {
        Ok(res_x) => res_x,
        Err(_) => {
            println!("Provided X-resolution was not a number");
            std::process::exit(-1);
        }
    };

    // Convert arg to a u32
    let pixel_res_y = match matches.value_of("resy").unwrap().parse() {
        Ok(res_y) => res_y,
        Err(_) => {
            println!("Provided Y-resolution was not a number");
            std::process::exit(-1);
        }
    };

    // Convert arg to a u32
    let num_samples = match matches.value_of("samples").unwrap().parse() {
        Ok(samples) => samples,
        Err(_) => {
            println!("Provided number of samples was not valid");
            std::process::exit(-1);
        }
    };

    // Create a scene to render
    let scene = scene::load_scene();

    // Start the rendering stopwatch
    let start_time = std::time::Instant::now();
    println!(
        "Now rendering a {} by {} image with {} samples per pixel",
        pixel_res_x, pixel_res_y, num_samples
    );

    // Render the scene
    let output_buffer = {
        // Camera contains the ray emitter and calculates colors for a PIXEL_RES_X
        // and PIXEL_RES_Y sized image
        let camera = camera::Camera::new(pixel_res_x, pixel_res_y);

        // Create PNG for final output
        let mut image_buffer = image::ImageBuffer::new(pixel_res_x, pixel_res_y);

        for y in 0..pixel_res_y {
            for x in 0..pixel_res_x {
                // Create ray based on offsets from origin to point on plane z = -1
                // The actual ray is exactly the opposite of how it works in real life
                let total_color: Vector3<f32> = (0..num_samples)
                    .into_par_iter()
                    .map(|_| {
                        // Create pRNG for MSAA
                        let mut rng = rand::thread_rng();

                        // Randomly offset each ray by a tiny, random amount to get nice AA
                        let horizontal_offset = (x as f32 + rng.next_f32()) / pixel_res_x as f32;
                        let vertical_offset = (y as f32 + rng.next_f32()) / pixel_res_y as f32;

                        // Initialize a ray starting at the camera aimed at these coords
                        let ray = camera.get_ray_at_coords(horizontal_offset, vertical_offset);

                        // Ray trace the ray and calculate the final color of the ray. color is a
                        // recursive function over the depth. So set depth to zero to start with
                        color(ray, &scene, 0)
                    })
                    .sum();

                // Final color is the average of all samples on a pixel
                let mut rgb = total_color.div_element_wise(num_samples as f32);

                // Convert from colors in range 0.0..1.0 to 0..255
                rgb *= 255.99;
                // Cast to bytes for storage in png
                let rgb = rgb.cast::<u8>().unwrap();
                let pixel = image::Rgba::from_channels(rgb.x, rgb.y, rgb.z, 255);
                // Reverse image beacuse I am indexing from top-down
                image_buffer.put_pixel(x, pixel_res_y - y - 1, pixel);
            }
        }

        image_buffer
    };

    // Write output to the current working directory
    let _ = output_buffer.save("output.png");

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!(
        "Raytracing took {}",
        util::format_seconds(elapsed_time.as_secs())
    );
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
                return scattered_ray.attenuation.mul_element_wise(color(
                    scattered_ray.ray,
                    world,
                    depth + 1,
                ));
            }
        }

        // Ray has scattered so many times that it has been completely absorbed
        return Vector3::new(0.0, 0.0, 0.0);
    } else {
        // Didn't hit anything so set up the skybox
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
