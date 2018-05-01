use std::f32;

use cg::{prelude::*, vec3};
use rand::{self, Rng};

use ray::Ray;
use shape::*;
use Vec3f;

#[derive(Debug)]
pub struct ScatteringEvent {
    pub r_out: Ray,
    pub attenuation: Vec3f,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<ScatteringEvent>;
}

pub struct Diffuse {
    albedo: Vec3f,
}

impl Diffuse {
    pub fn new(albedo: Vec3f) -> Diffuse {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Option<ScatteringEvent> {
        // Random direction for output ray
        let mut rng = rand::thread_rng();
        let u = rng.next_f32();
        let v = rng.next_f32();

        // With uniform hemisphere distribution
        // let out_dir = uniform_sample_hemisphere(u, v, &hit.n);
        // let f = self.albedo * f32::consts::FRAC_1_PI;
        // let pdf = 0.5 * f32::consts::FRAC_1_PI;

        // With cosine-weighted hemisphere distribution
        let out_dir = cosine_sample_hemisphere(u, v, &hit.n);
        let f = self.albedo * f32::consts::FRAC_1_PI;
        let pdf = out_dir.dot(hit.n).abs() * f32::consts::FRAC_1_PI;

        // Compute attenuation
        let attenuation = f / pdf;

        Some(ScatteringEvent {
            r_out: Ray::new(hit.p + 0.001 * hit.n, out_dir),
            attenuation,
        })
    }
}

pub struct Mirror;

impl Material for Mirror {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<ScatteringEvent> {
        let reflect = reflect(r_in.d.normalize(), hit.n);

        if reflect.dot(hit.n) > 0.0 {
            Some(ScatteringEvent {
                r_out: Ray::new(hit.p + 0.001 * hit.n, reflect),
                attenuation: vec3(1.0, 1.0, 1.0),
            })
        } else {
            None
        }
    }
}

fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - 2.0 * n.dot(v) * n
}

#[allow(dead_code)]
fn uniform_sample_hemisphere(u: f32, v: f32, n: &Vec3f) -> Vec3f {
    // Build an orthogonal coordinate system based around the normal
    let (tangent, bitangent) = coordinate_system(n);

    let z = u;
    let r = (1.0 - z * z).max(0.0).sqrt();
    let phi = 2.0 * f32::consts::PI * v;
    let x = r * phi.cos();
    let y = r * phi.sin();

    // Transform generated vector back into world space
    vec3(
        bitangent.x * x + tangent.x * y + n.x * z,
        bitangent.y * x + tangent.y * y + n.y * z,
        bitangent.z * x + tangent.z * y + n.z * z,
    )
}

#[allow(dead_code)]
fn cosine_sample_hemisphere(u: f32, v: f32, n: &Vec3f) -> Vec3f {
    // Build an orthogonal coordinate system based around the normal
    let (tangent, bitangent) = coordinate_system(n);

    // Generate a random direction in local coordinate space
    let r = f32::sqrt(u);
    let theta = 2.0 * f32::consts::PI * v;
    let x = r * f32::cos(theta);
    let y = r * f32::sin(theta);
    let z = f32::sqrt(f32::max(0.0, 1.0 - u));

    // Transform generated vector back into world space
    vec3(
        bitangent.x * x + tangent.x * y + n.x * z,
        bitangent.y * x + tangent.y * y + n.y * z,
        bitangent.z * x + tangent.z * y + n.z * z,
    )
}

/// Create an orthogonal coordinate system from a single vector.
fn coordinate_system(v1: &Vec3f) -> (Vec3f, Vec3f) {
    let v2 = if v1.x.abs() > v1.y.abs() {
        vec3(-v1.z, 0.0, v1.x) / (v1.x * v1.x + v1.z * v1.z).sqrt()
    } else {
        vec3(0.0, v1.z, -v1.y) / (v1.y * v1.y + v1.z * v1.z).sqrt()
    };

    let v3 = v1.cross(v2);

    (v2, v3)
}
