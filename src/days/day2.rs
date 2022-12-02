use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(data: u8) -> Result<Self, &'static str> {
        match data {
            b'A' | b'X' => Ok(Self::Rock),
            b'B' | b'Y' => Ok(Self::Paper),
            b'C' | b'Z' => Ok(Self::Scissors),
            _ => Err("invalid shape"),
        }
    }
    fn points(self) -> i32 {
        match self {
            Shape::Scissors => 3,
            Shape::Rock => 1,
            Shape::Paper => 2,
        }
    }
    fn battle(self, oponent: Shape) -> i32 {
        match self.cmp(&oponent) {
            Ordering::Greater => 6 + self.points(),
            Ordering::Equal => 3 + self.points(),
            Ordering::Less => self.points(),
        }
    }
    fn winning_shape(&self) -> Self {
        match self {
            Shape::Scissors => Shape::Paper,
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
        }
    }
    fn bribed_battle(self, data: u8) -> Result<i32, &'static str> {
        let oponent = match data {
            b'X' => self.winning_shape(),
            b'Y' => self.clone(),
            b'Z' => self.winning_shape().winning_shape(),
            _ => return Err("invalid shape"),
        };
        Ok(oponent.battle(self))
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Shape) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Shape) -> Ordering {
        match (self, other) {
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => Ordering::Less,
            (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper)
            | (Shape::Rock, Shape::Scissors) => Ordering::Greater,
            (_, _) => Ordering::Equal,
        }
    }
}

fn parse_line(line: &str) -> Result<(Shape, u8), &'static str> {
    match line.as_bytes() {
        &[elf, b' ', me] => Ok((Shape::parse(elf)?, me)),
        _ => Err("invalid line"),
    }
}

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (elf, me) = parse_line(line).unwrap();
            Shape::parse(me).unwrap().battle(elf)
        })
        .sum::<i32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (elf, me) = parse_line(line).unwrap();
            elf.bribed_battle(me).unwrap()
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!("15", part1("A Y\nB X\nC Z".to_string()));
        assert_eq!("12", part2("A Y\nB X\nC Z".to_string()));
    }
}
