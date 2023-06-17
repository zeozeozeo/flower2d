use std::time::{Duration, Instant};

use flower2d::Image;

fn render(image: &mut Image, _iter: u32) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set(
                x,
                y,
                (
                    (x * y * 2 % 255) as u8,
                    (x * y * 4 % 255) as u8,
                    (x * y * 8 % 255) as u8,
                    (x * y % 255) as u8,
                ),
            )
        }
    }
}

fn main() {
    const NUM_TESTS: i32 = 5;

    // allocate image
    let mut image = Image::new(1024, 1024);
    let mut times: Vec<f64> = vec![];

    // run NUM_TESTS tests
    for i in 0..NUM_TESTS {
        image.clear((0, 0, 0, 0)); // restore the image

        let start = Instant::now();
        render(&mut image, i as u32); // draw stuff
        let elapsed = start.elapsed();

        println!("Test {}/{NUM_TESTS} | {:?}", i + 1, elapsed);
        times.push(elapsed.as_secs_f64())
    }

    let total: f64 = times.iter().sum();
    println!(
        "Average  | {:?}",
        Duration::from_secs_f64(total / NUM_TESTS as f64)
    );

    image.save_image("benchmark.png").unwrap();
}
