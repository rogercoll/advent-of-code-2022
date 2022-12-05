fn parse_line(line: &str) -> Result<(u8, u8, u8, u8), &'static str> {
    let (l, r) = line.split_once(',').unwrap();
    let ((first, second), (third, fourth)) =
        (l.split_once('-').unwrap(), r.split_once('-').unwrap());
    Ok((
        first.parse::<u8>().unwrap(),
        second.parse::<u8>().unwrap(),
        third.parse::<u8>().unwrap(),
        fourth.parse::<u8>().unwrap(),
    ))
}

pub fn part1(input: String) -> String {
    input
        .split('\n')
        .filter(|line| {
            let ranges = parse_line(line).unwrap();
            (ranges.0 >= ranges.2 && ranges.1 <= ranges.3)
                | (ranges.2 >= ranges.0 && ranges.3 <= ranges.1)
        })
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .split('\n')
        .filter(|line| {
            let ranges = parse_line(line).unwrap();
            ranges.1 >= ranges.2 && ranges.0 <= ranges.3
        })
        .count()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "2",
            part1("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string())
        );
        assert_eq!(
            "4",
            part2("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string())
        );
    }
}
