use std::sync::Arc;

use cg::prelude::*;

use material::Material;
use ray::Ray;
use {Matrix4f, Point3f, Vec3f};

#[derive(Clone)]
pub struct Hit {
    pub p: Point3f,
    pub n: Vec3f,
    pub mat: Arc<Material>,
}

impl Hit {
    pub fn transform(&self, t: Matrix4f) -> Self {
        Hit {
            p: t.transform_point(self.p),
            n: Self::transform_normal(t, self.n),
            mat: self.mat.clone(),
        }
    }

    fn transform_normal(t: Matrix4f, n: Vec3f) -> Vec3f {
        let m = t.invert().unwrap();
        let (x, y, z) = (n.x, n.y, n.z);

        Vec3f::new(
            m[0][0] * x + m[1][0] * y + m[2][0] * z,
            m[0][1] * x + m[1][1] * y + m[2][1] * z,
            m[0][2] * x + m[1][2] * y + m[2][2] * z,
        )
    }
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

/// Rectangle shape in the XZ plane (normal pointing in the Y direction)
pub struct Rect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Arc<Material + Send + Sync>,
}

impl Rect {
    pub fn new(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        mat: Arc<Material + Send + Sync>,
    ) -> Self {
        Rect {
            x0,
            x1,
            z0,
            z1,
            k,
            material: mat,
        }
    }
}

impl Shape for Rect {
    fn intersect(&self, r: &mut Ray) -> Option<Hit> {
        let t = (self.k - r.o.y) / r.d.y;
        if t < 0.0 || t > r.t_max {
            return None;
        }

        let x = r.o.x + t * r.d.x;
        let z = r.o.z + t * r.d.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
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
    pub transform: Matrix4f,
    pub inv_transform: Matrix4f,
}

impl TransformedShape {
    pub fn new(s: Box<Shape + Send + Sync>, t: Matrix4f) -> TransformedShape {
        TransformedShape {
            shape: s,
            transform: t,
            inv_transform: t.invert().unwrap(),
        }
    }
}

impl Shape for TransformedShape {
    fn intersect(&self, ray: &mut Ray) -> Option<Hit> {
        let mut local_ray = ray.transform(self.inv_transform);
        let hit = self.shape.intersect(&mut local_ray).map(|h| {
            ray.t_max = local_ray.t_max;
            h.transform(self.transform)
        });
        hit
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
