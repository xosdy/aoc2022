use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .map(|ranges| {
                if contains_range(&ranges[0], &ranges[1]) || contains_range(&ranges[1], &ranges[0])
                {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn contains_range(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.contains(range2.start()) && range1.contains(range2.end())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .map(|ranges| {
                if overlaps_range(&ranges[0], &ranges[1]) || overlaps_range(&ranges[1], &ranges[0])
                {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn overlaps_range(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.contains(range2.start()) || range1.contains(range2.end())
}

fn parse(input: &str) -> Vec<Vec<RangeInclusive<u32>>> {
    input
        .lines()
        .map(|line| line.split(',').map(|range| parse_range(range)).collect())
        .collect()
}

fn parse_range(s: &str) -> RangeInclusive<u32> {
    let (start, end) = s.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
