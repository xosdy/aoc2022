use std::cmp::Reverse;

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = parse(input);
    calories.sort_by_key(|&x| Reverse(x));
    Some(calories.into_iter().take(3).sum())
}

pub fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
