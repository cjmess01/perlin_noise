use std::env;

mod noise;
use image::{ImageBuffer, Rgb};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(args)?;

    // This creates a png and fills it with the info
    let mut img = ImageBuffer::new(config.width as u32, config.height as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let nx = (x as f64 / config.width as f64) * config.frequency;
        let ny = (y as f64 / config.height as f64) * config.frequency;

        let r_height = noise::noise(nx, ny, config.seed)?;
        let g_height = noise::noise(nx, ny, config.seed + 1)?;
        let b_height = noise::noise(nx, ny, config.seed + 2)?;

        let r_height = map_noise_to_u8(r_height);
        let g_height = map_noise_to_u8(g_height);
        let b_height = map_noise_to_u8(b_height);

        *pixel = Rgb([r_height, g_height, b_height]);
    }
    img.save("output.png").unwrap();

    Ok(())
}

fn parse_arguments(args: Vec<String>) -> Result<Config, String> {
    if args.len() < 5 {
        return Err("Not enough arguments".into());
    }

    let width = args[1].parse().map_err(|_| "Bad width")?;
    let height = args[2].parse().map_err(|_| "Bad height")?;
    let frequency = args[3].parse().map_err(|_| "Bad frequency")?;
    let seed = args[4].parse().map_err(|_| "Bad seed")?;

    Ok(Config {
        width: width,
        height: height,
        frequency: frequency,
        seed: seed,
    })
}
fn map_noise_to_u8(x: f64) -> u8 {
    let clamped = x.clamp(-1.0, 1.0);
    let normalized = (clamped + 1.0) * 0.5;
    (normalized * 255.0).round() as u8
}
struct Config {
    width: usize,
    height: usize,
    frequency: f64,
    seed: u32,
}
