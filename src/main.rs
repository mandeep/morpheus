extern crate image;

use std::fs::File;


fn main() {
    let image = image::DynamicImage::new_rgb8(512, 512);
    let ref mut fout = File::create("output.png").unwrap();
    image.save(fout, image::PNG).unwrap();
}
