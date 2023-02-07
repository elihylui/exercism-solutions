// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as f64;
    if speed >= 1.0 && speed <= 4.0 {
        speed * 221.0
    } else if speed >= 5.0 && speed <= 8.0 {
        speed * 221.0 * 0.9
    } else {
        speed * 221.0 * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    //let speed = speed as u32;
    let minute = 60 as u32;
    let per_hour = production_rate_per_hour(speed) as u32;
    per_hour / minute
    //unimplemented!("calculate the amount of working items at speed: {}", speed)
}
