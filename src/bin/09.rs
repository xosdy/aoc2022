use std::collections::HashSet;

use nalgebra::{Point2, Vector2};

pub fn part_one(input: &str) -> Option<usize> {
    Some(tail_visited_count(&parse(input), 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(tail_visited_count(&parse(input), 10))
}

fn tail_visited_count(movements: &Vec<Movement>, knot: usize) -> usize {
    let mut rope = vec![Point2::origin(); knot];
    let mut tail_visited = HashSet::new();
    tail_visited.insert(rope.last().unwrap().clone());

    for &(direction, n) in movements {
        for _ in 0..n {
            rope[0] += direction;
            for i in 0..knot - 1 {
                rope[i + 1] = tail_step(&rope[i], &rope[i + 1]);
            }
            tail_visited.insert(rope.last().unwrap().clone());
        }
    }

    tail_visited.len()
}

fn tail_step(head: &Point2<i32>, tail: &Point2<i32>) -> Point2<i32> {
    let delta = head - tail;
    if delta.x.abs() <= 1 && delta.y.abs() <= 1 {
        *tail
    } else if delta.x.abs() > delta.y.abs() {
        Point2::new(head.x - delta.x.signum(), head.y)
    } else if delta.x.abs() < delta.y.abs() {
        Point2::new(head.x, head.y - delta.y.signum())
    } else {
        Point2::new(head.x - delta.x.signum(), head.y - delta.y.signum())
    }
}

type Movement = (Vector2<i32>, i32);

fn parse(input: &str) -> Vec<Movement> {
    input
        .lines()
        .flat_map(|line| {
            line.split_once(' ')
                .and_then(|(direction, n)| match (direction, n.parse().ok()?) {
                    ("U", n) => Some((Vector2::y(), n)),
                    ("D", n) => Some((-Vector2::y(), n)),
                    ("L", n) => Some((-Vector2::x(), n)),
                    ("R", n) => Some((Vector2::x(), n)),
                    _ => None,
                })
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("src/examples/09-2.txt").unwrap();
        assert_eq!(part_two(&input), Some(36));
    }
}
