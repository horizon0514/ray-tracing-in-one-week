use crate::material::Material;
use crate::{vector3::Point3, ray3::Ray3};
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere<M: Material> {
    pub center1: Point3,
    pub center2: Point3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center1: Point3, center2: Point3, radius: f32, material: M) -> Sphere<M> {
        Sphere { center1, center2, radius, material }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center1 + (self.center2 - self.center1) * time
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray3, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut t = (-half_b - discriminant.sqrt()) / a;
        if t < t_min || t > t_max {
            t = (-half_b + discriminant.sqrt()) / a;
        }

        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        // To compatible the negative radius, normal must be / self.radius, DO NOT use normal.unit_vector()
        let mut normal = (point - self.center(ray.time)) / self.radius;
        let is_front_face = ray.direction.dot(normal) < 0.0;

        if !is_front_face {
            normal = normal * (-1.0);
        }

        Some(HitRecord {
            t,
            point,
            normal,
            is_front_face,
            material: &self.material
        })
    }
}