use cg::prelude::*;

use Vec3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn zero() -> Ray {
        Ray {
            o: Vec3::zero(),
            d: Vec3::zero(),
        }
    }

    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.o + t * self.d
    }
}
