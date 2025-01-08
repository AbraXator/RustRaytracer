use crate::ray;
use crate::ray::{Ray, Sphere};
use crate::vec3::Vec3;

type Color = Vec3;

pub fn color_for_ray(ray: Ray) -> Vec3 {
    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5
    };
    let hit_sphere = ray.hit_sphere(&sphere);
    println!("{}", hit_sphere);
    if hit_sphere {
        return Color::new(1.0, 0.0, 0.0)
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    Color::new(a, a, a)
}

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    // Write out the pixel color components.
    println!("{} {} {}", rbyte, gbyte, bbyte);
}