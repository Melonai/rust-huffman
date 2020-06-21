use std::time::Instant;

pub fn log_time(start: Instant, message: &str) {
    println!("{}: {}", message, start.elapsed().as_secs_f32());
}

pub fn booleans_to_u8(booleans: Vec<bool>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for byte in booleans.chunks(8) {
        result.push(byte.iter().fold(0, |acc, b| acc << 1 | (*b as u8)));
    }
    result
}

pub fn bit_at(byte: u8, bit_index: u8) -> bool {
    byte & (1 << (7 - bit_index)) != 0
}
