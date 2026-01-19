use std::rc::Rc;

mod core;
mod object;
mod interval;
mod renderer;

fn main() {
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -110.0, -1.0), 100.0)));

    let mut camera = Camera::new();
    Camera::render(&mut camera, &world)
}
