use itertools::Itertools;
use std::vec;

type Move = (usize, usize, usize);

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (stacks_data, moves) = input.split_once("\n\n").unwrap();

    let stacks = stacks_data
        .split('\n')
        .rev()
        .skip(1)
        .flat_map(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|mut x| x.nth(1).unwrap_or(' '))
                .enumerate()
                .filter(|&(_, c)| c != ' ')
                .collect::<Vec<_>>()
        })
        .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b))
        .group_by(|&(i, _)| i)
        .into_iter()
        .map(|(_, v)| v.map(|x| x.1).collect())
        .collect_vec();

    let moves = moves
        .split('\n')
        .filter_map(|c| {
            c.split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect_tuple::<Move>()
        })
        .collect_vec();

    (stacks, moves)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);

    for (amount, from, to) in moves {
        for _ in 0..amount {
            if let Some(temp) = stacks[from - 1].pop() {
                stacks[to - 1].push(temp);
            }
        }
    }

    Some(
        stacks
            .into_iter()
            .filter_map(|x| x.into_iter().last())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);

    for (amount, from, to) in moves {
        let mut v: Vec<char> = vec![];
        for _ in 0..amount {
            v.push(stacks[from - 1].pop().unwrap());
        }
        for temp in v.into_iter().rev() {
            stacks[to - 1].push(temp);
        }
    }

    Some(
        stacks
            .into_iter()
            .filter_map(|x| x.into_iter().last())
            .collect(),
    )
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
        assert_eq!(part_two(&input), None);
    }
}
