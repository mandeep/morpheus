extern crate nalgebra:

use nalgebra::core::Vector3;


struct WaveFront {
    vertices: Vector3<f64>,
    faces: Vec<Vec<i32>>
}
