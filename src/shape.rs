use std::sync::Arc;

use cg::prelude::*;

use material::Material;
use ray::Ray;
use {Point3f, Vec3f};

#[derive(Clone)]
pub struct Hit {
    pub p: Point3f,
    pub n: Vec3f,
    pub mat: Arc<Material>,
}

pub trait Shape {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit>;
}

pub struct Sphere {
    pub centre: Point3f,
    pub radius: f32,
    pub material: Arc<Material + Send + Sync>,
}

impl Sphere {
    pub fn new(centre: Point3f, radius: f32, material: Arc<Material + Send + Sync>) -> Sphere {
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
        let a = ray.d.magnitude2();
        let b = ray.d.dot(oc);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discr_2 = b * b - a * c;

        if discr_2 >= 0.0 {
            let discr = f32::sqrt(discr_2);
            let mut t = (-b - discr) / a;
            if t >= 0.0 && t <= ray.t_max {
                let p = ray.at(t);
                let n = (p - self.centre).normalize();
                ray.t_max = t;

                return Some(Hit {
                    p,
                    n,
                    mat: self.material.clone(),
                });
            }
            t = (-b + discr) / a;
            if t >= 0.0 && t <= ray.t_max {
                let p = ray.at(t);
                let n = (p - self.centre).normalize();
                ray.t_max = t;

                return Some(Hit {
                    p,
                    n,
                    mat: self.material.clone(),
                });
            }
        }
        None
    }
}

pub struct Aggregation {
    pub shapes: Vec<Box<Shape + Send + Sync>>,
}

impl Shape for Aggregation {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit> {
        self.shapes
            .iter()
            .fold(None, |prev_hit, shape| shape.intersect(ray).or(prev_hit))
    }
}
