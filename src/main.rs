mod geometry;
mod color;
mod ray;

use geometry::vec3::Vec3;
use color::Color;
use crate::geometry::vec3::Point3;
use crate::ray::Ray;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384_usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    print!("P3\n{width} {height}\n255\n", width = image_width, height = image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&ray);
            print!("{}\n", pixel_color.format());
        }
    }

    eprint!("\nDone!\n");
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0); // normalize y coordinate

    let white = Color::rgb(1.0, 1.0, 1.0);
    let light_blue = Color::rgb(0.5, 0.7, 1.0);

    // lerp between white and blue depending on y coordinate of the ray/viewport intersection point
    (1.0 - t) * white + t * light_blue
}