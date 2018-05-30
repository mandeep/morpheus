extern crate image;
extern crate nalgebra;

use nalgebra::geometry::Point3;
use nalgebra::core:: Matrix4x1;


pub fn vectorize(matrix: Matrix4x1<f64>) -> Point3<f64> {
    Point3::new(matrix.row(0)[0] / matrix.row(3)[0], matrix.row(1)[0] / matrix.row(3)[0], matrix.row(2)[0] / matrix.row(3)[0])
}


pub fn matricize(point: Point3<f64>) -> Matrix4x1<f64> {
    Matrix4x1::new(point.x, point.y, point.z, 1.0)
}
