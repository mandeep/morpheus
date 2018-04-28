extern crate image;
extern crate nalgebra;

use std::fs::File;
use std::mem::swap;

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
///
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

fn main() {
    let width = 1600;
    let height = 1600;
    let mut buffer = image::ImageBuffer::new(width, height);
    let coordinates = wavefront::Object::new("head.obj");

    for face in coordinates.faces {
        for i in 0..3 {
            let v0 = coordinates.vertices[(face[i] - 1) as usize];
            let v1 = coordinates.vertices[(face[(i+1) % 3] - 1) as usize];
            let x0 = ((v0.x + 1.0) * width as f64 / 2.0).min(width as f64 - 1.0);
            let y0 = ((v0.y + 1.0) * height as f64 / 2.0).min(height as f64 - 1.0);
            let x1 = ((v1.x + 1.0) * width as f64 / 2.0).min(width as f64 - 1.0);
            let y1 = ((v1.y + 1.0) * height as f64 / 2.0).min(height as f64 - 1.0);
            draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, &mut buffer, image::Rgb([255, 255, 255]));
        }
    }

    let ref mut fout = File::create("../output.png").unwrap();
    image::ImageRgb8(buffer).flipv()
                            .save(fout, image::PNG)
                            .unwrap();
}
