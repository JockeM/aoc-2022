fn parse_range(s: &str) -> (u32, u32) {
    let (x0, x1) = s.split_once('-').unwrap();
    (x0.parse::<u32>().unwrap(), x1.parse::<u32>().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(|line| line.split_once(','))
            .map(|(a, b)| (parse_range(a), parse_range(b)))
            .filter_map(|((a0, a1), (b0, b1))| {
                if (a0 >= b0 && a1 <= b1) || b0 >= a0 && b1 <= a1 {
                    Some(())
                } else {
                    None
                }
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(|line| line.split_once(','))
            .map(|(a, b)| (parse_range(a), parse_range(b)))
            .filter_map(
                |((a0, a1), (b0, b1))| {
                    if a0 <= b1 && b0 <= a1 {
                        Some(())
                    } else {
                        None
                    }
                },
            )
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
