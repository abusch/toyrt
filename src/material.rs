use std::f32;

use cg::vec3;
use rand::{self, Rng};

use Vec3;
use ray::Ray;
use shape::*;

#[derive(Debug)]
pub struct ScatteringEvent {
    pub r_out: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<ScatteringEvent>;
}

pub struct Diffuse {
    albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Diffuse {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Option<ScatteringEvent> {
        // Random direction for output ray
        let mut rng = rand::thread_rng();
        let u = rng.next_f32();
        let v = rng.next_f32();
        let out_dir = uniform_sample_hemisphere(u, v);

        // Compute attenuation
        let attenuation = self.albedo;

        Some(ScatteringEvent {
            r_out: Ray::new(hit.p + 0.001 * hit.n, out_dir),
            attenuation,
        })
    }
}

fn uniform_sample_hemisphere(u: f32, v: f32) -> Vec3 {
    let z = u;
    let r = (1.0 - z * z).max(0.0).sqrt();
    let phi = 2.0 * f32::consts::PI * v;

    vec3(r * phi.cos(), r * phi.sin(), z)
}
