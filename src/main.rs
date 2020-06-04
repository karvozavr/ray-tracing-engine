fn main() {
    let image_width = 256usize;
    let image_height = 256usize;

    print!("P3\n{width} {height}\n255\n", width=image_width, height=image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining: {} ", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let f256 = 255.999;
            let ir = (f256 * r) as u32;
            let ig = (f256 * g) as u32;
            let ib = (f256 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprint!("\nDone!\n");
}
