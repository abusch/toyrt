use std::f32;

use crate::cg::prelude::*;

use crate::{Matrix4f, Point3f, Vec3f};

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vec3f,
    pub t_max: f32,
}

impl Ray {
    pub fn new(o: Point3f, d: Vec3f) -> Self {
        Ray {
            o,
            d,
            t_max: f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Point3f {
        self.o + t * self.d
    }

    pub fn transform(&self, t: Matrix4f) -> Self {
        Ray {
            o: t.transform_point(self.o),
            d: t.transform_vector(self.d),
            t_max: self.t_max,
        }
    }
}
