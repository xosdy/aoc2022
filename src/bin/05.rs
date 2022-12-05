use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    let mut data = parse(input);
    for proc in &data.procedures {
        let from_len = data.stacks[proc.from].len();
        let mut tmp = data.stacks[proc.from].split_off(from_len - proc.count);
        tmp.reverse();

        data.stacks[proc.to].extend(tmp);
    }

    Some(
        data.stacks
            .iter()
            .take_while(|x| !x.is_empty())
            .map(|x| x.last().unwrap())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut data = parse(input);
    for proc in &data.procedures {
        let from_len = data.stacks[proc.from].len();
        let tmp = data.stacks[proc.from].split_off(from_len - proc.count);
        data.stacks[proc.to].extend(tmp);
    }

    Some(
        data.stacks
            .iter()
            .take_while(|x| !x.is_empty())
            .map(|x| x.last().unwrap())
            .collect(),
    )
}

fn parse(input: &str) -> Data {
    let (stacks_str, procedures_str) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = vec![Default::default(); 9];
    for line in stacks_str.lines().filter(|s| !s.starts_with(" 1")) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].insert(0, c);
            }
        }
    }

    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    let procedures = re
        .captures_iter(procedures_str)
        .map(|cap| Procedure {
            count: cap[1].parse().unwrap(),
            from: cap[2].parse::<usize>().unwrap() - 1,
            to: cap[3].parse::<usize>().unwrap() - 1,
        })
        .collect();

    Data { stacks, procedures }
}

#[derive(Debug)]
struct Data {
    stacks: Vec<Vec<char>>,
    procedures: Vec<Procedure>,
}

#[derive(Debug)]
struct Procedure {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
