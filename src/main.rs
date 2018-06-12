extern crate image;
extern crate nalgebra;

use std::env;
use std::fs::File;

use nalgebra::core::Vector3;

mod render;
mod shader;
mod vector;
mod wavefront;


fn main() {
    let args: Vec<String> = env::args().collect();

    let (width, height, depth) = (1600, 1600, 255);

    let mut buffer = image::ImageBuffer::new(width, height);
    let mut zbuffer = image::ImageBuffer::new(width, height);

    let eye = Vector3::new(6.0, 5.0, 30.0);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let light_vector = Vector3::new(5.0, 5.0, 20.0).normalize();

    render::draw_triangle_mesh(&args[1], &mut buffer, &mut zbuffer, depth, &light_vector, &eye, &center, &up);

    let ref mut render = File::create(&args[2]).unwrap();
    let ref mut zbuffer_render = File::create(&args[3]).unwrap();

    image::ImageRgb8(buffer).flipv()
                            .save(render, image::PNG)
                            .unwrap();

    image::ImageLuma8(zbuffer).flipv()
                              .save(zbuffer_render, image::PNG)
                              .unwrap();
}
