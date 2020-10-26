use super::TAU;
use rand::Rng;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn new_random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(min, max);
        let y: f64 = rng.gen_range(min, max);
        let z: f64 = rng.gen_range(min, max);

        Vec3 { x, y, z }
    }

    pub fn new_random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::new_random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn new_random_unit() -> Self {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, TAU);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r: f64 = (1.0 - z.powi(2)).sqrt();

        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn unit(vec1: Vec3) -> Vec3 {
        vec1 / vec1.length()
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - &(2.0 * Vec3::dot(v, n) * n)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-uv, n);
        let r_out_perp =  etai_over_etat * (uv + &(&cos_theta * n));
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        return r_out_perp + r_out_parallel;
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
        -&self
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
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

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Mul<&Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        &self * &other
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
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

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        &self / &other
    }
}

impl ops::Div<&Vec3> for &f64 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Vec3 {
        Vec3::new(self / other.x, self / other.y, self / other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let vec1 = rand_vec3();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(-1_000_000_000.0, 1_000_000_000.0);
        let vec_result = Vec3::new(vec1.x * n, vec1.y * n, vec1.z * n);
        assert_eq!(n * vec1, vec_result);

        let vec1 = rand_vec3();
        let vec2 = rand_vec3();
        let vec_result = Vec3::new(vec1.x * vec2.x, vec1.y * vec2.y, vec1.z * vec2.z);
        assert_eq!(vec1 * vec2, vec_result);
    }

    #[test]
    fn div_vec3() {
        let vec1 = rand_vec3();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(-1_000.0, 1_000.0);
        let vec_result = Vec3::new(vec1.x / n, vec1.y / n, vec1.z / n);
        assert_eq!(vec1 / n, vec_result);

        let vec1 = rand_vec3();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(-1_000.0, 1_000.0);
        let vec_result = Vec3::new(n / vec1.x, n / vec1.y, n / vec1.z);
        assert_eq!(n / vec1, vec_result);
    }

    #[test]
    fn length_vec3() {
        let vec1 = rand_vec3();

        let result = (vec1.x.powi(2) + vec1.y.powi(2) + vec1.z.powi(2)).sqrt();

        assert_eq!(vec1.length(), result);
    }

    #[test]
    fn dot_vec3() {
        let vec1 = rand_vec3();
        let vec2 = rand_vec3();
        let result = vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
        assert_eq!(Vec3::dot(&vec1, &vec2), result);
    }

    #[test]
    fn cross_vec3() {
        let vec1 = rand_vec3();
        let vec2 = rand_vec3();
        let result = Vec3::new(
            vec1.y * vec2.z - vec1.z * vec2.y,
            vec1.z * vec2.x - vec1.x * vec2.z,
            vec1.x * vec2.y - vec1.y * vec2.x,
        );
        assert_eq!(Vec3::cross(&vec1, &vec2), result);
    }

    #[test]
    fn unit_vec3() {
        let mut rng = rand::thread_rng();

        let vec1 = rand_vec3();
        let result = &vec1 / &vec1.length();
        assert_eq!(Vec3::unit(vec1), result);

        let n = rng.gen_range(0.0, 1_000.0);
        let vec1 = Vec3::new(n, 0.0, 0.0);
        let result = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(Vec3::unit(vec1), result);

        let n = rng.gen_range(0.0, 1_000.0);
        let vec1 = Vec3::new(0.0, n, 0.0);
        let result = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Vec3::unit(vec1), result);

        let n = rng.gen_range(0.0, 1_000.0);
        let vec1 = Vec3::new(0.0, 0.0, n);
        let result = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::unit(vec1), result);
    }

    #[test]
    fn random_in_unit_sphere() {
        let rand_vec = Vec3::new_random_in_unit_sphere();

        assert!(rand_vec.length_squared() < 1.0);
    }

    #[test]
    fn random_unit() {
        let rand_vec = Vec3::new_random_unit();

        assert!(rand_vec.length_squared() > 0.9999 && rand_vec.length_squared() < 1.0001);
    }
}
