pub fn part1(input: String) -> String {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .split("\n\n")
        .fold([0, 0, 0], |mut maxs, chunk| {
            // get the position of the minimum value in the max array
            let (min_in_max, _) = maxs
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .expect("slice not empty");
            // replace the poisition of the minimum value with the max between the sum calories and
            // itself
            maxs[min_in_max] = std::cmp::max(
                chunk
                    .lines()
                    .map(|cal| cal.parse::<i32>().unwrap())
                    .sum::<i32>(),
                maxs[min_in_max],
            );
            maxs
        })
        .iter()
        .sum::<i32>()
        .to_string()
}
