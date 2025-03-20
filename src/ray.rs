use crate::vec3::Vec3;

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
        self.origin + self.direction * t
    }
}
