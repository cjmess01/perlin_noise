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

    // let mut v: Vec<i32> = Vec::new();
    //
    // for number in 0..width {
    //     v.push(number as i32);
    // }
    //
    // for (i, &item) in v.iter().enumerate() {
    //     println!("{i}:{item}");
    // }

    // let mut v: Vec<Vec<f32>> = Vec::new();
    // for number in 0..width {
    //     println!("{number}");
    //     let sub_list = &mut v[number];
    //     let elem = sub_list.get(0);
    //     println!("{elem}");
    // }
}
