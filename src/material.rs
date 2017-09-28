use cgmath::Vector3;
use cgmath::prelude::*;
use rand::{Rand, self};

use hit::HitRecord;
use ray::Ray;

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vector3<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian {
        albedo: Vector3<f32>,
    },
    Metallic {
        albedo: Vector3<f32>,
    }
}

impl Material {
    pub fn new_lambertian(r: f32, g: f32, b: f32) -> Material {
        Material::Lambertian {
            albedo: Vector3::new(r, g, b),
        }
    }

    pub fn new_metallic(r: f32, g: f32, b: f32) -> Material {
        Material::Metallic {
            albedo: Vector3::new(r, g, b),
        }
    }
}

impl Material {
    pub fn scatter(&self, ray: Ray, record: HitRecord) -> Option<ScatteredRay> {
        match self {
            &Material::Lambertian{ albedo } => {
                // Add a random value in a unit sphere to the surface normal to get the ray
                // bounce direction for a diffuse material
                let bounce_dir = record.position + record.normal + random_position_in_unit_sphere();
                let bounced_ray = Ray::new(record.position, bounce_dir - record.position);

                Some(ScatteredRay {
                    ray: bounced_ray,
                    attenuation: albedo,
                })
            },
            &Material::Metallic{ albedo } => {
                // Metallic materials just do a simple reflection
                // Just subtract the normal facing vector from the incoming ray
                let reflected = ray.direction() - 2.0 * (ray.direction().dot(record.normal)) * record.normal;
                let bounced_ray = Ray::new(record.position, reflected);

                Some(ScatteredRay {
                    ray: bounced_ray,
                    attenuation: albedo,
                })
            }
        }
    }
}

fn random_position_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    let mut random_position = Vector3::<f32>::rand(&mut rng);

    while random_position.distance2(Vector3::zero()) >= 1.0 {
        random_position = Vector3::<f32>::rand(&mut rng);
    }

    random_position
}