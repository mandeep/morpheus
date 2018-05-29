extern crate image;
extern crate nalgebra;

use std::fs::File;
use nalgebra::core::Vector3;

mod wavefront;
mod render;
mod vector;


fn main() {
    let (width, height, depth) = (1600, 1600, 255);

    let mut buffer = image::ImageBuffer::new(width, height);


    let eye = Vector3::new(6.0, 1.0, 10.0);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let light_vector = Vector3::new(1.0, -1.0, 1.0).normalize();

    render::draw_triangle_mesh("../porsche.obj", &mut buffer, depth, &light_vector, &eye, &center, &up);

    let ref mut fout = File::create("../triangle_mesh.png").unwrap();
    image::ImageRgb8(buffer).flipv()
                            .save(fout, image::PNG)
                            .unwrap();
}
