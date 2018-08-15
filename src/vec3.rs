use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Index, IndexMut, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn scale_up(self, factor: f64) -> Vec3 {
        Vec3::new(self.e[0] * factor, self.e[1] * factor, self.e[2] * factor)
    }

    pub fn scale_up_assign(&mut self, factor: f64) {
        self.e[0] *= factor;
        self.e[1] *= factor;
        self.e[2] *= factor;
    }

    pub fn scale_down(self, factor: f64) -> Vec3 {
        Vec3::new(self.e[0] / factor, self.e[1] / factor, self.e[2] / factor)
    }

    pub fn scale_down_assign(&mut self, factor: f64) {
        self.e[0] /= factor;
        self.e[1] /= factor;
        self.e[2] /= factor;
    }

    pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
        lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(lhs.e[1] * rhs.e[2] - lhs.e[2] * lhs.e[1],
                  -(lhs.e[0] * rhs.e[2] - lhs.e[2] * rhs.e[0]),
                  lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0])
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v.scale_down(v.length())
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[1];
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2])
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[1];
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[1];
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[1];
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
