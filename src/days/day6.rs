use std::collections::HashSet;

fn detect_packet(common_chars: usize, input: String) -> String {
    let mut pos: usize = 0;

    for i in common_chars..input.len() {
        if HashSet::<char>::from_iter(input[i - common_chars..i].chars())
            .len()
            .eq(&common_chars)
        {
            pos = i;
            break;
        }
    }

    pos.to_string()
}
pub fn part1(input: String) -> String {
    detect_packet(4, input)
}

pub fn part2(input: String) -> String {
    detect_packet(14, input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!("7", part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!("5", part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!("6", part1("nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!("10", part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()));
        assert_eq!("11", part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()));
        assert_eq!("19", part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!("23", part2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!("23", part2("nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!("29", part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()));
        assert_eq!("26", part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()));
    }
}
