use std::ops::Mul;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64
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

    pub fn set_color(mut self, color: Vec3) {
        self.color = color;
    }

    pub fn at(t: f64, ray: &Ray) -> Vec3 {
        let dir = Vec3::new(
            ray.direction.x() * t,
            ray.direction.y() * t,
            ray.direction.z() * t
        );

        ray.origin + dir
    }

    pub fn hit_sphere(sphere: &Sphere, ray: Ray ) -> bool {
        let point = sphere.center - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = -2f64 * ray.direction.dot(point);
        let c = point.dot(point) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4f64 * a * c;

        discriminant >= 0f64
    }
}
