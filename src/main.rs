extern crate image;

use std::fs::File;


fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut image::RgbImage) -> () {
    for t in 0..10000 {
        let t = t as f32 * 0.0001;
        let x = x0 as f32 * (1.0 - t) + x1 as f32 * t;
        let y = y0 as f32 * (1.0 - t) + y1 as f32 * t;
        image.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
    }
}

fn main() {
    let mut buffer = image::ImageBuffer::new(512, 512);
    line(0, 0, 512, 512, &mut buffer);
    let ref mut fout = File::create("output.png").unwrap();
    image::ImageRgb8(buffer).flipv().save(fout, image::PNG).unwrap();
}
