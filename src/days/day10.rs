pub fn part1(input: String) -> String {
    input
        .split('\n')
        .fold((0_i16, 1_i16, 0_i16), |(cycles, x, mut strength), line| {
            let (instruction_cycles, value) = match &line[0..4] {
                "noop" => (1, 0),
                "addx" => (2, line[5..].parse::<i16>().unwrap()),
                _ => panic!("Invalid instruction"),
            };
            (1..=instruction_cycles).for_each(|i| {
                strength += ((cycles + i - 20) % 40 == 0) as i16 * (cycles + i) * x;
            });
            (cycles + instruction_cycles, x + value, strength)
        })
        .2
        .to_string()
}

pub fn part2(input: String) -> String {
    let result = input
        .split('\n')
        .fold((1_i16, Vec::new()), |(mut x, mut pixels), line| {
            let (instruction_cycles, value) = match &line[0..4] {
                "noop" => (1, 0),
                "addx" => (2, line[5..].parse::<i16>().unwrap()),
                _ => panic!("Invalid instruction"),
            };
            (0..instruction_cycles).for_each(|_| {
                match (pixels.len() as i16 % 40) - x {
                    0 | 1 | -1 => pixels.push('#'),
                    _ => pixels.push('.'),
                };
            });
            x += value;
            (x, pixels)
        })
        .1
        .chunks(40)
        .map(|chunk| chunk.iter().map(|s| s.to_string()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    "\n".to_string() + &result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!("0", part1("noop\naddx 3\naddx -5".to_string()));
        assert_eq!(
            "13140",
            part1(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
                    .to_string()
            )
        );
    }
}
