extern crate nalgebra;
extern crate image;

use nalgebra::core::{Matrix2x3, Matrix4, Vector2, Vector3, Vector4};

use vector;
use wavefront;


/// Change the frame of reference of the viewer
///
/// The eye vector is where the camera is situated. The center
/// vector is where the camera should point in relation to
/// the up vector which is vertical when rendered.
///
pub fn lookat(eye: &Vector3<f64>, center: &Vector3<f64>, up: &Vector3<f64>) -> Matrix4<f64> {
    let z = (eye - center).normalize();
    let x = up.cross(&z).normalize();
    let y = z.cross(&x).normalize();

    let mut matrix: Matrix4<f64> = Matrix4::identity();
    let mut translation: Matrix4<f64> = Matrix4::identity();

    for i in 0..3 {
        matrix.row_mut(0)[i] = x[i];
        matrix.row_mut(1)[i] = y[i];
        matrix.row_mut(2)[i] = z[i];
        translation.row_mut(i)[3] = -center[i];
    }

    matrix * translation
}


/// Create a projection matrix with the given coefficient
pub fn projection(coefficient: f64) -> Matrix4<f64> {
    let mut matrix: Matrix4<f64> = Matrix4::identity();
    matrix.row_mut(3)[2] = coefficient;

    matrix

}


/// Map the bi-unit cube of [-1, 1] * [-1, 1] * [-1, 1] to the dimensions of the image
///
/// The x and y parameters specify the origin of the viewport while the
/// width and height parameters specify the width and height of the viewport.
///
pub fn viewport(x: u32, y: u32, width: u32, height: u32, depth: u32) -> Matrix4<f64> {
    let mut matrix = Matrix4::identity();

    matrix.row_mut(0)[3] = x as f64 + width as f64 / 2.0;
    matrix.row_mut(1)[3] = y as f64 + height as f64 / 2.0;
    matrix.row_mut(2)[3] = depth as f64 / 2.0;

    matrix.row_mut(0)[0] = width as f64 / 2.0;
    matrix.row_mut(1)[1] = height as f64 / 2.0;
    matrix.row_mut(2)[2] = depth as f64 / 2.0;

    matrix
}


/// Find the barycentric coordinates of the given point with respect to the given triangle
///
/// # Examples
///
/// ```
/// let points =  vec![Vector3::new(0, 0, 0), Vector3::new(2, 2, 2), Vector3::new(0, 2, 2)]
/// let point = Point3::new(1.0, 1.0, 0.0);
/// let barycentric_coordinates: Point3<f64> = find_barycentric(&points, &point);
/// ```
///
pub fn find_barycentric(points: &Vec<Vector2<f64>>, point: &Vector4<f64>) -> Vector3<f64> {
    let u = Vector3::new(points[2].x - points[0].x, points[1].x - points[0].x, points[0].x - point.x);
    let v = Vector3::new(points[2].y - points[0].y, points[1].y - points[0].y, points[0].y - point.y);

    let w = u.cross(&v);

    if (w.z).abs() < 0.01 {
        return Vector3::new(-1.0, 1.0, 1.0);
    } else {
        return Vector3::new(1.0 - (w.x + w.y) / w.z, w.y / w.z, w.x / w.z);
    }

}


/// Shader trait can be used to implement multiple shaders
pub trait Shader {
    fn vertex(&mut self, coordinates: &wavefront::Object,
                  view_port: &Matrix4<f64>, projection: &Matrix4<f64>,
                  model_view: &Matrix4<f64>, light_vector: &Vector3<f64>,
                  face_index: usize, vertex_index: usize) -> Vector4<f64>;


    fn fragment(&self, vertex: Vector3<f64>, texture: &image::RgbImage) -> image::Rgb<u8>;
}


pub struct FlatShader {
    pub varying_intensity: Vector3<f64>,
    pub varying_texture: Matrix2x3<f64>,
    pub world_coordinates: Vec<Vector3<f64>>
}


impl FlatShader {
    /// Create a new instance of a FlatShader
    pub fn new() -> FlatShader {
        FlatShader { varying_intensity: Vector3::zeros(),
                     varying_texture: Matrix2x3::zeros(),
                     world_coordinates: vec![Vector3::zeros(); 3] }
    }
}


impl Shader for FlatShader {
    /// Position the vertices into their scene coordinates
    fn vertex(&mut self, coordinates: &wavefront::Object,
                  view_port: &Matrix4<f64>, projection: &Matrix4<f64>,
                  model_view: &Matrix4<f64>, light_vector: &Vector3<f64>,
                  face_index: usize, vertex_index: usize) -> Vector4<f64> {

        let geometric_index = coordinates.geometric_faces[face_index][vertex_index] as usize;
        let texture_index = coordinates.texture_faces[face_index][vertex_index] as usize;

        self.varying_texture.set_column(vertex_index, &coordinates.texture_vertices[texture_index]);
        self.varying_intensity = *light_vector;

        let gl_vertex = vector::vectorize_to_4d(coordinates.geometric_vertices[geometric_index]);
        let projected_coordinate = vector::project_to_3d(projection * model_view * gl_vertex);

        (0..=2).for_each(|i| { self.world_coordinates[vertex_index][i] = projected_coordinate[i];});

        view_port * projection * model_view * gl_vertex
    }

    /// Set the light intensity of the given vertex as determined by the vertex shader
    fn fragment(&self, vertex: Vector3<f64>, texture: &image::RgbImage) -> image::Rgb<u8> {
        let normal = (self.world_coordinates[1] - self.world_coordinates[0])
            .cross(&(self.world_coordinates[2] - self.world_coordinates[0])).normalize();

        let intensity = normal.dot(&self.varying_intensity);

        let uv: Vector2<f64> = self.varying_texture * vertex;

        let width = (uv.x * texture.width() as f64) as usize;
        let height = (uv.y * texture.height() as f64) as usize;
        let mut texture_pixel = *texture.get_pixel(width as u32, height as u32);

        (0..=2).for_each(|i| {texture_pixel[i] = (texture_pixel[i] as f64 * intensity) as u8;});

        texture_pixel
    }
}


pub struct CelShader {
    pub varying_intensity: Vector3<f64>,
    pub varying_texture: Matrix2x3<f64>,
}


impl CelShader {
    /// Create a new instance of a CelShader
    pub fn new() -> CelShader {
        CelShader { varying_intensity: Vector3::zeros(),
                    varying_texture: Matrix2x3::zeros() }

    }
}


impl Shader for CelShader {
    /// Position the vertices into their scene coordinates
    fn vertex(&mut self, coordinates: &wavefront::Object,
                  view_port: &Matrix4<f64>, projection: &Matrix4<f64>,
                  model_view: &Matrix4<f64>, light_vector: &Vector3<f64>,
                  face_index: usize, vertex_index: usize) -> Vector4<f64> {

        let geometric_index = coordinates.geometric_faces[face_index][vertex_index] as usize;
        let texture_index = coordinates.texture_faces[face_index][vertex_index] as usize;
        let normal_index = coordinates.normal_faces[face_index][vertex_index] as usize;

        self.varying_intensity[vertex_index] = 0.0f64
            .max(coordinates.normal_vertices[normal_index].map(|n| n as f64)
                                                          .normalize()
                                                          .dot(&light_vector));

        self.varying_texture.set_column(vertex_index, &coordinates.texture_vertices[texture_index]);

        let gl_vertex = vector::vectorize_to_4d(coordinates.geometric_vertices[geometric_index]);

        view_port * projection * model_view * gl_vertex
    }

    /// Set the light intensity of the given vertex as determined by the vertex shader
    fn fragment(&self, vertex: Vector3<f64>, texture: &image::RgbImage) -> image::Rgb<u8> {
        let mut intensity: f64 = self.varying_intensity.dot(&vertex);
        let uv: Vector2<f64> = self.varying_texture * vertex;

        if intensity > 0.95 { intensity = 1.0; }
        else if intensity > 0.50 { intensity = 0.70; }
        else if intensity > 0.10 { intensity = 0.35; }
        else { intensity = 0.20; }

        let width = (uv.x * texture.width() as f64) as usize;
        let height = (uv.y * texture.height() as f64) as usize;
        let mut texture_pixel = *texture.get_pixel(width as u32, height as u32);

        (0..=2).for_each(|i| {texture_pixel[i] = (texture_pixel[i] as f64 * intensity) as u8;});

        texture_pixel
    }
}

pub struct GouraudShader {
    pub varying_intensity: Vector3<f64>,
    pub varying_texture: Matrix2x3<f64>
}


impl GouraudShader {
    /// Create a new instance of a GouraudShader
    pub fn new() -> GouraudShader {
        GouraudShader { varying_intensity: Vector3::zeros(), varying_texture: Matrix2x3::zeros() }
    }
}


impl Shader for GouraudShader {
    /// Position the vertices into their scene coordinates
    fn vertex(&mut self, coordinates: &wavefront::Object,
              view_port: &Matrix4<f64>, projection: &Matrix4<f64>,
              model_view: &Matrix4<f64>, light_vector: &Vector3<f64>,
              face_index: usize, vertex_index: usize) -> Vector4<f64> {

        let geometric_index = coordinates.geometric_faces[face_index][vertex_index] as usize;
        let texture_index = coordinates.texture_faces[face_index][vertex_index] as usize;
        let normal_index = coordinates.normal_faces[face_index][vertex_index] as usize;

        self.varying_intensity[vertex_index] = 0.0f64
            .max(coordinates.normal_vertices[normal_index].map(|n| n as f64)
                                                          .normalize()
                                                          .dot(&light_vector));

        self.varying_texture.set_column(vertex_index, &coordinates.texture_vertices[texture_index]);

        let gl_vertex = vector::vectorize_to_4d(coordinates.geometric_vertices[geometric_index]);

        view_port * projection * model_view * gl_vertex
    }

    /// Set the light intensity of the given vertex as determined by the vertex shader
    fn fragment(&self, vertex: Vector3<f64>, texture: &image::RgbImage) -> image::Rgb<u8> {
        let intensity: f64 = self.varying_intensity.dot(&vertex);
        let uv: Vector2<f64> = self.varying_texture * vertex;

        let width = (uv.x * texture.width() as f64) as usize;
        let height = (uv.y * texture.height() as f64) as usize;
        let mut texture_pixel = *texture.get_pixel(width as u32, height as u32);

        (0..=2).for_each(|i| {texture_pixel[i] = (texture_pixel[i] as f64 * intensity) as u8;});

        texture_pixel
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookat() {
        let eye: Vector3<f64> = Vector3::new(0.0, -1.0, 3.0);
        let center: Vector3<f64> = Vector3::zeros();
        let up: Vector3<f64> = Vector3::new(0.0, 1.0, 0.0);

        let view = lookat(&eye, &center, &up);

        assert_eq!(view.row(0)[0], 1.0);
        assert_eq!(view.row(0)[1], 0.0);
        assert_eq!(view.row(0)[2], 0.0);
        assert_eq!(view.row(0)[3], 0.0);

        assert_eq!(view.row(1)[0], 0.0);
        assert!(view.row(1)[1] - 0.948683 < 0.0001);
        assert!(view.row(1)[2] - 0.316228 < 0.0001);
        assert_eq!(view.row(1)[3], 0.0);

        assert_eq!(view.row(2)[0], 0.0);
        assert!(view.row(2)[1] - 0.316228 < 0.0001);
        assert!(view.row(2)[2] - 0.948683 < 0.0001);
        assert_eq!(view.row(2)[3], 0.0);

        assert_eq!(view.row(3)[0], 0.0);
        assert_eq!(view.row(3)[1], 0.0);
        assert_eq!(view.row(3)[2], 0.0);
        assert_eq!(view.row(3)[3], 1.0);
    }

    #[test]
    fn test_projection() {
        let eye: Vector3<f64> = Vector3::new(0.0, -1.0, 3.0);
        let center: Vector3<f64> = Vector3::zeros();

        let view = projection(-1.0 / (&eye - &center).norm());

        assert_eq!(view.row(0)[0], 1.0);
        assert_eq!(view.row(0)[1], 0.0);
        assert_eq!(view.row(0)[2], 0.0);
        assert_eq!(view.row(0)[3], 0.0);

        assert_eq!(view.row(1)[0], 0.0);
        assert_eq!(view.row(1)[1], 1.0);
        assert_eq!(view.row(1)[2], 0.0);
        assert_eq!(view.row(1)[3], 0.0);

        assert_eq!(view.row(2)[0], 0.0);
        assert_eq!(view.row(2)[1], 0.0);
        assert_eq!(view.row(2)[2], 1.0);
        assert_eq!(view.row(2)[3], 0.0);

        assert_eq!(view.row(3)[0], 0.0);
        assert_eq!(view.row(3)[1], 0.0);
        assert!(view.row(3)[2].is_sign_negative() && view.row(3)[2].abs() - 0.316228 < 0.0001);
        assert_eq!(view.row(3)[3], 1.0);
    }

    #[test]
    fn test_viewport() {
        let (width, height, depth) = (800, 800, 255);
        let view = viewport(width / 8, height / 8, width * 3/4, height * 3/4, depth);

        assert_eq!(view.row(0)[0], 300.0);
        assert_eq!(view.row(0)[1], 0.0);
        assert_eq!(view.row(0)[2], 0.0);
        assert_eq!(view.row(0)[3], 400.0);

        assert_eq!(view.row(1)[0], 0.0);
        assert_eq!(view.row(1)[1], 300.0);
        assert_eq!(view.row(1)[2], 0.0);
        assert_eq!(view.row(1)[3], 400.0);

        assert_eq!(view.row(2)[0], 0.0);
        assert_eq!(view.row(2)[1], 0.0);
        assert!(view.row(2)[2] - 127.5 < 0.0001);
        assert!(view.row(2)[3] - 127.5 < 0.0001);

        assert_eq!(view.row(3)[0], 0.0);
        assert_eq!(view.row(3)[1], 0.0);
        assert_eq!(view.row(3)[2], 0.0);
        assert_eq!(view.row(3)[3], 1.0);
    }
}
