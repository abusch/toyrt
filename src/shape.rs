use std::sync::Arc;

use cg::prelude::*;

use Vec3;
use material::Material;
use ray::Ray;

#[derive(Clone)]
pub struct Hit {
    pub p: Vec3,
    pub n: Vec3,
    pub mat: Arc<Material>,
}

pub trait Shape {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit>;
}

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Arc<Material>) -> Sphere {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit> {
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
            if t >= 0.0 && t <= ray.t_max {
                let p = ray.at(t);
                let n = (&p - self.centre).normalize();
                ray.t_max = t;

                Some(Hit {
                    p,
                    n,
                    mat: self.material.clone(),
                })
            } else {
                None
            }
        }
    }
}

pub struct Aggregation {
    pub shapes: Vec<Box<Shape>>,
}

impl Shape for Aggregation {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit> {
        self.shapes
            .iter()
            .fold(None, |prev_hit, shape| shape.intersect(ray).or(prev_hit))
    }
}
