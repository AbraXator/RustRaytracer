use std::rc::Rc;
use crate::hittable::{HittableList, Sphere};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::camera::*;

mod vec3;
mod ray;
mod hittable;
mod constants;
mod interval;
mod camera;

fn main() {
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -110.0, -1.0), 100.0)));

    let mut camera = Camera::new();
    Camera::render(&mut camera, &world)
}
