use ray::Ray;
use vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl<'a> Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord { t: 0.0, p: Vec3::default(), normal: Vec3::default() }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList<'a> {
    list: Vec<&'a dyn Hittable>
}

impl<'a> HittableList<'a> {
    pub fn new(list: Vec<&'a dyn Hittable>) -> HittableList<'a> {
        HittableList { list }
    }

    pub fn list(&mut self) -> &mut Vec<&'a dyn Hittable> {
        &mut self.list
    }
}

impl<'a> Default for HittableList<'a> {
    fn default() -> HittableList<'a> {
        HittableList { list: Vec::new() }
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list.len() {
            if self.list[i].hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}