extern crate cgmath as cg;
extern crate minifb;
extern crate rand;
extern crate rayon;

mod material;
mod ray;
mod shape;

use std::f32;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

use cg::{prelude::*, vec3};
use minifb::{Window, WindowOptions};
use rand::Rng;
use rayon::prelude::*;

use material::*;
use ray::Ray;
use shape::*;

type Vec3f = cg::Vector3<f32>;
type Point3f = cg::Point3<f32>;

fn main() {
    const NX: usize = 800;
    const NY: usize = 600;

    let mut window = match Window::new("toyrt", NX, NY, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let mut buf = vec![PixelSample::new(); NX * NY];
    let mut rendered_buf = vec![0u32; NX * NY];
    let ratio = NX as f32 / NY as f32;
    let mut total_samples = 0;
    let ns = 10;

    let world = world();
    let camera_centre = Point3f::new(0.0, 0.0, 0.5);
    loop {
        buf.par_chunks_mut(NX).enumerate().for_each(|(y, row)| {
            let mut rng = rand::thread_rng();
            for x in 0..NX {
                for _ in 0..ns {
                    let u = (((x as f32 + rng.next_f32()) / NX as f32) * 2.0 - 1.0) * ratio;
                    let v = (((NY - y) as f32 + rng.next_f32()) / NY as f32) * 2.0 - 1.0;
                    let mut ray = Ray::new(camera_centre, vec3(u, v, -1.0));
                    row[x].add(&colour(&world, &mut ray, 0));
                }
            }
        });
        total_samples += ns;
        print!("\rRendered {} samples...", total_samples);
        std::io::stdout().flush().unwrap();

        // Update display
        rendered_buf
            .par_chunks_mut(NX)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..NX {
                    let (r, g, b) = buf[x + NX * y].render();
                    row[x] = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                }
            });
        if window.is_open() {
            window.update_with_buffer(&rendered_buf).unwrap();
        } else {
            break;
        }
    }

    write_img(&buf, (NX, NY));
}

pub fn world() -> Aggregation {
    let ground: Arc<Material + Send + Sync> = Arc::new(Diffuse::new(vec3(0.8, 0.8, 0.0)));
    let diffuse: Arc<Material + Send + Sync> = Arc::new(Diffuse::new(vec3(0.8, 0.3, 0.3)));
    let mirror: Arc<Material + Send + Sync> = Arc::new(Mirror);
    Aggregation {
        shapes: vec![
            Box::new(Sphere::new(
                Point3f::new(-1.0, 0.0, -1.0),
                0.5,
                mirror.clone(),
            )),
            Box::new(Sphere::new(
                Point3f::new(0.0, 0.0, -1.0),
                0.5,
                diffuse.clone(),
            )),
            Box::new(Sphere::new(
                Point3f::new(0.0, -100.5, -1.0),
                100.0,
                ground.clone(),
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
    pub fn new() -> PixelSample {
        PixelSample {
            sum: Vec3f::zero(),
            n_sample: 0,
        }
    }

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
