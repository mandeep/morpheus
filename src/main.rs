extern crate image;

use std::fs::File;


fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, buffer: &mut image::RgbImage) -> () {
    let mut steep = false;
    
    if (x0 - x1).abs() < (y0 - y1).abs() {
        let x0temp = x0.clone();
        x0 = y0;
        y0 = x0temp;

        let x1temp = x1.clone();
        x1 = y1;
        y1 = x1temp;

        steep = true;
    }
    
    if x0 > x1 {
        let x0temp = x0.clone();
        x0 = x1;
        x1 = x0temp;

        let y0temp = y0.clone();
        y0 = y1;
        y1 = y0temp;
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut y = y0;
    let derror = (dy * 2).abs();
    let mut error = 0;

    for x in x0..x1+1 {
        if steep {
            buffer.put_pixel(y as u32, x as u32, image::Rgb([255, 255, 255]));
        } else {
            buffer.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
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
    line(0, 0, 511, 511, &mut buffer);
    line(13, 20, 400, 200, &mut buffer); 
    line(20, 13, 300, 100, &mut buffer); 
    line(80, 40, 130, 200, &mut buffer);
    let ref mut fout = File::create("output.png").unwrap();
    image::ImageRgb8(buffer).flipv()
                            .save(fout, image::PNG)
                            .unwrap();
}
