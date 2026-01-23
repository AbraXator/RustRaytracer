use std::io::stdout;
use std::ptr::null;
use std::rc::Rc;
use crate::interval::Interval;
use crate::core::Vec3;
use crate::renderer::ray::HitRecord;

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        Self { point, normal, t, front_face }
    }

    //outward_normal is supposed to be a normal vector
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0f32;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.len_squared();
        let b = ray.direction.dot(&oc);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (b - sqrtd) / a;
        if root <= interval.min || interval.max <= root {
            root = (b + sqrtd) / a;
            if root <= interval.min || interval.max <= root {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(root);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        true
    }
}

pub struct HittableList<> {
    pub objects: Vec<Rc<dyn Hit>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, h: Rc<dyn Hit>) {
        self.objects.push(h);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), interval.max, false);
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if object.as_ref().hit(ray, Interval::new(interval.min, closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record;
            }
        }

        hit_anything
    }
}
