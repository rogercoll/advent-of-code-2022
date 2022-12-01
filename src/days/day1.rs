pub fn part1(lines: Vec<String>) -> String {
    let (_, max): (i32, i32) = lines.iter().fold((0, 0), |(mut curr, mut max), cal| {
        if cal.is_empty() {
            max = std::cmp::max(curr, max);
            curr = 0;
        } else {
            curr += cal.parse::<i32>().unwrap();
        }
        (curr, max)
    });
    max.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let (_, max): (i32, [i32; 3]) =
        lines
            .iter()
            .fold((0, [0, 0, 0]), |(mut curr, mut maxs), cal| {
                if cal.is_empty() {
                    let (min_in_max, _) = maxs
                        .iter()
                        .enumerate()
                        .min_by_key(|&(_, v)| v)
                        .expect("slice not empty");
                    maxs[min_in_max] = std::cmp::max(curr, maxs[min_in_max]);
                    curr = 0;
                } else {
                    curr += cal.parse::<i32>().unwrap();
                }
                (curr, maxs)
            });
    max.iter().sum::<i32>().to_string()
}
