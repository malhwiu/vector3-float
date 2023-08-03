use std::ops::{Add, Sub};
use std::cmp::{PartialEq};

#[derive(Debug)]
pub struct Vector3 {
    x: f64,    y: f64,
    z: f64
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
}
