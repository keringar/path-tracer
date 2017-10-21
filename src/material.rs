use cgmath::Vector3;
use cgmath::prelude::*;
use rand::{Rand, self};

use hit::HitRecord;
use ray::Ray;

/// Contains the data of a newly created ray, attenuation is a measure of
/// how much the hit impacted the ray absorbtion
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vector3<f32>,
}

/// List of all possible materials
#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian {
        albedo: Vector3<f32>,
    },
    Metallic {
        albedo: Vector3<f32>,
        fuzziness: f32,
    },
    Dielectric {
        refractive_index: f32,
    }
}

impl Material {
    pub fn new_lambertian(r: f32, g: f32, b: f32) -> Material {
        Material::Lambertian {
            albedo: Vector3::new(r, g, b),
        }
    }

    pub fn new_metallic(r: f32, g: f32, b: f32, fuzziness: f32) -> Material {
        Material::Metallic {
            albedo: Vector3::new(r, g, b),
            fuzziness,
        }
    }

    pub fn new_dielectric(refractive_index: f32) -> Material {
        Material::Dielectric {
            refractive_index
        }
    }
}

impl Material {
    // Figure out what happens to a ray when it hits an object. Returns None if the ray was absorbed
    pub fn scatter(&self, ray: Ray, record: HitRecord) -> Option<ScatteredRay> {
        match self {
            // Add a random value in a unit sphere to the surface normal to get the ray, resulting in a perfectly
            // diffuse material
            &Material::Lambertian{ albedo } => {
                // bounce direction for a diffuse material
                let bounce_dir = record.position + record.normal + random_position_in_unit_sphere();
                let bounced_ray = Ray::new(record.position, bounce_dir - record.position);

                Some(ScatteredRay {
                    ray: bounced_ray,
                    attenuation: albedo,
                })
            },
            // Metallic materials just do a simple reflection, with an optional random fuzziness parameter
            &Material::Metallic{ albedo, fuzziness } => {
                // Calculate reflected ray vector with some cross products
                let reflected = reflect(ray.direction(), record.normal);//ray.direction() - 2.0 * (ray.direction().dot(record.normal)) * record.normal;
                // Add an fuziness parameter to the ray bounce direction
                let fuzzy_ray = reflected + (random_position_in_unit_sphere() * fuzziness);
                // Create a new ray starting from the hit location and pointing toward the reflected ray dir
                let bounced_ray = Ray::new(record.position, fuzzy_ray);

                Some(ScatteredRay {
                    ray: bounced_ray,
                    attenuation: albedo,
                })
            },
            // Materials like glass or water
            &Material::Dielectric{ refractive_index } => {
                // If > 0, the incoming ray is in the same dir as the normal, so for refractions
                // the normal used is opposite the normal normal
                let (normal_out, ni_over_nt) = if ray.direction().dot(record.normal) > 0.0 {
                    (-record.normal, 1.0 / refractive_index)
                } else {
                    (record.normal, refractive_index)
                };

                if let Some(refracted) = refract(ray.direction(), normal_out, ni_over_nt) {
                    // Ray is refracted
                    let refracted_ray = Ray::new(record.position, refracted);

                    Some(ScatteredRay {
                        ray: refracted_ray,
                        attenuation: Vector3::<f32>::new(1.0, 1.0, 1.0),
                    })
                } else {
                    // Ray is reflected
                    let reflected = reflect(ray.direction(), record.normal);
                    let reflected_ray = Ray::new(record.position, reflected);

                    Some(ScatteredRay {
                        ray: reflected_ray,
                        attenuation: Vector3::<f32>::new(1.0, 1.0, 1.0),
                    })
                }
            }
        }
    }
}

// See docs/Diffuse.PNG
fn random_position_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    let mut random_position = Vector3::<f32>::rand(&mut rng);

    while random_position.distance2(Vector3::zero()) >= 1.0 {
        random_position = Vector3::<f32>::rand(&mut rng);
    }

    random_position
}

// Returns a reflected ray from a normal
fn reflect(incoming_dir: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    incoming_dir - 2.0 * (incoming_dir.dot(normal)) * normal
}

// Returns either a refracted ray or none if it was reflected
fn refract(incoming_dir: Vector3<f32>, normal: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = incoming_dir.normalize();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - (ni_over_nt * ni_over_nt) * (1.0 - (dt * dt));

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt();
        
        Some(refracted)
    } else {
        None
    }
}