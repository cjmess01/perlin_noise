use rand::Rng;
use std::io;
fn main() {
    const MAX_WIDTH: usize = 1_000_000;
    const DEFAULT: usize = 10;

    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line");

    // if weird value, send back 10
    let width: usize = match width.trim().parse() {
        Ok(num) if num <= MAX_WIDTH => num,
        Ok(_) => {
            println!("Number is too large, using default");
            DEFAULT
        }
        Err(_) => {
            println!("Expected a number, got invalid value");
            DEFAULT
        }
    };

    let mut v: Vec<f32> = Vec::with_capacity(width);
    for _ in 0..v.capacity() {
        v.push(0.0);
    }
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
