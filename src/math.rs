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
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Self::new(self.x + other.x , self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Self::new(self.x - other.x , self.y - other.y, self.z - other.z)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn add_vec3() {
        let vec1 = Vec3::new(-1.0, 2.0, -3.0);
        let vec2 = Vec3::new(1.0, -5.0, 4.0);

        let vec_result = Vec3::new(0.0, -3.0, 1.0);


        assert_eq!(vec1 + vec2, vec_result);
    }

    #[test]
    fn sub_vec3() {
        let vec1 = Vec3::new(-1.0, 2.0, -3.0);
        let vec2 = Vec3::new(1.0, -5.0, 4.0);

        let vec_result = Vec3::new(-2.0, 7.0, -7.0);


        assert_eq!(vec1 - vec2, vec_result);
    }

}
