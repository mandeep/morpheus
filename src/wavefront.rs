extern crate nalgebra;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use nalgebra::core::Vector3;


pub struct Object {
    pub vertices: Vec<Vector3<f64>>,
    pub faces: Vec<Vec<i32>>
}


impl Object {
    pub fn new(filename: &str) -> Object {
        let file = BufReader::new(File::open(filename).unwrap());
        let mut vertices: Vec<Vector3<f64>> = Vec::new();
        let mut faces: Vec<Vec<i32>> = Vec::new();

        for line in file.lines().map(|l| l.unwrap()) {
            if line.starts_with("v ") {
                let v_coordinates = line.split_at(1).1
                                      .split_whitespace()
                                      .map(|n| n.parse().unwrap())
                                      .collect::<Vec<f64>>();

                vertices.push(Vector3::new(v_coordinates[0], v_coordinates[1], v_coordinates[2]));
            }
            else if line.starts_with("f ") {
                let f_coordinates = line.split_at(1).1
                                        .split_whitespace()
                                        .collect::<Vec<&str>>();
                                        
                let x = f_coordinates[0].split("/").map(|n| n.parse().unwrap()).nth(0).unwrap();
                let y = f_coordinates[1].split("/").map(|n| n.parse().unwrap()).nth(0).unwrap();
                let z = f_coordinates[2].split("/").map(|n| n.parse().unwrap()).nth(0).unwrap();

                faces.push(vec![x, y, z])
            }
        }
        Object { vertices: vertices, faces: faces }
    }
}
