extern crate nalgebra;
extern crate image;

use nalgebra::core::{Matrix4, Vector3, Vector4};

use vector;
use wavefront;


pub struct GouraudShader {
    pub varying_intensity: Vector3<f64>,
}


impl GouraudShader {
    pub fn vertex(&mut self, coordinates: &wavefront::Object, view_port: &Matrix4<f64>, projection: &Matrix4<f64>, model_view: &Matrix4<f64>,
                  light_vector: &Vector3<f64>, face_index: usize, vertex_index: usize) -> Vector4<f64> {

        let normal_index = coordinates.normal_faces[face_index][vertex_index] as usize;
        let geometric_index = coordinates.geometric_faces[face_index][vertex_index] as usize;

        self.varying_intensity[vertex_index] = 0.0f64.max(coordinates.normal_vertices[normal_index].map(|n| n as f64)
                                                                                                   .normalize()
                                                                                                   .dot(&light_vector));
        let gl_vertex: Vector4<f64> = vector::vectorize_to_4d(coordinates.geometric_vertices[geometric_index]);

        view_port * projection * model_view * gl_vertex
    }

    pub fn fragment(&self, pixel: Vector3<f64>, color: &mut image::Rgb<u8>) -> bool {
        let intensity: f64 = self.varying_intensity.dot(&pixel);

        for i in 0..=2 {
            color[i] = (255.0 * intensity) as u8;
        }

        false
    }
}

