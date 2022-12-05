use std::collections::HashMap;

fn parse_movements(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .split('\n')
        .map(|line| {
            let movements = line
                .split(' ')
                .map(|val| val.to_string())
                .collect::<Vec<String>>();
            (
                movements[1].parse::<usize>().unwrap(),
                movements[3].parse::<usize>().unwrap(),
                movements[5].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn parse_stacks(input: &str) -> HashMap<usize, Vec<char>> {
    let mut stacks = HashMap::<usize, Vec<char>>::new();
    input.split('\n').rev().skip(1).fold(0, |value, line| {
        line.split("")
            .enumerate()
            .filter(|cargo| cargo.1.chars().all(char::is_alphabetic) && !cargo.1.is_empty())
            .for_each(|cargo| {
                // cargos are separated every 4 chars
                let pos = (cargo.0 / 4) + 1;
                let value = cargo.1.chars().next().expect("Empty char");
                stacks.entry(pos).or_insert(vec![]).push(value);
            });
        value
    });
    stacks
}

fn compute_final_order(
    mut stacks: HashMap<usize, Vec<char>>,
    movements: Vec<(usize, usize, usize)>,
    ordered: bool,
) -> String {
    movements.iter().for_each(|mov| {
        let to_push = stacks.get_mut(&mov.1).unwrap();
        let mut values_to_push = to_push.split_off(to_push.len() - mov.0 as usize);
        ordered.then(|| values_to_push.reverse());
        stacks.get_mut(&mov.2).unwrap().append(&mut values_to_push);
    });

    let mut result = "".to_string();
    for a in 1..stacks.len() + 1 {
        result.push(stacks.get_mut(&a).unwrap().pop().unwrap());
    }

    result
}

pub fn part1(input: String) -> String {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    compute_final_order(
        parse_stacks(input_parts[0]),
        parse_movements(input_parts[1]),
        true,
    )
}

pub fn part2(input: String) -> String {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    compute_final_order(
        parse_stacks(input_parts[0]),
        parse_movements(input_parts[1]),
        false,
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "CMZ",
            part1("    [D]    \n[N] [C]    \n[Z] [M] [P] \n 1   2   3\n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string())
        );
        assert_eq!(
            "MCD",
            part2("    [D]    \n[N] [C]    \n[Z] [M] [P] \n 1   2   3\n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string())
        );
    }
}
