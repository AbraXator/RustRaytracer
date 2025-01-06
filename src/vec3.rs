use std::io::Bytes;
use std::ops::{Add, Div, Mul, Neg, Sub};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(
            rng.random_range(min..max),
            rng.random_range(min..max),
            rng.random_range(min..max),
        )
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z * self.z
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn unit_vector(self) -> Vec3 {
        Vec3::new(
            self.x / self.len(),
            self.y / self.len(),
            self.z / self.len()
        )
    }
}

type Point3 = Vec3;


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x() + other.x(),
            z: self.z() + other.z(),
            y: self.y() + other.z()
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x() - other.x(),
            z: self.z() - other.z(),
            y: self.y() - other.z()
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3 {
            x: self.x() / other,
            z: self.z() / other,
            y: self.y() / other
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x() / other.x(),
            z: self.z() / other.z(),
            y: self.y() / other.z()
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x() * other.x(),
            z: self.z() * other.z(),
            y: self.y() * other.z()
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3 {
            x: self.x() * other,
            z: self.z() * other,
            y: self.y() * other
        }
    }
}