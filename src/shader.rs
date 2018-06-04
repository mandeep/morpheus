extern crate nalgebra;
extern crate image;

use nalgebra::core::{Matrix4, Matrix4x1, Vector3};

mod vector;
mod wavefront;


struct GouraudShader {
    varying_intensity: Vector3<f64>,
}


impl GouraudShader {
    fn vertex(coordinates: &wavefront::Object, view_port: &Matrix4<f64>, projection: &Matrix4<f64>, model_view: &Matrix4<f64>, light_vector: &Vector3<f64>, face_index: usize, vertex_index: usize) -> Matrix4x1<f64> {
        varying_intensity[vertex_index] = 0.0.max(coordinates.normal_faces[face_index][vertex_index].normalize().cross(&light_vector));
        let gl_vertex: Matrix4x1<f64> = vector::matricize(coordinates.geometric_vertices[face_index][vertex_index]);

        view_port * projection * model_view * gl_vertex
    }

    fn fragment(pixel: Vector3<f64>, color: &mut image::Rgb<u8>) -> bool {
        let intensity: f64 = varying_intensity.cross(&pixel);
        color = image::Rgb([(255.0 * intensity) as u8, (255.0 * intensity) as u8, (255.0 * intensity) as u8]);

        false
    }
}
