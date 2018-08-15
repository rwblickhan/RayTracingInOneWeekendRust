extern crate rtracer;

use rtracer::vec3::Vec3;
use rtracer::ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: &Ray) -> Vec3 {
    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(sphere_center, 0.5, r);
    if t > 0.0 {
        let n_hat = Vec3::unit_vector(r.point_at_parameter(t) - sphere_center);
        return Vec3::new(n_hat.x() + 1.0, n_hat.y() + 1.0, n_hat.z() + 1.0).scale_up(0.5);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0).scale_up(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scale_up(t)
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
    for j in (0..num_y).rev() {
        for i in 0..num_x {
            let u = f64::from(i) / f64::from(num_x);
            let v = f64::from(j) / f64::from(num_y);
            let r = Ray::new(origin,
                             lower_left_corner + horizontal.scale_up(u) + vertical.scale_up(v));
            let col = color(&r);
            let int_r = (255.99 * col.r()).trunc() as i32;
            let int_g = (255.99 * col.g()).trunc() as i32;
            let int_b = (255.99 * col.b()).trunc() as i32;
            println!("{} {} {}", int_r, int_g, int_b)
        }
    }
}
