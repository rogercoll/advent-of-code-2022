pub fn part1(input: String) -> String {
    input
        .split('\n')
        .map(|ruckstack| {
            ruckstack
                .chars()
                .enumerate()
                .fold(([0_u8; 57], 0_usize), |mut bitmap, item| {
                    let mut pos: usize = if item.1.is_uppercase() { 26 } else { 0 };
                    pos += item.1.to_digit(36).unwrap() as usize - 10;
                    if bitmap.0[pos] >= 1 && (item.0 >= ruckstack.len() / 2) {
                        bitmap.1 = pos + 1
                    } else if item.0 < ruckstack.len() / 2 {
                        bitmap.0[pos] += 1;
                    }
                    bitmap
                })
                .1
        })
        .sum::<usize>()
        .to_string()
}

fn get_bitmap(ruckstack: &str) -> u64 {
    ruckstack
        .chars()
        .enumerate()
        .fold(0_u64, |mut bitmap, item| {
            let mut pos: usize = if item.1.is_uppercase() { 26 } else { 0 };
            pos += item.1.to_digit(36).unwrap() as usize - 10;
            bitmap |= 1 << pos;
            bitmap
        })
}

pub fn part2(input: String) -> String {
    input
        .split('\n')
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| {
            let bitmap = get_bitmap(chunk[0]) & get_bitmap(chunk[1]) & get_bitmap(chunk[2]);
            // returns the LO bit set to 1
            bitmap.trailing_zeros() + 1
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "157",
            part1("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string())
        );
        assert_eq!(
            "70",
            part2("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string())
        );
    }
}
