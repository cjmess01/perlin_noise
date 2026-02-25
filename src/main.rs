use std::env;

mod noise;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(args)?;

    let val = noise::noise(0.0, 0.0);

    Ok(())
}

fn parse_arguments(args: Vec<String>) -> Result<Config, String> {
    if args.len() < 4 {
        return Err("Not enough arguments".into());
    }

    let width = args[1].parse().map_err(|_| "Bad width")?;
    let height = args[2].parse().map_err(|_| "Bad height")?;
    let frequency = args[3].parse().map_err(|_| "Bad frequency")?;

    for i in 0..5 {
        for j in 0..5 {
            let val = noise::grad_dir_8(i, j, 0);
            println!("{} {}", val.0, val.1);
        }
    }

    Ok(Config {
        width: width,
        height: height,
        frequency: frequency,
    })
}
struct Config {
    width: usize,
    height: usize,
    frequency: f64,
}
