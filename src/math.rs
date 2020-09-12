use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        &self + &other
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        &self - &other
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        &self * &other
    }
}

impl ops::Mul<&f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &f64) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        &self / &other
    }
}

impl ops::Div<&f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: &f64) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use test;

    fn rand_f64() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(-1_000_000_000.0, 1_000_000_000.0)
    }

    fn rand_vec3() -> Vec3 {
        Vec3::new(rand_f64(), rand_f64(), rand_f64())
    }

    #[test]
    fn add_vec3() {
        let vec1 = rand_vec3();
        let vec2 = rand_vec3();
        let vec_result = Vec3::new(vec1.x + vec2.x, vec1.y + vec2.y, vec1.z + vec2.z);
        assert_eq!(vec1 + vec2, vec_result);
    }

    #[test]
    fn add_assign_vec3() {
        let mut vec1 = rand_vec3();
        let vec2 = rand_vec3();
        let vec_result = Vec3::new(vec1.x + vec2.x, vec1.y + vec2.y, vec1.z + vec2.z);
        vec1 += vec2;
        assert_eq!(vec1, vec_result);
    }

    #[test]
    fn sub_vec3() {
        let vec1 = rand_vec3();
        let vec2 = rand_vec3();
        let vec_result = Vec3::new(vec1.x - vec2.x, vec1.y - vec2.y, vec1.z - vec2.z);
        assert_eq!(vec1 - vec2, vec_result);
    }

    #[test]
    fn neg_vec3() {
        let vec1 = rand_vec3();
        let vec_result = Vec3::new(-vec1.x, -vec1.y, -vec1.z);
        assert_eq!(-vec1, vec_result);
    }

    #[test]
    fn sub_assign_vec3() {
        let mut vec1 = rand_vec3();
        let vec2 = rand_vec3();
        let vec_result = Vec3::new(vec1.x - vec2.x, vec1.y - vec2.y, vec1.z - vec2.z);
        vec1 -= vec2;
        assert_eq!(vec1, vec_result);
    }

    #[test]
    fn mul_vec3() {
        let vec1 = rand_vec3();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(-1_000_000_000.0, 1_000_000_000.0);
        let vec_result = Vec3::new(vec1.x * n, vec1.y * n, vec1.z * n);

        assert_eq!(vec1 * n, vec_result);
    }

    #[test]
    fn div_vec3() {
        let vec1 = rand_vec3();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(-1_000_000_000.0, 1_000_000_000.0);
        let vec_result = Vec3::new(vec1.x / n, vec1.y / n, vec1.z / n);

        assert_eq!(vec1 / n, vec_result);
    }

    #[test]
    fn length_vec3() {
        let vec1 = rand_vec3();

        let result = (vec1.x.powi(2) + vec1.y.powi(2) + vec1.z.powi(2)).sqrt();

        assert_eq!(vec1.length(), result);
    }
}
