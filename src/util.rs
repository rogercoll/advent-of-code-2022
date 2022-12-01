use std::{fs, time::Duration};

pub fn get_from_file(day: usize) -> Vec<String> {
    get_from_filename(format!("./inputs/day{}.txt", day))
}

fn get_from_filename(filename: String) -> Vec<String> {
    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        // .filter(|x: &&str| x.trim() != "")
        .map(|x| String::from(x.trim()))
        .collect();
    lines
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
