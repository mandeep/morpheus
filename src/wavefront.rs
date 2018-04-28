extern crate nalgebra:

use nalgebra::core::Vector3;


struct WaveFront {
    vertices: Vec<Vector3<f64>>,
    faces: Vec<Vec<i32>>
}


impl WaveFront {
    fn new(filename: &str) -> WaveFront {
        let file = BufReader::new(File::open(filename).unwrap());
        let mut vertices = Vec<Vector3<f64>> = Vec::new();
        let mut faces = Vec<Vec<i32>> = Vec::new();
    }
}
