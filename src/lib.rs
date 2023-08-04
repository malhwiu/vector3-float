use std::ops::{Add, Sub, Mul};
use std::cmp::{PartialEq};
use std::env;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    x: f64,    y: f64,
    z: f64
}

impl Vector3 {
    fn magnitude(&self) -> f64 {
        (*self**self).sqrt()
    }

    fn sqr_magnitude(&self) -> f64 {
        *self**self
    }

    fn angle(&self, v2: &Vector3) -> f64 {
        let dot: f64 = *self**v2;
        let magnitudes: (f64, f64) = (self.magnitude(), v2.magnitude());

        (dot / (magnitudes.0 * magnitudes.1)).acos().to_degrees()
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = f64;

    fn mul(self, rhs: Vector3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, v2: Vector3) -> Vector3 {
        Vector3 {x: self.x + v2.x, y: self.y + v2.y, z: self.z + v2.z}
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, v2: Vector3) -> Vector3 {
        Vector3 {x: self.x - v2.x, y: self.y - v2.y, z: self.z - v2.z}
    }
}

impl PartialEq<Vector3> for Vector3 {

    fn eq(self: &Vector3, v2: &Vector3) -> bool {
        if self.x != v2.x {return false};
        if self.y != v2.y {return false};
        if self.z != v2.z {return false};

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_two_vectors() {
        let vector1 = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };
        let vector2 = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };

        assert_eq!(vector1 - vector2, Vector3 {x: 0.0, y: 0.0, z: 0.0});
    }
    #[test]
    fn add_two_vectors() {
        let vector1 = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };
        let vector2 = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };

        assert_eq!(vector1 + vector2, Vector3 {x: 1.5 * 2.0, y: -4.3 * 2.0, z: 2.7 * 2.0});
    }
    #[test]
    fn vector_magnitude() {
        let vector = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };

        assert_eq!(vector.magnitude(), 5.294336596779619);
        assert_eq!(vector.sqr_magnitude(), 28.03);
    }
    #[test]
    fn dot_vectors() {
        let vector1 = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };
        let vector2 = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };

        assert_eq!(vector1 * vector2, 28.03);
    }

    #[test]
    fn angle_between() {
        let vector_a = Vector3 {
            x: 3.0, y: -2.0, z: 0.0
        };
        let vector_b = Vector3 {
            x: 1.0, y: 7.0, z: 0.0
        };

        assert_eq!(115.55996517182382, vector_a.angle(&vector_b));
    }


}
