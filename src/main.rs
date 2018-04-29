#![allow(dead_code)]
extern crate image;
extern crate nalgebra;

use std::fs::File;
use std::mem::swap;
use nalgebra::core::Vector2;

mod wavefront;


/// Bresenham's algorithm: Draw a line in the given color from (x0, y0) to (x1, y1)
///
/// # Examples
///
/// ```
/// let mut buffer = image::ImageBuffer::new(1921, 1081);
///
/// line(0, 0, 1920, 1080, &mut buffer, image::Rgb([255, 255, 255]))
/// ```
///
/// ```
/// let mut buffer = image::ImageBuffer::new(512, 512);
///
/// line(0, 0, 511, 511, &mut buffer, image::Rgb([128, 0, 255]))
/// ```
fn draw_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, buffer: &mut image::RgbImage, color: image::Rgb<u8>) {
    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    
    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut y = y0;
    let derror = (dy * 2).abs();
    let mut error = 0;

    for x in x0..=x1 {
        if steep {
            buffer.put_pixel(y as u32, x as u32, color);
        } else {
            buffer.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > dx {
            if y1 > y0 {
                y += 1;
            } else {
                y -= 1;
            }
            error -= dx * 2;
        }
    }
}

/// Draw a wire mesh on the given ImageBuffer with the coordinates from the given file
///
/// # Examples
///
/// ```
/// let width = 512;
/// let height = 512;
/// let mut buffer = image::ImageBuffer::new(width, height);
///
/// draw_wire_mesh("coordinates.obj", &mut buffer, width, height); 
/// ```
fn draw_wire_mesh(filename: &str, buffer: &mut image::RgbImage, width: u32, height: u32) {
    let coordinates = wavefront::Object::new(filename);

    for face in coordinates.faces {
        for i in 0..3 {
            let v0 = coordinates.vertices[(face[i] - 1) as usize];
            let v1 = coordinates.vertices[(face[(i+1) % 3] - 1) as usize];
            
            let x0 = ((v0.x + 1.0) * width as f64 / 2.0).min(width as f64 - 1.0);
            let y0 = ((v0.y + 1.0) * height as f64 / 2.0).min(height as f64 - 1.0);
            
            let x1 = ((v1.x + 1.0) * width as f64 / 2.0).min(width as f64 - 1.0);
            let y1 = ((v1.y + 1.0) * height as f64 / 2.0).min(height as f64 - 1.0);
            
            draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, buffer, image::Rgb([255, 255, 255]));
        }
    }
}

fn draw_triangle(mut t0: Vector2<f64>, mut t1: Vector2<f64>, mut t2: Vector2<f64>, buffer: &mut image::RgbImage, color: image::Rgb<u8>) {
    if t0.y == t1.y && t0.y == t2.y {
        return;
    }

    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
    }

    let triangle_height: f64 = t2.y - t0.y;

    for i in 0..triangle_height as i32 {
        let second_half: bool = i > (t1.y - t0.y) as i32 || (t1.y == t0.y);
        let segment_height = if second_half {t2.y - t1.y} else {t1.y - t0.y};
        
        let alpha: f64 = i as f64 / triangle_height;
        let beta: f64 = if second_half { (i as f64 - (t1.y - t0.y)) / segment_height} else {i as f64 / segment_height};

        let mut a = t0 + ((t2 - t0) * alpha);
        let mut b = if second_half {t1 + ((t2 - t1) * beta)} else {t0 + ((t1 - t0) * beta)};

        if a.x > b.x {
            swap(&mut a, &mut b);
        }

        for j in (a.x as u32)..=(b.x as u32) {
            buffer.put_pixel(j, t0.y as u32 + i as u32, color);
        }
    }
}

fn main() {
    let (width, height) = (200, 200);
    let mut buffer = image::ImageBuffer::new(width, height);

    let t0 = vec![Vector2::new(10.0, 70.0), Vector2::new(50.0, 160.0), Vector2::new(70.0, 80.0)]; 
    let t1 = vec![Vector2::new(180.0, 50.0), Vector2::new(150.0, 1.0), Vector2::new(70.0, 180.0)]; 
    let t2 = vec![Vector2::new(180.0, 150.0), Vector2::new(120.0, 160.0), Vector2::new(130.0, 180.0)]; 

    draw_triangle(t0[0], t0[1], t0[2], &mut buffer, image::Rgb([255, 0, 0]));    
    draw_triangle(t1[0], t1[1], t1[2], &mut buffer, image::Rgb([255, 255, 255]));    
    draw_triangle(t2[0], t2[1], t2[2], &mut buffer, image::Rgb([0, 255, 0]));    

    let ref mut fout = File::create("../triangle.png").unwrap();
    image::ImageRgb8(buffer).flipv()
                            .save(fout, image::PNG)
                            .unwrap();
}
