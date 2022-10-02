pub const N: usize = 32;
pub const W: usize = N * 2;
pub const H: usize = N;
pub const MAX_ITERATIONS: i32 = N as i32;
pub const EPS: f64 = 0.001;
// https://en.wikipedia.org/wiki/Phong_reflection_model
pub const AMBIENT_REFLECTION_CONSTANT: f64 = 0.1;
pub const AMBIENT_LIGHT_INTENSITY: f64 = 255.0;
pub const DIFFUSE_REFLECTION_CONSTANT: f64 = 0.9;
pub const DIFFUSE_LIGHT_INTENSITY: f64 = 255.0;
// SPECULAR_REFLECTION_CONSTANT = 0.1
// MATERIAL_SHININESS_CONSTANT = 0.1

// ASCII (old set of characters), these are my own
// ASCII    = []byte{' ', '.', ',', '-', '+', '*', 'o', '0', '@', '#'}
// ASCII (new set of characters) taken from https://www.a1k0n.net/2011/07/20/donut-math.html
//           output[xp, yp] = ".,-~:;=!*#$@"[luminance_index];
pub const ASCII: [char; 13] = [
    ' ', '.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@',
];
pub type Buffer = [[u8; W + 1]; H]; // = [[0; W + 1]; H];

pub fn new_buffer() -> Buffer {
    [[0; W + 1]; H]
}

pub const ASCII_LEN: usize = ASCII.len(); // (old set has length 10)

pub static mut GLOBAL_BUFFER: Buffer = [[0; W + 1]; H];
