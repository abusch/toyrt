use std::f32;

use {Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Point3,
    pub d: Vec3,
    pub t_max: f32,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3) -> Ray {
        Ray {
            o,
            d,
            t_max: f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.o + t * self.d
    }
}
