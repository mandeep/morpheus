#![allow(dead_code)]
extern crate image;
extern crate nalgebra;

use std::env;

use nalgebra::core::Vector3;

mod render;
mod shader;
mod vector;
mod wavefront;


fn main() {
    let args: Vec<String> = env::args().collect();

    let (width, height, depth) = (2048, 2048, 255);

    let mut buffer = image::ImageBuffer::new(width, height);

    let texture = image::open(&args[2]).unwrap().flipv().to_rgb();

    let eye = Vector3::new(0.0, 15.0, 70.0);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let light_vector = Vector3::new(0.0, 15.0, 70.0).normalize();

    render::draw_triangle_mesh(&args[1], &mut buffer, &texture, depth,
                               &light_vector, &eye, &center, &up);

    image::ImageRgb8(buffer).flipv()
                            .save("output.png")
                            .unwrap();
}
