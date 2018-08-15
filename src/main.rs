extern crate rtracer;

use std::f64::MAX;
use rtracer::hittable::{HitRecord, Hittable, HittableList};
use rtracer::ray::Ray;
use rtracer::sphere::Sphere;
use rtracer::vec3::Vec3;

fn color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, MAX, &mut rec) {
        return Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0).scale_up(0.5);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0).scale_up(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scale_up(t)
    }
}

fn main() {
    let num_x = 200;
    let num_y = 100;
    println!("P3");
    println!("{} {}", num_x, num_y);
    println!("255");
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let first_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let second_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let spheres = vec![&first_sphere as &dyn Hittable, &second_sphere as &dyn Hittable];
    let world = HittableList::new(spheres);
    for j in (0..num_y).rev() {
        for i in 0..num_x {
            let u = f64::from(i) / f64::from(num_x);
            let v = f64::from(j) / f64::from(num_y);
            let r = Ray::new(origin,
                             lower_left_corner + horizontal.scale_up(u) + vertical.scale_up(v));
            let col = color(&r, &world);
            let int_r = (255.99 * col.r()).trunc() as i32;
            let int_g = (255.99 * col.g()).trunc() as i32;
            let int_b = (255.99 * col.b()).trunc() as i32;
            println!("{} {} {}", int_r, int_g, int_b)
        }
    }
}
