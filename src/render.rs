extern crate image;
extern crate nalgebra;

use std::mem::swap;

use nalgebra::core::{Matrix2x3, Vector2, Vector3, Vector4};
use nalgebra::geometry::{Point2};

use shader;
use wavefront;
use vector;


/// Bresenham's algorithm: Draw a line in the given color from (x0, y0) to (x1, y1)
///
/// # Examples
///
/// ```
/// let mut buffer = image::ImageBuffer::new(1921, 1081);
///
/// draw_line(0, 0, 1920, 1080, &mut buffer, image::Rgb([255, 255, 255]))
/// ```
///
/// ```
/// let mut buffer = image::ImageBuffer::new(512, 512);
///
/// draw_line(0, 0, 511, 511, &mut buffer, image::Rgb([128, 0, 255]))
/// ```
fn draw_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32,
             buffer: &mut image::RgbImage, color: image::Rgb<u8>) {

    let steep = (x0 - x1).abs() < (y0 - y1).abs();

    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let delta_x = x1 - x0;
    let delta_y = y1 - y0;
    let delta_error = (delta_y * 2).abs();
    let mut y = y0;
    let mut error = 0;

    for x in x0 ..= x1 {
        if steep {
            buffer.put_pixel(y as u32, x as u32, color);
        } else {
            buffer.put_pixel(x as u32, y as u32, color);
        }
        error += delta_error;
        if error > delta_x {
            y += if y1 > y0 { 1 } else { -1 };
            error -= delta_x * 2;
        }
    }
}

/// Fill triangle in the given color with points t0, t1, and t2
///
/// # Examples
///
/// ```
/// let mut buffer = image::ImageBuffer::new(1921, 1081);
///
/// let mut t0 = Point2::new(0, 0);
/// let mut t1 = Point2::new(2, 2);
/// let mut t2 = Point2::new(0, 2);
///
/// fill_triangle(t0, t1, t2, &mut buffer, image::Rgb([255, 255, 255]))
/// ```
///
fn fill_triangle(mut t0: Point2<i32>, mut t1: Point2<i32>, mut t2: Point2<i32>,
                 buffer: &mut image::RgbImage, color: image::Rgb<u8>) {

    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
    }

    let triangle_height = t2.y - t0.y;

    for i in 0..triangle_height as i32 {
        let second_half = i > (t1.y - t0.y) as i32 || (t1.y == t0.y);
        let segment_height = if second_half {t2.y - t1.y} else {t1.y - t0.y};

        let alpha = i as f64 / triangle_height as f64;
        let beta = if second_half { (i as f64 - (t1.y - t0.y) as f64) / segment_height as f64} else
                   {i as f64 / segment_height as f64};

        let mut a = t0.x as f64 + ((t2 - t0).x as f64 * alpha);
        let mut b = if second_half {t1.x as f64 + ((t2 - t1).x as f64 * beta)} else
                    {t0.x as f64 + ((t1 - t0).x as f64 * beta)};

        if a > b {
            swap(&mut a, &mut b);
        }

        for j in (a as u32)..=(b as u32) {
            buffer.put_pixel(j, t0.y as u32 + i as u32, color);
        }
    }
}

/// Draw a filled triangle with the given points in the given color
///
/// # Examples
///
/// ```
/// let mut buffer = image::ImageBuffer::new(1921, 1081);
/// let points =  vec![Vector3::new(0, 0, 0), Vector3::new(2, 2, 2), Vector3::new(0, 2, 2)]
/// let zbuffer = vec![-1.0, -1.0, -1.0];
///
/// draw_triangle(&points, &mut buffer, &mut zbuffer, image::Rgb([255, 255, 255]))
/// ```
///
fn draw_triangle(points: &Vec<Vector4<f64>>, buffer: &mut image::RgbImage,
                 texture: &image::RgbImage, zbuffer: &mut image::GrayImage,
                 shader: shader::GouraudShader) {

    let mut bounding_box_minimum: Vector2<f64> = Vector2::new(buffer.width() as f64 - 1.0,
                                                              buffer.height() as f64 - 1.0);
    let mut bounding_box_maximum: Vector2<f64> = Vector2::new(0.0, 0.0);

    for i in 0..=2 {
        for j in 0..=1 {
            bounding_box_minimum[j] = bounding_box_minimum[j].min(points[i][j] / points[i][3]);
            bounding_box_maximum[j] = bounding_box_maximum[j].max(points[i][j] / points[i][3]);
        }
    }
    for x in bounding_box_minimum.x as i32 ..= bounding_box_maximum.x as i32 {
        for y in bounding_box_minimum.y as i32 ..= bounding_box_maximum.y as i32 {
            let mut point = Vector2::new(x as f64, y as f64);
            let mut color = image::Rgb([255, 255, 255]);

            let c: Vector3<f64> = shader::find_barycentric(
                                        vector::project_to_3d(points[0]).remove_row(2),
                                        vector::project_to_3d(points[1]).remove_row(2),
                                        vector::project_to_3d(points[2]).remove_row(2),
                                        &point);

            let z = points[0][2] * c.x + points[1][2] * c.y + points[2][2] * c.z;
            let w = points[0][3] * c.x + points[1][3] * c.y + points[2][3] * c.z;

            let fragment_depth = 0.max(255.min((z / w + 0.5) as u8));

            if c.x >= 0.0 && c.y >= 0.0 && c.z >= 0.0 &&
                zbuffer.get_pixel(point.x as u32, point.y as u32)[0] <= fragment_depth {

                let discard: bool = shader.fragment(c, &mut color, texture);
                if !discard {
                    zbuffer.put_pixel(point.x as u32, point.y as u32, image::Luma([fragment_depth]));
                    buffer.put_pixel(point.x as u32, point.y as u32, color);
                }
            }
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
/// draw_wire_mesh("coordinates.obj", &mut buffer);
/// ```
pub fn draw_wire_mesh(filename: &str, buffer: &mut image::RgbImage) {
    let coordinates = wavefront::Object::new(filename);

    for face in coordinates.geometric_faces {
        for i in 0..3 {
            let v0 = coordinates.geometric_vertices[(face[i] - 1) as usize];
            let v1 = coordinates.geometric_vertices[(face[(i+1) % 3] - 1) as usize];

            let x0 = ((v0.x + 1.0) * buffer.width() as f64 / 2.0).min(buffer.width() as f64 - 1.0);
            let y0 = ((v0.y + 1.0) * buffer.height() as f64 / 2.0).min(buffer.height() as f64 - 1.0);

            let x1 = ((v1.x + 1.0) * buffer.width() as f64 / 2.0).min(buffer.width() as f64 - 1.0);
            let y1 = ((v1.y + 1.0) * buffer.height() as f64 / 2.0).min(buffer.height() as f64 - 1.0);

            draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32,
                      buffer, image::Rgb([255, 255, 255]));
        }
    }
}

/// Draw a triangle mesh on the given ImageBuffer with the illumination provided by the given vector
///
/// # Examples
///
/// ```
/// let width = 512;
/// let height = 512;
/// let mut buffer = image::ImageBuffer::new(width, height);
/// let light_vector = Vector3::new(0.0, 0.0, -1.0).normalize();
///
/// draw_triangle_mesh("coordinates.obj", &mut buffer, light_vector);
/// ```
pub fn draw_triangle_mesh(filename: &str, buffer: &mut image::RgbImage,
                          texture: &image::RgbImage, zbuffer: &mut image::GrayImage,
                          depth: u32, light_vector: &Vector3<f64>, eye: &Vector3<f64>,
                          center: &Vector3<f64>, up: &Vector3<f64>) {

    let coordinates = wavefront::Object::new(filename);

    let model_view = shader::lookat(eye, center, up);
    let projection = shader::projection(-1.0 / (eye - center).norm());
    let view_port = shader::viewport(buffer.width() / 8, buffer.height() / 8,
                                     buffer.width() * 3 / 4, buffer.height() * 3 / 4,
                                     depth);

    for face_index in 0..coordinates.geometric_faces.len() {
        let mut shader = shader::GouraudShader{ varying_intensity: Vector3::zeros(),
                                                varying_texture: Matrix2x3::zeros() };
        let mut screen_coordinates: Vec<Vector4<f64>> = Vec::new();

        for vertex_index in 0..=2 {
            screen_coordinates.push(shader.vertex(&coordinates, &view_port, &projection,
                                                  &model_view, &light_vector,
                                                  face_index, vertex_index));
        }

        draw_triangle(&screen_coordinates, buffer, texture, zbuffer, shader);
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use super::*;

    #[test]
    fn test_draw_line() {
        let mut dir = env::temp_dir();
        dir.push("test_draw_line.png");

        let (width, height) = (1600, 1600);

        let mut buffer = image::ImageBuffer::new(width, height);

        draw_line(0, 0, 800, 800, &mut buffer, image::Rgb([255, 255, 255]));
        draw_line(137, 450, 879, 299, &mut buffer, image::Rgb([255, 255, 255]));
        draw_line(435, 532, 17, 743, &mut buffer, image::Rgb([255, 255, 255]));
        draw_line(164, 57, 1, 1045, &mut buffer, image::Rgb([255, 255, 255]));
        draw_line(1300, 45, 800, 900, &mut buffer, image::Rgb([255, 255, 255]));
        draw_line(1500, 1599, 0, 1590, &mut buffer, image::Rgb([255, 255, 255]));
        draw_line(1400, 409, 1500, 900, &mut buffer, image::Rgb([255, 255, 255]));

        let ref mut fout = File::create(&dir).unwrap();

        image::ImageRgb8(buffer).flipv()
                                .save(fout, image::PNG)
                                .unwrap();

        // test must be run in the project root directory
        let test_file = image::open("./tests/test_lines.png").unwrap().to_rgb();
        let output_file = image::open(&dir).unwrap().to_rgb();

        assert_eq!(test_file.height(), output_file.height());

        for x in 0..1599 {
            for y in 0..1599 {
                assert_eq!(test_file.get_pixel(x, y), output_file.get_pixel(x, y));
            }
        }
    }


    #[test]
    fn test_fill_triangle() {
        let mut dir = env::temp_dir();
        dir.push("test_fill_triangle.png");

        let (width, height) = (1600, 1600);

        let mut buffer = image::ImageBuffer::new(width, height);

        fill_triangle(Point2::new(0, 0), Point2::new(343, 499), Point2::new(1135, 1478),
                      &mut buffer, image::Rgb([255, 255, 255]));

        let ref mut fout = File::create(&dir).unwrap();

        image::ImageRgb8(buffer).flipv()
                                .save(fout, image::PNG)
                                .unwrap();

        let test_file = image::open("./tests/test_triangle.png").unwrap().to_rgb();
        let output_file = image::open(&dir).unwrap().to_rgb();

        assert_eq!(test_file.height(), output_file.height());

        for x in 0..1599 {
            for y in 0..1599 {
                assert_eq!(test_file.get_pixel(x, y), output_file.get_pixel(x, y));
            }
        }
    }
}
