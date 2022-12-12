#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Vec<String>,
    divisible: i64,
    brothers: [usize; 2],
    inspected: i64,
}

fn parse_monkeys(input: String) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_data| {
            let lines: Vec<Vec<&str>> = monkey_data
                .split('\n')
                .map(|line| line.split_whitespace().collect::<Vec<&str>>())
                .collect();
            Monkey {
                items: lines[1]
                    .iter()
                    .skip(2)
                    .map(|num| num.strip_suffix(',').unwrap_or(num).parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
                operation: Vec::from([
                    lines[2][3].to_string(),
                    lines[2][4].to_string(),
                    lines[2][5].to_string(),
                ]),
                divisible: (lines[3][3]).parse::<i64>().unwrap(),
                brothers: [
                    (lines[4][5]).parse::<usize>().unwrap(),
                    (lines[5][5]).parse::<usize>().unwrap(),
                ],
                inspected: 0,
            }
        })
        .collect::<Vec<Monkey>>()
}

pub fn part1(input: String) -> String {
    let mut monkeys = parse_monkeys(input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.clone() {
                let mut worry_level = match monkeys[i].operation[1].as_bytes() {
                    b"+" => item + monkeys[i].operation[2].parse::<i64>().unwrap_or(item),
                    b"*" => item * monkeys[i].operation[2].parse::<i64>().unwrap_or(item),
                    _ => panic!("Invalid operation"),
                };
                worry_level /= 3;
                let brothers = monkeys[i].brothers;
                match worry_level % monkeys[i].divisible {
                    0 => monkeys[brothers[0]].items.push(worry_level),
                    _ => monkeys[brothers[1]].items.push(worry_level),
                }
                monkeys[i].inspected += 1;
            }
            monkeys[i].items.clear();
        }
    }
    let maxs = monkeys.iter().fold([0_i64, 0_i64], |mut maxs, monkey| {
        if monkey.inspected > maxs[0] {
            maxs[0] = monkey.inspected;
            maxs.sort();
        }
        maxs
    });
    (maxs[0] * maxs[1]).to_string()
}

pub fn part2(input: String) -> String {
    let mut monkeys = parse_monkeys(input);
    let common: i64 = monkeys.iter().map(|monkey| monkey.divisible).product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.clone() {
                let mut worry_level = match monkeys[i].operation[1].as_bytes() {
                    b"+" => item + monkeys[i].operation[2].parse::<i64>().unwrap_or(item),
                    b"*" => item * monkeys[i].operation[2].parse::<i64>().unwrap_or(item),
                    _ => panic!("Invalid operation"),
                };
                worry_level %= common;
                let brothers = monkeys[i].brothers;
                match worry_level % monkeys[i].divisible {
                    0 => monkeys[brothers[0]].items.push(worry_level),
                    _ => monkeys[brothers[1]].items.push(worry_level),
                }
                monkeys[i].inspected += 1;
            }
            monkeys[i].items.clear();
        }
    }
    let maxs = monkeys.iter().fold([0_i64, 0_i64], |mut maxs, monkey| {
        if monkey.inspected > maxs[0] {
            maxs[0] = monkey.inspected;
            maxs.sort();
        }
        maxs
    });
    (maxs[0] * maxs[1]).to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "10605",
            part1(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
                    .to_string()
            )
        );
        assert_eq!(
            "2713310158",
            part2(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
                    .to_string()
            )
        );
    }
}
