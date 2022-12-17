use indexmap::IndexMap;
use std::{env, time::Instant};

mod days;
mod util;

type PartFn = fn(String) -> String;

fn main() {
    let fns: IndexMap<usize, [PartFn; 2]> = IndexMap::from([
        (1, [days::day1::part1, days::day1::part2]),
        (2, [days::day2::part1, days::day2::part2]),
        (3, [days::day3::part1, days::day3::part2]),
        (4, [days::day4::part1, days::day4::part2]),
        (5, [days::day5::part1, days::day5::part2]),
        (6, [days::day6::part1, days::day6::part2]),
        (7, [days::day7::part1, days::day7::part2]),
        (8, [days::day8::part1, days::day8::part2]),
        (9, [days::day9::part1, days::day9::part2]),
        (10, [days::day10::part1, days::day10::part2]),
        (11, [days::day11::part1, days::day11::part2]),
        (12, [days::day12::part1, days::day12::part2]),
        (13, [days::day13::part1, days::day13::part2]),
        (14, [days::day14::part1, days::day14::part2]),
    ]);
    process_args(fns);
}

fn process_args(fns: IndexMap<usize, [PartFn; 2]>) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        fns.iter().for_each(|(day, dfn)| run_specific(*dfn, *day));
        return;
    }
    let day = match args[1].parse::<usize>() {
        Ok(day) => day,
        Err(_) => panic!("incorrect syntax"),
    };
    if args.len() < 3 {
        match fns.get(&day) {
            Some(dfn) => run_specific(*dfn, day),
            None => panic!("Day not implemented!"),
        }
    }
}

fn run_specific(fns: [PartFn; 2], day: usize) {
    println!("{} Day: {} {}", "🎄", day, "🎄");
    let input = util::get_from_file(day);
    let inputp2 = input.clone();
    let part1_start = Instant::now();
    println!("\t🌟 Part{}:\t{}", 1, fns[0](input));
    let part1_duration = part1_start.elapsed();
    let part2_start = Instant::now();
    println!("\t🌟🌟 Part{}:\t{}", 2, fns[1](inputp2));
    let part2_duration = part2_start.elapsed();
    println!(
        "\tCompleted in {}\t(p1:{}, p2:{})",
        util::format_duration(part1_duration + part2_duration),
        util::format_duration(part1_duration),
        util::format_duration(part2_duration)
    );
}
