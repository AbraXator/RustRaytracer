use crate::constants::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn interval() -> Interval {
        Self { min: INFINITY, max: -INFINITY }
    }

    pub fn new(min: f64, max: f64) -> Interval {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, point: f64) -> bool {
        self.min <= point && point <= self.max
    }

    pub fn surrounds(&self, point: f64) -> bool {
        self.min < point && point < self.max
    }

    pub fn clamp (&self, x: f64) -> f64 {
        if x < self.min { return self.min }
        if x > self.max { return self.min }
        x
    }
}