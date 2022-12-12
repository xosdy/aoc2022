use std::{cmp::Reverse, collections::VecDeque};

use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse(input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                let worry_level = (monkeys[i].operation)(item) / 3;
                let next = if worry_level % monkeys[i].test_divisor == 0 {
                    monkeys[i].test_result.0
                } else {
                    monkeys[i].test_result.1
                };
                monkeys[next].items.push_back(worry_level);
            }
            monkeys[i].items.clear();
        }
    }

    let mut iter = monkeys
        .iter()
        .map(|x| x.inspection_count)
        .sorted_unstable_by_key(|&x| Reverse(x));

    Some(iter.next()? * iter.next()?)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse(input);
    let modulo: i64 = monkeys.iter().map(|x| x.test_divisor).product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                let worry_level = (monkeys[i].operation)(item) % modulo;
                let next = if worry_level % monkeys[i].test_divisor == 0 {
                    monkeys[i].test_result.0
                } else {
                    monkeys[i].test_result.1
                };
                monkeys[next].items.push_back(worry_level);
            }
            monkeys[i].items.clear();
        }
    }

    let mut iter = monkeys
        .iter()
        .map(|x| x.inspection_count)
        .sorted_unstable_by_key(|&x| Reverse(x));

    Some(iter.next()? * iter.next()?)
}

fn parse(input: &str) -> Vec<Monkey> {
    let re = Regex::new(r"Monkey (\d+):\s*Starting items: (.*)\s*Operation: new = old (.) (\d+|old)\n\s*Test: divisible by (\d+)\s*If true: throw to monkey (\d+)\s*If false: throw to monkey (\d)").unwrap();
    re.captures_iter(input)
        .flat_map(|cap| {
            Some(Monkey {
                items: cap[2]
                    .split(", ")
                    .flat_map(|x| x.parse::<i64>().ok())
                    .collect(),
                operation: match (&cap[3], &cap[4]) {
                    ("+", "old") => Box::new(|x| x + x),
                    ("+", n) => {
                        let n: i64 = n.parse().ok()?;
                        Box::new(move |x| x + n)
                    }
                    ("*", "old") => Box::new(|x| x * x),
                    ("*", n) => {
                        let n: i64 = n.parse().ok()?;
                        Box::new(move |x| x * n)
                    }
                    _ => unreachable!(),
                },
                test_divisor: cap[5].parse().ok()?,
                test_result: (cap[6].parse().ok()?, cap[7].parse().ok()?),
                inspection_count: 0,
            })
        })
        .collect()
}

struct Monkey {
    items: VecDeque<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test_divisor: i64,
    test_result: (usize, usize),
    inspection_count: usize,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
