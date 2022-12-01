use std::collections::HashMap;
use std::{env, time::Instant};

mod days;
mod util;

fn main() {
    let fns: HashMap<usize, [fn(Vec<String>) -> String; 2]> =
        HashMap::from([(1, [days::day1::part1, days::day1::part2])]);
    process_args(fns);
}

fn process_args(fns: HashMap<usize, [fn(Vec<String>) -> String; 2]>) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // TODO: info message to provide day and/or part
        return;
    }
    let day = match args[1].parse::<usize>() {
        Ok(day) => day,
        Err(_) => panic!("incorrect syntax"),
    };
    if args.len() < 3 {
        match fns.get(&day) {
            Some(dfn) => return run_specific(*dfn, day),
            None => panic!("Day not implemented!"),
        }
    }
}

fn run_specific(fns: [fn(Vec<String>) -> String; 2], day: usize) {
    let input = util::get_from_file(day);
    let inputp2 = input.clone();
    let part1_start = Instant::now();
    println!("day{}part{}:\t{}", day, 1, fns[0](input));
    let part1_duration = part1_start.elapsed();
    let part2_start = Instant::now();
    println!("day{}part{}:\t{}", day, 2, fns[1](inputp2));
    let part2_duration = part2_start.elapsed();
    println!(
        "Completed in {}\t(p1:{}, p2:{})",
        util::format_duration(part1_duration + part2_duration),
        util::format_duration(part1_duration),
        util::format_duration(part2_duration)
    );
}
