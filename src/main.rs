extern crate cgmath as cg;

mod ray;
mod shape;

use std::f32;
use std::fs::File;
use std::io::{BufWriter, Write};

use cg::{prelude::*, vec3};

use ray::Ray;
use shape::*;

type Vec3 = cg::Vector3<f32>;

const NX: usize = 400;
const NY: usize = 200;

pub fn colour(shape: &Shape, r: &mut Ray) -> Vec3 {
    if let Some(hit) = shape.intersect(r) {
        let v = r.d.dot(hit.n).abs();
        vec3(v, v, v)
    } else {
        let unit_vec = r.d.normalize();
        let t = 0.5 * (unit_vec.y + 1.0);

        t * vec3(0.2, 0.3, 1.0) + (1.0 - t) * vec3(1.0, 1.0, 1.0)
    }
}

fn main() {
    let mut buf = vec![Vec3::zero(); NX * NY];
    let ratio = NX as f32 / NY as f32;

    let world = Aggregation {
        shapes: vec![
            Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(vec3(0.0, -1000.5, -1.0), 1000.0)),
        ],
    };
    for y in 0..NY {
        let v = ((NY - y) as f32 / NY as f32) * 2.0 - 1.0;
        for x in 0..NX {
            let u = ((x as f32 / NX as f32) * 2.0 - 1.0) * ratio;
            let mut ray = Ray::new(Vec3::zero(), vec3(u, v, -1.0));

            buf[y * NX + x] = colour(&world, &mut ray);
        }
    }

    write_img(&buf);
}

fn write_img(img: &[Vec3]) {
    let file = File::create("out.ppm").expect("Could not create out.ppm");
    let mut out = BufWriter::new(file);

    writeln!(out, "P3 {} {}\n{}", NX, NY, 255).unwrap();
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
