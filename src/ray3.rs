use crate::vector3::{Point3, Vector3};

pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vector3, time: f32) -> Ray3 {
        Ray3 {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}
