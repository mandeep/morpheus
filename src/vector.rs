extern crate nalgebra;

use nalgebra::core::{Vector3, Vector4};
use nalgebra::geometry::{Point2, Point3};


/// Project 3D coordinates into 2D by dividing the x and y coordinate by the z coordinate
///
/// # Examples
///
/// ```
/// let matrix: Vector3<f64> = Vector3::new(2.0, 4.0, 2.0);
/// let point: Point2<f64> = project2(matrix);
/// assert!(point.x == 1.0 && point.y == 2.0);
/// ```
///
pub fn project2(vector: Vector3<f64>) -> Point2<f64> {
    Point2::new(vector.x / vector.z, vector.y / vector.z)
}


/// Project 4D coordinates into 3D by dividing the x, y, and z coordinate by the w coordinate
///
/// # Examples
///
/// ```
/// let matrix: Vector4<f64> = Vector4::new(2.0, 4.0, 6.0, 2.0);
/// let point: Point3<f64> = project3(matrix);
/// assert!(point.x == 1.0 && point.y == 2.0 && point.z == 3.0);
/// ```
///
pub fn project3(matrix: Vector4<f64>) -> Point3<f64> {
    Point3::new(matrix.row(0)[0] / matrix.row(3)[0], matrix.row(1)[0] / matrix.row(3)[0], matrix.row(2)[0] / matrix.row(3)[0])
}


/// Create a 3D vector from a 2D point by setting the z coordinate to 1.0
///
/// # Examples
///
/// ```
/// let point: Point2<f64> = Point2::new(1.0, 2.0);
/// let vector: Vector3<f64> = vectorize3(point);
/// assert!(vector.x == 1.0 && vector.y == 2.0 && vector.z == 1.0);
/// ```
///
pub fn vectorize3(point: Point2<f64>) -> Vector3<f64> {
    Vector3::new(point.x, point.y, 1.0)
}


/// Create a 4x1 matrix from a 3D point by setting the last row to 1.0
///
/// # Examples
///
/// ```
/// let point: Point3<f64> = Point3::new(1.0, 2.0, 3.0);
/// let matrix: Vector4<f64> = vectorize4(point);
/// assert!(matrix.row(0)[0] == 1.0 && matrix.row(1)[0] == 2.0 && matrix.row(2)[0] == 3.0 && matrix.row(3)[0] == 1.0);
/// ```
///
pub fn vectorize4(point: Point3<f64>) -> Vector4<f64> {
    Vector4::new(point.x, point.y, point.z, 1.0)
}
