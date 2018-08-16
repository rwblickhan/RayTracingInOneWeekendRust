use hittable::{HitRecord, Hittable};
use material::Material;
use ray::Ray;
use vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Material) -> Sphere {
        Sphere { center, radius, mat }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere::new(Vec3::default(), 0.0, Material::Lambertian { albedo: Vec3::new(1.0, 1.0, 1.0) })
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(oc, r.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center).scale_down(self.radius);
                rec.mat = Some(self.mat);
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center).scale_down(self.radius);
                rec.mat = Some(self.mat);
                return true;
            }
        }
        false
    }
}
