use std::ops::Mul;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub color: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
            color: Vec3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }

    pub fn at(self, t: f64) -> Vec3 {
        let dir = Vec3::new(
            self.direction.x() * t,
            self.direction.y() * t,
            self.direction.z() * t
        );

        self.origin + dir
    }

    pub fn hit_sphere(&self, sphere: &Sphere ) -> bool {
        let point = sphere.center - self.origin;
        let a = self.direction.dot(self.direction);
        let b = -2.0 * self.direction.dot(point);
        let c = point.dot(point) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}
