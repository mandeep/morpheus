extern crate image;

use std::fs::File;


fn main() {
    let image = image::ImageBuffer::new(512, 512);
    let ref mut fout = File::create("output.png").unwrap();
    image::ImageLuma8(image).save(fout, image::PNG).unwrap();
}
