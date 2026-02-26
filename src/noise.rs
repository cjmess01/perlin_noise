// Public noise function
pub fn noise(x: f64, y: f64, seed: u32) -> Result<f64, String> {
    let a = (x.floor(), y.floor());
    let b = (x.ceil(), y.floor());
    let c = (x.floor(), y.ceil());
    let d = (x.ceil(), y.ceil());

    // Because I am doing copies as i32, the tuple values aren't actually moved
    let va = grad_dir_16(a.0 as i32, a.1 as i32, seed);
    let vb = grad_dir_16(b.0 as i32, b.1 as i32, seed);
    let vc = grad_dir_16(c.0 as i32, c.1 as i32, seed);
    let vd = grad_dir_16(d.0 as i32, d.1 as i32, seed);

    let da = distance(x, y, a.0, a.1);
    let db = distance(x, y, b.0, b.1);
    let dc = distance(x, y, c.0, c.1);
    let dd = distance(x, y, d.0, d.1);

    // Influences
    let ia = dot_product(da, va);
    let ib = dot_product(db, vb);
    let ic = dot_product(dc, vc);
    let id = dot_product(dd, vd);

    let (u, v) = fade_coordinates((x, y));

    let x_lerp1 = lerp(ia, ib, u);
    let x_lerp2 = lerp(ic, id, u);
    let y_lerp = lerp(x_lerp1, x_lerp2, v);
    Ok(y_lerp)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}
// Fades coordinate for lerping
fn fade_coordinates(point: (f64, f64)) -> (f64, f64) {
    fn fade_scaler(a: f64) -> f64 {
        (6.0 * a.powf(5.0)) - (15.0 * a.powf(4.0)) + (10.0 * a.powf(3.0))
    }

    let u = fade_scaler(point.0 - point.0.floor());
    let v = fade_scaler(point.1 - point.1.floor());
    (u, v)
}

// Helper function to compute dot product
fn dot_product(dist: (f64, f64), vpoint: (f64, f64)) -> f64 {
    dist.0 * vpoint.0 + dist.1 * vpoint.1
}

// Helper function to compute distance between points
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    let x_diff = x1 - x2;
    let y_diff = y1 - y2;
    (x_diff, y_diff)
}

// given a vertex meet, return a gradient
fn grad_dir_16(i: i32, j: i32, seed: u32) -> (f64, f64) {
    GRAD16[grad_index_16(i, j, seed) as usize]
}

const GRAD16: [(f64, f64); 16] = [
    (1.0, 0.0),         // E
    (0.707, 0.707),     // NE
    (0.0, 1.0),         // N
    (-0.707, 0.707),    // NW
    (-1.0, 0.0),        // W
    (-0.707, -0.707),   // SW
    (0.0, -1.0),        // S
    (0.707, -0.707),    // SE
    (0.9239, 0.3827),   // ENE
    (-0.9239, 0.3827),  // WNW
    (-0.9239, -0.3827), // WSW
    (0.9239, -0.3827),  // ESE
    (0.3827, 0.9239),   // NNE
    (-0.3827, 0.9239),  // NNW
    (-0.3827, -0.9239), // SSW
    (0.3827, -0.9239),  // SSE
];

fn mix32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

fn grad_index_16(i: i32, j: i32, seed: u32) -> u8 {
    let iu = i as u32;
    let ju = j as u32;
    let h = mix32(iu.wrapping_mul(0x9E37_79B9) ^ ju.wrapping_mul(0x85EB_CA6B) ^ seed);
    (h & 15) as u8 // 0..15
}
