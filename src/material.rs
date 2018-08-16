extern crate rand;

use hittable::HitRecord;
use ray::Ray;
use material::rand::prelude::*;
use vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ref_idx: f64 },
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
            Material::Dielectric { ref ref_idx } => {
                let reflected = reflect(r_in.direction(), rec.normal);
                let attenuation = Vec3::new(1.0, 1.0, 0.0);
                let outward_normal = if Vec3::dot(r_in.direction(), rec.normal) > 0.0 {
                    rec.normal.scale_up(-1.0)
                } else {
                    rec.normal
                };
                let ni_over_nt = if Vec3::dot(r_in.direction(), rec.normal) > 0.0 {
                    *ref_idx
                } else {
                    1.0 / *ref_idx
                };
                let cos = if Vec3::dot(r_in.direction(), rec.normal) > 0.0 {
                    ref_idx * Vec3::dot(r_in.direction(), rec.normal) / r_in.direction().length()
                } else {
                    -Vec3::dot(r_in.direction(), rec.normal) / r_in.direction().length()
                };

                let (reflect_prob, refracted) = match refract(r_in.direction(), outward_normal, ni_over_nt) {
                    None => (1.0, None),
                    Some(refracted) => (schlick(cos, *ref_idx), Some(refracted))
                };

                if random::<f64>() < reflect_prob {
                    ScatterDetails {
                        attenuation,
                        scattered: Ray::new(rec.p, reflected),
                        produced_scatter: true,
                    }
                } else {
                    ScatterDetails {
                        attenuation,
                        // ugly but safe
                        scattered: Ray::new(rec.p, refracted.unwrap()),
                        produced_scatter: true,
                    }
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

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some((uv - n.scale_up(dt)).scale_up(ni_over_nt) - n.scale_up(discriminant.sqrt()));
    } else {
        return None;
    }
}

fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cos, 5)
}