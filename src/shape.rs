use cg::prelude::*;

use ray::Ray;
use {Hit, Vec3};

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32) -> Sphere {
        Sphere { centre, radius }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.o - self.centre;
        let a = ray.d.dot(ray.d);
        let b = 2.0 * ray.d.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discr_2 = b * b - 4.0 * a * c;

        if discr_2 < 0.0 {
            None
        } else {
            let discr = f32::sqrt(discr_2);
            let t = (-b - discr) / (2.0 * a);
            let p = ray.at(t);
            let n = (&p - self.centre).normalize();

            Some(Hit { p, n })
        }
    }
}
