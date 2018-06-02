#![allow(dead_code)]
extern crate image;
extern crate nalgebra;

use std::mem::swap;

use nalgebra::core::{Matrix4, Vector3};
use nalgebra::geometry::{Point2, Point3};

use wavefront;
use vector;


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
fn fill_triangle(mut t0: Point2<i32>, mut t1: Point2<i32>, mut t2: Point2<i32>, buffer: &mut image::RgbImage, color: image::Rgb<u8>) {
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
        let beta = if second_half { (i as f64 - (t1.y - t0.y) as f64) / segment_height as f64} else {i as f64 / segment_height as f64};

        let mut a = t0.x as f64 + ((t2 - t0).x as f64 * alpha);
        let mut b = if second_half {t1.x as f64 + ((t2 - t1).x as f64 * beta)} else {t0.x as f64 + ((t1 - t0).x as f64 * beta)};

        if a > b {
            swap(&mut a, &mut b);
        }

        for j in (a as u32)..=(b as u32) {
            buffer.put_pixel(j, t0.y as u32 + i as u32, color);
        }
    }
}

/// Change the frame of reference of the viewer
///
/// The eye vector is where the camera is situated. The center
/// vector is where the camera should point in relation to
/// the up vector which is vertical when rendered.
///
fn lookat(eye: &Vector3<f64>, center: &Vector3<f64>, up: &Vector3<f64>) -> Matrix4<f64> {
    let z = (eye - center).normalize();
    let x = up.cross(&z).normalize();
    let y = z.cross(&x).normalize();

    let mut matrix: Matrix4<f64> = Matrix4::identity();
    let mut transpose: Matrix4<f64> = Matrix4::identity();

    for i in 0..3 {
        matrix.row_mut(0)[i] = x[i];
        matrix.row_mut(1)[i] = y[i];
        matrix.row_mut(2)[i] = z[i];
        transpose.row_mut(i)[3] = -center[i];
    }

    matrix * transpose
}

/// Map the bi-unit cube of [-1, 1] * [-1, 1] * [-1, 1] to the dimensions of the image
///
/// The x and y parameters specify the origin of the viewport while the
/// width and height parameters specify the width and height of the viewport.
///
fn viewport(x: u32, y: u32, width: u32, height: u32, depth: u32) -> Matrix4<f64> {
    let mut matrix = Matrix4::identity();

    matrix.row_mut(0)[3] = x as f64 + width as f64 / 2.0;
    matrix.row_mut(1)[3] = y as f64 + height as f64 / 2.0;
    matrix.row_mut(2)[3] = depth as f64 / 2.0;

    matrix.row_mut(0)[0] = width as f64 / 2.0;
    matrix.row_mut(1)[1] = height as f64 / 2.0;
    matrix.row_mut(2)[2] = depth as f64 / 2.0;

    matrix
}

/// Find the barycentric coordinates of the given point with respect to the given triangle
///
/// # Examples
///
/// ```
/// let points =  vec![Vector3::new(0, 0, 0), Vector3::new(2, 2, 2), Vector3::new(0, 2, 2)]
/// let point = Point3::new(1.0, 1.0, 0.0);
/// let barycentric_coordinates: Point3<f64> = find_barycentric(&points, &point);
/// ```
///
fn find_barycentric(points: &Vec<Point3<f64>>, point: &Point3<f64>) -> Point3<f64> {
    let u = Vector3::new(points[2].x - points[0].x, points[1].x - points[0].x, points[0].x - point.x);
    let v = Vector3::new(points[2].y - points[0].y, points[1].y - points[0].y, points[0].y - point.y);

    let w = u.cross(&v);

    if (w.z).abs() < 1.0 {
        return Point3::new(-1.0, 1.0, 1.0);
    } else {
        return Point3::new(1.0 - (w.x + w.y) as f64 / w.z as f64, w.y as f64 / w.z as f64, w.x as f64 / w.z as f64);
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
fn draw_triangle(points: &Vec<Point3<f64>>, buffer: &mut image::RgbImage, zbuffer: &mut Vec<f64>, color: image::Rgb<u8>) {
    let mut bounding_box_minimum: Point2<f64> = Point2::new(buffer.width() as f64 - 1.0, buffer.height() as f64 - 1.0);
    let mut bounding_box_maximum: Point2<f64> = Point2::new(0.0, 0.0);

    for point in points {
        bounding_box_minimum.x = bounding_box_minimum.x.min(point.x);
        bounding_box_minimum.y = bounding_box_minimum.y.min(point.y);
        bounding_box_maximum.x = bounding_box_maximum.x.max(point.x);
        bounding_box_maximum.y = bounding_box_maximum.y.max(point.y);
    }

    for x in bounding_box_minimum.x as i32 ..= bounding_box_maximum.x as i32 {
        for y in bounding_box_minimum.y as i32 ..= bounding_box_maximum.y as i32 {
            let mut point = Point3::new(x as f64, y as f64, 0.0);
            let barycentric_coordinates: Point3<f64> = find_barycentric(points, &point);
            if barycentric_coordinates.x >= 0.0 && barycentric_coordinates.y >= 0.0 && barycentric_coordinates.z >= 0.0 {
                for i in 0..3 {
                    point.z += points[i].z * barycentric_coordinates[i];
                }
                if zbuffer[(point.x as u32 + (point.y as u32 * buffer.width())) as usize] < point.z {
                    zbuffer[(point.x as u32 + (point.y as u32 * buffer.width())) as usize] = point.z;
                    buffer.put_pixel(point.x as u32, point.y as u32, color)
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

            draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, buffer, image::Rgb([255, 255, 255]));
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
pub fn draw_triangle_mesh(filename: &str, buffer: &mut image::RgbImage, depth: u32, light_vector: &Vector3<f64>, eye: &Vector3<f64>, center: &Vector3<f64>, up: &Vector3<f64>) {
    let coordinates = wavefront::Object::new(filename);

    let mut zbuffer = vec![-1.0; (buffer.width() * buffer.height()) as usize];

    let model_view = lookat(eye, center, up);
    let mut projection: Matrix4<f64> = Matrix4::identity();
    let view_port = viewport(buffer.width() / 8, buffer.height() / 8, buffer.width() * 3 / 4, buffer.height() * 3 / 4, depth);

    projection.row_mut(3)[2] = -1.0 / (eye - center).norm();

    for face in coordinates.geometric_faces {
        let mut screen_coordinates: Vec<Point3<f64>> = Vec::new();
        let mut world_coordinates: Vec<Point3<f64>> = Vec::new();

        for i in 0..3 {
            let world_coordinate: Point3<f64> = coordinates.geometric_vertices[(face[i] - 1) as usize];
            let screen_coordinate = vector::vectorize(view_port * projection * model_view * vector::matricize(world_coordinate));

            screen_coordinates.push(screen_coordinate);
            world_coordinates.push(world_coordinate);
        }

        let normal: Vector3<f64> = (world_coordinates[2] - world_coordinates[0]).cross(&(world_coordinates[1] - world_coordinates[0])).normalize();
        let intensity: f64 = normal.dot(&light_vector);

        draw_triangle(&screen_coordinates, buffer, &mut zbuffer, image::Rgb([(255.0 * intensity) as u8, (255.0 * intensity) as u8, (255.0 * intensity) as u8]));
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
            for y in 0..799 {
                assert_eq!(test_file.get_pixel(x, y), output_file.get_pixel(x, y));
            }
        }
    }
}
