#![cfg_attr(not(test), no_std)]

// Copyright (c) 2024 Nikolai Serafimovich

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without limitation of the rights
// to use, copy, modify, merge, publish, distribute and/or sublicense
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// MPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[cfg(test)]
mod tests;

use core::ops::{Add, Sub, Mul, Div};
use core::cmp::PartialEq;

use libm;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3 {
    pub x: f64,    
    pub y: f64,
    pub z: f64
}

#[allow(unused)]
impl Vector3 {
    #[no_mangle]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x,
            y,
            z
        }
    }

    #[no_mangle]
    pub fn new_zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Return the vector from the memory representation in **big-endian** byte order. Order -> **x**, **y**, **z**
    pub fn from_be_bytes(bytes: [u8; 24]) -> Vector3 {
        Vector3 {
            x: f64::from_be_bytes(bytes[..8].try_into().unwrap()),
            y: f64::from_be_bytes(bytes[8..16].try_into().unwrap()),
            z: f64::from_be_bytes(bytes[16..].try_into().unwrap())
        } 
    }

    /// Return the memory representation of this vector as a byte array in **big-endian** byte order. Order -> **x**, **y**, **z**
    pub fn to_be_bytes(&self) -> [u8; 24] {
        let mut result: [u8; 24] = [0; 24];

        result[..=7].clone_from_slice(&self.x.to_be_bytes());
        result[8..=15].clone_from_slice(&self.y.to_be_bytes());
        result[16..].clone_from_slice(&self.z.to_be_bytes());

        result
    }

    /// Get vector's length
    pub fn magnitude(&self) -> f64 {
        libm::sqrt(*self**self)
    }
    /// Same as `magnitude()`, but **not** sqrted
    pub fn sqr_magnitude(&self) -> f64 {
        *self**self
    }
    /// Normalize vector or set it's length to `1`, but keep the same direction
    pub fn normalize(&self) -> Vector3 {
        (1.0 / libm::sqrt(self.x * self.x + self.y * self.y + self.z + self.z)) * *self
    }
    /// Raises each axis of the vector to a floating point power
    pub fn powf(&self, power: f64) -> Vector3 {
        Vector3 { x: libm::pow(self.x, power), y: libm::pow(self.y, power), z: libm::pow(self.z, power) }
    }

    /// Get angle between two vectors in **degrees(!!!)**
    /// 
    /// **LEGACY**
    pub fn angle(&self, v2: &Vector3) -> f64 {
        let dot: f64 = *self**v2;
        let magnitudes: (f64, f64) = (self.magnitude(), v2.magnitude());

        libm::acos(dot / (magnitudes.0 * magnitudes.1)).to_degrees()
    }

    /// Get angle between two vectors in **degrees**
    pub fn angle_degrees(&self, rhs: &Vector3) -> f64 {
        let dot: f64 = *self**rhs;
        let magnitudes: (f64, f64) = (self.magnitude(), rhs.magnitude());

        libm::acos(dot / (magnitudes.0 * magnitudes.1)).to_degrees()
    }

    /// Get angle between two vectors in **radians**
    pub fn angle_radians(&self, rhs: &Vector3) -> f64 {
        let dot: f64 = *self**rhs;
        let magnitudes: (f64, f64) = (self.magnitude(), rhs.magnitude());

        libm::acos(dot / (magnitudes.0 * magnitudes.1))
    }

    /// Project on (or onto) vector 
    pub fn project_on(&self, b: &Vector3) -> Vector3 {
        *b*((*self * *b) / (*b * *b))
    }

    /// Get vector between projected and projectee vectors 
    pub fn reject_from(&self, b: &Vector3) -> Vector3 {
        *self - self.project_on(b)
    }

    /// Get cross product of the two vectors
    pub fn cross(&self, b: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x
        )
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
    /// The entrywise product of A and B
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
