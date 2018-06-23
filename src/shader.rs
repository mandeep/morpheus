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
    let mut transpose: Matrix4<f64> = Matrix4::identity();

    for i in 0..3 {
        matrix.row_mut(0)[i] = x[i];
        matrix.row_mut(1)[i] = y[i];
        matrix.row_mut(2)[i] = z[i];
        transpose.row_mut(i)[3] = -center[i];
    }

    matrix * transpose
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
pub fn find_barycentric(a: Vector2<f64>, b: Vector2<f64>,
                    c: Vector2<f64>, p: &Vector2<f64>) -> Vector3<f64> {

    let mut s = vec![Vector3::zeros(), Vector3::zeros(), Vector3::zeros()];

    for i in (0..2).rev() {
        s[i][0] = c[i] - a[i];
        s[i][1] = b[i] - a[i];
        s[i][2] = a[i] - p[i];
    }

    let u: Vector3<f64> = s[0].cross(&s[1]);

    if (u.z).abs() < 0.01 {
        return Vector3::new(-1.0, 1.0, 1.0);
    } else {
        return Vector3::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    }

}

pub struct GouraudShader {
    pub varying_intensity: Vector3<f64>,
    pub varying_texture: Matrix2x3<f64>
}

impl GouraudShader {
    /// Position the vertices into their scene coordinates
    pub fn vertex(&mut self, coordinates: &wavefront::Object,
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
    pub fn fragment(&self, coordinates: &wavefront::Object, vertex: Vector3<f64>,
                    color: &mut image::Rgb<u8>, texture: &image::RgbImage) -> bool {

        let intensity: f64 = self.varying_intensity.dot(&vertex);
        let uv: Vector2<f64> = self.varying_texture * vertex;

        let width = (uv.x as u32 * texture.width()) as usize;
        let height = (uv.y as u32 * texture.height()) as usize;
        let diffuse = coordinates.texture_vertices[coordinates.texture_faces[width][height] as usize];
        let diffuse_pixel = texture.get_pixel(diffuse.x as u32, diffuse.y as u32);

        for i in 0..=2 {
            color[i] = (diffuse_pixel[i] as f64 * intensity) as u8;
        }

        false
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
