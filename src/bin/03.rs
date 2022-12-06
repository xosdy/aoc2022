use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                let (first, last) = s.split_at(s.len() / 2);
                let first = first.chars().collect::<HashSet<_>>();
                let last = last.chars().collect::<HashSet<_>>();
                first
                    .intersection(&last)
                    .map(|&x| alphabet_to_priority(x))
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|lines| {
                lines
                    .map(|s| s.chars().collect::<HashSet<_>>())
                    .reduce(|acc, x| acc.intersection(&x).copied().collect())
                    .unwrap()
                    .into_iter()
                    .map(alphabet_to_priority)
                    .sum::<u32>()
            })
            .sum(),
    )
}

fn alphabet_to_priority(alphabet: char) -> u32 {
    if alphabet.is_ascii_lowercase() {
        alphabet as u32 - 'a' as u32 + 1
    } else {
        alphabet as u32 - 'A' as u32 + 27
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
