// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const STD_CONSTRUCTION_RATE: usize = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let full_rate = (STD_CONSTRUCTION_RATE * speed as usize) as f64;

    match (speed) {
        0 => 0.0,
        1..=4 => full_rate,
        5..=8 => full_rate * 0.9,
        9..=10 => full_rate * 0.77,
        _ => panic!("Speed out of bounds"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
