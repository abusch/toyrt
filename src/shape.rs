use std::sync::Arc;

use cg::prelude::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::{Point3f, Vec3f};

#[derive(Clone)]
pub struct Hit {
    pub p: Point3f,
    pub n: Vec3f,
    pub mat: Arc<Material>,
}

impl Hit {
    pub fn transform(&self, t: Transform) -> Self {
        Hit {
            p: t.transform_point(self.p),
            n: t.transform_normal(self.n),
            mat: self.mat.clone(),
        }
    }
}

pub trait Shape {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit>;
}

pub struct Sphere {
    pub radius: f32,
    pub material: Arc<Material + Send + Sync>,
}

impl Sphere {
    pub fn new(radius: f32, material: Arc<Material + Send + Sync>) -> Sphere {
        Sphere { radius, material }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit> {
        let oc = ray.o.to_vec();
        let a = ray.d.magnitude2();
        let b = ray.d.dot(oc);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discr_2 = b * b - a * c;

        if discr_2 >= 0.0 {
            let discr = f32::sqrt(discr_2);
            let mut t = (-b - discr) / a;
            if t >= 0.0 && t <= ray.t_max {
                let p = ray.at(t);
                let n = p.to_vec().normalize();
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
                let n = p.to_vec().normalize();
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

/// Rectangle shape in the XZ plane (normal pointing in the Y direction), centred around the
/// origin, of size 1x1, at a height of h.
pub struct Plane {
    k: f32,
    material: Arc<Material + Send + Sync>,
}

impl Plane {
    pub fn new(k: f32, mat: Arc<Material + Send + Sync>) -> Self {
        Plane { k, material: mat }
    }
}

impl Shape for Plane {
    fn intersect(&self, r: &mut Ray) -> Option<Hit> {
        let t = (self.k - r.o.y) / r.d.y;
        if t < 0.0 || t > r.t_max {
            return None;
        }

        let x = r.o.x + t * r.d.x;
        let z = r.o.z + t * r.d.z;
        if x < -0.5 || x > 0.5 || z < -0.5 || z > 0.5 {
            return None;
        }
        // TODO uv
        // rec.u = (x - self.x0) / (self.x1 - self.x0);
        // rec.v = (z - self.z0) / (self.z1 - self.z0);
        r.t_max = t;
        Some(Hit {
            n: Vec3f::unit_y(),
            p: r.at(t),
            mat: self.material.clone(),
        })
    }

    // fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut Aabb) -> bool {
    //     *aabb = Aabb::new(
    //         &Vec3::new(self.x0, self.k - 0.0001, self.z0),
    //         &Vec3::new(self.x1, self.k + 0.0001, self.z1),
    //     );
    //     true
    // }
}

pub struct TransformedShape {
    pub shape: Box<Shape + Send + Sync>,
    pub transform: Transform,
}

impl TransformedShape {
    pub fn new(s: Box<Shape + Send + Sync>, t: Transform) -> TransformedShape {
        TransformedShape {
            shape: s,
            transform: t,
        }
    }
}

impl Shape for TransformedShape {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit> {
        let mut local_ray = ray.transform(self.transform.m_inv);
        self.shape.intersect(&mut local_ray).map(|h| {
            ray.t_max = local_ray.t_max;
            h.transform(self.transform)
        })
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
