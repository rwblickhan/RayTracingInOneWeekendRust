use vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b.scale_up(t)
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray::new(Vec3::default(), Vec3::default())
    }
}