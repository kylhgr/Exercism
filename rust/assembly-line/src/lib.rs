// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    //unimplemented!("calculate hourly production rate at speed: {}", speed)
    let success_rate: f64 = get_success_rate(speed);
    speed as f64 * 221.0 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
    //unimplemented!("calculate the amount of working items at speed: {}", speed)
}

pub fn get_success_rate(speed: u8) -> f64 {
    match speed {
        1..=4 => 1.00,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    }

}
