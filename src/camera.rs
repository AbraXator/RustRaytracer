use std::ops::Add;
use crate::constants::INFINITY;
use crate::hittable::{Hit, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 480;
const FOCAL_LENGTH: f64 = 1.0;
const SAMPLES_PER_PIXEL: i32 = 100;

pub struct Camera {
    image_height: i32,
    pixel_sample_scale: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Add<Vec3> for f64 {
    type Output = Vec3;


    fn add(self, rhs: Vec3) -> Self::Output {
        self + rhs
    }
}

impl Camera {
    pub fn new () -> Camera {
        Self {
            image_height: 0,
            pixel_sample_scale: 0.0,
            center: Vec3::zero(),
            pixel00_loc: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }

    pub fn render(camera: &mut Camera, world: &dyn Hit) {
        Camera::initialize(camera);

        println!("P3\n{} {}\n255", IMAGE_WIDTH, camera.image_height);

        for j in 0..camera.image_height {
            for i in 0..IMAGE_WIDTH {
                let color = Vec3::zero();
                for i in 0..SAMPLES_PER_PIXEL {
                    let mut ray = Self::get_ray(&camera, i, j);
                    ray.set_color(ray.color + ray_color(&ray, world));
                }
                let pixel_center = camera.pixel00_loc + (camera.pixel_delta_u * i as f64) + (camera.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - camera.center;
                let mut ray = Ray::new(camera.center, ray_direction);
                ray.set_color(ray_color(&ray, world));

                write_color(&ray.color);
            }
        }
    }

    pub fn initialize(camera: &mut Camera) {
        camera.image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
        camera.image_height = if camera.image_height < 1 { 1 } else { camera.image_height };

        camera.pixel_sample_scale = 1.0 / (SAMPLES_PER_PIXEL as f64);

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (IMAGE_WIDTH as f64 / camera.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        camera.center = Vec3::new(0.0, 0.0, 0.0);
        camera.pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
        camera.pixel_delta_v = viewport_v / camera.image_height as f64;

        let viewport_upper_left = camera.center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2f64 - viewport_v / 2f64;
        camera.pixel00_loc = viewport_upper_left + (camera.pixel_delta_u + camera.pixel_delta_v) * 0.5;
    }

    pub fn get_ray(camera: &Camera, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = camera.pixel00_loc
                                + (camera.pixel_delta_u * (i as f64 + offset.x()))
                                + (camera.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin = camera.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn sample_square() -> Vec3 {
        return Vec3::new(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.0);
    }
}



fn ray_color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    let mut record = HitRecord::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);
    if world.hit(ray, Interval::new(0.0, INFINITY), &mut record) {
        return (record.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

pub fn write_color(pixel_color: &Vec3) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0.0, 0.9999999);
    let rbyte = (255.999 * intensity.clamp(r)) as i32;
    let gbyte = (255.999 * intensity.clamp(g)) as i32;
    let bbyte = (255.999 * intensity.clamp(b)) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}