extern crate rand;
extern crate rtracer;

use std::f64::MAX;
use rand::prelude::*;
use rtracer::camera::Camera;
use rtracer::hittable::{HitRecord, Hittable, HittableList};
use rtracer::material::Material;
use rtracer::ray::Ray;
use rtracer::sphere::Sphere;
use rtracer::vec3::Vec3;

fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, MAX, &mut rec) {
        if depth >= 50 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if let None = rec.mat {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let scatter_details = rec.mat.unwrap().scatter(r, &rec);
        if !scatter_details.produced_scatter {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        scatter_details.attenuation * color(&scatter_details.scattered, world, depth + 1)
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0).scale_up(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scale_up(t)
    }
}

fn main() {
    let num_x = 200;
    let num_y = 100;
    let num_samples = 100;
    println!("P3");
    println!("{} {}", num_x, num_y);
    println!("255");
    let first_matte_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0),
                                         0.5,
                                         Material::Lambertian { albedo: Vec3::new(0.1, 0.2, 0.5) });
    let second_matte_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0),
                                          100.0,
                                          Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) });
    let metal_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0),
                                   0.5,
                                   Material::new_metal(Vec3::new(0.8, 0.6, 0.2), 1.0));
    let glass_sphere_outer = Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                                         0.5,
                                         Material::Dielectric { ref_idx: 1.5 });
    let glass_sphere_inner = Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                                         -0.45,
                                         Material::Dielectric { ref_idx: 1.5 });
    let spheres = vec![
        &first_matte_sphere as &dyn Hittable,
        &second_matte_sphere as &dyn Hittable,
        &metal_sphere as &dyn Hittable,
        &glass_sphere_outer as &dyn Hittable,
        &glass_sphere_inner as &dyn Hittable];
    let world = HittableList::new(spheres);
    let cam = Camera::default();
    for j in (0..num_y).rev() {
        for i in 0..num_x {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..num_samples {
                let u = (f64::from(i) + random::<f64>()) / f64::from(num_x);
                let v = (f64::from(j) + random::<f64>()) / f64::from(num_y);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col.scale_down_assign(f64::from(num_samples));
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let int_r = (255.99 * col.r()).trunc() as i32;
            let int_g = (255.99 * col.g()).trunc() as i32;
            let int_b = (255.99 * col.b()).trunc() as i32;
            println!("{} {} {}", int_r, int_g, int_b)
        }
    }
}
