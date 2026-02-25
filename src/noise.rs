pub fn noise(x: f64, y: f64) -> Result<f64, String> {
    Ok(5.0)
}

const GRAD8: [(f32, f32); 8] = [
    (1.0, 0.0),       // E
    (0.707, 0.707),   // NE
    (0.0, 1.0),       // N
    (-0.707, 0.707),  // NW
    (-1.0, 0.0),      // W
    (-0.707, -0.707), // SW
    (0.0, -1.0),      // S
    (0.707, -0.707),  // SE
];

fn mix32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

fn grad_index_8(i: i32, j: i32, seed: u32) -> u8 {
    let iu = i as u32;
    let ju = j as u32;
    let h = mix32(iu.wrapping_mul(0x9E37_79B9) ^ ju.wrapping_mul(0x85EB_CA6B) ^ seed);
    (h & 7) as u8 // 0..7
}

// given a vertex meet, return a gradient
pub fn grad_dir_8(i: i32, j: i32, seed: u32) -> (f32, f32) {
    GRAD8[grad_index_8(i, j, seed) as usize]
}
