pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .map(|x| match x {
                "A X" => 3 + 1,
                "B X" => 1,
                "C X" => 6 + 1,
                "A Y" => 6 + 2,
                "B Y" => 3 + 2,
                "C Y" => 2,
                "A Z" => 3,
                "B Z" => 6 + 3,
                "C Z" => 3 + 3,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .map(|x| match x {
                "A X" => 3,
                "A Y" => 3 + 1,
                "A Z" => 6 + 2,
                "B X" => 1,
                "B Y" => 3 + 2,
                "B Z" => 6 + 3,
                "C X" => 2,
                "C Y" => 3 + 3,
                "C Z" => 6 + 1,
                _ => 0,
            })
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
