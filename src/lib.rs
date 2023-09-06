use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,    
    pub y: f64,
    pub z: f64
}

#[allow(unused)]
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x,
            y,
            z
        }
    }

    pub fn new_zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Return the memory representation of this vector as a byte array in **big-endian** byte order. Order -> `x, y, z`
    pub fn to_be_bytes(&self) -> [u8; 24] {
        let mut result: [u8; 24] = [0; 24];

        result[0..7].clone_from_slice(&self.x.to_be_bytes());
        result[8..15].clone_from_slice(&self.y.to_be_bytes());
        result[16..24].clone_from_slice(&self.z.to_be_bytes());

        result
    }

    /// Get vector's length
    pub fn magnitude(&self) -> f64 {
        (*self**self).sqrt()
    }
    /// Same as `magnitude()`, but **not** sqrted
    pub fn sqr_magnitude(&self) -> f64 {
        *self**self
    }
    /// Normalize vector or set it's length to `1`, but keep the same direction
    pub fn normalize(&self) -> Vector3 {
        (1.0 / (self.x * self.x + self.y * self.y + self.z + self.z).sqrt()) * *self
    }
    /// Raises each axis of the vector to a floating point power
    pub fn powf(&self, power: f64) -> Vector3 {
        Vector3 { x: self.x.powf(power), y: self.y.powf(power), z: self.z.powf(power) }
    }

    /// Get angle between two vectors
    pub fn angle(&self, v2: &Vector3) -> f64 {
        let dot: f64 = *self**v2;
        let magnitudes: (f64, f64) = (self.magnitude(), v2.magnitude());

        (dot / (magnitudes.0 * magnitudes.1)).acos().to_degrees()
    }
    /// Project on vector
    pub fn project_on(&self, b: &Vector3) -> Vector3 {
        *b*((*self * *b) / (*b * *b))
    }

    /// Get vector between projected and projectee vectors 
    pub fn reject_from(&self, b: &Vector3) -> Vector3 {
        *self - self.project_on(b)
    }
}


impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 { x: rhs.x * self, y: rhs.y * self, z: rhs.z * self }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 { x: rhs * self.x, y: rhs * self.y, z: rhs * self.z }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, b: f64) -> Vector3 {
        self * (1.0 / b)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = f64;
    /// Get dot product of two vectors
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
/// Subtract two vectors
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
    fn project() {
        let vector_a = Vector3 {
            x: 4.0,
            y: 4.0,
            z: 0.0
        };
        let vector_b = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 0.0
        };

        assert_eq!(vector_a.project_on(&vector_b), Vector3 { x: 2.4, y: 4.8, z: 0.0});
    }

    #[test]
    fn scalar_multiply() {
        let vec = Vector3 {
            x: 1.5, y: -4.3, z: 2.7
        };

        let val: f64 = 5.0;

        assert_eq!(Vector3 {x: 1.5 * val, y: -4.3 * val, z: 2.7 * val}, vec * val);
    }

    #[test]
    fn vector_normalization() {
        let v = Vector3 {x: 10.0, y: 5.0, z: 0.0};

        assert_eq!(Vector3 {x: 0.8944271909999159, y: 0.4472135954999579, z: 0.0}, v.normalize());
        assert_eq!(1.0, v.normalize().magnitude().round());
    }
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
