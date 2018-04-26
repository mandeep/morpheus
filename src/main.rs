extern crate image;

use std::fs::File;
use std::mem::swap;


fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, buffer: &mut image::RgbImage, color: image::Rgb<u8>) -> () {
    let mut steep = false;
    
    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
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

    for x in x0..x1+1 {
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
    let mut buffer = image::ImageBuffer::new(512, 512);
    line(0, 0, 511, 511, &mut buffer, image::Rgb([255, 255, 255]));
    line(13, 20, 400, 200, &mut buffer, image::Rgb([255, 0, 0])); 
    line(20, 13, 300, 100, &mut buffer, image::Rgb([0, 255, 0])); 
    line(80, 40, 130, 200, &mut buffer, image::Rgb([0, 0, 255]));
    let ref mut fout = File::create("output.png").unwrap();
    image::ImageRgb8(buffer).flipv()
                            .save(fout, image::PNG)
                            .unwrap();
}
