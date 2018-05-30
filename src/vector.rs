extern crate image;
extern crate nalgebra;

use nalgebra::geometry::Point3;
use nalgebra::core::Matrix4x1;


/// Project 4D coordinates into 3D by dividing the x, y, and z coordinate by the w coordinate
///
/// # Examples
///
/// ```
/// let matrix: Matrix4x1<f64> = Matrix4x1::new(2.0, 4.0, 6.0, 2.0);
/// let point: Point3<f64> = vectorize(matrix);
/// assert!(point.x == 1.0 && point.y == 2.0 && point.z == 3.0);
/// ```
///
pub fn vectorize(matrix: Matrix4x1<f64>) -> Point3<f64> {
    Point3::new(matrix.row(0)[0] / matrix.row(3)[0], matrix.row(1)[0] / matrix.row(3)[0], matrix.row(2)[0] / matrix.row(3)[0])
}


/// Create a 4x1 matrix from a 3D point by setting the last row to 1.0
///
/// # Examples
///
/// ```
/// let point: Point3<f64> = Point3::new(1.0, 2.0, 3.0);
/// let matrix: Matrix4x1<f64> = matricize(point);
/// assert!(matrix.row(0)[0] == 1.0 && matrix.row(1)[0] == 2.0 && matrix.row(2)[0] == 3.0 && matrix.row(3)[0] == 1.0);
/// ```
///
pub fn matricize(point: Point3<f64>) -> Matrix4x1<f64> {
    Matrix4x1::new(point.x, point.y, point.z, 1.0)
}
