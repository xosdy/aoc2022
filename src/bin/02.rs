use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ParseError),
        }
    }
}

struct Round1 {
    opponent: Shape,
    me: Shape,
}

impl Round1 {
    fn score(&self) -> u32 {
        let outcome = match (self.opponent, self.me) {
            (x, y) if x == y => 3,
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => 6,
            (_, _) => 0,
        };

        self.me as u32 + outcome
    }
}

impl FromStr for Round1 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace().map(|x| x.parse().unwrap());

        Ok(Round1 {
            opponent: iter.next().unwrap(),
            me: iter.next().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<Round1>().unwrap().score())
            .sum(),
    )
}

enum End {
    Lose,
    Draw,
    Win,
}

impl End {
    fn score(&self) -> u32 {
        match self {
            End::Lose => 0,
            End::Draw => 3,
            End::Win => 6,
        }
    }
}

impl FromStr for End {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(End::Lose),
            "Y" => Ok(End::Draw),
            "Z" => Ok(End::Win),
            _ => Err(ParseError),
        }
    }
}

struct Round2 {
    opponent: Shape,
    end: End,
}

impl Round2 {
    fn score(&self) -> u32 {
        let shape = match self.end {
            End::Lose => match self.opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            End::Win => match self.opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            End::Draw => self.opponent,
        };

        shape as u32 + self.end.score()
    }
}

impl FromStr for Round2 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();

        Ok(Round2 {
            opponent: iter.next().unwrap().parse().unwrap(),
            end: iter.next().unwrap().parse().unwrap(),
        })
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<Round2>().unwrap().score())
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
