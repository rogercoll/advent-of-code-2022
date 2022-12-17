use serde::Deserialize;
use std::{cmp::Ordering, iter::once};

#[derive(Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Item {
    Integer(u8),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // self is integer
            (Item::Integer(l), Item::Integer(r)) => l.cmp(r),
            (Item::Integer(_), Item::List(r)) if r.len() == 1 => self.cmp(&r[0]),
            (Item::Integer(l), Item::List(_)) => Item::List(vec![Item::Integer(*l)]).cmp(other),
            // self is list
            (Item::List(_), Item::Integer(_)) => other.cmp(self).reverse(),
            (Item::List(l), Item::List(r)) => match (l.len(), r.len()) {
                (0, 0) => Ordering::Equal,
                (0, _) => Ordering::Less,
                (_, 0) => Ordering::Greater,
                (_, _) => match l[0].cmp(&r[0]) {
                    Ordering::Equal => {
                        Item::List(l[1..].to_vec()).cmp(&Item::List(r[1..].to_vec()))
                    }
                    ordering => ordering,
                },
            },
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_pair(input: &str) -> (Item, Item) {
    let pair = input.split_once('\n').unwrap();
    (
        serde_json::from_str(pair.0).unwrap(),
        serde_json::from_str(pair.1).unwrap(),
    )
}

pub fn part1(input: String) -> String {
    input
        .split("\n\n")
        .map(parse_pair)
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b).is_le())
        .map(|(pos, _)| pos + 1)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (div_pack_1, div_pack_2) = (
        Item::List(vec![Item::Integer(2)]),
        Item::List(vec![Item::Integer(6)]),
    );

    let mut items = input
        .split("\n\n")
        .map(parse_pair)
        .flat_map(|tuple| once(tuple.0).chain(once(tuple.1)))
        .collect::<Vec<Item>>();

    items.push(div_pack_1.clone());
    items.push(div_pack_2.clone());

    items.sort();
    items
        .iter()
        .enumerate()
        .filter_map(|(pos, item)| {
            if *item == div_pack_1 || *item == div_pack_2 {
                return Some(pos + 1);
            }
            None
        })
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "13",
            part1(
                "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]"
                    .to_string()
            )
        );
        assert_eq!(
            "140",
            part2(
                "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]"
                    .to_string()
            )
        );
    }
}
