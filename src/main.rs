use crate::color::color_for_ray;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod vec3;
mod color;
mod ray;
mod mth;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width = 256;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    const FOCAL_LENGTH: f64 = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width/image_height) as f64;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2f64 - viewport_v / 2f64;
    let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00 + (pixel_delta_u * j as f64) + (pixel_delta_v * i as f64);
            let ray_direction = pixel_center - camera_center;
            let mut ray = Ray::new(camera_center, ray_direction);
            let ray_color = color_for_ray(ray);
            ray.set_color(ray_color);

            color::write_color(&ray.color);
        }
    }
}
