extern crate image;

use std::fs::File;


fn main() {
    let buffer = image::ImageBuffer::new(512, 512);
    let ref mut fout = File::create("output.png").unwrap();
    image::ImageRgba8(buffer).save(fout, image::PNG).unwrap();
}
