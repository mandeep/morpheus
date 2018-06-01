extern crate nalgebra;
extern crate image;

use nalgebra::core::Vector3;


struct GoraudShader {
    varying_intensity: f64,
}


impl GoraudShader {
    fn vertex(face_index: u32, vertex_index: u32) {

    }

    fn fragment(barycentric: Vector3<f64>, color: image::Rgb<u8>) {

    }
}
