use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);

    Some(
        (0..grid.len())
            .flat_map(|y| {
                (0..grid[0].len())
                    .map(|x| is_visible(&grid, x, y))
                    .collect_vec()
            })
            .filter(|v| *v)
            .count(),
    )
}

fn is_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    is_edge(grid, x, y)
        || [
            *grid[y][..x].iter().max().unwrap(),
            *grid[y][x + 1..].iter().max().unwrap(),
            (0..y).map(|y| grid[y][x]).max().unwrap(),
            (y + 1..grid.len()).map(|y| grid[y][x]).max().unwrap(),
        ]
        .into_iter()
        .any(|v| grid[y][x] > v)
}

fn is_edge(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    x == 0 || x == grid.len() - 1 || y == 0 || y == grid[0].len() - 1
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);

    (0..grid.len())
        .flat_map(|y| (0..grid[0].len()).map(|x| score(&grid, x, y)).collect_vec())
        .max()
}

fn score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let cell = grid[y][x];
    [
        viewing_distance(grid[y][..x].iter().rev().copied(), cell),
        viewing_distance(grid[y][x + 1..].iter().copied(), cell),
        viewing_distance((0..y).rev().map(|y| grid[y][x]), cell),
        viewing_distance((y + 1..grid.len()).map(|y| grid[y][x]), cell),
    ]
    .iter()
    .product()
}

fn viewing_distance(iter: impl Iterator<Item = u32>, cell: u32) -> u32 {
    let mut distance = 0;
    for x in iter {
        distance += 1;

        if cell <= x {
            break;
        }
    }
    distance
}

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|row| row.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
