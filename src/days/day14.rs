use itertools::Itertools;

use nom::{
    character::{complete::digit1, streaming::char},
    combinator::map_opt,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const W: usize = 675;
const H: usize = 175;

pub fn parse_numbers(input: &str) -> IResult<&str, usize> {
    map_opt(digit1, |s: &str| s.parse::<usize>().ok())(input)
}

fn parse_line(input: &str) -> Vec<(usize, usize)> {
    let coord = separated_pair(parse_numbers, char(','), parse_numbers);
    separated_list1(nom::bytes::complete::tag(" -> "), coord)(input)
        .unwrap()
        .1
}

fn sand_pour(cavern: [[bool; W]; H], (mut x, y): (usize, usize)) -> Option<(usize, usize)> {
    (y + 1..H - 1)
        .find(|y| {
            [x, x - 1, x + 1]
                .into_iter()
                .find(|x| !cavern[*y + 1][*x])
                .map(|xx| x = xx)
                .is_none()
        })
        .map(|y| (x, y))
}

fn sand_pour_floor(
    cavern: [[bool; W]; H],
    (mut x, y): (usize, usize),
    lowest: usize,
) -> Option<(usize, usize)> {
    (y..H - 1)
        .find(|y| {
            *y == (lowest - 1)
                || [x, x - 1, x + 1]
                    .into_iter()
                    .find(|x| !cavern[*y + 1][*x])
                    .map(|xx| x = xx)
                    .is_none()
        })
        .map(|y| (x, y))
}

fn parse_cavern(input: String) -> ([[bool; W]; H], usize) {
    let mut lowest = 0;
    let mut cavern = [[false; W]; H];
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .for_each(|positions| {
            positions
                .into_iter()
                .tuple_windows::<((usize, usize), (usize, usize))>()
                .for_each(|(ini, fin)| {
                    lowest = lowest.max(ini.1.max(fin.1));
                    if ini.0 == fin.0 {
                        (ini.1.min(fin.1)..=ini.1.max(fin.1)).for_each(|i| cavern[i][ini.0] = true);
                    } else {
                        (ini.0.min(fin.0)..=ini.0.max(fin.0)).for_each(|j| cavern[ini.1][j] = true);
                    }
                })
        });
    (cavern, lowest)
}

pub fn part1(input: String) -> String {
    let ((mut cavern, _), mut sand) = (parse_cavern(input), 0);
    while let Some((x, y)) = sand_pour(cavern, (500, 0)) {
        cavern[y][x] = true;
        sand += 1;
    }

    sand.to_string()
}

pub fn part2(input: String) -> String {
    let ((mut cavern, floor), mut sand) = (parse_cavern(input), 0);

    while let Some((x, y)) = sand_pour_floor(cavern, (500, 0), floor + 2) {
        cavern[y][x] = true;
        sand += 1;
        if y == 0 {
            break;
        }
    }

    sand.to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "24",
            part1("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string())
        );
        assert_eq!(
            "93",
            part2("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string())
        );
    }
}
