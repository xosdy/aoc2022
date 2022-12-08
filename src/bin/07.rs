pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(&mut input.lines())
            .into_iter()
            .filter(|&x| x < 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sizes = parse(&mut input.lines());
    let need = 30000000 - (70000000 - sizes.last().unwrap());
    sizes.sort_unstable();
    sizes.into_iter().find(|&x| x > need)
}

fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let mut result = vec![];
    let mut total = 0;
    loop {
        match input.next() {
            Some("$ cd ..") | None => break,
            Some(s) if s.starts_with("$ cd") => {
                result.append(&mut parse(input));
                total += result.last().unwrap()
            }
            Some(s) => {
                if let Some(size) = s
                    .split_once(' ')
                    .and_then(|(maybe_size, _)| maybe_size.parse::<u64>().ok())
                {
                    total += size;
                }
            }
        }
    }

    result.push(total);
    result
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
