extern crate cgmath as cg;
extern crate rand;

mod material;
mod ray;
mod shape;

use std::f32;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

use cg::{prelude::*, vec3};
use rand::Rng;

use material::*;
use ray::Ray;
use shape::*;

type Vec3 = cg::Vector3<f32>;

const NX: usize = 400;
const NY: usize = 200;

pub fn colour(shape: &Shape, r: &mut Ray, depth: u32) -> Vec3 {
    if let Some(hit) = shape.intersect(r) {
        if let Some(mut scattering_event) = hit.mat.scatter(r, &hit) {
            if depth < 50 {
                // cos_theta attenuation factor
                let cos_theta = scattering_event.r_out.d.dot(hit.n).abs();
                let scattered = colour(shape, &mut scattering_event.r_out, depth + 1);
                scattering_event.attenuation.mul_element_wise(scattered) * cos_theta
            } else {
                Vec3::zero()
            }
        } else {
            Vec3::zero()
        }
    } else {
        let unit_vec = r.d.normalize();
        let t = 0.5 * (unit_vec.y + 1.0);

        t * vec3(0.5, 0.7, 1.0) + (1.0 - t) * vec3(1.0, 1.0, 1.0)
    }
}

fn main() {
    let mut buf = vec![Vec3::zero(); NX * NY];
    let ratio = NX as f32 / NY as f32;
    let ns = 100;
    let mut rng = rand::thread_rng();

    let ground: Arc<Material> = Arc::new(Diffuse::new(vec3(0.8, 0.8, 0.0)));
    let diffuse: Arc<Material> = Arc::new(Diffuse::new(vec3(0.8, 0.3, 0.3)));
    let mirror: Arc<Material> = Arc::new(Mirror);
    let world = Aggregation {
        shapes: vec![
            Box::new(Sphere::new(vec3(-1.0, 0.0, -1.0), 0.5, mirror.clone())),
            Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5, diffuse.clone())),
            Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0, ground.clone())),
        ],
    };
    let camera_centre = vec3(0.0, 0.0, 0.5);
    for y in 0..NY {
        for x in 0..NX {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (((x as f32 + rng.next_f32()) / NX as f32) * 2.0 - 1.0) * ratio;
                let v = (((NY - y) as f32 + rng.next_f32()) / NY as f32) * 2.0 - 1.0;
                let mut ray = Ray::new(camera_centre, vec3(u, v, -1.0));
                col += colour(&world, &mut ray, 0);
            }

            buf[y * NX + x] = col / ns as f32;
        }
    }

    write_img(&buf);
}

fn write_img(img: &[Vec3]) {
    let file = File::create("out.ppm").expect("Could not create out.ppm");
    let mut out = BufWriter::new(file);

    writeln!(out, "P3 {} {}\n255", NX, NY).unwrap();
    for y in 0..NY {
        for x in 0..NX {
            let (r, g, b) = img[y * NX + x].into();
            write!(
                out,
                "{} {} {} ",
                (f32::powf(r, 1.0 / 2.2) * 255.99) as u8,
                (f32::powf(g, 1.0 / 2.2) * 255.99) as u8,
                (f32::powf(b, 1.0 / 2.2) * 255.99) as u8
            ).unwrap();
        }
        writeln!(out).unwrap();
    }
}
