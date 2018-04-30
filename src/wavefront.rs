extern crate nalgebra;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use nalgebra::core::{Vector2, Vector3};


pub struct Object {
    pub geometric_vertices: Vec<Vector3<f64>>,
    pub geometric_faces: Vec<Vec<i32>>,
    pub texture_vertices: Vec<Vector2<f64>>,
    pub texture_faces: Vec<Vec<i32>>
}


impl Object {
    pub fn new(filename: &str) -> Object {
        let file = BufReader::new(File::open(filename).unwrap());
        let mut geometric_vertices: Vec<Vector3<f64>> = Vec::new();
        let mut geometric_faces: Vec<Vec<i32>> = Vec::new();
        let mut texture_vertices: Vec<Vector2<f64>> = Vec::new();
        let mut texture_faces: Vec<Vec<i32>> = Vec::new();

        for line in file.lines().map(|l| l.unwrap()) {
            if line.starts_with("v ") {
                let v_coordinates = line.split_at(2).1
                                      .split_whitespace()
                                      .map(|n| n.parse().unwrap())
                                      .collect::<Vec<f64>>();

                geometric_vertices.push(Vector3::new(v_coordinates[0], v_coordinates[1], v_coordinates[2]));
            }
            else if line.starts_with("vt ") {
                let vt_coordinates = line.split_at(3).1
                                         .split_whitespace()
                                         .map(|n| n.parse().unwrap())
                                         .collect::<Vec<f64>>();

                texture_vertices.push(Vector2::new(vt_coordinates[0], vt_coordinates[1]));
            }
            else if line.starts_with("f ") {            
                let f_coordinates = line.split_at(2).1
                                        .split(|c| c == '/' || c == ' ')
                                        .map(|n| n.parse().unwrap())
                                        .collect::<Vec<i32>>();
    
                geometric_faces.push(vec![f_coordinates[0], f_coordinates[3], f_coordinates[6]]);
                texture_faces.push(vec![f_coordinates[1], f_coordinates[4], f_coordinates[7]]);
            }
        }
        Object { geometric_vertices: geometric_vertices, geometric_faces: geometric_faces,
                 texture_vertices: texture_vertices, texture_faces: texture_faces }
    }
}
