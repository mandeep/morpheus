extern crate nalgebra;

use nalgebra::core::{Vector2, Vector3, Vector4};


/// Project 3D coordinates into 2D by dividing the x and y coordinate by the z coordinate
///
/// # Examples
///
/// ```
/// let matrix: Vector3<f64> = Vector3::new(2.0, 4.0, 2.0);
/// let point: Vector2<f64> = project_to_2d(matrix);
/// assert!(point.x == 1.0 && point.y == 2.0);
/// ```
///
pub fn project_to_2d(vector: Vector3<f64>) -> Vector2<f64> {
    Vector2::new(vector.x / vector.z, vector.y / vector.z)
}


/// Project 4D coordinates into 3D by dividing the x, y, and z coordinate by the w coordinate
///
/// # Examples
///
/// ```
/// let matrix: Vector4<f64> = Vector4::new(2.0, 4.0, 6.0, 2.0);
/// let point: Vector3<f64> = project_to_3d(matrix);
/// assert!(point.x == 1.0 && point.y == 2.0 && point.z == 3.0);
/// ```
///
pub fn project_to_3d(vector: Vector4<f64>) -> Vector3<f64> {
    Vector3::new(vector.x / vector.w, vector.y / vector.w, vector.z / vector.w)
}


/// Create a 3D vector from a 2D point by setting the z coordinate to 1.0
///
/// # Examples
///
/// ```
/// let point: Vector2<f64> = Vector2::new(1.0, 2.0);
/// let vector: Vector3<f64> = vectorize_to_3d(point);
/// assert!(vector.x == 1.0 && vector.y == 2.0 && vector.z == 1.0);
/// ```
///
pub fn vectorize_to_3d(point: Vector2<f64>) -> Vector3<f64> {
    Vector3::new(point.x, point.y, 1.0)
}


/// Create a 4D vector from a 3D point by setting the w coordinate to 1.0
///
/// # Examples
///
/// ```
/// let point: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
/// let vector: Vector4<f64> = vectorize_to_4d(point);
/// assert!(vector.x == 1.0 && vector.y == 2.0 && vector.z == 3.0 && vector.w == 1.0);
/// ```
///
pub fn vectorize_to_4d(point: Vector3<f64>) -> Vector4<f64> {
    Vector4::new(point.x, point.y, point.z, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_2d() {
        let matrix: Vector3<f64> = Vector3::new(2.0, 4.0, 2.0);
        let point: Vector2<f64> = project_to_2d(matrix);
        assert!(point.x == 1.0 && point.y == 2.0);
    }

    #[test]
    fn test_project_3d() {
        let matrix: Vector4<f64> = Vector4::new(2.0, 4.0, 6.0, 2.0);
        let point: Vector3<f64> = project_to_3d(matrix);
        assert!(point.x == 1.0 && point.y == 2.0 && point.z == 3.0);
    }

    #[test]
    fn test_vectorize_3d() {
        let point: Vector2<f64> = Vector2::new(1.0, 2.0);
        let vector: Vector3<f64> = vectorize_to_3d(point);
        assert!(vector.x == 1.0 && vector.y == 2.0 && vector.z == 1.0);
    }

    #[test]
    fn test_vectorize_4d() {
        let point: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let vector: Vector4<f64> = vectorize_to_4d(point);
        assert!(vector.x == 1.0 && vector.y == 2.0 && vector.z == 3.0 && vector.w == 1.0);
    }
}
