use std::collections::HashSet;

fn movement(direction: &str) -> (i16, i16) {
    match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Invalid directrion"),
    }
}

fn unique_movements(input: String, knots: &mut [(i16, i16)]) -> String {
    input
        .split('\n')
        .fold(HashSet::<(i16, i16)>::new(), |mut set, line| {
            let (direction, steps) = line.split_once(' ').unwrap();
            let move_to = movement(direction);
            for _ in 0..steps.parse::<i16>().unwrap() {
                knots[0].0 += move_to.0;
                knots[0].1 += move_to.1;
                for i in 1..knots.len() {
                    let diff: (i16, i16) =
                        (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);
                    if diff.0.abs() > 1 || diff.1.abs() > 1 {
                        knots[i].0 += diff.0.signum();
                        knots[i].1 += diff.1.signum();
                    }
                    set.insert(knots[knots.len() - 1]);
                }
            }
            set
        })
        .len()
        .to_string()
}

pub fn part1(input: String) -> String {
    unique_movements(input, &mut [(0, 0); 2])
}

pub fn part2(input: String) -> String {
    unique_movements(input, &mut [(0, 0); 10])
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "13",
            part1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string())
        );
        assert_eq!(
            "36",
            part2("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20".to_string())
        );
    }
}
