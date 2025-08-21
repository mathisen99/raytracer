// Import necessary modules and types.
use std::ops::{Add, Mul, Neg, Sub};


// Define a 3D vector struct with components x, y, and z.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    // Create a new Vec3 with given x, y, and z components.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    // Create a zero vector with all components set to 0.0.
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    // Calculate the square of the vector's Euclidean norm.
    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Calculate the length (Euclidean norm) of the vector.
    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    // Calculate the dot product between two vectors.
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Calculate the cross product between two vectors.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Normalize the vector to have a unit length.
    pub fn normalize(&self) -> Vec3 {
        let inv_len = self.length().recip();
        Vec3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }

    // Reflect the vector with respect to a given normal vector.
    pub fn reflect(&self, other: Vec3) -> Vec3 {
        *self - other * other.dot(*self) * 2.0
    }

    // Correct the vector by nudging it slightly in a given direction.
    pub fn correct(&self, other: Vec3) -> Vec3 {
        *self + (other * crate::EPSILON)
    }

    pub fn abs(&self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub(crate) fn splat(value: f64) -> Vec3 {
        Vec3::new(value, value, value)
    }
}

impl Add for Vec3 {
    type Output = Self;

    // Define vector addition.
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    // Define vector subtraction.
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    // Define scalar multiplication.
    fn mul(self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    // Define vector component-wise multiplication.
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    // Define negation (change direction) of a vector.
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
