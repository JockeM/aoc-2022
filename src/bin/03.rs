use itertools::Itertools;
use std::collections::HashSet;

fn get_value(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                let hs: HashSet<char> = HashSet::from_iter(a.chars());
                b.chars().find(|c| hs.contains(c))
            })
            .map(|c| get_value(c))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .chunks(3)
            .into_iter()
            .filter_map(|lines| {
                lines
                    .flat_map(|l| l.chars().unique())
                    .counts()
                    .iter()
                    .find(|&(_, &v)| v == 3)
                    .map(|kv| kv.0.clone())
            })
            .map(|c| get_value(c))
            .sum::<u32>(),
    )
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
