use std::f32;

use Vec3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
    pub t_max: f32,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray {
            o,
            d,
            t_max: f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.o + t * self.d
    }
}
