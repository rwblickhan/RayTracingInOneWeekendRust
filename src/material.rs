extern crate rand;

use hittable::HitRecord;
use ray::Ray;
use material::rand::prelude::*;
use vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
}

impl Material {
    pub fn new_metal(albedo: Vec3, fuzz: f64) -> Material {
        Material::Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

pub struct ScatterDetails {
    pub attenuation: Vec3,
    pub scattered: Ray,
    pub produced_scatter: bool,
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterDetails {
        match self {
            Material::Lambertian { ref albedo } => {
                let target = rec.p + rec.normal + random_in_unit_sphere();
                ScatterDetails {
                    attenuation: albedo.clone(),
                    scattered: Ray::new(rec.p, target - rec.p),
                    produced_scatter: true,
                }
            }
            Material::Metal { ref albedo, ref fuzz } => {
                let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
                let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere().scale_up(*fuzz));
                ScatterDetails {
                    attenuation: albedo.clone(),
                    scattered,
                    produced_scatter: Vec3::dot(scattered.direction(), rec.normal) > 0.0,
                }
            }
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(random::<f64>(), random::<f64>(), random::<f64>()).scale_up(2.0)
        - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = Vec3::new(random::<f64>(), random::<f64>(), random::<f64>()).scale_up(2.0)
            - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n.scale_up(2.0 * Vec3::dot(v, n))
}