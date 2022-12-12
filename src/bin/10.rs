use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut result = 0;

    let mut tick = |x: &i32| {
        cycle += 1;
        if cycle <= 220 && (cycle - 20) % 40 == 0 {
            result += x * cycle;
        }
    };

    for instruction in parse(input) {
        match instruction {
            Instruction::Noop => tick(&x),
            Instruction::Add(n) => {
                tick(&x);
                tick(&x);
                x += n;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut x = 1;
    let mut cycle = 0;
    let mut result = String::new();

    let mut tick = |x: &i32| {
        let position: i32 = cycle % 40;
        cycle += 1;

        result.push(if position.abs_diff(*x) <= 1 { '#' } else { '.' });

        if position == 39 {
            result.push('\n');
        }
    };

    for instruction in parse(input) {
        match instruction {
            Instruction::Noop => tick(&x),
            Instruction::Add(n) => {
                tick(&x);
                tick(&x);
                x += n;
            }
        }
    }

    Some(result)
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().flat_map(|line| {
        let args = line.split_ascii_whitespace().collect_vec();
        match args[0] {
            "noop" => Some(Instruction::Noop),
            "addx" => Some(Instruction::Add(args[1].parse().ok()?)),
            _ => None,
        }
    })
}

enum Instruction {
    Noop,
    Add(i32),
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                .to_string()
            )
        );
    }
}
