use std::f32;

use cg::prelude::*;
use rand::{self, Rng};

use crate::ray::Ray;
use crate::{Point3f, Vec3f};

pub struct Camera {
    origin: Point3f,
    lower_left_corner: Point3f,
    horizontal: Vec3f,
    vertical: Vec3f,
    u: Vec3f,
    v: Vec3f,
    _time0: f32,
    _time1: f32,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3f,
        lookat: Point3f,
        vup: Vec3f,
        fov: f32,
        aspect: f32,
        aperture: f32,
        t0: f32,
        t1: f32,
        focus_distance: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = fov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u).normalize();

        // let lower_left_corner = Vec3::new(-half_width, -half_height, -1.);
        let lower_left_corner = origin
            - half_width * focus_distance * u
            - half_height * focus_distance * v
            - focus_distance * w;
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            _time0: t0,
            _time1: t1,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3f {
    let mut rng = rand::thread_rng();
    loop {
        let p =
            2.0 * Vec3f::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3f::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}
