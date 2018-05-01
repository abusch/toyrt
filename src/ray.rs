use std::f32;

use {Point3f, Vec3f};

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vec3f,
    pub t_max: f32,
}

impl Ray {
    pub fn new(o: Point3f, d: Vec3f) -> Ray {
        Ray {
            o,
            d,
            t_max: f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Point3f {
        self.o + t * self.d
    }
}
