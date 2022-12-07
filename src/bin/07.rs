use std::collections::HashMap;

use itertools::Itertools;

fn parse(input: &str) -> HashMap<Vec<&str>, u32> {
    let (_, result) = input.lines().fold(
        (vec![], HashMap::new()),
        |(mut current_dir, mut result), line| {
            let splits = line.split(' ').collect_vec();
            match splits[..] {
                ["$", "cd", "/"] => current_dir.clear(),
                ["$", "cd", ".."] => {
                    current_dir.pop();
                }
                ["$", "cd", cd] => current_dir.push(cd),
                [num_str, _] => {
                    if let Ok(number) = num_str.parse::<u32>() {
                        let mut d = current_dir.clone();
                        result
                            .entry(d.clone())
                            .and_modify(|count| *count += number)
                            .or_insert(number);

                        while d.pop().is_some() {
                            result
                                .entry(d.clone())
                                .and_modify(|count| *count += number)
                                .or_insert(number);
                        }
                    }
                }
                _ => {}
            };

            (current_dir, result)
        },
    );
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).values().filter(|&&v| v < 100000).sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let needed = data.get(&vec![]).map(|x| x - 40_000_000).unwrap();
    data.values().filter(|&&x| x > needed).min().copied()
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
