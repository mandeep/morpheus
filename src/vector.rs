extern crate image;
extern crate nalgebra;

use nalgebra::geometry::Point3;
use nalgebra::core::Matrix4;


pub fn vectorize(matrix: Matrix4<f64>) -> Point3<f64> {
    Point3::new(matrix.row(0)[0] / matrix.row(3)[0], matrix.row(1)[0] / matrix.row(3)[0], matrix.row(2)[0] / matrix.row(3)[0])
}


pub fn matricize(point: Point3<f64>) -> Matrix4<f64> {
    let mut matrix: Matrix4<f64> = Matrix4::identity();
    matrix.row_mut(0)[0] = point.x;
    matrix.row_mut(1)[0] = point.y;
    matrix.row_mut(2)[0] = point.y;

    matrix
}
