use std::{fs, time::Duration};

// get_from_file returns an string with whole input file
pub fn get_from_file(day: usize) -> String {
    fs::read_to_string(format!("./inputs/day{}.txt", day))
        .unwrap()
        .trim()
        .to_string()
}

pub fn format_duration(d: Duration) -> String {
    if d.as_millis() > 1000 {
        return format!("{:.1}s", d.as_millis() as f64 / 1000 as f64);
    }
    if d.as_micros() > 1000 {
        return format!("{:.1}ms", d.as_micros() as f64 / 1000 as f64);
    }
    if d.as_nanos() > 1000 {
        return format!("{:.1}μs", d.as_nanos() as f64 / 1000 as f64);
    }
    format!("{}ns", d.as_nanos())
}
