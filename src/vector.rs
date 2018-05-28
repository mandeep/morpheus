extern crate image;
extern crate nalgebra;

use nalgebra::geometry::Point3;
use nalgebra::core::Matrix4;


pub fn vectorize(matrix: Matrix4<f64>) -> Point3<f64> {
    Point3::new(matrix.row(0)[0] / matrix.row(3)[0], matrix.row(1)[0] / matrix.row(3)[0], matrix.row(2)[0] / matrix.row(3)[0])
}


pub fn matricize(point: Point3<f64>) -> Matrix4<f64> {
    Matrix4::new(point.x, 0.0, 0.0, 0.0,
                 point.y, 0.0, 0.0, 0.0,
                 point.z, 0.0, 0.0, 0.0,
                 0.0, 0.0, 0.0, 0.0)
}
