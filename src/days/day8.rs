fn read_grid(input: String) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

pub fn part1(input: String) -> String {
    let grid = read_grid(input);
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(col, value)| {
                    // edge
                    ((*col == 0) || (*col == grid.len() - 1) || (i == 0) || (i == grid.len() - 1))
                        || !(0..i).any(|up| grid[up][*col] >= **value)
                        || !(0..*col).any(|left| grid[i][left] >= **value)
                        || !(i + 1..grid.len()).any(|down| grid[down][*col] >= **value)
                        || !(*col + 1..grid.len()).any(|right| grid[i][right] >= **value)
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn compute_views_horizontal(j: usize, row: usize, value: u32, grid: &[Vec<u32>]) -> usize {
    ((1..j)
        .map(|jj| grid[row][j - jj])
        .position(|h| h >= value)
        .unwrap_or_else(|| j.wrapping_sub(1))
        .wrapping_add(1))
        * ((j + 1..grid.len())
            .map(|j| grid[row][j])
            .position(|h| h >= value)
            .unwrap_or_else(|| (grid.len() - j).wrapping_sub(2))
            .wrapping_add(1))
}

fn compute_views_vertical(i: usize, col: usize, value: u32, grid: &[Vec<u32>]) -> usize {
    let y = i;
    ((1..y)
        .map(|yy| grid[y - yy][col])
        .position(|h| h >= value)
        .unwrap_or_else(|| y.wrapping_sub(1))
        .wrapping_add(1))
        * ((y + 1..grid.len())
            .map(|y| grid[y][col])
            .position(|h| h >= value)
            .unwrap_or_else(|| (grid.len() - y).wrapping_sub(2))
            .wrapping_add(1))
}

pub fn part2(input: String) -> String {
    let grid = read_grid(input);
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(col, value)| {
                    if *value <= 1 {
                        return 0;
                    }
                    compute_views_vertical(i, col, *value, &grid)
                        * compute_views_horizontal(col, i, *value, &grid)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        assert_eq!("21", part1("30373\n25512\n65332\n33549\n35390".to_string()));
        assert_eq!("8", part2("30373\n25512\n65332\n33549\n35390".to_string()));
    }
}
