extern crate rtracer;

use rtracer::vec3::Vec3;

fn main() {
    let num_x = 200;
    let num_y = 100;
    println!("P3");
    println!("{} {}", num_x, num_y);
    println!("255");
    for j in (0..num_y).rev() {
        for i in 0..num_x {
            let col = Vec3::new(f64::from(i) / f64::from(num_x),
                                f64::from(j) / f64::from(num_y),
                                0.2);
            let int_r = (255.99 * col.r()).trunc() as i32;
            let int_g = (255.99 * col.g()).trunc() as i32;
            let int_b = (255.99 * col.b()).trunc() as i32;
            println!("{} {} {}", int_r, int_g, int_b)
        }
    }
}
