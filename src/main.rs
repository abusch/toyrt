#![feature(rust_2018_preview)]
#![feature(core_intrinsics)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, many_single_char_names))]
extern crate cgmath as cg;
extern crate minifb;
extern crate rand;
extern crate rayon;

mod camera;
mod material;
mod ray;
mod shape;
mod transform;

use std::f32;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

use cg::{prelude::*, vec3};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rayon::prelude::*;

use camera::Camera;
use material::*;
use ray::Ray;
use shape::*;

type Vec3f = cg::Vector3<f32>;
type Point3f = cg::Point3<f32>;
type Matrix4f = cg::Matrix4<f32>;

fn main() {
    let width: usize = 800;
    let height: usize = 600;

    let mut window = match Window::new("toyrt", width, height, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let mut buf = vec![PixelSample::default(); width * height];
    let mut rendered_buf = vec![0u32; width * height];
    let ns = 100;

    let world = world();
    let camera = Camera::new(
        Point3f::new(0.0, 0.0, 0.5),
        Point3f::new(0.0, 0.0, 0.0),
        Vec3f::unit_y(),
        90.0,
        width as f32 / height as f32,
        0.0,
        0.0,
        1.0,
        0.5,
    );
    buf.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
        let mut rng = rand::thread_rng();
        for (x, pixel) in row.iter_mut().enumerate() {
            for _ in 0..ns {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = ((height - y) as f32 + rng.gen::<f32>()) / height as f32;
                let mut ray = camera.get_ray(u, v);
                pixel.add(&colour(&world, &mut ray, 0));
            }
        }
    });
    println!("\rRendered {} samples...", ns);

    // Update display
    rendered_buf
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..width {
                let (r, g, b) = buf[x + width * y].render();
                row[x] = (u32::from(r) << 16) | (u32::from(g) << 8) | (u32::from(b));
            }
        });
    window.update_with_buffer(&rendered_buf).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }

    write_img(&buf, (width, height));
}

pub fn world() -> impl Shape {
    let ground: Arc<Material + Send + Sync> = Arc::new(Diffuse::new(vec3(0.8, 0.8, 0.0)));
    let diffuse: Arc<Material + Send + Sync> = Arc::new(Diffuse::new(vec3(0.8, 0.3, 0.3)));
    let mirror: Arc<Material + Send + Sync> = Arc::new(Mirror);
    Aggregation {
        shapes: vec![
            Box::new(TransformedShape::new(
                Box::new(Sphere::new(0.5, mirror.clone())),
                Matrix4f::from_translation(vec3(-1.0, 0.0, -1.0)).into(),
            )),
            Box::new(TransformedShape::new(
                Box::new(Sphere::new(0.5, diffuse.clone())),
                Matrix4f::from_translation(vec3(0.0, 0.0, -1.0)).into(),
            )),
            // Box::new(Sphere::new(
            //     Point3f::new(0.0, -100.5, -1.0),
            //     100.0,
            //     ground.clone(),
            // )),
            Box::new(TransformedShape::new(
                Box::new(Plane::new(-0.5, ground.clone())),
                Matrix4f::from_translation(vec3(0.0, 0.0, -1.0)).into(),
            )),
        ],
    }
}

pub fn colour(shape: &Shape, r: &mut Ray, depth: u32) -> Vec3f {
    if let Some(hit) = shape.intersect(r) {
        if let Some(mut scattering_event) = hit.mat.scatter(r, &hit) {
            if depth < 50 {
                // cos_theta attenuation factor
                let cos_theta = scattering_event.r_out.d.dot(hit.n).abs();
                let scattered = colour(shape, &mut scattering_event.r_out, depth + 1);
                scattering_event.attenuation.mul_element_wise(scattered) * cos_theta
                    / scattering_event.pdf
            } else {
                Vec3f::zero()
            }
        } else {
            Vec3f::zero()
        }
    } else {
        let unit_vec = r.d.normalize();
        let t = 0.5 * (unit_vec.y + 1.0);

        t * vec3(0.5, 0.7, 1.0) + (1.0 - t) * vec3(1.0, 1.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct PixelSample {
    sum: Vec3f,
    n_sample: u32,
}

impl PixelSample {
    pub fn add(&mut self, sample: &Vec3f) {
        self.sum += *sample;
        self.n_sample += 1;
    }

    pub fn render(&self) -> (u8, u8, u8) {
        let final_color = self.sum / self.n_sample as f32;
        let (r, g, b) = final_color.into();
        (
            (f32::powf(r, 1.0 / 2.2) * 255.99) as u8,
            (f32::powf(g, 1.0 / 2.2) * 255.99) as u8,
            (f32::powf(b, 1.0 / 2.2) * 255.99) as u8,
        )
    }
}

impl Default for PixelSample {
    fn default() -> Self {
        PixelSample {
            sum: Vec3f::zero(),
            n_sample: 0,
        }
    }
}

fn write_img(img: &[PixelSample], (width, height): (usize, usize)) {
    let file = File::create("out.ppm").expect("Could not create out.ppm");
    let mut out = BufWriter::new(file);

    writeln!(out, "P3 {} {}\n255", width, height).unwrap();
    for y in 0..height {
        for x in 0..width {
            let (r, g, b) = img[y * width + x].render();
            write!(out, "{} {} {} ", r, g, b).unwrap();
        }
        writeln!(out).unwrap();
    }
}
