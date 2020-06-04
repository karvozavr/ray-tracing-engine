mod geometry;
mod color;
mod ray;

use geometry::vec3::Vec3;
use color::Color;

fn main() {
    let image_width = 256usize;
    let image_height = 256usize;

    let v: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    eprintln!("Testing: {:?}", v * 7.5 - Vec3::one());
    print!("P3\n{width} {height}\n255\n", width = image_width, height = image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color = Color::rgb(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            println!("{}", pixel_color.format());
        }
    }

    eprint!("\nDone!\n");
}
