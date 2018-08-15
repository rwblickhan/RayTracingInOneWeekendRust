use ray::Ray;
use vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3,
               lower_left_corner: Vec3,
               horizontal: Vec3,
               vertical: Vec3) -> Camera {
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn lower_left_corner(&self) -> Vec3 {
        self.lower_left_corner
    }

    pub fn horizontal(&self) -> Vec3 {
        self.horizontal
    }

    pub fn vertical(&self) -> Vec3 {
        self.vertical
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner
                     + self.horizontal.scale_up(u)
                     + self.vertical.scale_up(v)
                     - self.origin)
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(-2.0, -1.0, -1.0),
                    Vec3::new(4.0, 0.0, 0.0),
                    Vec3::new(0.0, 2.0, 0.0))
    }
}