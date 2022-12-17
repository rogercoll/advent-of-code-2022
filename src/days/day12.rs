use std::collections::{HashSet, VecDeque};

type StartingPoint = fn(char) -> bool;

fn bfs(data: &Vec<Vec<u8>>, start: (isize, isize), end: (isize, isize)) -> Option<i16> {
    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::new();
    queue.push_back((0_i16, start));

    while let Some((distance, (x, y))) = queue.pop_front() {
        for (xx, yy) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if xx >= 0
                && yy >= 0
                && xx < data.len() as isize
                && yy < data[xx as usize].len() as isize
                && data[xx as usize][yy as usize] <= data[x as usize][y as usize] + 1
                && visited.insert((xx, yy))
            {
                if (xx, yy) == end {
                    return Some(distance + 1);
                }
                queue.push_back((distance + 1, (xx, yy)));
            }
        }
    }
    None
}

fn parse_input(
    input: String,
    starting_point: StartingPoint,
) -> (Vec<Vec<u8>>, Vec<(isize, isize)>, (isize, isize)) {
    let (mut start, mut end) = (Vec::new(), (0, 0));
    let heightmap: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, cell)| {
                    if starting_point(cell) {
                        start.push((i as isize, j as isize));
                    };
                    match cell {
                        'E' => {
                            end = (i as isize, j as isize);
                            25
                        }
                        'S' => 0,
                        _ => cell as u8 - 97,
                    }
                })
                .collect()
        })
        .collect();
    (heightmap, start, end)
}

pub fn part1(input: String) -> String {
    let (heightmap, starts, end) = parse_input(input, |c| c == 'S');
    starts
        .iter()
        .map(|start| bfs(&heightmap, *start, end))
        .filter_map(|result| result)
        .min()
        .unwrap()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (heightmap, starts, end) = parse_input(input, |c| c == 'S' || c == 'a');
    starts
        .iter()
        .map(|start| bfs(&heightmap, *start, end))
        .filter_map(|result| result)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!(
            "31",
            part1("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi".to_string())
        );
        assert_eq!(
            "29",
            part2("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi".to_string())
        );
    }
}
