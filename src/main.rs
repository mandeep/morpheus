extern crate image;
extern crate nalgebra;

use std::fs::File;
use std::env;
use nalgebra::core::Vector3;

mod wavefront;
mod render;
mod vector;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];

    let (width, height, depth) = (1600, 1600, 255);

    let mut buffer = image::ImageBuffer::new(width, height);

    let eye = Vector3::new(6.0, 5.0, 30.0);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let light_vector = Vector3::new(5.0, 5.0, 20.0).normalize();

    render::draw_triangle_mesh(input_file, &mut buffer, depth, &light_vector, &eye, &center, &up);

    let ref mut fout = File::create(output_file).unwrap();
    image::ImageRgb8(buffer).flipv()
                            .save(fout, image::PNG)
                            .unwrap();
}
