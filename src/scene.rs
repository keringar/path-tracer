use cgmath::Vector3;
use sphere;
use material::Material;
use hit::Hittable;
use hittable_list;

pub fn load_scene() -> hittable_list::HittableList {
    let lambertian_blue = Material::new_lambertian(0.1, 0.2, 0.5);
    let lambertian_yellow = Material::new_lambertian(0.8, 0.8, 0.0);
    let metallic = Material::new_metallic(0.8, 0.6, 0.2, 1.0);
    let dielectric = Material::new_dielectric(1.5);

    let sphere_zero = sphere::Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, lambertian_blue);
    let sphere_one = sphere::Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, metallic);
    let sphere_two = sphere::Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, dielectric);
    let big_sphere = sphere::Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, lambertian_yellow);

    let mut world = hittable_list::HittableList::new();
    world.insert(Box::new(sphere_zero));
    world.insert(Box::new(sphere_one));
    world.insert(Box::new(sphere_two));
    world.insert(Box::new(big_sphere));

    world
}
